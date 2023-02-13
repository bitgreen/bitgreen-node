use super::*;

pub mod v1 {
	use super::*;
	use crate::types::CandidateInfoOf;
	use frame_support::{
		migration,
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
		BoundedVec,
	};
	use sp_std::vec::Vec;

	pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("MIGRATION : About to execute parachain-staking migration!");

			// retreive the current invulnerables list
			let current_validators = migration::get_storage_value::<Vec<T::AccountId>>(
				b"ParachainStaking",
				b"Invulnerables",
				&[],
			);

			if let Some(current_validators) = current_validators {
				// convert to new format
				let invulnerables: BoundedVec<CandidateInfoOf<T>, T::MaxInvulnerables> =
					current_validators
						.iter()
						.cloned()
						.map(|account| CandidateInfoOf::<T> {
							who: account,
							deposit: Default::default(),
							delegators: Default::default(),
							total_stake: Default::default(),
						})
						.collect::<Vec<CandidateInfoOf<T>>>()
						.try_into()
						.expect("current validators too large");

				// insert new invulnerables
				<Invulnerables<T>>::put(invulnerables.clone());

				log::info!(
					"MIGRATION : Migrated {:?} to new invulnerables format!",
					invulnerables.len()
				);
			}

			T::DbWeight::get().reads_writes(2, 2)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// new version must be set.
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 2);
			Ok(())
		}
	}
}
