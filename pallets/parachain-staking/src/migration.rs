use super::*;

pub mod v3 {
	use super::*;
	use crate::types::{CandidateInfoOf, DelegationInfoOf};
	use frame_support::{
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
		BoundedVec,
	};
	use sp_runtime::Saturating;
	use sp_std::vec::Vec;

	pub struct MigrateToV3<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV3<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("MIGRATION : About to execute parachain-staking migration V3!");

			// retreive the current invulnerables list
			let current_invulnerables = <Invulnerables<T>>::get();
			let updated_invulnerables =
				Self::remove_duplicate_delegators(current_invulnerables.into_inner());
			let bounded_updated_invulnerables: BoundedVec<_, _> =
				updated_invulnerables.try_into().unwrap();
			// insert new invulnerables
			<Invulnerables<T>>::put(bounded_updated_invulnerables);

			log::info!("MIGRATION : Invulnerables migration complete!");

			let current_candidates = <Candidates<T>>::get();
			let updated_candidates =
				Self::remove_duplicate_delegators(current_candidates.into_inner());
			let bounded_updated_candidates: BoundedVec<_, _> =
				updated_candidates.try_into().unwrap();
			// insert new invulnerables
			<Candidates<T>>::put(bounded_updated_candidates);

			log::info!("MIGRATION : Candidates migration complete!");

			T::DbWeight::get().reads_writes(2, 2)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// new version must be set.
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 3);
			Ok(())
		}
	}

	impl<T: Config> MigrateToV3<T> {
		pub fn remove_duplicate_delegators(
			validators: Vec<CandidateInfoOf<T>>,
		) -> Vec<CandidateInfoOf<T>> {
			let mut updated_validators: Vec<CandidateInfoOf<T>> = Default::default();
			for mut validator in validators {
				let mut updated_delegators: BoundedVec<DelegationInfoOf<T>, T::MaxDelegators> =
					Default::default();
				for delegator in validator.delegators {
					// search if the delegator exists in the updated_delegators list
					let seek = updated_delegators.binary_search_by(|v| v.who.cmp(&delegator.who));

					match seek {
						// if it does exist, we add the duplicate stake amount to the existing
						// validator
						Ok(i) => {
							let current_deposit = updated_delegators[i].deposit;
							updated_delegators[i].deposit =
								current_deposit.saturating_add(delegator.deposit);
						},
						// if it does not exist, we add a new entry to the delegators list
						Err(_) => {
							updated_delegators.try_push(delegator).unwrap();
						},
					}
				}
				validator.delegators = updated_delegators;
				updated_validators.push(validator);
			}

			updated_validators
		}
	}
}
