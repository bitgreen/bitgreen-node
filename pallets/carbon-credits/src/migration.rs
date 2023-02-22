use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub mod v1 {
	use super::*;
	use crate::types::ProjectDetail;

	use frame_support::{
		pallet_prelude::Weight,
		traits::{Get, OnRuntimeUpgrade},
	};

	pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!("MIGRATION : About to execute carbon-credits migration!");

			// convert the project type to new format
			Projects::<T>::translate::<OldProjectDetail<T>, _>(
				|_key, old| -> Option<ProjectDetail<T>> {
					let converted_project = ProjectDetail {
						originator: old.originator,
						name: old.name,
						description: old.description,
						location: Default::default(), /* set the location as default string since
						                               * we don't have any live projects yet */
						images: old.images,
						videos: old.videos,
						documents: old.documents,
						registry_details: old.registry_details,
						sdg_details: old.sdg_details,
						royalties: old.royalties,
						batch_groups: old.batch_groups,
						created: old.created,
						updated: old.updated,
						approved: old.approved,
					};
					Some(converted_project)
				},
			);

			log::info!("MIGRATION : Carbon credits migration complete!");

			T::DbWeight::get().reads_writes(1, 1)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// new version must be set.
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 1);
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
pub struct OldProjectDetail<T: pallet::Config> {
	/// The originator of the project
	pub originator: T::AccountId,
	/// Name of the project
	pub name: ShortStringOf<T>,
	/// Description of the project
	pub description: LongStringOf<T>,
	/// Location co-ordinates of thie project
	pub location: BoundedVec<(u32, u32), <T as pallet::Config>::MaxCoordinatesLength>,
	/// List of ipfs-hashes of images related to the project
	pub images: IpfsLinkListsOf<T>,
	/// List of ipfs-hashes of videos related to the project
	pub videos: IpfsLinkListsOf<T>,
	/// List of ipfs-hashes of documents related to the project
	pub documents: IpfsLinkListsOf<T>,
	/// Details of the project as represented in registry
	pub registry_details: RegistryListOf<T>,
	/// SDG details
	pub sdg_details: SDGTypesListOf<T>,
	/// The royalties to be paid when tokens are purchased
	pub royalties: Option<RoyaltyRecipientsOf<T>>,
	/// groups included in the project
	pub batch_groups: BatchGroupMapOf<T>,
	// origination details
	/// Creation time of project
	pub created: T::BlockNumber,
	/// Last updation time of project
	pub updated: Option<T::BlockNumber>,

	/// approval status - a project can only mint tokens once approved
	pub approved: bool,
}
