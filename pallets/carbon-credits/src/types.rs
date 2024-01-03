// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedBTreeMap};
use frame_system::pallet_prelude::BlockNumberFor;
use primitives::{
	Batch, BatchGroup, BatchRetireData, ProjectType, RegistryDetails, Royalty, SDGDetails,
};

use crate::pallet;

/// AuthorizedAccounts type of pallet
pub type AuthorizedAccountsListOf<T> = BoundedVec<
	<T as frame_system::Config>::AccountId,
	<T as pallet::Config>::MaxAuthorizedAccountCount,
>;

// -- Types for representing strings in pallet -- //
/// Type for short strings an descriptions
pub type ShortStringOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxShortStringLength>;

/// Type for longer strings and descriptions
pub type LongStringOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxLongStringLength>;

/// Type for storing ipfs links
pub type IpfsLinkOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxIpfsReferenceLength>;

/// Type for lists of ipfs links
pub type IpfsLinkListsOf<T> = BoundedVec<IpfsLinkOf<T>, <T as pallet::Config>::MaxDocumentCount>;

/// A project can address more than one SDG, this type stores the
/// list of SDGs the project addresses, upper bound is max number of existing SDGs
pub type SDGTypesListOf<T> = BoundedVec<SDGDetails<ShortStringOf<T>>, ConstU32<17>>;

/// List of registrys the projects are included in
pub type RegistryListOf<T> = BoundedVec<RegistryDetails<ShortStringOf<T>>, ConstU32<5>>;

/// List of royalty recipients for a project
pub type RoyaltyRecipientsOf<T> = BoundedVec<
	Royalty<<T as frame_system::Config>::AccountId>,
	<T as pallet::Config>::MaxRoyaltyRecipients,
>;

// Type of batch used by the pallet
pub type BatchOf<T> = Batch<ShortStringOf<T>, <T as pallet::Config>::Balance>;

// Type of group used by the pallet
pub type BatchGroupOf<T> = BatchGroup<
	ShortStringOf<T>,
	<T as pallet::Config>::AssetId,
	<T as pallet::Config>::Balance,
	BatchOf<T>,
	<T as pallet::Config>::MaxGroupSize,
>;

// List of groups used by the pallet
pub type BatchGroupListOf<T> = BoundedVec<BatchGroupOf<T>, <T as pallet::Config>::MaxGroupSize>;

// Map of groups used by the GroupId
pub type BatchGroupMapOf<T> = BoundedBTreeMap<
	<T as pallet::Config>::GroupId,
	BatchGroupOf<T>,
	<T as pallet::Config>::MaxGroupSize,
>;

/// Inputs given by project originator during project creation
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectCreateParams<T: pallet::Config> {
	/// Name of the project
	pub name: ShortStringOf<T>,
	/// Description of the project
	pub description: LongStringOf<T>,
	/// Location co-ordinates of thie project
	pub location: LongStringOf<T>,
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
	/// List of batch groups in the project
	pub batch_groups: BatchGroupListOf<T>,
	/// Type of carbon credit project
	pub project_type: Option<ProjectType>,
}

/// Details of the project stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectDetail<T: pallet::Config> {
	/// The originator of the project
	pub originator: T::AccountId,
	/// Name of the project
	pub name: ShortStringOf<T>,
	/// Description of the project
	pub description: LongStringOf<T>,
	/// Location co-ordinates of thie project
	pub location: LongStringOf<T>,
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
	/// Type of carbon credit project
	pub project_type: Option<ProjectType>,
	// origination details
	/// Creation time of project
	pub created: BlockNumberFor<T>,
	/// Last updation time of project
	pub updated: Option<BlockNumberFor<T>>,

	/// approval status - a project can only mint tokens once approved
	pub approved: ProjectApprovalStatus,
}

/// Batch retire data used by pallet
pub type BatchRetireDataOf<T> = BatchRetireData<ShortStringOf<T>, <T as pallet::Config>::Balance>;

/// List of retired batches, this can go upto the size of the batch group
pub type BatchRetireDataList<T> =
	BoundedVec<BatchRetireDataOf<T>, <T as pallet::Config>::MaxGroupSize>;

/// Details stored for a retirement event, this is linked to the NFT generated during retirement
/// Every NFT represents a unique retirement event
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RetiredCarbonCreditsData<T: pallet::Config> {
	/// The AccountId that retired the credits
	pub account: T::AccountId,
	/// The details of the batches the tokens were retired from
	pub retire_data: BatchRetireDataList<T>,
	/// The 'BlockNumber' of retirement
	pub timestamp: BlockNumberFor<T>,
	/// The total count of credits retired
	pub count: T::Balance,
	/// Retirement reason
	pub reason: ShortStringOf<T>,
}

/// Enum representing the approval status of a project.
#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, MaxEncodedLen, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProjectApprovalStatus {
	/// The project is pending approval.
	#[default]
	Pending,

	/// The project has been approved.
	Approved,

	/// The project has been rejected.
	Rejected,
}

impl ProjectApprovalStatus {
	/// Check if the project is approved.
	///
	/// Returns `true` if the project is approved, `false` otherwise.
	pub fn is_approved(self) -> bool {
		use ProjectApprovalStatus::*;
		match self {
			Approved => true,
			Pending => false,
			Rejected => false,
		}
	}

	/// Check if the project is rejected.
	///
	/// Returns `true` if the project is rejected, `false` otherwise.
	pub fn is_rejected(self) -> bool {
		use ProjectApprovalStatus::*;
		match self {
			Approved => false,
			Pending => false,
			Rejected => true,
		}
	}
}
