#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::traits::Zero,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]

	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// Currency config - this is what the pallet accepts as payment
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Maximum length of key
		type MaxKeyLength: Get<u32>;
		/// Maximum length of value
		type MaxValueLength: Get<u32>;
		/// The deposit charged to store data
		type DepositPerByte: Get<CurrencyBalanceOf<Self>>;
	}

	// -- Types for representing key and value in pallet -- //
	/// Type for short strings an descriptions
	pub type KeyOf<T> = BoundedVec<u8, <T as Config>::MaxKeyLength>;

	/// Type for longer strings and descriptions
	pub type ValueOf<T> = BoundedVec<u8, <T as Config>::MaxValueLength>;

	pub type CurrencyBalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type StoredData<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, KeyOf<T>, ValueOf<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new data has been stored on chain
		DataStored { key: KeyOf<T>, who: T::AccountId },
		/// An existing data has been removed
		DataCleared { key: KeyOf<T>, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoDataStored,
		/// cannot pass empty key
		EmptyInput,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn store_data(
			origin: OriginFor<T>,
			key: KeyOf<T>,
			value: ValueOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!key.len().is_zero() && !value.len().is_zero(), Error::<T>::EmptyInput);

			// calculate deposit fee
			let deposit_amount = T::DepositPerByte::get() * (value.len() as u32).into();

			// reserve deposit fee
			T::Currency::reserve(&who, deposit_amount)?;

			// Update storage.
			<StoredData<T>>::insert(who.clone(), key.clone(), value);

			// Emit an event.
			Self::deposit_event(Event::DataStored { key, who });

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn clear_data(origin: OriginFor<T>, key: KeyOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!key.len().is_zero(), Error::<T>::EmptyInput);

			let data = <StoredData<T>>::get(who.clone(), key.clone());

			// does some data exist
			ensure!(data.is_some(), Error::<T>::NoDataStored);

			// calculate deposit fee
			let deposit_amount =
				T::DepositPerByte::get() * (data.expect("already checked!").len() as u32).into();

			// reserve deposit fee
			T::Currency::unreserve(&who, deposit_amount);

			// Update storage.
			<StoredData<T>>::take(who.clone(), key.clone());

			// Emit an event.
			Self::deposit_event(Event::DataCleared { key, who });

			Ok(())
		}
	}
}
