// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! # KYC Module
//!
//! Allows control of membership of a set of `AccountId`s, useful for managing membership of a
//! collective.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	traits::{Contains, Currency, ExistenceRequirement, Get},
	BoundedVec, PalletId,
};
use sp_runtime::traits::{AccountIdConversion, StaticLookup};
use sp_std::prelude::*;
pub mod weights;
pub use bitgreen_primitives::UserLevel;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub use pallet::*;
pub use weights::WeightInfo;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(4);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	pub type BalanceOf<T, I> =
		<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Required origin for adding a member (though can always be Root).
		type AddOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Maximum amount of authorised accounts permitted
		type MaxAuthorizedAccountCount: Get<u32>;

		/// The currency used for the pallet
		type Currency: Currency<Self::AccountId>;

		/// The KYC pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// The current membership, stored as an ordered Vec.
	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserLevel, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn authorized_accounts)]
	// List of AuthorizedAccounts for the pallet
	pub type AuthorizedAccounts<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxAuthorizedAccountCount>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn airdrop_amount)]
	// Amount to airdrop on every kyc success
	pub type AirdropAmount<T: Config<I>, I: 'static = ()> = StorageValue<_, BalanceOf<T, I>>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub members: Vec<(T::AccountId, UserLevel)>,
		pub phantom: PhantomData<I>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self { members: Default::default(), phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {
			for (member, level) in self.members.iter() {
				Members::<T, I>::insert(member, level);
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// The given member was added
		MemberAdded { who: T::AccountId, kyc_level: UserLevel },
		/// The given member was removed
		MemberRemoved { who: T::AccountId },
		/// Two members were swapped; see the transaction for who.
		MemberModified { who: T::AccountId, old_level: UserLevel, new_level: UserLevel },
		/// A new AuthorizedAccount has been added
		AuthorizedAccountAdded { account_id: T::AccountId },
		/// An AuthorizedAccount has been removed
		AuthorizedAccountRemoved { account_id: T::AccountId },
		/// User has received airdrop for kyc approval
		KYCAirdrop { who: T::AccountId, amount: BalanceOf<T, I> },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Already a member.
		AlreadyMember,
		/// Not a member.
		NotMember,
		/// Too many members.
		TooManyMembers,
		/// Adding a new authorized account failed
		TooManyAuthorizedAccounts,
		/// Cannot add a duplicate authorised account
		AuthorizedAccountAlreadyExists,
		/// No authorization account
		NotAuthorised,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Add a member `who` to the set.
		///
		/// May only be called from `T::AddOrigin`.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn add_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			kyc_level: UserLevel,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;
			let who = T::Lookup::lookup(who)?;

			// ensure the user kyc does not already exist
			ensure!(Members::<T, I>::get(who.clone()).is_none(), Error::<T, I>::AlreadyMember);

			// insert new kyc
			Members::<T, I>::insert(who.clone(), kyc_level.clone());

			let _ = Self::transfer_kyc_airdrop(who.clone());

			Self::deposit_event(Event::MemberAdded { who, kyc_level });
			Ok(())
		}

		/// Remove a member `who` from the set.
		///
		/// May only be called from `T::RemoveOrigin`.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn remove_member(origin: OriginFor<T>, who: AccountIdLookupOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;

			let who = T::Lookup::lookup(who)?;

			// ensure the user kyc does not already exist
			ensure!(Members::<T, I>::get(who.clone()).is_some(), Error::<T, I>::NotMember);

			// remove kyc
			Members::<T, I>::remove(who.clone());

			Self::deposit_event(Event::MemberRemoved { who });
			Ok(())
		}

		/// Add a member `who` to the set.
		///
		/// May only be called from `T::AddOrigin`.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn modify_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			kyc_level: UserLevel,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;
			let who = T::Lookup::lookup(who)?;

			Members::<T, I>::try_mutate(who.clone(), |level| -> DispatchResult {
				let level_clone = level.clone();
				// ensure already existing member
				let old_level = level_clone.as_ref().ok_or(Error::<T, I>::NotMember)?;
				*level = Some(kyc_level.clone());

				Self::deposit_event(Event::MemberModified {
					who,
					old_level: old_level.clone(),
					new_level: kyc_level,
				});
				Ok(())
			})
		}

		/// Add a new account to the list of authorised Accounts
		/// The caller must be from a permitted origin
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn force_add_authorized_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::AddOrigin::ensure_origin(origin)?;
			// add the account_id to the list of authorized accounts
			AuthorizedAccounts::<T, I>::try_mutate(|account_list| -> DispatchResult {
				ensure!(
					!account_list.contains(&account_id),
					Error::<T, I>::AuthorizedAccountAlreadyExists
				);

				account_list
					.try_push(account_id.clone())
					.map_err(|_| Error::<T, I>::TooManyAuthorizedAccounts)?;
				Ok(())
			})?;

			Self::deposit_event(Event::AuthorizedAccountAdded { account_id });
			Ok(())
		}

		/// Remove an account from the list of authorised accounts
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn force_remove_authorized_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::AddOrigin::ensure_origin(origin)?;
			// remove the account_id from the list of authorized accounts if already exists
			AuthorizedAccounts::<T, I>::try_mutate(|account_list| -> DispatchResult {
				if let Ok(index) = account_list.binary_search(&account_id) {
					account_list.swap_remove(index);
					Self::deposit_event(Event::AuthorizedAccountRemoved { account_id });
				}

				Ok(())
			})
		}

		/// Set the airdrop amount for each successful kyc
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::add_member(1))]
		pub fn force_set_kyc_airdrop(
			origin: OriginFor<T>,
			amount: Option<BalanceOf<T, I>>,
		) -> DispatchResult {
			T::AddOrigin::ensure_origin(origin)?;
			// remove the account_id from the list of authorized accounts if already exists
			AirdropAmount::<T, I>::set(amount);
			Ok(())
		}
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Checks if the given account_id is part of authorized account list
	pub fn check_authorized_account(
		account_id: &T::AccountId,
	) -> frame_support::pallet_prelude::DispatchResult {
		let authorized_accounts = AuthorizedAccounts::<T, I>::get();
		if !authorized_accounts.contains(account_id) {
			Err(Error::<T, I>::NotAuthorised.into())
		} else {
			Ok(())
		}
	}

	/// The account ID of the KYC pallet
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	/// Airdrop native tokens to user
	pub fn transfer_kyc_airdrop(
		who: T::AccountId,
	) -> frame_support::pallet_prelude::DispatchResult {
		// transfer airdrop if the amount is set
		if let Some(amount) = Self::airdrop_amount() {
			let airdrop_executed = T::Currency::transfer(
				&Self::account_id(),
				&who,
				amount,
				ExistenceRequirement::AllowDeath,
			);

			if airdrop_executed.is_ok() {
				Self::deposit_event(Event::KYCAirdrop { who, amount });
			}
		}
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Contains<T::AccountId> for Pallet<T, I> {
	fn contains(t: &T::AccountId) -> bool {
		Members::<T, I>::get(t).is_some()
	}
}
