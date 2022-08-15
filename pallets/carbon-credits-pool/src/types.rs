// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! Types for CarbonCredits-pools
use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedBTreeMap, BoundedVec};
use primitives::{IssuanceYear, RegistryName};
use scale_info::TypeInfo;

/// List of whitelisted registrys
pub type RegistryNameList<T> = BoundedVec<RegistryName, <T as Config>::MaxRegistryListCount>;

/// List of whitelisted project ids
pub type MaxProjectIdList<T> =
    BoundedVec<<T as pallet_carbon_credits::Config>::AssetId, <T as Config>::MaxProjectIdList>;

/// type to receive symbol data
pub type SymbolStringOf<T> = BoundedVec<u8, <T as Config>::MaxAssetSymbolLength>;

/// The configuration of a pool
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolConfig<RegistryList, MaxProjectIdList> {
    pub registry_list: Option<RegistryList>,
    pub project_id_list: Option<MaxProjectIdList>,
}

/// Map storing the details of a given project in a pool
/// ProjectId => Amount of tokens in pool
pub type ProjectDetails<T> = BoundedBTreeMap<
    <T as pallet_carbon_credits::Config>::AssetId,
    <T as pallet_carbon_credits::Config>::Balance,
    <T as Config>::MaxProjectIdList,
>;

/// Map storing the available credits in the pool by issuance year
/// IssuanceYear => ProjectDetail
pub type CreditsMap<T> =
    BoundedBTreeMap<IssuanceYear, ProjectDetails<T>, <T as Config>::MaxProjectIdList>;

/// The data stored for a pool
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool<AccountId, PoolConfig, CreditsMap> {
    /// The admin of the pool
    pub admin: AccountId,
    /// The configs applicable to this pool
    pub config: PoolConfig,
    /// The maximum limit of projects this pool can accept
    pub max_limit: u32,
    /// The credits stored in the pool
    pub credits: CreditsMap,
}

/// Pool config for CarbonCredits pools pallet
pub type PoolConfigOf<T> = PoolConfig<RegistryNameList<T>, MaxProjectIdList<T>>;

/// Pool for this pallet
pub type PoolOf<T> = Pool<<T as frame_system::Config>::AccountId, PoolConfigOf<T>, CreditsMap<T>>;
