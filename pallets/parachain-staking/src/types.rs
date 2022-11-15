use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::{pallet_prelude::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;

use super::*;

/// Basic information about a collation candidate.
#[derive(
	PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
)]
pub struct CandidateInfo<AccountId, Balance, DelegationInfo, MaxDelegators: Get<u32>> {
	/// Account identifier.
	pub who: AccountId,
	/// Reserved deposit from candidate
	pub deposit: Balance,
	/// List of delegators
	pub delegators: BoundedVec<DelegationInfo, MaxDelegators>,
	/// List of total stake (candidate + delegators)
	pub total_stake: Balance,
}

/// Basic information about a delegator
#[derive(
	PartialEq,
	Eq,
	Clone,
	Encode,
	Decode,
	RuntimeDebug,
	scale_info::TypeInfo,
	MaxEncodedLen,
	PartialOrd,
	Ord,
)]
pub struct DelegationInfo<AccountId, Balance> {
	/// Account identifier.
	pub who: AccountId,
	/// Reserved deposit.
	pub deposit: Balance,
}

pub type DelegationInfoOf<T> = DelegationInfo<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

pub type CandidateInfoOf<T> = CandidateInfo<
	<T as frame_system::Config>::AccountId,
	BalanceOf<T>,
	DelegationInfoOf<T>,
	<T as Config>::MaxDelegators,
>;
