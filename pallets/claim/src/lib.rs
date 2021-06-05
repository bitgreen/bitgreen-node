#![cfg_attr(not(feature = "std"), no_std)]

/// Modules to claim move balances into the "substrate" blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{ensure_root,ensure_signed};
use sp_std::prelude::*;
use core::str;
use sha2::{Sha256, Digest};
use base64::decode;
//use secp256k1::{PublicKey,Message,Signature};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Module configuration
pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as bitgclaim {
		// we use a safe crypto hashing with blake2_128
		// Keeps the address of the previous blockchain and the balance at the swapping block number
		Balance get(fn get_balance): map hasher(blake2_128_concat) Vec<u8> => Option<u32>;
	}
}
// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		DepositClaimAccepted(AccountId,Vec<u8>),
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Deposit cannot be zero
		DepositCannotBeZero,
		/// The address is already on chain
		DuplicatedAddress,
		/// Wrong address lenght, it must be 34 bytes
		WrongAddressLength,
		/// Wrong public key lenght, it must be 50 bytes
		WrongPublicKeyLength,
		/// Address of the old chain has not been found
		OldAddressNotfound,
		/// Wrong Signature in the claim request
		WrongSignature,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;
		/// Store a new deposit, used for the genesis of the blockchain (no gas fees are charged because it's part of the genesis)
		#[weight = 0]
		pub fn new_deposit(origin, address: Vec<u8>, deposit: u32) -> dispatch::DispatchResult {
			// check the request is signed from Super User only
			let _sender = ensure_root(origin)?;
			//check address length
			ensure!(address.len() == 34, Error::<T>::WrongAddressLength); 
			// check the balance is > 0
			ensure!(deposit > 0, Error::<T>::DepositCannotBeZero); 
			// check that the address is not already present
			ensure!(Balance::contains_key(&address)==false, Error::<T>::DuplicatedAddress);
			// Update deposit
			Balance::insert(address,deposit);
			// we do not emit events for this call because this call is used at the Genesis only.
			// Return a successful DispatchResult
			Ok(())
		}
		/// Claim a deposit from the old blockchain (no gas fees are charged to allow a claim a deposit having 0 balance)
		#[weight = 0]
		pub fn claim_deposit(origin, oldaddress: Vec<u8>,oldpublickey: Vec<u8>,signature: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed 
			let sender = ensure_signed(origin)?;
			//check old address length
			ensure!(oldaddress.len() == 34, Error::<T>::WrongAddressLength); 
			//check public key length
			ensure!(oldpublickey.len() >= 64, Error::<T>::WrongPublicKeyLength); 
			// check that the old address is already present in the chain
			ensure!(Balance::contains_key(&oldaddress)==true, Error::<T>::OldAddressNotfound);
			// check signature from oldchain
			ensure!(verify_signature_bitcoin_message(oldaddress.clone(),signature.clone(),oldpublickey.clone())==true,Error::<T>::WrongSignature);
			// Burn old chain deposit
			Balance::remove(&oldaddress);
			// TODO: Mint deposit in the new chain

			// Generate event
			Self::deposit_event(RawEvent::DepositClaimAccepted(sender,oldaddress));
			// Return a successful DispatchResult
			Ok(())
		}
	}
}
// Function to verify signature on a message
// message is a clear text to sign
// signature should be encoded in Base64
// public key is expected in hex decimal without 0x on top
fn verify_signature_bitcoin_message(message:Vec<u8>,signature:Vec<u8>,publickey:Vec<u8>)->bool {
    // encoding public key in secp256k1::Publickey
    let publickeyv: Vec<u8> = hex::decode(publickey).unwrap();
    let mut publickeybin: [u8;33]=[0;33];
    let mut x=0;
    for b in publickeyv { 
        publickeybin[x]=b;
        x=x+1;
        if x>32{
            break;
        }
    }
    let ctx_publickey=secp256k1::PublicKey::parse_compressed(&publickeybin).unwrap();
    // end encoding public key 
    // encoding signature in secp256k1::Signature
    // decode signature from base64
    let signatureb=&decode(signature).unwrap();
    // convert the signatureb to 64 bytes from 65 bytes ignoring the first byte added from Bitcoin/Bitgreen runtime
    let mut signaturebr:[u8;64] = [0; 64];
    x=0;
    for b in signatureb { 
        // jump first byte
        if  x==0 {
            x=x+1;
            continue;
        }
        signaturebr[x-1]=*b;
        x=x+1;
        if x>63{
            break;
        }
    }
    //load the signature in "Signature" object
    let ctx_signature = secp256k1::Signature::parse(&signaturebr);
    // end encoding signature in secp256k1::Signature
    // encoding message in secp256k1::Message
    let hashmessage: Vec<u8> = hash_msg_bitcoin(message);
    // convert to a static bytes array of 32 bytes
    let mut hashmessagestatic:[u8;32] = [0; 32];
    let mut x=0;
    for b in hashmessage { 
        hashmessagestatic[x]=b;
        x=x+1;
        if x>31{
            break;
        }
    }
    // convert the message hash in Message Structure
    let ctx_message = secp256k1::Message::parse(&hashmessagestatic);
	// returns true/false as verification result
    secp256k1::verify(&ctx_message,&ctx_signature,&ctx_publickey)
    
}
// Function to hash a message in Bitcon/Bitgreen standard
// the maximum length of the message is 252 bytes 
// if you need to extend the maximum lenght, you should implement this:
// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
fn hash_msg_bitcoin(msg:Vec<u8>) -> Vec<u8> {
    let prepend=b"Bitcoin Signed Message:\n";
    //allocate a new vector
    let mut msgbitcoin:Vec<u8> = Vec::new();
    // add 0x18 as first byte
    msgbitcoin.push(0x18);
    //add the prepend constant
    for i in prepend{
        msgbitcoin.push(*i);   
    }
    //add the length of the message (maximum 252 bytes) 
    let lenmsg=msg.len() as u8;
    msgbitcoin.push(lenmsg);
    //add the message body
    for x in msg {
        msgbitcoin.push(x);
    }
    //make a first sha256
    // create a Sha256 object
    let mut hasher1 = Sha256::new();
    // write the vector created to sha256 object
    hasher1.update(msgbitcoin);
    // get sha256 result
    let result = hasher1.finalize();
    // create a second Sha256 object
    let mut hasher2 = Sha256::new();
    // write the previous sha256 to the new sha256 object
    hasher2.update(result);
    // get last sha256 result
    let hashresult = hasher2.finalize();
    // return the sha256 in a Vec<u8>
    return hashresult.to_vec();
}

