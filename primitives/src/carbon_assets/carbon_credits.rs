use super::*;

/// Representation of a group of credits. Groups are collections of batches of credits
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CarbonCreditBatchGroup<StringType, AssetId, Balance, Batch, MaxBatches: Get<u32>> {
	/// Descriptive name for this batch of credits
	pub name: StringType,
	/// UUID for this batch group
	pub uuid: StringType,
	/// AssetId representing the asset for this group
	pub asset_id: AssetId,
	/// The total_supply of the credits - this represents the total supply of the
	/// credits in all the batches of group.
	pub total_supply: Balance,
	/// The amount of tokens minted for this group
	pub minted: Balance,
	/// The amount of tokens minted for this group
	pub retired: Balance,
	/// The list of batches of credits
	/// A group can represent Carbon credits from multiple batches
	/// For example a project can have 100 tokens of 2019 vintage and 200 tokens of 2020 vintage.
	/// In this case the project can package these two vintages to create a carbon credit token
	/// that has a supply of 300 tokens. These vintages can be represented inside a batchgroup, in
	/// this case, it is important to remember that the minting and retirement always gives
	/// priority to the oldest vintage. Example : in the above case of 300 tokens, when the
	/// originator mints 100 tokens, we first mint the oldest (2019) credits and only once the
	/// supply is exhausted we move on the next vintage, same for retirement.
	pub batches: BoundedVec<Batch, MaxBatches>,
}

/// Credits in a project are represented in terms of batches, these batches are usually seperated in
/// terms of 'vintages'. The vintage refers to the `age` of the credit. So a batch could hold
/// 500credits with 2020 vintage. We use `issuance_year` to represent the vintage of the credit,
/// this is important in minting and retirement options since in a project with multiple vintages we
/// always mint/retire tokens from the oldest vintage.
///
/// When a project is created, we take the total supply of the credits available (entire supply in
/// the registry), then as the originator chooses, tokens can be minted for each credit at once or
/// in a staggered manner. In every mint, the `minted` count is incremented and when credit is
/// retired, the `retired` count is incremented.
///
/// Conditions :
///    - `minted` is always less than or equal to `total_supply`
///     - `retired` is always less than or equal to `minted`
///
///  Example : For a project that has a supply of 100 tokens, minted and retired 100 tokens, the
/// struct will look as follows   Batch {
///         ...,
///         total_supply : 100,
///         minted : 100,
///         retired : 100
///     }
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CarbonCreditBatch<StringType, Balance> {
	/// Descriptive name for this batch of credits
	pub name: StringType,
	/// UUID for this batch, usually provided by the registry
	pub uuid: StringType,
	/// The year the associated credits were issued
	pub issuance_year: IssuanceYear,
	/// start date for multi year batch
	pub start_date: u16,
	/// end date for multi year batch
	pub end_date: u16,
	/// The total_supply of the credits - this represents the total supply of the
	/// credits in the registry.
	pub total_supply: Balance,
	/// The amount of tokens minted for this VCU
	pub minted: Balance,
	/// The amount of tokens minted for this VCU
	pub retired: Balance,
}
