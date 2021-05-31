#![cfg_attr(not(feature = "std"), no_std)]

/// Modules to claim move balances into the "substrate" blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{ensure_root,ensure_signer};
use sp_std::prelude::*;
use core::str;

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
		// Address of the old chain has not been found
		OldAddressNotfound,
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
			let _sender = ensure_signer(origin)?;
			//check old address length
			ensure!(oldaddress.len() == 34, Error::<T>::WrongAddressLength); 
			//check public key length
			ensure!(oldpublickey.len() >= 50, Error::<T>::WrongPublicKeyLength); 
			// check that the old address is already present in the chain
			ensure!(Balance::contains_key(&oldaddress)==true, Error::<T>::OldAddressNotfound);
			// TODO: check signature from oldchain

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
