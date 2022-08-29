#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type GeneralStorage<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat,Vec<u8>, Vec<u8>>;



	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewDataStored(Vec<u8>, T::AccountId),
		ErasedData(Vec<u8>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The json data must long at the least 1 char
		TooShortJsonDataLength,
		/// The json datac cannot be longer of 8192 chars
		TooLongJsonDataLength,
		/// The key must long at the least 1 char
		TooShortKeyLength,
		/// The json datac cannot be longer of 128 chars
		TooLongKeyLength,
		/// The select data cannot be found on chain
		StorageDataNotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// function to store general data, typically a json string, by the way any kind of data is accepted with a limit to 8192 bytes and 128 bytes for the key
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn storedata(origin: OriginFor<T>, key: Vec<u8>,jsondata: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;
			// check the size of the data
			ensure!(
                jsondata.len() >= 1,
                Error::<T>::TooShortJsonDataLength
            );
            ensure!(
                jsondata.len() < 8192,
                Error::<T>::TooLongJsonDataLength
            );
			// check the size of the key
			ensure!(
                key.len() >= 1,
                Error::<T>::TooShortKeyLength
            );
            ensure!(
                key.len() < 128,
                Error::<T>::TooShortKeyLength
            );
			// Update storage.
			GeneralStorage::<T>::insert(who.clone(),key.clone(), jsondata);
			// Emit an event.
			Self::deposit_event(Event::NewDataStored(key, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		// function to erase data, only the original signer can do it
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn erasedata(origin: OriginFor<T>, key: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;
			// check the keys are present
			ensure!(
                GeneralStorage::<T>::contains_key(who.clone(),key.clone()) == false,
                Error::<T>::StorageDataNotFound
            );
			// Erase the storage.
			GeneralStorage::<T>::take(who.clone(), key.clone());
			// Emit an event.
			Self::deposit_event(Event::ErasedData(key, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
