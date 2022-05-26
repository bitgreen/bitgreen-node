use crate::pallet;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;

/// AuthorizedAccounts type of pallet
pub type AuthorizedAccountsListOf<T> = BoundedVec<
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::MaxAuthorizedAccountCount,
>;

pub type ShortStringOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxShortStringLength>;

pub type LongStringOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxLongStringLength>;

pub type IpfsLinkOf<T> = BoundedVec<u8, <T as pallet::Config>::MaxIpfsReferenceLength>;

pub type IpfsLinkListsOf<T> = BoundedVec<IpfsLinkOf<T>, <T as pallet::Config>::MaxDocumentCount>;

pub type BatchGroupOf<T> = BoundedVec<Batch<T>, <T as pallet::Config>::MaxGroupSize>;

pub type SDGTypesListOf<T> = BoundedVec<SDGDetails<ShortStringOf<T>>, ConstU32<12>>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegistryDetails<StringType> {
    pub name: StringType,
    pub id: StringType,
    pub summary: StringType,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SdgType {
    NoPoverty,
    ZeroHunger,
    GoodHealthAndWellBeing,
    QualityEducation,
    GenderEquality,
    CleanWaterAndSanitation,
    AffordableAndCleanEnergy,
    DecentWorkAndEconomicGrowth,
    IndustryInnovationAndInfrastructure,
    ReducedInequalities,
    SustainableCitiesAndCommunities,
    ResponsibleConsumptionAndProduction,
    ClimateAction,
    LifeBelowWater,
    LifeOnLand,
    PeaceJusticeAndStrongInstitutions,
    ParternshipsForTheGoals,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SDGDetails<StringType> {
    pub sdg_type: SdgType,
    pub description: StringType,
    pub refrences: StringType,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch<T: pallet::Config> {
    pub name: ShortStringOf<T>,
    pub uuid: ShortStringOf<T>,
    pub issuance_year: u32,
    pub start_date: u32,
    pub end_date: u32,

    // batch credits details
    pub total_supply: T::Balance,
    pub minted: T::Balance,
    pub retired: T::Balance,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectCreateParams<T: pallet::Config> {
    // Project Details
    pub name: ShortStringOf<T>,
    pub description: LongStringOf<T>,
    // TODO : Improve this data type
    pub location: [(u32, u32); 8],
    pub images: IpfsLinkListsOf<T>,
    pub videos: IpfsLinkListsOf<T>,
    pub documents: IpfsLinkListsOf<T>,
    // Registry details
    pub registry_details: RegistryDetails<ShortStringOf<T>>,

    // SDG details
    pub sdg_details: SDGTypesListOf<T>,

    // batch details
    pub batches: BatchGroupOf<T>,

    // credits details
    pub unit_price: T::Balance,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectDetail<T: pallet::Config> {
    // Project Details
    pub originator: T::AccountId,
    pub name: ShortStringOf<T>,
    pub description: LongStringOf<T>,
    // TODO : Improve this data type
    pub location: [(u32, u32); 8],
    pub images: IpfsLinkListsOf<T>,
    pub videos: IpfsLinkListsOf<T>,
    pub documents: IpfsLinkListsOf<T>,
    // Registry details
    pub registry_details: RegistryDetails<ShortStringOf<T>>,

    // SDG details
    pub sdg_details: SDGTypesListOf<T>,

    // batch details
    pub batches: BatchGroupOf<T>,

    // origination details
    pub created: T::Moment,
    pub updated: Option<T::Moment>,

    // approval details
    pub approved: bool,

    // credits details
    pub asset_id: Option<T::AssetId>,
    pub total_supply: T::Balance,
    pub minted: T::Balance,
    pub retired: T::Balance,
    pub unit_price: T::Balance,
}
