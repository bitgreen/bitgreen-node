// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! Parachain Staking Pallet
//! Minimal staking pallet that implements collator selection by total backed stake. Unlike the approach taken by the frame staking pallet
//! we have opted for a more simpler model of the stakers selecting the collator they chose to stake with, rather than run an election process.
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

mod types;
use types::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::{Currency, ReservableCurrency};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Zero;
	use sp_std::convert::TryInto;

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency type
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		/// Number of rounds that candidates remain bonded before exit request is executable
		#[pallet::constant]
		type LeaveCandidatesDelay: Get<RoundIndex>;
		/// Number of rounds candidate requests to decrease self-bond must wait to be executable
		#[pallet::constant]
		type CandidateBondLessDelay: Get<RoundIndex>;
		/// Number of rounds that delegators remain bonded before exit request is executable
		#[pallet::constant]
		type LeaveDelegatorsDelay: Get<RoundIndex>;
		/// Number of rounds that delegations remain bonded before revocation request is executable
		#[pallet::constant]
		type RevokeDelegationDelay: Get<RoundIndex>;
		/// Minimum number of selected candidates every round
		#[pallet::constant]
		type MinSelectedCandidates: Get<u32>;
		/// Maximum number of candidates to store
		#[pallet::constant]
		type MaxCandidates: Get<u32> + TypeInfo;
		/// Maximum top delegations counted per candidate
		#[pallet::constant]
		type MaxTopDelegationsPerCandidate: Get<u32>;
		/// Maximum bottom delegations (not counted) per candidate
		#[pallet::constant]
		type MaxBottomDelegationsPerCandidate: Get<u32>;
		/// Maximum delegations for a candidate
		#[pallet::constant]
		type MaxDelegationsPerCandidate: Get<u32>;
		/// Minimum stake required for any candidate to be in `SelectedCandidates` for the round
		#[pallet::constant]
		type MinCollatorStk: Get<BalanceOf<Self>>;
		/// Minimum stake required for any account to be a collator candidate
		#[pallet::constant]
		type MinCandidateStake: Get<BalanceOf<Self>>;
		/// Minimum stake for any registered on-chain account to delegate
		#[pallet::constant]
		type MinDelegation: Get<BalanceOf<Self>>;
		/// Minimum stake for any registered on-chain account to be a delegator
		#[pallet::constant]
		type MinDelegatorStk: Get<BalanceOf<Self>>;
		// /// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The pool of collator candidates, each with their total backing stake
	#[pallet::storage]
	#[pallet::getter(fn candidate_pool)]
	pub(crate) type CandidateSet<T: Config> = StorageValue<_, OrderedCandidateSetOf<T>>;

	/// The list of delegators
	#[pallet::storage]
	#[pallet::getter(fn delegations)]
	pub(crate) type Delegations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, DelegationListOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Account joined the set of collator candidates.
		JoinedCollatorCandidates {
			account: T::AccountId,
			amount_locked: BalanceOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		CandidateExists,
		/// Errors should have helpful documentation associated with them.
		DelegatorExists,
		/// Candidate bond amount is below minimum threshold
		CandidateBondBelowMin,
		/// The candidate set is full
		CandidateSetFull,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Join the set of collator candidates
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn join_candidates(
			origin: OriginFor<T>,
			bond: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			let mut candidates =
				CandidateSet::<T>::get().unwrap_or_else(|| OrderedCandidateSetOf::<T>::new());

			// ensure not already a candidate
			ensure!(
				!candidates.is_candidate(&caller),
				Error::<T>::CandidateExists
			);

			// ensure the bond amount is greater than minimum
			ensure!(
				bond >= T::MinCandidateStake::get(),
				Error::<T>::CandidateBondBelowMin
			);

			// reserve the bond amount
			T::Currency::reserve(&caller, bond)?;

			let new_candidate = CandidateInfoOf::<T> {
				account: caller.clone(),
				bonded: bond,
				delegated: Zero::zero(),
				total_bond: bond,
			};

			candidates
				.insert_candidate(new_candidate)
				.map_err(|_| Error::<T>::CandidateSetFull)?;

			// insert empty delegations
			<Delegations<T>>::insert(&caller, BoundedVec::<_, _>::default());

			Self::deposit_event(Event::JoinedCollatorCandidates {
				account: caller,
				amount_locked: bond,
			});

			Ok(().into())
		}
	}
}
