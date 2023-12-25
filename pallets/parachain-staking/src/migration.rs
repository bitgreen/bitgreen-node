use super::*;

pub mod v3 {
	use super::*;

	use frame_support::{
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
	};

	pub struct MigrateToV3<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV3<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("MIGRATION : About to execute parachain-staking migration V3!");
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
}
