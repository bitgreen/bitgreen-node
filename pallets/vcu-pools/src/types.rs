use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::Get;
use frame_support::BoundedVec;
use primitives::RegistryName;
use scale_info::TypeInfo;

pub type RegistryNameList<T> = BoundedVec<RegistryName, <T as Config>::MaxRegistryListCount>;

pub type IssuanceYearList<T> = BoundedVec<u32, <T as Config>::MaxIssuanceYearCount>;

pub type MaxProjectIdList<T> = BoundedVec<u32, <T as Config>::MaxProjectIdList>;

pub type SymbolStringOf<T> = BoundedVec<u32, <T as Config>::MaxAssetSymbolLength>;

/// The configuration of a pool
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolConfig<RegistryList, IssuanceYearList, MaxProjectIdList> {
    pub registry_list: Option<RegistryList>,
    pub issuance_year_list: Option<IssuanceYearList>,
    pub project_id_list: Option<MaxProjectIdList>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCUData {
    pub project_id: u32,
    pub issuance_year: u32,
    pub amount: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool<
    AccountId,
    RegistryList,
    IssuanceYearList,
    MaxProjectIdList,
    MaxVCUProjectsInPool: Get<u32>,
    SymbolString,
> {
    pub admin: AccountId,
    pub config: PoolConfig<RegistryList, IssuanceYearList, MaxProjectIdList>,
    pub max_limit: u32,
    pub asset_symbol: SymbolString,
    pub credits: BoundedVec<VCUData, MaxVCUProjectsInPool>,
}

pub type PoolOf<T> = Pool<
    <T as frame_system::Config>::AccountId,
    RegistryNameList<T>,
    IssuanceYearList<T>,
    MaxProjectIdList<T>,
    <T as Config>::MaxVCUProjectsInPool,
    SymbolStringOf<T>,
>;
