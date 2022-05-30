use crate::pallet;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use sp_runtime::Percent;

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

/// Data to represent the data of the project as recoreded by the respective Registry
/// This might differ from the project owner's name/description and hence important to store
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegistryDetails<StringType> {
    /// The name of the project in the registry
    pub name: StringType,
    /// The id of the project in the registry
    pub id: StringType,
    /// The project summary in the registry
    pub summary: StringType,
}

/// The possible values for SDG's addressed by a project
/// Full list here : https://sdgs.un.org/
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

/// The details of SDGs that the project addresses
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SDGDetails<StringType> {
    /// The `SdgType` that the project solves
    pub sdg_type: SdgType,
    /// Short description of how the project solves the SDG
    pub description: StringType,
    /// A reference to the project docs related to SDG
    pub refrences: StringType,
}

/// A project can address more than one SDG, this type stores the
/// list of SDGs the project addresses, upper bound is max number of existing SDGs
pub type SDGTypesListOf<T> = BoundedVec<SDGDetails<ShortStringOf<T>>, ConstU32<17>>;

/// Projects can have rolyalties attached to the tokens, these royalties
/// are paid out when the token is purchased
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Royalty<AccountId> {
    /// The account_id of the royalty recipeint
    pub account_id: AccountId,
    /// The percent of fees to be paid to the recipient
    pub percent_of_fees: Percent,
}

/// List of royalty recipients for a project
pub type RoyaltyRecipientsOf<T> = BoundedVec<
    Royalty<<T as frame_system::Config>::AccountId>,
    <T as pallet::Config>::MaxRoyaltyRecipients,
>;

/// Credits in a project are represented in terms of batches, these batches are usually seperated in terms of 'vintages'. The vintage
/// refers to the `age` of the credit. So a batch could hold 500credits with 2020 vintage.
/// We use `issuance_year` to represent the vintage of the credit, this is important in minting and retirement options since in a project
/// with multiple vintages we always mint/retire tokens from the oldest vintage.
///
/// When a project is created, we take the total supply of the credits available (entire supply in the registry), then as the originator
/// chooses, tokens can be minted for each credit at once or in a staggered manner. In every mint, the `minted` count is incremented and
/// when credit is retired, the `retired` count is incremented.
///
/// Conditions :
///    - `minted` is always less than or equal to `total_supply`
///     - `retired` is always less than or equal to `minted`
///
///  Example : For a project that has a supply of 100 tokens, minted and retired 100 tokens, the struct will look as follows
///   Batch {
///         ...,
///         total_supply : 100,
///         minted : 100,
///         retired : 100
///     }
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[derive(frame_support::DebugNoBound)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch<T: pallet::Config> {
    /// Descriptive name for this batch of credits
    pub name: ShortStringOf<T>,
    /// UUID for this batch, usually provided by the registry
    pub uuid: ShortStringOf<T>,
    /// The year the associated credits were issued
    pub issuance_year: u32,
    /// start date for multi year batch
    pub start_date: u32,
    /// end date for multi year batch
    pub end_date: u32,
    /// The total_supply of the credits - this represents the total supply of the
    /// credits in the registry.
    pub total_supply: T::Balance,
    /// The amount of tokens minted for this VCU
    pub minted: T::Balance,
    /// The amount of tokens minted for this VCU
    pub retired: T::Balance,
}

/// A project can represent VCUs from multiple batches
/// For example a project can have 100 tokens of 2019 vintage and 200 tokens of 2020 vintage. In this case
/// the project can package these two vintages to create a vcu token that has a supply of 300 tokens.
/// These vintages can be represented inside a batchgroup, in this case, it is important to remember that
/// the minting and retirement always gives priority to the oldest vintage.
/// Example : in the above case of 300 tokens, when the originator mints 100 tokens, we first mint the oldest (2019) credits
/// and only once the supply is exhausted we move on the next vintage, same for retirement.
pub type BatchGroupOf<T> = BoundedVec<Batch<T>, <T as pallet::Config>::MaxGroupSize>;

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
    pub location: [(u32, u32); 8],
    /// List of ipfs-hashes of images related to the project
    pub images: IpfsLinkListsOf<T>,
    /// List of ipfs-hashes of videos related to the project
    pub videos: IpfsLinkListsOf<T>,
    /// List of ipfs-hashes of documents related to the project
    pub documents: IpfsLinkListsOf<T>,
    /// Details of the project as represented in registry
    pub registry_details: RegistryDetails<ShortStringOf<T>>,
    /// SDG details
    pub sdg_details: SDGTypesListOf<T>,
    /// List of batches in the project
    pub batches: BatchGroupOf<T>,
    // Price in USD for a single credit
    pub unit_price: T::Balance,
    /// The royalties to be paid when tokens are purchased
    pub royalties: RoyaltyRecipientsOf<T>,
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
    pub location: [(u32, u32); 8],
    /// List of ipfs-hashes of images related to the project
    pub images: IpfsLinkListsOf<T>,
    /// List of ipfs-hashes of videos related to the project
    pub videos: IpfsLinkListsOf<T>,
    /// List of ipfs-hashes of documents related to the project
    pub documents: IpfsLinkListsOf<T>,
    /// Details of the project as represented in registry
    pub registry_details: RegistryDetails<ShortStringOf<T>>,
    /// SDG details
    pub sdg_details: SDGTypesListOf<T>,
    /// List of batches in the project
    pub batches: BatchGroupOf<T>,
    /// The royalties to be paid when tokens are purchased
    pub royalties: RoyaltyRecipientsOf<T>,

    // origination details
    /// Creation time of project
    pub created: T::Moment,
    /// Last updation time of project
    pub updated: Option<T::Moment>,

    /// approval status - a project can only mint tokens once approved
    pub approved: bool,

    // credits details
    /// The asset_id for the project
    pub asset_id: Option<T::AssetId>,
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
