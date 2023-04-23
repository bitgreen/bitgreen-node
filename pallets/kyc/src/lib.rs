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
	traits::{
		ChangeMembers, Contains, Currency, ExistenceRequirement, Get, InitializeMembers,
		SortedMembers,
	},
	BoundedVec, PalletId,
};
use sp_runtime::traits::{AccountIdConversion, StaticLookup};
use sp_std::prelude::*;
pub mod weights;

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

		/// The receiver of the signal for when the membership has been initialized. This happens
		/// pre-genesis and will usually be the same as `MembershipChanged`. If you need to do
		/// something different on initialization, then you can change this accordingly.
		type MembershipInitialized: InitializeMembers<Self::AccountId>;

		/// The receiver of the signal for when the membership has changed.
		type MembershipChanged: ChangeMembers<Self::AccountId>;

		/// The maximum number of members that this membership can have.
		///
		/// This is used for benchmarking. Re-run the benchmarks if this changes.
		///
		/// This is enforced in the code; the membership size can not exceed this limit.
		type MaxMembers: Get<u32>;

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
		StorageValue<_, BoundedVec<T::AccountId, T::MaxMembers>, ValueQuery>;

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
		pub members: BoundedVec<T::AccountId, T::MaxMembers>,
		pub phantom: PhantomData<I>,
	}

	#[cfg(feature = "std")]
	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self { members: Default::default(), phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			use sp_std::collections::btree_set::BTreeSet;
			let members_set: BTreeSet<_> = self.members.iter().collect();
			assert_eq!(
				members_set.len(),
				self.members.len(),
				"Members cannot contain duplicate accounts."
			);

			let mut members = self.members.clone();
			members.sort();
			T::MembershipInitialized::initialize_members(&members);
			<Members<T, I>>::put(members);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// The given member was added
		MemberAdded { who: T::AccountId },
		/// The given member was removed
		MemberRemoved { who: T::AccountId },
		/// Two members were swapped; see the transaction for who.
		MembersSwapped,
		/// The membership was reset; see the transaction for who the new set is.
		MembersReset,
		/// One of the members' keys changed.
		KeyChanged,
		/// Phantom member, never used.
		Dummy { _phantom_data: PhantomData<(T::AccountId, <T as Config<I>>::RuntimeEvent)> },
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
		#[pallet::weight(50_000_000)]
		pub fn add_member(origin: OriginFor<T>, who: AccountIdLookupOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;
			let who = T::Lookup::lookup(who)?;

			let mut members = <Members<T, I>>::get();
			let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;
			members
				.try_insert(location, who.clone())
				.map_err(|_| Error::<T, I>::TooManyMembers)?;

			<Members<T, I>>::put(&members);

			T::MembershipChanged::change_members_sorted(&[who.clone()], &[], &members[..]);

			let _ = Self::transfer_kyc_airdrop(who.clone());

			Self::deposit_event(Event::MemberAdded { who });
			Ok(())
		}

		/// Remove a member `who` from the set.
		///
		/// May only be called from `T::RemoveOrigin`.
		#[pallet::call_index(1)]
		#[pallet::weight(50_000_000)]
		pub fn remove_member(origin: OriginFor<T>, who: AccountIdLookupOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;

			let who = T::Lookup::lookup(who)?;

			let mut members = <Members<T, I>>::get();
			let location = members.binary_search(&who).ok().ok_or(Error::<T, I>::NotMember)?;
			members.remove(location);

			<Members<T, I>>::put(&members);

			T::MembershipChanged::change_members_sorted(&[], &[who.clone()], &members[..]);

			Self::deposit_event(Event::MemberRemoved { who });
			Ok(())
		}

		/// Swap out one member `remove` for another `add`.
		///
		/// May only be called from `T::SwapOrigin`.
		///
		/// Prime membership is *not* passed from `remove` to `add`, if extant.
		#[pallet::call_index(2)]
		#[pallet::weight(50_000_000)]
		pub fn swap_member(
			origin: OriginFor<T>,
			remove: AccountIdLookupOf<T>,
			add: AccountIdLookupOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;

			let remove = T::Lookup::lookup(remove)?;
			let add = T::Lookup::lookup(add)?;

			if remove == add {
				return Ok(())
			}

			let mut members = <Members<T, I>>::get();
			let location = members.binary_search(&remove).ok().ok_or(Error::<T, I>::NotMember)?;
			let _ = members.binary_search(&add).err().ok_or(Error::<T, I>::AlreadyMember)?;
			members[location] = add.clone();
			members.sort();

			<Members<T, I>>::put(&members);

			T::MembershipChanged::change_members_sorted(&[add], &[remove], &members[..]);

			Self::deposit_event(Event::MembersSwapped);
			Ok(())
		}

		/// Change the membership to a new set, disregarding the existing membership. Be nice and
		/// pass `members` pre-sorted.
		///
		/// May only be called from `T::ResetOrigin`.
		#[pallet::call_index(3)]
		#[pallet::weight(50_000_000)]
		pub fn reset_members(origin: OriginFor<T>, members: Vec<T::AccountId>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;

			let mut members: BoundedVec<T::AccountId, T::MaxMembers> =
				BoundedVec::try_from(members).map_err(|_| Error::<T, I>::TooManyMembers)?;
			members.sort();
			<Members<T, I>>::mutate(|m| {
				T::MembershipChanged::set_members_sorted(&members[..], m);
				*m = members;
			});

			Self::deposit_event(Event::MembersReset);
			Ok(())
		}

		/// Swap out the sending member for some other key `new`.
		///
		/// May only be called from `Signed` origin of a current member.
		///
		/// Prime membership is passed from the origin account to `new`, if extant.
		#[pallet::call_index(4)]
		#[pallet::weight(50_000_000)]
		pub fn change_key(origin: OriginFor<T>, new: AccountIdLookupOf<T>) -> DispatchResult {
			let remove = ensure_signed(origin)?;
			let new = T::Lookup::lookup(new)?;

			if remove != new {
				let mut members = <Members<T, I>>::get();
				let location =
					members.binary_search(&remove).ok().ok_or(Error::<T, I>::NotMember)?;
				let _ = members.binary_search(&new).err().ok_or(Error::<T, I>::AlreadyMember)?;
				members[location] = new.clone();
				members.sort();

				<Members<T, I>>::put(&members);

				T::MembershipChanged::change_members_sorted(
					&[new],
					&[remove],
					&members[..],
				);
			}

			Self::deposit_event(Event::KeyChanged);
			Ok(())
		}

		/// Add a new account to the list of authorised Accounts
		/// The caller must be from a permitted origin
		#[pallet::call_index(5)]
		#[pallet::weight(50_000_000)]
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
		#[pallet::weight(50_000_000)]
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
		#[pallet::weight(50_000_000)]
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
		Self::members().binary_search(t).is_ok()
	}
}

