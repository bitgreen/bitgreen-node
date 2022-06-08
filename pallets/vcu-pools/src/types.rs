use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use primitives::RegistryName;
use scale_info::TypeInfo;

pub type RegistryNameList<T> = BoundedVec<RegistryName, <T as Config>::MaxRegistryListCount>;

pub type IssuanceYearList<T> = BoundedVec<u32, <T as Config>::MaxIssuanceYearCount>;

pub type MaxProjectIdList<T> = BoundedVec<u32, <T as Config>::MaxProjectIdList>;

pub type SymbolStringOf<T> = BoundedVec<u8, <T as Config>::MaxAssetSymbolLength>;

/// The configuration of a pool
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default, TypeInfo, MaxEncodedLen)]
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

pub type CreditsList<T> = BoundedVec<VCUData, <T as Config>::MaxProjectIdList>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool<AccountId, PoolConfig, SymbolString, CreditsList> {
    pub admin: AccountId,
    pub config: PoolConfig,
    pub max_limit: Option<u32>,
    pub asset_symbol: SymbolString,
    pub credits: CreditsList,
}

pub type PoolConfigOf<T> =
    PoolConfig<RegistryNameList<T>, IssuanceYearList<T>, MaxProjectIdList<T>>;

pub type PoolOf<T> = Pool<
    <T as frame_system::Config>::AccountId,
    PoolConfigOf<T>,
    SymbolStringOf<T>,
    CreditsList<T>,
>;
