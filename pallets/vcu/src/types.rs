use crate::pallet;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;

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
