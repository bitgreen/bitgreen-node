use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::Get, BoundedVec, RuntimeDebug};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Basic information about a collation candidate.
#[derive(
	PartialEq,
	Eq,
	Clone,
	Encode,
	Decode,
	RuntimeDebug,
	scale_info::TypeInfo,
	MaxEncodedLen,
	Ord,
	PartialOrd,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DelegationInfo<AccountId, Balance> {
	/// Account identifier.
	pub who: AccountId,
	/// Reserved deposit.
	pub deposit: Balance,
}

/// Basic information about a delegator unbonding
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UnbondedDelegationInfo<Balance, BlockNumber> {
	/// Reserved deposit.
	pub deposit: Balance,
	/// Unbonded block
	pub unbonded_at: BlockNumber,
}

/// Basic information about a unbonded candidate.
#[derive(
	PartialEq,
	Eq,
	Clone,
	Encode,
	Decode,
	RuntimeDebug,
	scale_info::TypeInfo,
	MaxEncodedLen,
	Ord,
	PartialOrd,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UnbondedCandidateInfo<Balance, DelegationInfo, MaxDelegators: Get<u32>, BlockNumber> {
	/// Reserved deposit from candidate
	pub deposit: Balance,
	/// List of delegators
	pub delegators: BoundedVec<DelegationInfo, MaxDelegators>,
	/// List of total stake (candidate + delegators)
	pub total_stake: Balance,
	/// Unbonded block
	pub unbonded_at: BlockNumber,
}

pub type DelegationInfoOf<T> = DelegationInfo<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

pub type CandidateInfoOf<T> = CandidateInfo<
	<T as frame_system::Config>::AccountId,
	BalanceOf<T>,
	DelegationInfoOf<T>,
	<T as Config>::MaxDelegators,
>;

pub type UnbondedDelegationInfoOf<T> =
	UnbondedDelegationInfo<BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

pub type UnbondedCandidateInfoOf<T> = UnbondedCandidateInfo<
	BalanceOf<T>,
	DelegationInfoOf<T>,
	<T as Config>::MaxDelegators,
	<T as frame_system::Config>::BlockNumber,
>;
