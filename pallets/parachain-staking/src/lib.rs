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

pub mod migration;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchClass, DispatchResultWithPostInfo},
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::{AccountIdConversion, CheckedSub, Saturating, Zero},
		traits::{Currency, EnsureOrigin, ReservableCurrency, ValidatorRegistration},
		BoundedVec, PalletId,
	};
	use frame_system::{pallet_prelude::*, Config as SystemConfig};
	use pallet_session::SessionManager;
	use sp_runtime::{
		traits::{CheckedAdd, Convert},
		FixedPointNumber, Percent,
	};
	use sp_staking::SessionIndex;
	use sp_std::fmt::Debug;

	use crate::types::{
		CandidateInfoOf, DelegationInfoOf, UnbondedCandidateInfoOf, UnbondedDelegationInfoOf,
	};
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
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Origin that can dictate updating parameters of this pallet.
		type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

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
		type MaxDelegators: Get<u32>
			+ TypeInfo
			+ Clone
			+ MaybeSerializeDeserialize
			+ PartialOrd
			+ Ord
			+ Debug;

		/// Minim amount that should be delegated
		type MinDelegationAmount: Get<BalanceOf<Self>>;

		// Will be kicked if block is not produced in threshold.
		type KickThreshold: Get<Self::BlockNumber>;

		/// A stable ID for a validator.
		type ValidatorId: Member + Parameter;

		/// A conversion from account ID to validator ID.
		///
		/// Its cost must be at most one storage read.
		type ValidatorIdOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;

		/// Validate a user is registered
		type ValidatorRegistration: ValidatorRegistration<Self::ValidatorId>;

		// Delay before unbonded stake can be withdrawn
		type UnbondingDelay: Get<Self::BlockNumber>;

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
		StorageValue<_, BoundedVec<CandidateInfoOf<T>, T::MaxInvulnerables>, ValueQuery>;

	/// The (community, limited) collation candidates.
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T: Config> =
		StorageValue<_, BoundedVec<CandidateInfoOf<T>, T::MaxCandidates>, ValueQuery>;

	/// The delegates that have unbounded
	#[pallet::storage]
	#[pallet::getter(fn unbonded_delegates)]
	pub type UnbondedDelegates<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, UnbondedDelegationInfoOf<T>>;

	/// The delegates that have been removed
	#[pallet::storage]
	#[pallet::getter(fn unbonded_candidates)]
	pub type UnbondedCandidates<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, UnbondedCandidateInfoOf<T>>;

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
		pub invulnerables: Vec<CandidateInfoOf<T>>,
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
				BoundedVec::<CandidateInfoOf<T>, T::MaxInvulnerables>::try_from(
					self.invulnerables.clone(),
				)
				.expect("genesis invulnerables are more than T::MaxInvulnerables");
			assert!(
				T::MaxCandidates::get() >= self.desired_candidates,
				"genesis desired_candidates are more than T::MaxCandidates",
			);

			<DesiredCandidates<T>>::put(self.desired_candidates);
			<CandidacyBond<T>>::put(self.candidacy_bond);
			<Invulnerables<T>>::put(bounded_invulnerables);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewInvulnerables { invulnerables: Vec<CandidateInfoOf<T>> },
		NewDesiredCandidates { desired_candidates: u32 },
		NewCandidacyBond { bond_amount: BalanceOf<T> },
		CandidateAdded { account_id: T::AccountId, deposit: BalanceOf<T> },
		CandidateRemoved { account_id: T::AccountId },
		NewDelegation { account_id: T::AccountId, candidate: T::AccountId, amount: BalanceOf<T> },
		DelegationRemoved { account_id: T::AccountId, candidate: T::AccountId },
		InflationAmountSet { amount: BalanceOf<T> },
		CollatorRewardsTransferred { account_id: T::AccountId, amount: BalanceOf<T> },
		DelegatorRewardsTransferred { account_id: T::AccountId, amount: BalanceOf<T> },
		UnbondedWithdrawn { account_id: T::AccountId, amount: BalanceOf<T> },
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
		/// Below Minimum delegation amount
		LessThanMinimumDelegation,
		/// User already has another unbonding in progress
		UnbondingInProgress,
		/// No unbonding delegation found for user
		NoUnbondingDelegation,
		/// The unbonding delay has not been reached
		UnbondingDelayNotPassed,
		/// Already delegated
		AlreadyDelegated,
		/// The account is already a candidate
		DelegatorAccountSameAsCandidateAccount,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the list of invulnerable (fixed) collators.
		#[pallet::weight(T::WeightInfo::set_invulnerables(new.len() as u32))]
		pub fn set_invulnerables(
			origin: OriginFor<T>,
			new: Vec<CandidateInfoOf<T>>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			let bounded_invulnerables = BoundedVec::<_, T::MaxInvulnerables>::try_from(new)
				.map_err(|_| Error::<T>::TooManyInvulnerables)?;

			// check if the invulnerables have associated validator keys before they are set
			for invulnerable in bounded_invulnerables.iter() {
				let validator_key = T::ValidatorIdOf::convert(invulnerable.who.clone())
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
			<DesiredCandidates<T>>::put(max);
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
			<CandidacyBond<T>>::put(bond);
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

			// ensure not already a candidate or invulnerable
			ensure!(Self::find_candidate(who.clone()).is_none(), Error::<T>::AlreadyCandidate);
			ensure!(Self::find_invulnerable(who.clone()).is_none(), Error::<T>::AlreadyCandidate);

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
			ensure!(amount >= T::MinDelegationAmount::get(), Error::<T>::LessThanMinimumDelegation);

			let mut is_invulnerable = false;

			// ensure the candidate exists
			let mut candidate = match Self::find_candidate(candidate_id.clone()) {
				Some(candidate) => candidate,
				None => {
					// if not in candidates list, check in invulnerables list
					is_invulnerable = true;
					Self::find_invulnerable(candidate_id.clone()).ok_or(Error::<T>::NotCandidate)?
				},
			};

			// ensure the delegator is not a candidate
			// we do not want duplicate candidates and delegators
			if let Some(_candidate) = Self::find_candidate(who.clone()) {
				return Err(Error::<T>::DelegatorAccountSameAsCandidateAccount.into())
			};

			// try to reserve the delegation amount
			<T as Config>::Currency::reserve(&who, amount)?;

			// add the delegator to the list of delegators
			let delegation_info = DelegationInfoOf::<T> { who: who.clone(), deposit: amount };

			// ensure not already delegated
			ensure!(
				candidate
					.delegators
					.clone()
					.into_inner()
					.binary_search_by(|v| { v.who.cmp(&who) })
					.is_err(),
				Error::<T>::AlreadyDelegated
			);

			candidate
				.delegators
				.try_push(delegation_info)
				.map_err(|_| Error::<T>::TooManyDelegations)?;

			candidate.total_stake = candidate
				.total_stake
				.checked_add(&amount)
				.ok_or(Error::<T>::ArithmeticOverflow)?;

			Self::update_candidate(candidate.clone(), is_invulnerable)?;

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

			let mut is_invulnerable = false;

			let mut candidate = match Self::find_candidate(candidate_id.clone()) {
				Some(candidate) => candidate,
				None => {
					// if not in candidates list, check in invulnerables list
					is_invulnerable = true;
					Self::find_invulnerable(candidate_id.clone()).ok_or(Error::<T>::NotCandidate)?
				},
			};

			// remove delegator from candidates
			let delegator_index = candidate
				.delegators
				.iter()
				.position(|d| d.who == who)
				.ok_or(Error::<T>::NotDelegator)?;
			let delegator = candidate.delegators.swap_remove(delegator_index);

			// ensure another unbonding is not in progress
			ensure!(!UnbondedDelegates::<T>::contains_key(&who), Error::<T>::UnbondingInProgress);

			// add the delegate to the unreserved queue
			let now = frame_system::Pallet::<T>::block_number();
			UnbondedDelegates::<T>::insert(
				who.clone(),
				UnbondedDelegationInfoOf::<T> { deposit: delegator.deposit, unbonded_at: now },
			);

			candidate.total_stake = candidate
				.total_stake
				.checked_sub(&delegator.deposit)
				.ok_or(Error::<T>::ArithmeticOverflow)?;

			if !is_invulnerable {
				<Candidates<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
					let index = candidates
						.iter()
						.position(|candidate| candidate.who == candidate_id)
						.ok_or(Error::<T>::NotCandidate)?;

					let _ = candidates.remove(index);

					candidates
						.try_insert(index, candidate.clone())
						.map_err(|_| Error::<T>::TooManyCandidates)?;
					Ok(candidates.len())
				})?;
			} else {
				<Invulnerables<T>>::try_mutate(|candidates| -> Result<usize, DispatchError> {
					let index = candidates
						.iter()
						.position(|candidate| candidate.who == candidate_id)
						.ok_or(Error::<T>::NotCandidate)?;

					let _ = candidates.remove(index);

					candidates
						.try_insert(index, candidate.clone())
						.map_err(|_| Error::<T>::TooManyCandidates)?;
					Ok(candidates.len())
				})?;
			}

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
			T::UpdateOrigin::ensure_origin(origin)?;
			InflationAmountPerBlock::<T>::set(amount);
			// emit event
			Self::deposit_event(Event::InflationAmountSet { amount });
			Ok(())
		}

		/// Withdraw unbonded delegation after unbonding delay
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn withdraw_unbonded(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// ensure the unbonding details exist
			let delegation = UnbondedDelegates::<T>::try_get(who.clone())
				.map_err(|_| Error::<T>::NoUnbondingDelegation)?;

			// ensure the unbonding period has passed
			let now = frame_system::Pallet::<T>::block_number();
			let unbonding_delay = T::UnbondingDelay::get();
			let time_passed =
				now.checked_sub(&delegation.unbonded_at).ok_or(Error::<T>::ArithmeticOverflow)?;

			ensure!(time_passed >= unbonding_delay, Error::<T>::UnbondingDelayNotPassed);

			// withdraw the user deposit
			let remaining_reward = <T as Config>::Currency::unreserve(&who, delegation.deposit);

			// if any rewards was paid out to the user, we deposit the amount to the account
			if remaining_reward != 0_u32.into() {
				// transfer the user reward to user account
				<T as Config>::Currency::deposit_creating(&who, remaining_reward);
			}

			// delete the unbonded delegation
			UnbondedDelegates::<T>::remove(who.clone());

			// emit event
			Self::deposit_event(Event::UnbondedWithdrawn {
				account_id: who,
				amount: delegation.deposit,
			});
			Ok(())
		}

		/// Withdraw deposit and complete candidate exit
		#[pallet::weight(T::WeightInfo::leave_intent(T::MaxCandidates::get()))]
		pub fn candidate_withdraw_unbonded(
			origin: OriginFor<T>,
			candidate: T::AccountId,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// ensure the unbonding details exist
			let delegation = UnbondedCandidates::<T>::try_get(candidate.clone())
				.map_err(|_| Error::<T>::NoUnbondingDelegation)?;

			// ensure the unbonding period has passed
			let now = frame_system::Pallet::<T>::block_number();
			let unbonding_delay = T::UnbondingDelay::get();
			let time_passed =
				now.checked_sub(&delegation.unbonded_at).ok_or(Error::<T>::ArithmeticOverflow)?;

			ensure!(time_passed >= unbonding_delay, Error::<T>::UnbondingDelayNotPassed);

			// unreserve all delegators to the candidate
			for delegator in delegation.delegators.iter() {
				// withdraw the user deposit
				let remaining_reward =
					<T as Config>::Currency::unreserve(&delegator.who, delegator.deposit);

				// if any rewards was paid out to the user, we deposit the amount to the account
				if remaining_reward != 0_u32.into() {
					// transfer the user reward to user account
					<T as Config>::Currency::deposit_creating(&delegator.who, remaining_reward);
				}
			}

			<LastAuthoredBlock<T>>::remove(candidate.clone());

			// withdraw the candidate deposit
			let remaining_reward =
				<T as Config>::Currency::unreserve(&candidate, delegation.deposit);

			// if any rewards was paid out to the candidate, we deposit the amount to the account
			if remaining_reward != 0_u32.into() {
				// transfer the user reward to user account
				<T as Config>::Currency::deposit_creating(&candidate, remaining_reward);
			}

			// delete the unbonded delegation
			UnbondedCandidates::<T>::remove(candidate.clone());

			// emit event
			Self::deposit_event(Event::UnbondedWithdrawn {
				account_id: candidate,
				amount: delegation.deposit,
			});
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

					let now = frame_system::Pallet::<T>::block_number();

					// add the candidate to the unbonding queue
					UnbondedCandidates::<T>::insert(
						who,
						UnbondedCandidateInfoOf::<T> {
							deposit: candidate.deposit,
							delegators: candidate.delegators,
							total_stake: candidate.total_stake,
							unbonded_at: now,
						},
					);

					Ok(candidates.len())
				})?;
			Self::deposit_event(Event::CandidateRemoved { account_id: who.clone() });
			Ok(current_count)
		}

		/// Finds a candidate with AccountId if it exists
		fn find_candidate(who: T::AccountId) -> Option<CandidateInfoOf<T>> {
			Self::candidates().into_iter().find(|c| c.who == who)
		}

		/// Finds an invulnerable with AccountId if it exists
		fn find_invulnerable(who: T::AccountId) -> Option<CandidateInfoOf<T>> {
			Self::invulnerables().into_iter().find(|c| c.who == who)
		}

		/// Finds a candidate/invulnerable with AccountId if it exists
		/// Returns (CandidateInfoOf<T>, is_invulnerable)
		fn get_candidate(who: T::AccountId) -> Option<(CandidateInfoOf<T>, bool)> {
			// first search within candidate list
			match Candidates::<T>::get().into_iter().find(|c| c.who == who) {
				Some(candidate) => Some((candidate, false)),
				// also search in invulnerable list if not found
				None => Invulnerables::<T>::get()
					.into_iter()
					.find(|c| c.who == who)
					.map(|candidate| (candidate, true)),
			}
		}

		/// Updates the candidates storage with new value
		fn update_candidate(
			candidate: CandidateInfoOf<T>,
			is_invulnerable: bool,
		) -> DispatchResult {
			if !is_invulnerable {
				<Candidates<T>>::try_mutate(|candidates| -> DispatchResult {
					let index = candidates
						.iter()
						.position(|c| c.who == candidate.who)
						.ok_or(Error::<T>::NotCandidate)?;

					let _ = candidates.remove(index);

					candidates
						.try_insert(index, candidate.clone())
						.map_err(|_| Error::<T>::TooManyCandidates)?;
					Ok(())
				})
			} else {
				<Invulnerables<T>>::try_mutate(|candidates| -> DispatchResult {
					let index = candidates
						.iter()
						.position(|c| c.who == candidate.who)
						.ok_or(Error::<T>::NotCandidate)?;

					let _ = candidates.remove(index);

					candidates
						.try_insert(index, candidate.clone())
						.map_err(|_| Error::<T>::TooManyCandidates)?;
					Ok(())
				})
			}
		}

		/// Assemble the current set of candidates and invulnerables into the next collator set.
		///
		/// This is done on the fly, as frequent as we are told to do so, as the session manager.
		pub fn assemble_collators(
			candidates: BoundedVec<T::AccountId, T::MaxCandidates>,
		) -> Vec<T::AccountId> {
			let mut collators: Vec<T::AccountId> =
				Self::invulnerables().into_iter().map(|c| c.who).collect();
			collators.extend(candidates);
			collators
		}

		/// Calculate the total stake of the given delegator set
		pub fn sum_delegator_set_stake(
			set: &BoundedVec<DelegationInfoOf<T>, T::MaxDelegators>,
		) -> BalanceOf<T> {
			let mut delegators_total_stake: BalanceOf<T> = Default::default();

			for delegator in set.iter() {
				delegators_total_stake =
					delegators_total_stake.checked_add(&delegator.deposit).unwrap_or_default();
			}
			delegators_total_stake
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
							debug_assert!(false, "failed to remove candidate {why:?}");
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
			let fee_reward = T::Currency::free_balance(&pot)
				.checked_sub(&T::Currency::minimum_balance())
				.unwrap_or_else(Zero::zero);
			log::info!("fee_reward {:?}", fee_reward);

			// add inflation rewards to the parachain_staking_pot
			let reward = Self::inflation_reward_per_block()
				.checked_add(&fee_reward)
				.unwrap_or_else(Zero::zero);
			log::info!("total_reward {:?}", reward);

			// fetch the candidate details for the author
			if let Some((mut candidate, is_invulnerable)) = Self::get_candidate(author.clone()) {
				if !candidate.delegators.is_empty() {
					// total delegator reward is 90%
					let delegator_reward = Percent::from_percent(90).mul_floor(reward);

					let delegator_data = candidate.delegators.clone();
					let delegators_total_stake: BalanceOf<T> =
						Self::sum_delegator_set_stake(&delegator_data);
					log::info!("delegators_total_stake {:?}", delegators_total_stake);

					let mut new_delegator_data: Vec<DelegationInfoOf<T>> = Default::default();
					for mut delegator in delegator_data.into_iter() {
						// reward users based on the share of stake they hold compared to the pool
						let delegator_deposit_as_u128: u128 =
							delegator.deposit.try_into().unwrap_or_default();
						let delegators_total_stake_as_u128: u128 =
							delegators_total_stake.try_into().unwrap_or_default();
						let delegator_share_of_total_stake = sp_runtime::FixedU128::from_rational(
							delegator_deposit_as_u128,
							delegators_total_stake_as_u128,
						);
						log::info!(
							"delegator_share_of_total_stake {:?}",
							delegator_share_of_total_stake
						);

						let reward_for_delegator = delegator_share_of_total_stake
							.checked_mul_int(delegator_reward)
							.unwrap_or_default();
						log::info!(
							"reward_for_delegator {:?} is {:?}",
							delegator.who,
							reward_for_delegator
						);

						// update the delegator stake with the reward amount
						delegator.deposit = delegator.deposit.saturating_add(reward_for_delegator);

						new_delegator_data.push(delegator);
					}

					// this should not fail because the bounds are the same
					candidate.delegators = new_delegator_data.try_into().unwrap();

					// send rest of reward to collator
					let collator_reward = Percent::from_percent(10).mul_floor(reward);
					candidate.deposit = candidate.deposit.saturating_add(collator_reward);

					candidate.total_stake = candidate.total_stake.saturating_add(reward);
				} else {
					// `reward` pot account minus ED, this should never fail.
					candidate.deposit = candidate.deposit.saturating_add(reward);
					candidate.total_stake = candidate.total_stake.saturating_add(reward);
				}

				let _ = Self::update_candidate(candidate, is_invulnerable);
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

			log::info!("newvalidators for new session {} at #{:?}", index, result,);

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
