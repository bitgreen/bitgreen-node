use super::*;
use sp_runtime::Percent;

pub type IssuanceYear = u32;

/// The possible values for Registry Names
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RegistryName {
    Verra,
    GoldStandard,
    AmericanCarbonRegistry,
    ClimateActionReserve,
}

/// Data to represent the data of the project as recoreded by the respective Registry
/// This might differ from the project owner's name/description and hence important to store
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegistryDetails<StringType> {
    /// The name of registry the project belongs to
    pub registry: RegistryName,
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
    pub references: StringType,
}

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
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch<StringType, Balance> {
    /// Descriptive name for this batch of credits
    pub name: StringType,
    /// UUID for this batch, usually provided by the registry
    pub uuid: StringType,
    /// The year the associated credits were issued
    pub issuance_year: IssuanceYear,
    /// start date for multi year batch
    pub start_date: u32,
    /// end date for multi year batch
    pub end_date: u32,
    /// The total_supply of the credits - this represents the total supply of the
    /// credits in the registry.
    pub total_supply: Balance,
    /// The amount of tokens minted for this VCU
    pub minted: Balance,
    /// The amount of tokens minted for this VCU
    pub retired: Balance,
}

/// The details of a retired batch of VCU
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRetireData<StringType, Balance> {
    /// Name of the batch
    pub name: StringType,
    /// uuid of the batch
    pub uuid: StringType,
    /// issuance_year of the batch
    pub issuance_year: IssuanceYear,
    /// The count of tokens retired
    pub count: Balance,
}
