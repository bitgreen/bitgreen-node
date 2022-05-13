use crate::pallet;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;

/// a value: json structure as follows:
/// {
///     Description: Vec<u8> (max 64 bytes) (mandatory)
///     ProofOwnership: ipfs link to a folder with the proof of ownership (mandatory)
///     OtherDocuments: [{description:string,ipfs:hash},{....}], (Optional)
///     ExpiringDateTime: DateTime, (YYYY-MM-DD hh:mm:ss) (optional)
///     NumberofShares: Integer (maximum 10000 shares mandatory)
/// }
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetGeneratingVCUContent<Time> {
    pub description: Vec<u8>,
    pub proof_of_ownership: Vec<u8>,
    pub other_documents: Option<Vec<u8>>,
    pub expiry: Option<Time>,
    pub number_of_shares: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetsGeneratingVCUScheduleContent {
    pub period_days: u64,
    pub amount_vcu: u128,
    pub token_id: u32,
}

/// To store a "bundle" of AGV that has the constraint of using the same "asset id"
/// but potentially different schedules or Oracle for the generation of the VCU.
///
/// example: {"description":"xxxxxxx","agvs":[{"accountid","xxxxxxx","id":xx},{..}],assetid:xx}
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BundleAssetGeneratingVCUContent<AccountId, AssetId> {
    pub description: Vec<u8>,
    pub asset_id: AssetId,
    pub bundle: Vec<BundleAssetGeneratingVCUData<AccountId>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BundleAssetGeneratingVCUData<AccountId> {
    pub account_id: AccountId,
    pub id: u32,
}