impl<T: Config<I>, I: 'static> SortedMembers<T::AccountId> for Pallet<T, I> {
	fn sorted_members() -> Vec<T::AccountId> {
		Self::members().to_vec()
	}

	fn count() -> usize {
		Members::<T, I>::decode_len().unwrap_or(0)
	}
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark {
	use super::{Pallet as Membership, *};
	use frame_benchmarking::v1::{account, benchmarks_instance_pallet, whitelist, BenchmarkError};
	use frame_support::{assert_ok, traits::EnsureOrigin};
	use frame_system::RawOrigin;

	const SEED: u32 = 0;

	fn set_members<T: Config<I>, I: 'static>(members: Vec<T::AccountId>, prime: Option<usize>) {
		let reset_origin = T::ResetOrigin::try_successful_origin()
			.expect("ResetOrigin has no successful origin required for the benchmark");
		let prime_origin = T::PrimeOrigin::try_successful_origin()
			.expect("PrimeOrigin has no successful origin required for the benchmark");

		assert_ok!(<Membership<T, I>>::reset_members(reset_origin, members.clone()));
		if let Some(prime) = prime.map(|i| members[i].clone()) {
			let prime_lookup = T::Lookup::unlookup(prime);
			assert_ok!(<Membership<T, I>>::set_prime(prime_origin, prime_lookup));
		} else {
			assert_ok!(<Membership<T, I>>::clear_prime(prime_origin));
		}
	}

	benchmarks_instance_pallet! {
		add_member {
			let m in 1 .. (T::MaxMembers::get() - 1);

			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T, I>(members, None);
			let new_member = account::<T::AccountId>("add", m, SEED);
			let new_member_lookup = T::Lookup::unlookup(new_member.clone());
		}: {
			assert_ok!(<Membership<T, I>>::add_member(
				T::AddOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
				new_member_lookup,
			));
		} verify {
			assert!(<Members<T, I>>::get().contains(&new_member));
			#[cfg(test)] crate::tests::clean();
		}

		// the case of no prime or the prime being removed is surely cheaper than the case of
		// reporting a new prime via `MembershipChanged`.
		remove_member {
			let m in 2 .. T::MaxMembers::get();

			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T, I>(members.clone(), Some(members.len() - 1));

			let to_remove = members.first().cloned().unwrap();
			let to_remove_lookup = T::Lookup::unlookup(to_remove.clone());
		}: {
			assert_ok!(<Membership<T, I>>::remove_member(
				T::RemoveOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
				to_remove_lookup,
			));
		} verify {
			assert!(!<Members<T, I>>::get().contains(&to_remove));
			// prime is rejigged
			assert!(<Prime<T, I>>::get().is_some() && T::MembershipChanged::get_prime().is_some());
			#[cfg(test)] crate::tests::clean();
		}

		// we remove a non-prime to make sure it needs to be set again.
		swap_member {
			let m in 2 .. T::MaxMembers::get();

			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T, I>(members.clone(), Some(members.len() - 1));
			let add = account::<T::AccountId>("member", m, SEED);
			let add_lookup = T::Lookup::unlookup(add.clone());
			let remove = members.first().cloned().unwrap();
			let remove_lookup = T::Lookup::unlookup(remove.clone());
		}: {
			assert_ok!(<Membership<T, I>>::swap_member(
				T::SwapOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
				remove_lookup,
				add_lookup,
			));
		} verify {
			assert!(!<Members<T, I>>::get().contains(&remove));
			assert!(<Members<T, I>>::get().contains(&add));
			// prime is rejigged
			assert!(<Prime<T, I>>::get().is_some() && T::MembershipChanged::get_prime().is_some());
			#[cfg(test)] crate::tests::clean();
		}

		// er keep the prime common between incoming and outgoing to make sure it is rejigged.
		reset_member {
			let m in 1 .. T::MaxMembers::get();

			let members = (1..m+1).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T, I>(members.clone(), Some(members.len() - 1));
			let mut new_members = (m..2*m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
		}: {
			assert_ok!(<Membership<T, I>>::reset_members(
				T::ResetOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
				new_members.clone(),
			));
		} verify {
			new_members.sort();
			assert_eq!(<Members<T, I>>::get(), new_members);
			// prime is rejigged
			assert!(<Prime<T, I>>::get().is_some() && T::MembershipChanged::get_prime().is_some());
			#[cfg(test)] crate::tests::clean();
		}

		change_key {
			let m in 1 .. T::MaxMembers::get();

			// worse case would be to change the prime
			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			let prime = members.last().cloned().unwrap();
			set_members::<T, I>(members.clone(), Some(members.len() - 1));

			let add = account::<T::AccountId>("member", m, SEED);
			let add_lookup = T::Lookup::unlookup(add.clone());
			whitelist!(prime);
		}: {
			assert_ok!(<Membership<T, I>>::change_key(RawOrigin::Signed(prime.clone()).into(), add_lookup));
		} verify {
			assert!(!<Members<T, I>>::get().contains(&prime));
			assert!(<Members<T, I>>::get().contains(&add));
			// prime is rejigged
			assert_eq!(<Prime<T, I>>::get().unwrap(), add);
			#[cfg(test)] crate::tests::clean();
		}

		set_prime {
			let m in 1 .. T::MaxMembers::get();
			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			let prime = members.last().cloned().unwrap();
			let prime_lookup = T::Lookup::unlookup(prime.clone());
			set_members::<T, I>(members, None);
		}: {
			assert_ok!(<Membership<T, I>>::set_prime(
				T::PrimeOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
				prime_lookup,
			));
		} verify {
			assert!(<Prime<T, I>>::get().is_some());
			assert!(<T::MembershipChanged>::get_prime().is_some());
			#[cfg(test)] crate::tests::clean();
		}

		clear_prime {
			let m in 1 .. T::MaxMembers::get();
			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			let prime = members.last().cloned().unwrap();
			set_members::<T, I>(members, None);
		}: {
			assert_ok!(<Membership<T, I>>::clear_prime(
				T::PrimeOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			));
		} verify {
			assert!(<Prime<T, I>>::get().is_none());
			assert!(<T::MembershipChanged>::get_prime().is_none());
			#[cfg(test)] crate::tests::clean();
		}

		impl_benchmark_test_suite!(Membership, crate::tests::new_bench_ext(), crate::tests::Test);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_membership;
	use frame_system::RawOrigin;

	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

	use frame_support::{
		assert_noop, assert_ok, bounded_vec, ord_parameter_types, parameter_types,
		traits::{ConstU32, ConstU64, GenesisBuild},
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			Membership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		}
	);

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}

	impl pallet_balances::Config for Test {
		type AccountStore = System;
		type Balance = u128;
		type DustRemoval = ();
		type RuntimeEvent = RuntimeEvent;
		type ExistentialDeposit = ExistentialDeposit;
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type WeightInfo = ();
	}

	parameter_types! {
		pub static Members: Vec<u64> = vec![];
		pub static Prime: Option<u64> = None;
		pub const KycPalletId: PalletId = PalletId(*b"bitg/kyc");
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type RuntimeOrigin = RuntimeOrigin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type RuntimeCall = RuntimeCall;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u128>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = ConstU32<16>;
	}
	ord_parameter_types! {
		pub const One: u64 = 1;
		pub const Two: u64 = 2;
		pub const Three: u64 = 3;
		pub const Four: u64 = 4;
		pub const Five: u64 = 5;
	}

	pub struct TestChangeMembers;
	impl ChangeMembers<u64> for TestChangeMembers {
		fn change_members_sorted(incoming: &[u64], outgoing: &[u64], new: &[u64]) {
			let mut old_plus_incoming = Members::get();
			old_plus_incoming.extend_from_slice(incoming);
			old_plus_incoming.sort();
			let mut new_plus_outgoing = new.to_vec();
			new_plus_outgoing.extend_from_slice(outgoing);
			new_plus_outgoing.sort();
			assert_eq!(old_plus_incoming, new_plus_outgoing);

			Members::set(new.to_vec());
			Prime::set(None);
		}
		fn set_prime(who: Option<u64>) {
			Prime::set(who);
		}
		fn get_prime() -> Option<u64> {
			Prime::get()
		}
	}

	impl InitializeMembers<u64> for TestChangeMembers {
		fn initialize_members(members: &[u64]) {
			MEMBERS.with(|m| *m.borrow_mut() = members.to_vec());
		}
	}

	impl Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type AddOrigin = frame_system::EnsureRoot<u64>;
		type MembershipInitialized = TestChangeMembers;
		type MembershipChanged = TestChangeMembers;
		type MaxMembers = ConstU32<10>;
		type MaxAuthorizedAccountCount = ConstU32<10>;
		type PalletId = KycPalletId;
		type Currency = Balances;
		type WeightInfo = ();
	}

	pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		pallet_membership::GenesisConfig::<Test> {
			members: bounded_vec![10, 20, 30],
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}

	#[cfg(feature = "runtime-benchmarks")]
	pub(crate) fn new_bench_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[cfg(feature = "runtime-benchmarks")]
	pub(crate) fn clean() {
		Members::set(vec![]);
		Prime::set(None);
	}

	#[test]
	fn query_membership_works() {
		new_test_ext().execute_with(|| {
			assert_eq!(Membership::members(), vec![10, 20, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), vec![10, 20, 30]);
		});
	}

	#[test]
	fn add_member_works() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));
			assert_noop!(
				Membership::add_member(RuntimeOrigin::signed(5), 15),
				crate::Error::<Test, _>::NotAuthorised
			);
			assert_noop!(
				Membership::add_member(RuntimeOrigin::signed(authorised_account), 10),
				Error::<Test, _>::AlreadyMember
			);
			assert_ok!(Membership::add_member(RuntimeOrigin::signed(authorised_account), 15));
			assert_eq!(Membership::members(), vec![10, 15, 20, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	fn add_member_airdrop_works() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));

			// set the airdrop amount
			let airdrop_amount = 10;
			assert_ok!(Membership::force_set_kyc_airdrop(
				RawOrigin::Root.into(),
				Some(airdrop_amount),
			));

			// set some balance to the pallet account
			let kyc_pallet_account: u64 = PalletId(*b"bitg/kyc").into_account_truncating();
			Balances::make_free_balance_be(&kyc_pallet_account, 100);

			let balance_before_kyc = Balances::free_balance(&15);
			assert_ok!(Membership::add_member(RuntimeOrigin::signed(authorised_account), 15));
			assert_eq!(Membership::members(), vec![10, 15, 20, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
			assert_eq!(Balances::free_balance(&15), balance_before_kyc + airdrop_amount);
		});
	}

	#[test]
	fn remove_member_works() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));
			assert_noop!(
				Membership::remove_member(RuntimeOrigin::signed(5), 20),
				Error::<Test, _>::NotAuthorised
			);
			assert_noop!(
				Membership::remove_member(RuntimeOrigin::signed(authorised_account), 15),
				Error::<Test, _>::NotMember
			);
			assert_ok!(Membership::remove_member(RuntimeOrigin::signed(authorised_account), 20));
			assert_eq!(Membership::members(), vec![10, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	fn swap_member_works() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));
			assert_noop!(
				Membership::swap_member(RuntimeOrigin::signed(5), 10, 25),
				Error::<Test, _>::NotAuthorised
			);
			assert_noop!(
				Membership::swap_member(RuntimeOrigin::signed(authorised_account), 15, 25),
				Error::<Test, _>::NotMember
			);
			assert_noop!(
				Membership::swap_member(RuntimeOrigin::signed(authorised_account), 10, 30),
				Error::<Test, _>::AlreadyMember
			);

			assert_ok!(Membership::swap_member(RuntimeOrigin::signed(authorised_account), 20, 20));
			assert_eq!(Membership::members(), vec![10, 20, 30]);

			assert_ok!(Membership::swap_member(RuntimeOrigin::signed(authorised_account), 10, 25));
			assert_eq!(Membership::members(), vec![20, 25, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	fn swap_member_works_that_does_not_change_order() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));
			assert_ok!(Membership::swap_member(RuntimeOrigin::signed(authorised_account), 10, 5));
			assert_eq!(Membership::members(), vec![5, 20, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	fn change_key_works() {
		new_test_ext().execute_with(|| {
			let authorised_account = 1;
			assert_ok!(Membership::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account,
			));
			assert_noop!(
				Membership::change_key(RuntimeOrigin::signed(3), 25),
				Error::<Test, _>::NotMember
			);
			assert_noop!(
				Membership::change_key(RuntimeOrigin::signed(10), 20),
				Error::<Test, _>::AlreadyMember
			);
			assert_ok!(Membership::change_key(RuntimeOrigin::signed(10), 40));
			assert_eq!(Membership::members(), vec![20, 30, 40]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	fn change_key_works_that_does_not_change_order() {
		new_test_ext().execute_with(|| {
			assert_ok!(Membership::change_key(RuntimeOrigin::signed(10), 5));
			assert_eq!(Membership::members(), vec![5, 20, 30]);
			assert_eq!(MEMBERS.with(|m| m.borrow().clone()), Membership::members().to_vec());
		});
	}

	#[test]
	#[should_panic(expected = "Members cannot contain duplicate accounts.")]
	fn genesis_build_panics_with_duplicate_members() {
		pallet_membership::GenesisConfig::<Test> {
			members: bounded_vec![1, 2, 3, 1],
			phantom: Default::default(),
		}
		.build_storage()
		.unwrap();
	}
}
