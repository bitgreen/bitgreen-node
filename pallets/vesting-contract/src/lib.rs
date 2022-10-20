// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! ## Vesting Contract Pallet
//! The goal of the pallet is to create vesting contracts for uniques addresses. This is different from other vesting pallets since our goal
//! is to use unique addresses for every payout/unlock rather than a scheduled payout to same address.
//! For example, if a recipient has an amount of 100BBB vested over 50 blocks, and unlocked propotionality over 10 blocks.
//! In this case we would have 20BBB every 10 blocks until the entire amount is vested after 50 blocks, to execute this the recipient
//! has to create 5 different addresses (one account for every transaction) and these addresses and amounts are added as individual contracts
//! to the pallet storage
//! Example : Account A -> 20 BBB -> Expiry at block 10
//!   Account B -> 20 BBB -> Expiry at block 20
//!   Account C -> 20 BBB -> Expiry at block 30
//!   Account D -> 20 BBB -> Expiry at block 40
//!   Account E -> 20 BBB -> Expiry at block 50
//! This can also be used for individual one time contracts and future contracts can be modified or revoked.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `withdraw_vested`: Withdraw an expired contract amount
//!
//! ### Permissioned Functions
//!
//! * `add_new_contract`: Add a new contract to the pallet
//! * `remove_contract`: Remove existing contract from pallet
//! * `bulk_add_new_contracts`: Same as add_new_contract but for multiple contracts
//! * `bulk_remove_new_contracts`: Same as remove_contract but for multiple contracts
//! * `force_withdraw_vested`: Withdraw vested amount to a recipient
//!
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::traits::{Currency, ExistenceRequirement::*};
use frame_support::{
	ensure, pallet_prelude::DispatchResult, sp_runtime::traits::AccountIdConversion, traits::Get,
};
use sp_runtime::{
	traits::{CheckedAdd, CheckedSub},
	ArithmeticError,
};

mod functions;
pub use functions::*;

mod pre_validate;
pub use pre_validate::*;

mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::convert::TryInto;

	use super::*;

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
	pub type BulkContractInputOf<T> = BulkContractInput<
		<T as frame_system::Config>::AccountId,
		<T as frame_system::Config>::BlockNumber,
		BalanceOf<T>,
	>;

	/// List of BulkContractInput
	pub type BulkContractInputs<T> =
		BoundedVec<BulkContractInputOf<T>, <T as Config>::MaxContractInputLength>;

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

		/// The Vesting Contract pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
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
		ContractRemoved { recipient: T::AccountId },
		/// An existing contract has been completed/withdrawn
		ContractWithdrawn {
			recipient: T::AccountId,
			expiry: T::BlockNumber,
			amount: BalanceOf<T>,
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
		/// The contract has not expired
		ContractNotExpired,
		/// Contract already exists, remove old contract before adding new
		ContractAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add a new contract on chain
		/// A contract is considered valid if the following conditions are satisfied
		/// - the recipient does not already have a contract
		/// - The expiry block is in the future
		/// - If the pallet has balance to payout this contract
		#[transactional]
		#[pallet::weight(T::WeightInfo::add_new_contract())]
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

		/// Remove a contract from storage
		#[transactional]
		#[pallet::weight(T::WeightInfo::remove_contract())]
		pub fn remove_contract(
			origin: OriginFor<T>,
			recipient: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to remove recipient
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_remove_contract(recipient)?;
			Ok(().into())
		}

		/// Same as add_contract but take multiple accounts as input
		/// If any of the contracts fail to be processed all inputs are rejected
		#[pallet::weight(T::WeightInfo::bulk_add_new_contracts(recipients.len() as u32))]
		#[transactional]
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

		/// Same as remove_contract but take multiple accounts as input
		/// If any of the contracts fail to be processed all inputs are rejected
		#[pallet::weight(T::WeightInfo::bulk_add_new_contracts(recipients.len() as u32))]
		#[transactional]
		pub fn bulk_remove_contracts(
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

		/// Withdraw amount from a vested (expired) contract
		///
		/// WARNING: Insecure unless the chain includes `PrevalidateVestingWithdraw` as a `SignedExtension`.
		///
		/// Unsigned Validation:
		/// A call to withdraw vested is deemed valid if the sender has an existing contract
		#[pallet::weight((
			T::WeightInfo::withdraw_vested(),
			DispatchClass::Normal,
			Pays::No
		))]
		#[transactional]
		pub fn withdraw_vested(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to remove recipients
			let who = ensure_signed(origin)?;
			Self::do_withdraw_vested(who)?;
			Ok(().into())
		}

		/// Call withdraw_vested for any account with a valid contract
		#[pallet::weight(T::WeightInfo::force_withdraw_vested())]
		#[transactional]
		pub fn force_withdraw_vested(
			origin: OriginFor<T>,
			recipient: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// ensure caller is allowed to force withdraw
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_withdraw_vested(recipient)?;
			Ok(().into())
		}
	}
}
