use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::DispatchResult;
use frame_support::{ensure, traits::Contains};
use frame_support::{BoundedBTreeMap, BoundedVec};
use primitives::{IssuanceYear, RegistryName};
use scale_info::TypeInfo;

pub type RegistryNameList<T> = BoundedVec<RegistryName, <T as Config>::MaxRegistryListCount>;

pub type IssuanceYearList<T> = BoundedVec<u32, <T as Config>::MaxIssuanceYearCount>;

pub type MaxProjectIdList<T> =
    BoundedVec<<T as pallet_vcu::Config>::AssetId, <T as Config>::MaxProjectIdList>;

pub type SymbolStringOf<T> = BoundedVec<u8, <T as Config>::MaxAssetSymbolLength>;

/// The configuration of a pool
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolConfig<RegistryList, MaxProjectIdList> {
    pub registry_list: Option<RegistryList>,
    pub project_id_list: Option<MaxProjectIdList>,
}

pub type ProjectDetails<T> = BoundedBTreeMap<
    <T as pallet_vcu::Config>::AssetId,
    <T as pallet_vcu::Config>::Balance,
    <T as Config>::MaxProjectIdList,
>;

pub type CreditsMap<T> =
    BoundedBTreeMap<IssuanceYear, ProjectDetails<T>, <T as Config>::MaxProjectIdList>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool<AccountId, PoolConfig, SymbolString, CreditsMap> {
    pub admin: AccountId,
    pub config: PoolConfig,
    pub max_limit: u32,
    pub asset_symbol: SymbolString,
    pub credits: CreditsMap,
}

pub type PoolConfigOf<T> = PoolConfig<RegistryNameList<T>, MaxProjectIdList<T>>;

pub type PoolOf<T> =
    Pool<<T as frame_system::Config>::AccountId, PoolConfigOf<T>, SymbolStringOf<T>, CreditsMap<T>>;
