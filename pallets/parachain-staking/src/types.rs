use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::DispatchResult;
use frame_support::pallet_prelude::Get;
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;

use super::*;

pub type RoundIndex = u32;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
/// The current round index and transition information
pub struct RoundInfo<BlockNumber> {
	/// Current round index
	pub current: RoundIndex,
	/// The first block of the current round
	pub first: BlockNumber,
	/// The length of the current round in number of blocks
	pub length: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
/// Information on candidate
pub struct CandidateInfo<AccountId, Amount> {
	/// The account id of the candidate
	pub account: AccountId,
	/// The amount bonded by the candidate
	pub bonded: Amount,
	/// The amount delegated to the candidate
	pub delegated: Amount,
	/// The total bonded amount, this should be the bonded + delegated
	pub total_bond: Amount,
}

/// Information on candidate
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Delegation<AccountId, Amount> {
	/// The account id of the delegate
	pub account: AccountId,
	/// The amount bonded by the delegate
	pub amount: Amount,
}

pub type CandidateInfoOf<T> = CandidateInfo<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

pub type DelegationListOf<T> = BoundedVec<
	Delegation<<T as frame_system::Config>::AccountId, BalanceOf<T>>,
	<T as Config>::MaxDelegationsPerCandidate,
>;

/// List of candidates ordered in decreasing order of total_bond
#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderedCandidateSet<AccountId, Amount, MaxCandidates: Get<u32>>(
	BoundedVec<CandidateInfo<AccountId, Amount>, MaxCandidates>,
);

impl<AccountId: std::cmp::PartialEq, Amount: Eq + Ord, MaxCandidates: Get<u32>>
	OrderedCandidateSet<AccountId, Amount, MaxCandidates>
{
	/// Create a new ordered candidate set
	pub fn new() -> Self { Self(BoundedVec::<_, _>::default()) }

	/// Insert a new candidate to the set
	pub fn insert_candidate(
		&mut self,
		new_candidate: CandidateInfo<AccountId, Amount>,
	) -> Result<(), ()> {
		// insert the candidate
		self.0.try_push(new_candidate)?;
		// sort again to ensure stable ordering
		self.0
			.as_mut()
			.sort_by(|a, b| b.total_bond.cmp(&a.total_bond));
		Ok(())
	}

	/// Returns true if given candidate is in the set
	pub fn is_candidate(&self, account: &AccountId) -> bool {
		if self.0.iter().any(|c| &c.account == account) {
			return true;
		}
		return false;
	}
}

pub type OrderedCandidateSetOf<T> = OrderedCandidateSet<
	<T as frame_system::Config>::AccountId,
	BalanceOf<T>,
	<T as pallet::Config>::MaxCandidates,
>;
