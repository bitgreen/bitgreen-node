#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::traits::Currency;
use frame_support::{
	ensure, pallet_prelude::DispatchResult, sp_runtime::traits::AccountIdConversion, traits::Get,
};
use sp_runtime::{
	traits::{CheckedAdd, CheckedSub},
	ArithmeticError,
};

mod functions;
pub use functions::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::convert::TryInto;

	/// The data stored for every contract
	#[derive(
		Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, PartialOrd, Ord, Debug,
	)]
	pub struct ContractDetail<Time, Balance> {
		/// The time after which the contract can be paid out
		pub expiry: Time,
		/// The amount paid out at expiry
		pub amount: Balance,
	}

	/// The input data for bulk adding contracts
	#[derive(
		Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, PartialOrd, Ord, Debug,
	)]
	pub struct BulkContractInput<AccountId, Time, Balance> {
		/// The recipient of the contract amount
		pub recipient: AccountId,
		/// The time after which the contract can be paid out
		pub expiry: Time,
		/// The amount paid out at expiry
		pub amount: Balance,
	}

	/// Pallet version of Contract Detail
	pub type ContractDetailOf<T> =
		ContractDetail<<T as frame_system::Config>::BlockNumber, BalanceOf<T>>;

	/// Pallet version of BulkContractInput
	pub type BlockContractInputOf<T> = BulkContractInput<
		<T as frame_system::Config>::AccountId,
		<T as frame_system::Config>::BlockNumber,
		BalanceOf<T>,
	>;

	/// List of BulkContractInput
	pub type BulkContractInputs<T> =
		BoundedVec<BlockContractInputOf<T>, <T as Config>::MaxContractInputLength>;

	/// List of accountIds to be used for bulk_remove
	pub type BulkContractRemove<T> =
		BoundedVec<<T as frame_system::Config>::AccountId, <T as Config>::MaxContractInputLength>;

	/// Pallet version of balance
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The origin with force set priviliges
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// Maximum length of contract input length
		type MaxContractInputLength: Get<u32>;

		/// Maximum length of contract expiry in a block
		type MaxContractExpiryInABlock: Get<u32>;

		/// The Vesting Contract pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn vesting_contracts)]
	pub(super) type VestingContracts<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ContractDetailOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn vesting_balance)]
	// Current vesting balance held by the pallet account. This value is stored
	// to quickly lookup the amount currently owed by the pallet to different contracts
	// Ideally this should be equal to the pallet account balance.
	pub type VestingBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new contract has been added to storage
		ContractAdded {
			recipient: T::AccountId,
			expiry: T::BlockNumber,
			amount: BalanceOf<T>,
		},
		/// Contract removed from storage
		ContractRemoved {
			recipient: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Contract not found in storage
		ContractNotFound,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The contract expiry has already passed
		ExpiryInThePast,
		/// The pallet account does not have funds to pay contract
		PalletOutOfFunds,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_new_contract(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			expiry: T::BlockNumber,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to add new recipient
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_add_new_contract(recipient, expiry, amount)?;
			Ok(().into())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_contract(
			origin: OriginFor<T>,
			recipient: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to remove recipient
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_remove_contract(recipient)?;
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn bulk_add_new_contracts(
			origin: OriginFor<T>,
			recipients: BulkContractInputs<T>,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to add new recipient
			T::ForceOrigin::ensure_origin(origin)?;
			for input in recipients {
				Self::do_add_new_contract(input.recipient, input.expiry, input.amount)?;
			}
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn bulk_remove_contract(
			origin: OriginFor<T>,
			recipients: BulkContractRemove<T>,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to remove recipients
			T::ForceOrigin::ensure_origin(origin)?;
			for recipient in recipients {
				Self::do_remove_contract(recipient)?;
			}
			Ok(().into())
		}

		// Ext to withdraw


		// Ext to claim expired payout
		

		// Ext for force payout

		// ext to handle pallet balance
	}
}
