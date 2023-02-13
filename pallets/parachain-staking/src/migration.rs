use super::*;

pub mod v1 {
	use super::*;
	use frame_support::{
		migration,
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
		BoundedVec,
	};
	use sp_std::vec::Vec;

	pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("MIGRATION : About to execute parachain-staking migration!");

			// // use the current validators to seed the invulnerables list
			// let current_validators =
			// 	migration::get_storage_value::<Vec<T::AccountId>>(b"Session", b"Validators", &[]);

			// if let Some(current_validators) = current_validators {
			// 	// convert to bounded format to insert to invulnerables
			// 	let invulnerables: BoundedVec<T::AccountId, T::MaxInvulnerables> =
			// 		current_validators.try_into().expect("current validators too large");

			// 	// insert new invulnerables
			// 	<Invulnerables<T>>::put(invulnerables.clone());

			// 	log::info!(
			// 		"MIGRATION : Migrated {:?} to new invulnerables list!",
			// 		invulnerables.len()
			// 	);
			// }

			T::DbWeight::get().reads_writes(2, 2)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// new version must be set.
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 1);
			Ok(())
		}
	}
}
