use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub mod v2 {
	use super::*;
	use crate::types::ProjectDetail;

	use frame_support::{
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
	};

	pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("V2 MIGRATION : About to execute carbon-credits migration!");

			// convert the project type to new format
			RetiredCredits::<T>::translate::<OldRetiredCarbonCreditsData<T>, _>(
				|_asset_id, _item_id, old| -> Option<RetiredCarbonCreditsData<T>> {
					let converted_data = RetiredCarbonCreditsData {
						account: old.account,
						retire_data: old.retire_data,
						timestamp: old.timestamp,
						count: old.count,
						reason: Default::default(), // new value
					};
					Some(converted_data)
				},
			);

			log::info!("MIGRATION : Carbon credits migration complete!");

			T::DbWeight::get().reads_writes(1, 1)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// new version must be set.
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 2);
			Ok(())
		}
	}
}

/// Details of the project stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OldRetiredCarbonCreditsData<T: pallet::Config> {
	/// The AccountId that retired the credits
	pub account: T::AccountId,
	/// The details of the batches the tokens were retired from
	pub retire_data: BatchRetireDataList<T>,
	/// The 'BlockNumber' of retirement
	pub timestamp: T::BlockNumber,
	/// The total count of credits retired
	pub count: T::Balance,
}
