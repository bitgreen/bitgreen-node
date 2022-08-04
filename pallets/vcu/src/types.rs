// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::pallet;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use primitives::{Batch, BatchRetireData, RegistryDetails, Royalty, SDGDetails};

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

/// Type for storing location co-ordinates
pub type LocationCoordinatesOf<T> =
    BoundedVec<(u32, u32), <T as pallet::Config>::MaxCoordinatesLength>;

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

/// A project can represent VCUs from multiple batches
/// For example a project can have 100 tokens of 2019 vintage and 200 tokens of 2020 vintage. In this case
/// the project can package these two vintages to create a vcu token that has a supply of 300 tokens.
/// These vintages can be represented inside a batchgroup, in this case, it is important to remember that
/// the minting and retirement always gives priority to the oldest vintage.
/// Example : in the above case of 300 tokens, when the originator mints 100 tokens, we first mint the oldest (2019) credits
/// and only once the supply is exhausted we move on the next vintage, same for retirement.
pub type BatchGroupOf<T> = BoundedVec<BatchOf<T>, <T as pallet::Config>::MaxGroupSize>;

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
    // TODO : Improve this data type
    /// Location co-ordinates of thie project
    pub location: LocationCoordinatesOf<T>,
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
    /// List of batches in the project
    pub batches: BatchGroupOf<T>,
    // Price in USD for a single credit
    pub unit_price: T::Balance,
    /// The royalties to be paid when tokens are purchased
    pub royalties: Option<RoyaltyRecipientsOf<T>>,
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
    // TODO : Improve this data type
    /// Location co-ordinates of thie project
    pub location: LocationCoordinatesOf<T>,
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
    /// List of batches in the project
    pub batches: BatchGroupOf<T>,
    /// The royalties to be paid when tokens are purchased
    pub royalties: Option<RoyaltyRecipientsOf<T>>,

    // origination details
    /// Creation time of project
    pub created: T::BlockNumber,
    /// Last updation time of project
    pub updated: Option<T::BlockNumber>,

    /// approval status - a project can only mint tokens once approved
    pub approved: bool,

    // credits details
    /// The total_supply of the project, in case of a single batch
    /// this value is equal to the batch total_supply, in case of multiple
    /// batches (batch group) this value is the sum of all the total_supply of
    /// all the batches in the group.
    pub total_supply: T::Balance,
    /// The count of tokens minted related to the project
    pub minted: T::Balance,
    /// The count of tokens retired related to the project
    pub retired: T::Balance,
    // Price in USD for a single credit
    pub unit_price: T::Balance,
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
pub struct RetiredVcuData<T: pallet::Config> {
    /// The AccountId that retired the credits
    pub account: T::AccountId,
    /// The details of the batches the tokens were retired from
    pub retire_data: BatchRetireDataList<T>,
    /// The 'BlockNumber' of retirement
    pub timestamp: T::BlockNumber,
    /// The total count of credits retired
    pub count: T::Balance,
}
