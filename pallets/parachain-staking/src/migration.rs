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
	use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

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
				log::info!("MIGRATION : Processing validator: {:?}", validator);
				let mut updated_delegators: BoundedVec<DelegationInfoOf<T>, T::MaxDelegators> =
					Default::default();
				let mut updated_delegators_map: BTreeMap<T::AccountId, BalanceOf<T>> =
					BTreeMap::new();
				for delegator in validator.delegators {
					log::info!("MIGRATION : Processing delegator: {:?}", delegator);
					log::info!(
						"MIGRATION : Current updated delegators list: {:?}",
						updated_delegators
					);
					// search if the delegator exists in the updated_delegators list
					if updated_delegators_map.contains_key(&delegator.who) {
						if let Some(deposit) = updated_delegators_map.get_mut(&delegator.who) {
							*deposit = deposit.saturating_add(delegator.deposit);
						}
					} else {
						updated_delegators_map.insert(delegator.who, delegator.deposit);
					};
				}

				for (delegator, deposit) in updated_delegators_map {
					updated_delegators
						.try_push(DelegationInfoOf::<T> { who: delegator, deposit })
						.unwrap();
				}

				validator.delegators = updated_delegators;
				updated_validators.push(validator);
			}

			updated_validators
		}
	}
}
