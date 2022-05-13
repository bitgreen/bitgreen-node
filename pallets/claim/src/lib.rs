#![cfg_attr(not(feature = "std"), no_std)]
/// Modules to claim move balances into the "substrate" blockchain
pub use pallet::*;

// SBP M1 review: missing documentation, tests & benchmarks.

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use base64::decode;
use core::str;
use frame_support::{ensure, traits::Currency, weights::Pays};
use frame_system::{ensure_root, ensure_signed};
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};
use sp_std::prelude::*;

pub type Balance = u128;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // we use a safe crypto hashing with blake2_128
    // Keeps the address of the previous blockchain and the balance at the swapping block number
    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
    pub type BalanceClaim<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Balance>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        DepositClaimAccepted(T::AccountId, Vec<u8>, Balance),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
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

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Store a new deposit, used for the genesis of the blockchain (no gas fees are charged because it's part of the genesis)
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn new_deposit(
            origin: OriginFor<T>,
            address: Vec<u8>,
            deposit: Balance,
        ) -> DispatchResultWithPostInfo {
            // check the request is signed from Super User only
            let _sender = ensure_root(origin)?;
            //check address length
            ensure!(address.len() == 34, Error::<T>::WrongAddressLength);
            // check the balance is > 0
            ensure!(deposit > 0, Error::<T>::DepositCannotBeZero);
            // check that the address is not already present
            ensure!(
                BalanceClaim::<T>::contains_key(&address) == false,
                Error::<T>::DuplicatedAddress
            );
            // Update deposit
            BalanceClaim::<T>::insert(address, deposit);
            // we do not emit events for this call because this call is used at the Genesis only.
            // Return a successful DispatchResult
            Ok(Pays::No.into())
        }
        /// Claim a deposit from the old blockchain (no gas fees are charged to allow the usage of "proxy" account with minimum balance)
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn claim_deposit(
            origin: OriginFor<T>,
            oldaddress: Vec<u8>,
            oldpublickey: Vec<u8>,
            signature: Vec<u8>,
            recipient: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // check the request is signed
            let _sender = ensure_signed(origin)?;
            //check old address length
            ensure!(oldaddress.len() == 34, Error::<T>::WrongAddressLength);
            //check public key length
            ensure!(oldpublickey.len() >= 64, Error::<T>::WrongPublicKeyLength);
            // check that the old address is already present in the chain
            ensure!(
                BalanceClaim::<T>::contains_key(&oldaddress) == true,
                Error::<T>::OldAddressNotfound
            );
            // check signature from oldchain
            ensure!(
                Self::verify_signature_ecdsa_address(
                    oldaddress.clone(),
                    signature.clone(),
                    oldpublickey.clone()
                ) == true,
                Error::<T>::WrongSignature
            );
            // get amount of the deposit and burn it, in case of error we set 0
            let deposit: Balance = match BalanceClaim::<T>::take(&oldaddress) {
                Some(d) => d,
                None => 0,
            };
            ensure!(deposit > 0, Error::<T>::DepositCannotBeZero);

            // mint deposit in the new chain (it creates the account if it's not on chain)
            let _result = T::Currency::deposit_creating(&recipient, (deposit as u32).into());
            // Generate event
            Self::deposit_event(Event::DepositClaimAccepted(recipient, oldaddress, deposit));
            // Return a successful DispatchResult
            Ok(Pays::No.into())
        }
    }

    impl<T: Config> Pallet<T> {
        // Function to verify signature on an old address
        // the "oldaddress" should the address that has been signed and matching the deposit claim
        // signature should encoded in DER binary format and then encoded in base64
        // public key should be the raw public key of 64 bytes encoded in base64
        pub fn verify_signature_ecdsa_address(
            oldaddress: Vec<u8>,
            signature: Vec<u8>,
            publickey: Vec<u8>,
        ) -> bool {
            // compute sha256 of the message
            let mut hasher = Sha256::new();
            // write the vector message to sha256 object
            hasher.update(oldaddress.clone());
            // get sha256 result
            let hash = hasher.finalize().to_vec();
            // convert to a static bytes array of 32 bytes
            let mut hashmessage: [u8; 32] = [0; 32];
            let mut x = 0;
            for b in hash {
                hashmessage[x] = b;
                x = x + 1;
                if x > 31 {
                    break;
                }
            }
            // convert the message hash in Message Structure
            let ctx_message = secp256k1::Message::parse(&hashmessage);
            // encoding public key in secp256k1::Publickey
            let publickeyb = &decode(publickey).unwrap();
            let mut publickeybin: [u8; 64] = [0; 64];
            let mut publickeybinx: [u8; 32] = [0; 32];
            let mut publickeybiny: [u8; 32] = [0; 32];
            let mut x = 0;
            for b in publickeyb {
                publickeybin[x] = *b;
                if x < 32 {
                    publickeybinx[x] = *b;
                }
                if x >= 32 {
                    publickeybiny[x - 32] = *b;
                }
                x = x + 1;
                if x >= 64 {
                    break;
                }
            }
            let ctx_publickey = secp256k1::PublicKey::parse_slice(
                &publickeybin.clone(),
                Some(secp256k1::PublicKeyFormat::Raw),
            )
            .unwrap();
            // end encoding public key  */
            // encoding signature in secp256k1::Signature
            // decode signature from base64
            let signatureb = &decode(signature).unwrap();
            // load signature from "der" encoding
            let ctx_signature = secp256k1::Signature::parse_der(&signatureb).unwrap();
            // verify the signature
            let result = secp256k1::verify(&ctx_message, &ctx_signature, &ctx_publickey);
            if result == false {
                return false;
            }
            // verify that the address matches the public key
            let mut hashera = Sha256::new();
            // compute the prefix
            let mut prefixpk: [u8; 1] = [2; 1];
            if publickeybiny[31] % 2 == 0 {
                prefixpk[0] = 2;
            } else {
                prefixpk[0] = 3;
            }
            // write the vector message to sha256 object
            hashera.update(&prefixpk); // add 0x04 on top
            hashera.update(publickeybinx);
            // get sha256 result
            let hasha = hashera.finalize().to_vec();
            // apply RIPEMD160 to the previous hash
            let mut hasherb = Ripemd160::new();
            hasherb.update(hasha);
            let hashdouble = hasherb.finalize().to_vec();
            // compute checksum with first 4 bytes of sha256(sha256(version+hashdouble))
            // set version type to decimal 38 (G in base58) and add the double hash in a vector
            // reference: https://github.com/bitgreenarchive/bitgreen/blob/26419cafe4556ca1e60f966a928280881b5db533/src/chainparams.cpp#L231
            let mut buffer = Vec::<u8>::new();
            buffer.push(38);
            for b in hashdouble.clone() {
                buffer.push(b);
            }
            // compute the first sha256 on (version+hashdouble)
            let mut hasher1 = Sha256::new();
            hasher1.update(buffer.clone());
            let hash1 = hasher1.finalize().to_vec();
            // compute the second sha256 on the previous hash sha256
            let mut hasher2 = Sha256::new();
            hasher2.update(hash1);
            let hash2 = hasher2.finalize().to_vec();
            //add first 4 bytes of the second hash to the buffer
            buffer.push(hash2[0]);
            buffer.push(hash2[1]);
            buffer.push(hash2[2]);
            buffer.push(hash2[3]);
            // check if the address computed is equal to the signed address
            let bs58 = bs58::encode(buffer).into_vec();
            if bs58 == oldaddress {
                return true;
            } else {
                return false;
            }
        }
    }
}
