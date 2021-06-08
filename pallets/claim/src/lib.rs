#![cfg_attr(not(feature = "std"), no_std)]

/// Modules to claim move balances into the "substrate" blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, weights::Pays};
use frame_system::{ensure_root,ensure_signed};
use sp_std::prelude::*;
use core::str;
use sha2::{Sha256, Digest};
use base64::decode;

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
        #[weight = (1000, Pays::No)]
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
        #[weight = (1000, Pays::No)]
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
			ensure!(verify_signature_ecdsa_message(oldaddress.clone(),signature.clone(),oldpublickey.clone())==true,Error::<T>::WrongSignature);
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
// the "message" should the clear message signed (can be ascii data or binary)
// signature should encoded in DER binary format and then encoded in base64
// public key should be the raw public key of 64 bytes encoded in base64
fn verify_signature_ecdsa_message(message:Vec<u8>,signature:Vec<u8>,publickey:Vec<u8>) -> bool {   
    // compute sha256 of the message
    let mut hasher = Sha256::new();
    // write the vector message to sha256 object
    hasher.update(message);
    // get sha256 result
    let hash = hasher.finalize().to_vec();
    // convert to a static bytes array of 32 bytes
    let mut hashmessage:[u8;32] = [0; 32];
    let mut x=0;
    for b in hash { 
        hashmessage[x]=b;
        x=x+1;
        if x>31{
            break;
        }
    }
    // convert the message hash in Message Structure
    let ctx_message = secp256k1::Message::parse(&hashmessage);
    // encoding public key in secp256k1::Publickey
    let publickeyb=&decode(publickey).unwrap();
    let mut publickeybin: [u8;64]=[0;64];
    let mut x=0;
    for b in publickeyb { 
        publickeybin[x]=*b;
        x=x+1;
        if x>63{
            break;
        }
    }
    let ctx_publickey=secp256k1::PublicKey::parse_slice(&publickeybin, Some(secp256k1::PublicKeyFormat::Raw)).unwrap();
    // end encoding public key  */
    // encoding signature in secp256k1::Signature
    // decode signature from base64
    let signatureb=&decode(signature).unwrap();
    // load signature from "der" encoding
    let ctx_signature = secp256k1::Signature::parse_der(&signatureb).unwrap();
    // verify the signature
    secp256k1::verify(&ctx_message,&ctx_signature,&ctx_publickey)

}


