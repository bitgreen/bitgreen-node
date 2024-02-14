use super::*;

/// Representation of a donation to a group of credits
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Default, Debug, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DonationsBatchGroup<StringType, Balance> {
	/// Descriptive name for this batch of donation
	pub name: StringType,
	/// UUID for this batch group
	pub uuid: StringType,
	/// The total_supply of the credits - this represents the total supply of the
	/// credits in all the batches of group.
	pub total_supply: Balance,
	/// AssetId representing the asset for this group
	pub asset_id: AssetId,
	/// The amount of tokens minted for this group
	pub minted: Balance,
}
