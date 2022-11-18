// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//

//! Parachain Staking pallet
//!
//! A pallet to manage collators in a parachain.
//!
//! ## Overview
//!
//! The Collator Selection pallet manages the collators of a parachain. **Collation is _not_ a
//! secure activity** and this pallet does not implement any game-theoretic mechanisms to meet BFT
//! safety assumptions of the chosen set.
//!
//! ## Terminology
//!
//! - Collator: A parachain block producer.
//! - Bond: An amount of `Balance` _reserved_ for candidate registration.
//! - Invulnerable: An account guaranteed to be in the collator set.
//!
//! ## Implementation
//!
//! The final `Collators` are aggregated from two individual lists:
//!
//! 1. [`Invulnerables`]: a set of collators appointed by governance. These accounts will always be
//!    collators.
//! 2. [`Candidates`]: these are *candidates to the collation task* and may or may not be elected as
//!    a final collator.
//!
//! The current implementation resolves congestion of [`Candidates`] in a first-come-first-serve
//! manner.
//!
//! Candidates will not be allowed to get kicked or leave_intent if the total number of candidates
//! fall below MinCandidates. This is for potential disaster recovery scenarios.
//!
//! ### Rewards
//!
//! The Collator Selection pallet maintains an on-chain account (the "Pot"). In each block, the
//! collator who authored it receives:
//!
//! - Half the value of the Pot.
//! - Half the value of the transaction fees within the block. The other half of the transaction
//!   fees are deposited into the Pot.
//!
//! To initiate rewards an ED needs to be transferred to the pot address.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::{AccountIdConversion, CheckedSub, Saturating, Zero},
		traits::{
			Currency, EnsureOrigin, ExistenceRequirement::KeepAlive, ReservableCurrency,
			ValidatorRegistration,
		},
		weights::DispatchClass,
		BoundedVec, PalletId,
	};
	use frame_system::{pallet_prelude::*, Config as SystemConfig};
	use pallet_session::SessionManager;
	use sp_runtime::{
		traits::{CheckedAdd, CheckedDiv, Convert},
		Percent,
	};
	use sp_staking::SessionIndex;

	use crate::types::{CandidateInfoOf, DelegationInfoOf};
	pub use crate::weights::WeightInfo;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

	/// A convertor from collators id. Since this pallet does not have stash/controller, this is
	/// just identity.
	pub struct IdentityCollator;
	impl<T> sp_runtime::traits::Convert<T, Option<T>> for IdentityCollator {
		fn convert(t: T) -> Option<T> {
			Some(t)
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Origin that can dictate updating parameters of this pallet.
		type UpdateOrigin: EnsureOrigin<Self::Origin>;

		/// Account Identifier from which the internal Pot is generated.
		type PotId: Get<PalletId>;

		/// Maximum number of candidates that we should have. This is enforced in code.
		///
		/// This does not take into account the invulnerables.
		type MaxCandidates: Get<u32>;

		/// Minimum number of candidates that we should have. This is used for disaster recovery.
		///
		/// This does not take into account the invulnerables.
		type MinCandidates: Get<u32>;

		/// Maximum number of invulnerables. This is enforced in code.
		type MaxInvulnerables: Get<u32>;

		/// Maximum number of delegators for a single candidate
		type MaxDelegators: Get<u32> + TypeInfo + Clone;

		/// Minim amount that should be delegated
		type MinDelegationAmount: Get<BalanceOf<Self>>;

		// Will be kicked if block is not produced in threshold.
		type KickThreshold: Get<Self::BlockNumber>;

		/// A stable ID for a validator.
		type ValidatorId: Member + Parameter;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// A conversion from account ID to validator ID.
		///
		/// Its cost must be at most one storage read.
		type ValidatorIdOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;

		/// Validate a user is registered
		type ValidatorRegistration: ValidatorRegistration<Self::ValidatorId>;

		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The invulnerable, fixed collators.
	#[pallet::storage]
	#[pallet::getter(fn invulnerables)]
	pub type Invulnerables<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxInvulnerables>, ValueQuery>;

	/// The (community, limited) collation candidates.
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T: Config> =
		StorageValue<_, BoundedVec<CandidateInfoOf<T>, T::MaxCandidates>, ValueQuery>;

	/// Last block authored by collator.
	#[pallet::storage]
	#[pallet::getter(fn last_authored_block)]
	pub type LastAuthoredBlock<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, T::BlockNumber, ValueQuery>;

	/// Desired number of candidates.
	///
	/// This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct.
	#[pallet::storage]
	#[pallet::getter(fn desired_candidates)]
	pub type DesiredCandidates<T> = StorageValue<_, u32, ValueQuery>;

	/// Fixed amount to deposit to become a collator.
	///
	/// When a collator calls `leave_intent` they immediately receive the deposit back.
	#[pallet::storage]
	#[pallet::getter(fn candidacy_bond)]
	pub type CandidacyBond<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Fixed amount to reward to collator
	///
	/// This amount is rewarded to collators and stakers every block
	#[pallet::storage]
	#[pallet::getter(fn inflation_reward_per_block)]
	pub type InflationAmountPerBlock<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub invulnerables: Vec<T::AccountId>,
		pub candidacy_bond: BalanceOf<T>,
		pub desired_candidates: u32,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				invulnerables: Default::default(),
				candidacy_bond: Default::default(),
				desired_candidates: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let duplicate_invulnerables =
				self.invulnerables.iter().collect::<std::collections::BTreeSet<_>>();
			assert!(
				duplicate_invulnerables.len() == self.invulnerables.len(),
				"duplicate invulnerables in genesis."
			);

			let bounded_invulnerables =
				BoundedVec::<_, T::MaxInvulnerables>::try_from(self.invulnerables.clone())
					.expect("genesis invulnerables are more than T::MaxInvulnerables");
			assert!(
				T::MaxCandidates::get() >= self.desired_candidates,
				"genesis desired_candidates are more than T::MaxCandidates",
			);

			<DesiredCandidates<T>>::put(&self.desired_candidates);
			<CandidacyBond<T>>::put(&self.candidacy_bond);
			<Invulnerables<T>>::put(bounded_invulnerables);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewInvulnerables { invulnerables: Vec<T::AccountId> },
		NewDesiredCandidates { desired_candidates: u32 },
		NewCandidacyBond { bond_amount: BalanceOf<T> },
		CandidateAdded { account_id: T::AccountId, deposit: BalanceOf<T> },
		CandidateRemoved { account_id: T::AccountId },
		NewDelegation { account_id: T::AccountId, candidate: T::AccountId, amount: BalanceOf<T> },
		DelegationRemoved { account_id: T::AccountId, candidate: T::AccountId },
		InflationAmountSet { amount: BalanceOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Too many candidates
		TooManyCandidates,
		/// Too few candidates
		TooFewCandidates,
		/// Unknown error
		Unknown,
		/// Permission issue
		Permission,
		/// User is already a candidate
		AlreadyCandidate,
		/// User is not a candidate
		NotCandidate,
		/// Too many invulnerables
		TooManyInvulnerables,
		/// User is already an Invulnerable
		AlreadyInvulnerable,
		/// Account has no associated validator ID
		NoAssociatedValidatorId,
		/// Validator ID is not yet registered
		ValidatorNotRegistered,
		/// Deledation limit is reached
		TooManyDelegations,
		/// Arithmetic overflow
		ArithmeticOverflow,
		/// Not a delegator
		NotDelegator,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the list of invulnerable (fixed) collators.
		#[pallet::weight(T::WeightInfo::set_invulnerables(new.len() as u32))]
		pub fn set_invulnerables(
			origin: OriginFor<T>,
			new: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			let bounded_invulnerables = BoundedVec::<_, T::MaxInvulnerables>::try_from(new)
				.map_err(|_| Error::<T>::TooManyInvulnerables)?;

			// check if the invulnerables have associated validator keys before they are set
			for account_id in bounded_invulnerables.iter() {
				let validator_key = T::ValidatorIdOf::convert(account_id.clone())
					.ok_or(Error::<T>::NoAssociatedValidatorId)?;
				ensure!(
					T::ValidatorRegistration::is_registered(&validator_key),
					Error::<T>::ValidatorNotRegistered
				);
			}

			<Invulnerables<T>>::put(&bounded_invulnerables);
			Self::deposit_event(Event::NewInvulnerables {
				invulnerables: bounded_invulnerables.to_vec(),
			});
			Ok(().into())
		}

		/// Set the ideal number of collators (not including the invulnerables).
		/// If lowering this number, then the number of running collators could be higher than this
		/// figure. Aside from that edge case, there should be no other way to have more collators
		/// than the desired number.
		#[pallet::weight(T::WeightInfo::set_desired_candidates())]
		pub fn set_desired_candidates(
			origin: OriginFor<T>,
			max: u32,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			// we trust origin calls, this is just a for more accurate benchmarking
			if max > T::MaxCandidates::get() {
				log::warn!("max > T::MaxCandidates; you might need to run benchmarks again");
			}
			<DesiredCandidates<T>>::put(&max);
			Self::deposit_event(Event::NewDesiredCandidates { desired_candidates: max });
			Ok(().into())
		}

		/// Set the candidacy bond amount.
		#[pallet::weight(T::WeightInfo::set_candidacy_bond())]
		pub fn set_candidacy_bond(
			origin: OriginFor<T>,
			bond: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			<CandidacyBond<T>>::put(&bond);
			Self::deposit_event(Event::NewCandidacyBond { bond_amount: bond });
			Ok(().into())
		}

		/// Register this account as a collator candidate. The account must (a) already have
		/// registered session keys and (b) be able to reserve the `CandidacyBond`.
		///
		/// This call is not available to `Invulnerable` collators.
		#[pallet::weight(T::WeightInfo::register_as_candidate(T::MaxCandidates::get()))]
		pub fn register_as_candidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// ensure we are below limit.
			let length = <Candidates<T>>::decode_len().unwrap_or_default();
			ensure!((length as u32) < Self::desired_candidates(), Error::<T>::TooManyCandidates);
			ensure!(!Self::invulnerables().contains(&who), Error::<T>::AlreadyInvulnerable);

			let validator_key = T::ValidatorIdOf::convert(who.clone())
				.ok_or(Error::<T>::NoAssociatedValidatorId)?;
			ensure!(
				T::ValidatorRegistration::is_registered(&validator_key),
				Error::<T>::ValidatorNotRegistered
			);

			let deposit = Self::candidacy_bond();
			// First authored block is current block plus kick threshold to handle session delay
			let incoming = CandidateInfoOf::<T> {
				who: who.clone(),
				deposit,
				delegators: Default::default(),
				total_stake: deposit,
			};

			let current_count =
				<Candidates<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
					if candidates.iter().any(|candidate| candidate.who == who) {
						Err(Error::<T>::AlreadyCandidate)?
					} else {
						T::Currency::reserve(&who, deposit)?;
						candidates.try_push(incoming).map_err(|_| Error::<T>::TooManyCandidates)?;

						// sort the candidates by total_stake
						candidates.sort_by(|a, b| a.total_stake.cmp(&b.total_stake));

						<LastAuthoredBlock<T>>::insert(
							who.clone(),
							frame_system::Pallet::<T>::block_number() + T::KickThreshold::get(),
						);
						Ok(candidates.len())
					}
				})?;

			Self::deposit_event(Event::CandidateAdded { account_id: who, deposit });
			Ok(Some(T::WeightInfo::register_as_candidate(current_count as u32)).into())
		}

		/// Deregister `origin` as a collator candidate. Note that the collator can only leave on
		/// session change. The `CandidacyBond` will be unreserved immediately.
		///
		/// This call will fail if the total number of candidates would drop below `MinCandidates`.
		///
		/// This call is not available to `Invulnerable` collators.
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn leave_intent(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(
				Self::candidates().len() as u32 > T::MinCandidates::get(),
				Error::<T>::TooFewCandidates
			);
			let current_count = Self::try_remove_candidate(&who)?;

			Ok(Some(T::WeightInfo::leave_intent(current_count as u32)).into())
		}

		/// Delegate to an existing candidate, delegators stake a bond amount to support the
		/// selected candidate
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn delegate(
			origin: OriginFor<T>,
			candidate_id: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// ensure the amount is above minimum
			ensure!(amount >= T::MinDelegationAmount::get(), Error::<T>::TooFewCandidates);

			let mut candidate =
				Self::find_candidate(candidate_id).ok_or(Error::<T>::NotCandidate)?;

			// try to reserve the delegation amount
			<T as Config>::Currency::reserve(&who, amount)?;

			// add the delegator to the list of delegators
			let delegation_info = DelegationInfoOf::<T> { who: who.clone(), deposit: amount };

			candidate
				.delegators
				.try_push(delegation_info)
				.map_err(|_| Error::<T>::TooManyDelegations)?;

			candidate.total_stake = candidate
				.total_stake
				.checked_add(&amount)
				.ok_or(Error::<T>::ArithmeticOverflow)?;

			<Candidates<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
				let index = candidates
					.iter()
					.position(|candidate| candidate.who == who)
					.ok_or(Error::<T>::NotCandidate)?;

				candidates
					.try_insert(index, candidate.clone())
					.map_err(|_| Error::<T>::TooManyCandidates)?;
				Ok(candidates.len())
			})?;

			Self::deposit_event(Event::NewDelegation {
				account_id: who,
				candidate: candidate.who,
				amount,
			});

			Ok(())
		}

		/// Undelegate and remove stake from an existing delegation
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn undelegate(origin: OriginFor<T>, candidate_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut candidate =
				Self::find_candidate(candidate_id).ok_or(Error::<T>::NotCandidate)?;

			// remove delegator from candidates
			let delegator_index = candidate
				.delegators
				.iter()
				.position(|d| d.who == who)
				.ok_or(Error::<T>::NotDelegator)?;
			let delegator = candidate.delegators.swap_remove(delegator_index);

			// try to unreserve the delegation amount
			<T as Config>::Currency::reserve(&who, delegator.deposit)?;

			candidate.total_stake = candidate
				.total_stake
				.checked_sub(&delegator.deposit)
				.ok_or(Error::<T>::ArithmeticOverflow)?;

			<Candidates<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
				let index = candidates
					.iter()
					.position(|candidate| candidate.who == who)
					.ok_or(Error::<T>::NotCandidate)?;

				candidates
					.try_insert(index, candidate.clone())
					.map_err(|_| Error::<T>::TooManyCandidates)?;
				Ok(candidates.len())
			})?;

			Self::deposit_event(Event::DelegationRemoved {
				account_id: who,
				candidate: candidate.who,
			});

			Ok(())
		}

		/// Undelegate and remove stake from an existing delegation
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn set_block_inflation_reward(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			// ensure the caller is allowed origin
			T::ForceOrigin::ensure_origin(origin)?;
			InflationAmountPerBlock::<T>::set(amount);
			// emit event
			Self::deposit_event(Event::InflationAmountSet { amount });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get a unique, inaccessible account id from the `PotId`.
		pub fn account_id() -> T::AccountId {
			T::PotId::get().into_account_truncating()
		}

		/// Removes a candidate if they exist and sends them back their deposit
		fn try_remove_candidate(who: &T::AccountId) -> Result<usize, DispatchError> {
			let current_count =
				<Candidates<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
					let index = candidates
						.iter()
						.position(|candidate| candidate.who == *who)
						.ok_or(Error::<T>::NotCandidate)?;
					let candidate = candidates.remove(index);
					T::Currency::unreserve(who, candidate.deposit);
					<LastAuthoredBlock<T>>::remove(who.clone());
					Ok(candidates.len())
				})?;
			Self::deposit_event(Event::CandidateRemoved { account_id: who.clone() });
			Ok(current_count)
		}

		/// Finds a candidate with AccountId if it exists
		fn find_candidate(who: T::AccountId) -> Option<CandidateInfoOf<T>> {
			Candidates::<T>::get().into_iter().find(|c| c.who == who)
		}

		/// Finds a candidate with AccountId if it exists
		fn get_delegators(who: T::AccountId) -> BoundedVec<DelegationInfoOf<T>, T::MaxDelegators> {
			let candidate = Candidates::<T>::get().into_iter().find(|c| c.who == who);
			if let Some(candidate) = candidate {
				return candidate.delegators
			}
			Default::default()
		}

		/// Assemble the current set of candidates and invulnerables into the next collator set.
		///
		/// This is done on the fly, as frequent as we are told to do so, as the session manager.
		pub fn assemble_collators(
			candidates: BoundedVec<T::AccountId, T::MaxCandidates>,
		) -> Vec<T::AccountId> {
			let mut collators = Self::invulnerables().to_vec();
			collators.extend(candidates);
			collators
		}

		/// Kicks out candidates that did not produce a block in the kick threshold
		/// and refund their deposits.
		pub fn kick_stale_candidates(
			candidates: BoundedVec<CandidateInfoOf<T>, T::MaxCandidates>,
		) -> BoundedVec<T::AccountId, T::MaxCandidates> {
			let now = frame_system::Pallet::<T>::block_number();
			let kick_threshold = T::KickThreshold::get();
			candidates
				.into_iter()
				.filter_map(|c| {
					let last_block = <LastAuthoredBlock<T>>::get(c.who.clone());
					let since_last = now.saturating_sub(last_block);
					if since_last < kick_threshold ||
						Self::candidates().len() as u32 <= T::MinCandidates::get()
					{
						Some(c.who)
					} else {
						let outcome = Self::try_remove_candidate(&c.who);
						if let Err(why) = outcome {
							log::warn!("Failed to remove candidate {:?}", why);
							debug_assert!(false, "failed to remove candidate {:?}", why);
						}
						None
					}
				})
				.collect::<Vec<_>>()
				.try_into()
				.expect("filter_map operation can't result in a bounded vec larger than its original; qed")
		}
	}

	/// Keep track of number of authored blocks per authority, uncles are counted as well since
	/// they're a valid proof of being online.
	impl<T: Config + pallet_authorship::Config>
		pallet_authorship::EventHandler<T::AccountId, T::BlockNumber> for Pallet<T>
	{
		fn note_author(author: T::AccountId) {
			let pot = Self::account_id();
			// assumes an ED will be sent to pot.
			let reward = T::Currency::free_balance(&pot)
				.checked_sub(&T::Currency::minimum_balance())
				.unwrap_or_else(Zero::zero);

			// find the list of all delegators for this author
			let delegators = Self::get_delegators(author.clone());
			if !delegators.is_empty() {
				// total delegator reward is 90%
				let delegator_reward = Percent::from_percent(90).mul_floor(reward);
				let reward_for_one_delegator = delegator_reward
					.checked_div(&(delegators.len() as u32).into())
					.unwrap_or_default();
				for delegator in delegators.iter() {
					let _success = T::Currency::transfer(
						&pot,
						&delegator.who,
						reward_for_one_delegator,
						KeepAlive,
					);
					debug_assert!(_success.is_ok());
				}

				// send rest of reward to collator
				let collator_reward = Percent::from_percent(10).mul_floor(reward);
				let _success = T::Currency::transfer(&pot, &author, collator_reward, KeepAlive);
				debug_assert!(_success.is_ok());
			} else {
				// `reward` pot account minus ED, this should never fail.
				let _success = T::Currency::transfer(&pot, &author, reward, KeepAlive);
				debug_assert!(_success.is_ok());
			}

			<LastAuthoredBlock<T>>::insert(author, frame_system::Pallet::<T>::block_number());

			frame_system::Pallet::<T>::register_extra_weight_unchecked(
				T::WeightInfo::note_author(),
				DispatchClass::Mandatory,
			);
		}

		fn note_uncle(_author: T::AccountId, _age: T::BlockNumber) {
			// we dont care
		}
	}

	/// Play the role of the session manager.
	impl<T: Config> SessionManager<T::AccountId> for Pallet<T> {
		fn new_session(index: SessionIndex) -> Option<Vec<T::AccountId>> {
			log::info!(
				"assembling new collators for new session {} at #{:?}",
				index,
				<frame_system::Pallet<T>>::block_number(),
			);

			let candidates = Self::candidates();
			let candidates_len_before = candidates.len();
			let active_candidates = Self::kick_stale_candidates(candidates);
			let removed = candidates_len_before - active_candidates.len();
			let result = Self::assemble_collators(active_candidates);

			frame_system::Pallet::<T>::register_extra_weight_unchecked(
				T::WeightInfo::new_session(candidates_len_before as u32, removed as u32),
				DispatchClass::Mandatory,
			);
			Some(result)
		}

		fn start_session(_: SessionIndex) {
			// we don't care.
		}

		fn end_session(_: SessionIndex) {
			// we don't care.
		}
	}
}
