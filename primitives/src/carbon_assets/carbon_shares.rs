use super::*;

/// Representation of a group of Shares. Groups are collections of batches of Shares
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CarbonSharesBatchGroup<StringType, AssetId, Balance, Batch, MaxBatches: Get<u32>> {
	/// Descriptive name for this batch of Shares
	pub name: StringType,
	/// UUID for this batch group
	pub uuid: StringType,
	/// AssetId representing the asset for this group
	pub asset_id: AssetId,
	/// The total_supply of the Shares - this represents the total supply of the
	/// Shares in all the batches of group.
	pub total_supply: Balance,
	/// The amount of tokens minted for this group
	pub minted: Balance,
	/// The amount of tokens converted to forwards
	pub converted_to_forwards: Balance,
	/// The list of batches of Shares
	/// A group can represent Carbon Shares from multiple batches
	/// For example a project can have 100 tokens of 2019 vintage and 200 tokens of 2020 vintage.
	/// In this case the project can package these two vintages to create a carbon Shares token
	/// that has a supply of 300 tokens. These vintages can be represented inside a batchgroup, in
	/// this case, it is important to remember that the minting and retirement always gives
	/// priority to the oldest vintage. Example : in the above case of 300 tokens, when the
	/// originator mints 100 tokens, we first mint the oldest (2019) Shares and only once the
	/// supply is exhausted we move on the next vintage, same for retirement.
	pub batches: BoundedVec<Batch, MaxBatches>,
}

/// Shares in a project are represented in terms of batches, these batches are usually seperated in
/// terms of 'vintages'. The vintage refers to the `age` of the Shares. So a batch could hold
/// 500Shares with 2020 vintage. We use `issuance_year` to represent the vintage of the Shares,
/// this is important in minting and retirement options since in a project with multiple vintages we
/// always mint/retire tokens from the oldest vintage.
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CarbonSharesBatch<StringType, Balance> {
	/// Descriptive name for this batch of Shares
	pub name: StringType,
	/// UUID for this batch, usually provided by the registry
	pub uuid: StringType,
	/// The year the associated Shares were issued
	pub issuance_year: IssuanceYear,
	/// start date for multi year batch
	pub start_date: u16,
	/// end date for multi year batch
	pub end_date: u16,
	/// The total_supply of the Shares - this represents the total supply of the
	/// Shares in the registry.
	pub total_supply: Balance,
	/// The amount of tokens minted for this VCU
	pub minted: Balance,
	/// The amount of tokens converted to forwards
	pub converted_to_forwards: Balance,
}
