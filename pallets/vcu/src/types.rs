use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;

/// The input params for creating a new VCU
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCUCreationParams<AccountId, Balance, VcuId, BundleList> {
    /// The type of VCU used [Single, Bundle]
    pub vcu_type: VCUType<VcuId, BundleList>,
    /// The account that owns/controls the VCU class
    pub originator: AccountId,
    /// The amount of VCU units to create
    pub amount: Balance,
    /// The account that receives the amount of VCU units
    pub recipient: AccountId,
}

/// The VCUDetails as stored in pallet storage
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCUDetail<AccountId, Balance, AssetId, VcuId, BundleList> {
    /// The account that owns/controls the VCU class
    pub originator: AccountId,
    /// Count of current active units of VCU
    pub supply: Balance,
    /// Count of retired units of VCU
    pub retired: Balance,
    /// The AssetId that represents the Fungible class of VCU
    pub asset_id: AssetId,
    /// The type of VCU [Bundle, Single]
    pub vcu_type: VCUType<VcuId, BundleList>,
}

/// The types of VcuId, VCUs can be created from one single type or can be a mix
/// of multiple different types called a Bundle
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VCUType<VcuId, BundleList> {
    /// Represents a list of different types of VCU units
    Bundle(BundleList),
    /// Represents a single type
    Single(VcuId),
}
