#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_std::prelude::*;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
//use sp_core::u32_trait::{ _3, _4,};
use sp_runtime::{
	ApplyExtrinsicResult, generic, create_runtime_str, impl_opaque_keys, MultiSignature,
	Perbill, curve::PiecewiseLinear,
	transaction_validity::{TransactionValidity, TransactionSource,TransactionPriority},
};
use sp_runtime::traits::{
	AccountIdLookup, OpaqueKeys, BlakeTwo256, Block as BlockT, Verify, IdentifyAccount, NumberFor,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use pallet_grandpa::fg_primitives;
use sp_version::RuntimeVersion;
//use frame_system::{EnsureRoot, EnsureOneOf};
use frame_system::{EnsureRoot};
use pallet_session::historical as session_historical;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

// staking pallets
use sp_staking::SessionIndex;

#[cfg(feature = "std")]
pub use pallet_staking::StakerStatus;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
pub use frame_support::{
	construct_runtime, parameter_types, StorageValue,
	traits::{KeyOwnerProofSystem, Randomness},
	weights::{
		Weight, IdentityFee,
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
	},
};
use pallet_transaction_payment::CurrencyAdapter;

/// import pallet contracts (!Ink Native language)
use pallet_contracts::weights::WeightInfo;

/// Constant values used within the runtime.
/// weights definition for gas fees charge in the runtime 
mod weights;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;
// collective definition
type CouncilCollective = pallet_collective::Instance1;
type TechnicalCollective = pallet_collective::Instance2;

// Election Session
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
//pub const ElectionLookahead: BlockNumber = EPOCH_DURATION_IN_BLOCKS / 4;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub babe: Babe,
			pub grandpa: Grandpa,
			pub im_online: ImOnline,
		}
	}
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("bitg-node"),
	impl_name: create_runtime_str!("bitg-node"),
	authoring_version: 1,
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

/// The BABE epoch configuration at genesis.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
pub const BABE_GENESIS_EPOCH_CONFIG: babe_primitives::BabeEpochConfiguration =
	babe_primitives::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: babe_primitives::AllowedSlots::PrimaryAndSecondaryVRFSlots
	};

/// An instant or duration in time.
pub type Moment = u64;

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 4 * HOURS;
// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// Constant definition used for Contracts Pallet
/// Contracts price units.
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;
const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}
/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// end definitions for Contracts pallet

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

// Configure the modules (pallets) to be included in runtime.

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
}
//Aura pallet - POA Consensus (to be remove before lauching)
impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
}
// This manages the GRANDPA authority set ready for the native code. These authorities are only for GRANDPA finality, not for consensus overall.
impl pallet_grandpa::Config for Runtime {
	//The event type of this module.
	type Event = Event;
	//The function call.
	type Call = Call;
	// A system for proving ownership of keys, i.e. that a given key was part of a validator set, needed for validating equivocation reports.
	type KeyOwnerProofSystem = Historical;
	// The proof of key ownership, used for validating equivocation reports. The proof must include the session index and validator count of the session at which the equivocation occurred.
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    // The identification of a key owner, used when reporting equivocations.
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;
	// The equivocation handling subsystem, defines methods to report an offence (after the equivocation has been validated) and 
	// for submitting a transaction to report an equivocation (from an offchain context). 
	// When enabling equivocation handling (i.e. this type isn't set to ()) you must use this pallet's ValidateUnsigned in the runtime definition.
	type HandleEquivocation = pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	//Weights for this pallet.
	type WeightInfo = ();
}
// Pallet iam-online
// if the local node is a validator (i.e. contains an authority key), this module gossips a heartbeat transaction with each new session. 
// The heartbeat functions as a simple mechanism to signal that the node is online in the current era.
// Received heartbeats are tracked for one era and reset with each new era. 
// The module exposes two public functions to query if a heartbeat has been received in the current era or session.
// The heartbeat is a signed transaction, which was signed using the session key and includes the recent best block number of the local validators chain as well as the NetworkState. 
// It is submitted as an Unsigned Transaction via off-chain workers.
parameter_types! {
	pub NposSolutionPriority: TransactionPriority =
		Perbill::from_percent(90) * TransactionPriority::max_value();
	// we set the pallet at the maximum prioriryt fo unsigned trasactions
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	// set session duration in blocks number
	pub SessionDuration: BlockNumber = 10;
}
impl pallet_im_online::Config for Runtime {
	// The identifier type for an authority.
	type AuthorityId = ImOnlineId;
	// The overarching event type.
	type Event = Event;
	// A type for retrieving the validators supposed to be online in a session.
	type ValidatorSet = Historical;
	// An expected duration of the session.
    // This parameter is used to determine the longevity of heartbeat transaction and 
	// a rough time when we should start considering sending heartbeats, since the workers avoids sending them at the very beginning of the session, 
	// assuming there is a chance the authority will produce a block and they won't be necessary.
	type SessionDuration= SessionDuration;
	// It gives us the ability to submit unresponsiveness offence reports.
	type ReportUnresponsiveness = Offences;
	// A configuration for base priority of unsigned transactions.
    // It can be tuned when multiple pallets send unsigned transactions.
	type UnsignedPriority = ImOnlineUnsignedPriority;
	// Weight information for gas fees on extrinsics of this pallet.
	type WeightInfo = weights::pallet_im_online::WeightInfo<Runtime>;
}
// end Pallet iam-online

// Pallet timestamp
// The Timestamp pallet allows the validators to set and validate a timestamp with each block.
// It uses inherents for timestamp data, which is provided by the block author and validated/verified by other validators. 
// The timestamp can be set only once per block and must be set each block. 
// There could be a constraint on how much time must pass before setting the new timestamp.
// The Timestamp pallet is the recommended way to query the on-chain time instead of using an approach based on block numbers. 
// The block number based time measurement can cause issues because of cumulative calculation errors and hence should be avoided.
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}
// end timestampp pallet

// Implementation for Contracts Pallet
parameter_types! {
    pub const TombstoneDeposit: Balance = deposit(
        1,
        sp_std::mem::size_of::<pallet_contracts::ContractInfo<Runtime>>() as u32
    );
    pub const DepositPerContract: Balance = TombstoneDeposit::get();
    pub const DepositPerStorageByte: Balance = deposit(0, 1);
    pub const DepositPerStorageItem: Balance = deposit(1, 0);
    pub RentFraction: Perbill = Perbill::from_rational_approximation(1u32, 30 * DAYS);
    pub const SurchargeReward: Balance = 150 * MILLICENTS;
    pub const SignedClaimHandicap: u32 = 2;
    pub const MaxDepth: u32 = 32;
    pub const MaxValueSize: u32 = 16 * 1024;
    // The lazy deletion runs inside on_initialize.
    pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
        BlockWeights::get().max_block;
    // The weight needed for decoding the queue should be less or equal than a fifth
    // of the overall weight dedicated to the lazy deletion.
    pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
        )) / 5) as u32;
    pub MaxCodeSize: u32 = 128 * 1024;
}

impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = MaxDepth;
    type MaxValueSize = MaxValueSize;
    type WeightPrice = pallet_transaction_payment::Module<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type MaxCodeSize = MaxCodeSize;
}
// End implementation for Contracts Pallet
// Authorship Pallet
parameter_types! {
	pub const UncleGenerations: u32 = 0;
}
impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ImOnline);
}
// end Authorship Pallet
// Balances Pallet
// The Balances module provides functionality for handling accounts and balances.
// The Balances module provides functions for:
// - Getting and setting free balances.
// - Retrieving total, reserved and unreserved balances.
// - Repatriating a reserved balance to a beneficiary account that exists.
// - Transferring a balance between accounts (when not reserved).
// - Slashing an account balance.
// - Account creation and removal.
// - Managing total issuance.
// - Setting and managing locks.
parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	//Handler for the unbalanced reduction when removing a dust account. (not set for now)
	type DustRemoval = ();
	//The minimum amount required to keep an account open.
	type ExistentialDeposit = ExistentialDeposit;
	//The means of storing the balances of an account.
	type AccountStore = System;
	// weight info for gas fees
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	//The maximum number of locks that should exist on an account. Not strictly enforced, but used for weight estimation.
	type MaxLocks = MaxLocks;
	
}
// End Balances Pallet

// Transactions fees payment Pallet
// This module provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included. This includes:
// - base fee: This is the minimum amount a user pays for a transaction. It is declared as a base weight in the runtime and converted to a fee using WeightToFee.
// - weight fee: A fee proportional to amount of weight a transaction consumes.
// - length fee: A fee proportional to the encoded length of the transaction.
// - tip: An optional tip. Tip increases the priority of the transaction, giving it a higher chance to be included by the transaction queue.
// - The base fee and adjusted weight and length fees constitute the inclusion fee, which is the minimum fee for a transaction to be included in a block.
// The formula of final fee:
//   inclusion_fee = base_fee + length_fee + [targeted_fee_adjustment * weight_fee];
//   final_fee = inclusion_fee + tip;
parameter_types! {
	pub const TransactionByteFee: Balance = 1;
}
impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}
// Super User (sudo) Pallet
// The Sudo module allows for a single account (called the "sudo key") to execute dispatchable functions that require 
// a Root call or designate a new account to replace them as the sudo key. Only one account can be the sudo key at a time.
impl pallet_sudo::Config for Runtime {
	// The overarching event type.
	type Event = Event;
	// A sudo-able call.
	type Call = Call;
}
//end sudo pallet

// pallet offences (used to slash funds of bad validators)
parameter_types! {
	pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * 100_000;
}
impl pallet_offences::Config for Runtime {
	// The overarching event type
	type Event = Event;
	//Full identification of the validator.
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	// the  handler called for every offence report.
	type OnOffenceHandler = Staking;
	// The a soft limit on maximum weight that may be consumed while dispatching deferred offences in on_initialize. 
	// Note it's going to be exceeded before we stop adding to it, so it has to be set conservatively.
	type WeightSoftLimit = OffencesWeightSoftLimit;
}
// end pallet offences 

// Pallet collective 
// Collective system: Members of a set of account IDs can make their collective feelings known through dispatched calls from one of two specialized origins.
// The membership can be provided in one of two ways: either directly, using the Root-dispatchable function set_members, or indirectly, through implementing the ChangeMembers. 
// The pallet assumes that the amount of members stays at or below MaxMembers for its weight calculations, but enforces this neither in set_members nor in change_members_sorted.
// A "prime" member may be set to help determine the default vote behavior based on chain config. If PreimDefaultVote is used, the prime vote acts as the default vote in case 
// of any abstentions after the voting period. If MoreThanMajorityThenPrimeDefaultVote is used, then abstentations will first follow the majority of the collective voting, and then the prime member.
// Voting happens through motions comprising a proposal (i.e. a curried dispatchable) plus a number of approvals required for it to pass and be called. 
// Motions are open for members to vote on for a minimum period given by MotionDuration. 
// As soon as the needed number of approvals is given, the motion is closed and executed. 
// If the number of approvals is not reached during the voting period, then close may be called by any account in order to force the end the motion explicitly. 
// If a prime member is defined then their vote is used in place of any abstentions and the proposal is executed if there are enough approvals counting the new votes.
// If there are not, or if no prime is set, then the motion is dropped without being executed.
parameter_types! {
	pub const CouncilCollectiveMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilCollectiveMaxProposals: u32 = 100;
    pub const CouncilCollectiveMaxMembers: u32 = 100;
    pub const TechnicalCollectiveMotionDuration: BlockNumber = 5 * DAYS;
    pub const TechnicalCollectiveMaxProposals: u32 = 100;
    pub const TechnicalCollectiveMaxMembers: u32 = 100;
}
impl pallet_collective::Config<CouncilCollective> for Runtime {
	// The outer origin type.
    type Origin = Origin;
	// The outer call dispatch type.
    type Proposal = Call;
	//The overachy event type
    type Event = Event;
	// The time-out for council motions.
    type MotionDuration = CouncilCollectiveMotionDuration;
	// Maximum number of proposals allowed to be active in parallel.
    type MaxProposals = CouncilCollectiveMaxProposals;
	// The maximum number of members supported by the pallet. Used for weight estimation.
    type MaxMembers = CouncilCollectiveMaxMembers;
	// Default vote strategy of this collective.
    type DefaultVote = pallet_collective::PrimeDefaultVote;
	// Weight information for extrinsics in this pallet.
    type WeightInfo = (); // no gas fees 
}
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	// The outer origin type.
    type Origin = Origin;
	// The outer call dispatch type.
    type Proposal = Call;
	//The overachy event type
    type Event = Event;
	// The time-out for council motions.
    type MotionDuration = TechnicalCollectiveMotionDuration;
	// Maximum number of proposals allowed to be active in parallel.
    type MaxProposals = TechnicalCollectiveMaxProposals;
	// The maximum number of members supported by the pallet. Used for weight estimation.
    type MaxMembers = TechnicalCollectiveMaxMembers;
	
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = ();
}
// Democracy module
// The Democracy pallet handles the administration of general stakeholder voting.
// There are two different queues that a proposal can be added to before it becomes a referendum, 
// 1) the proposal queue consisting of all public proposals and 
// 2) the external queue consisting of a single proposal that originates from one of the external origins (such as a collective group).
// Every launch period - a length defined in the runtime - the Democracy pallet launches a referendum 
// from a proposal that it takes from either the proposal queue or the external queue in turn. 
// Any token holder in the system can vote on referenda. 
// The voting system uses time-lock voting by allowing the token holder to set their conviction behind a vote. 
// The conviction will dictate the length of time the tokens will be locked, as well as the multiplier that scales the vote power.
parameter_types! {
	pub const LaunchPeriod: BlockNumber = 28 * DAYS;
	pub const VotingPeriod: BlockNumber = 28 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub const EnactmentPeriod: BlockNumber = 28 * DAYS;
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	// One cent: $10,000 / MB
	pub const PreimageByteDeposit: Balance = 1 * CENTS;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}
impl pallet_democracy::Config for Runtime {

	type Proposal = Call;
	// the event type like everywhere
	type Event = Event;
	// Currency type for this module.
	type Currency = Balances;
	// The minimum period of locking and the period between a proposal being approved and enacted.
	// It should generally be a little more than the unstake period to ensure that voting stakers have an opportunity to remove themselves from the system in the case where they are on the losing side of a vote.
	type EnactmentPeriod = EnactmentPeriod;
	// How often (in blocks) new public referenda are launched.
	type LaunchPeriod = LaunchPeriod;
	// How often (in blocks) to check for new votes.
	type VotingPeriod = VotingPeriod;
	// The minimum amount to be used as a deposit for a public referendum proposal.
	type MinimumDeposit = MinimumDeposit;
	// Origin from which the next tabled referendum may be forced. This is a normal "super-majority-required" referendum.
	type ExternalOrigin = EnsureRoot<AccountId>;
	// Origin from which the next tabled referendum may be forced; this allows for the tabling of a majority-carries referendum.
	type ExternalMajorityOrigin = EnsureRoot<AccountId>;
	// Origin from which the next tabled referendum may be forced; this allows for the tabling of a negative-turnout-bias (default-carries) referendum.
	type ExternalDefaultOrigin = EnsureRoot<AccountId>;
	// Origin from which the next majority-carries (or more permissive) referendum may be tabled to vote according to the FastTrackVotingPeriod asynchronously 
	// in a similar manner to the emergency origin. It retains its threshold method.
	type FastTrackOrigin = EnsureRoot<AccountId>;
	// Origin from which the next majority-carries (or more permissive) referendum may be tabled to vote immediately and 
	// asynchronously in a similar manner to the emergency origin. It retains its threshold method.
	type InstantOrigin = EnsureRoot<AccountId>;
	// Indicator for whether an emergency origin is even allowed to happen. 
	// We may want to set this permanently to false, others may want to condition it on things such as an upgrade having happened recently.
	type InstantAllowed = InstantAllowed;
	// Minimum voting period allowed for a fast-track referendum.
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// Origin from which any referendum may be cancelled in an emergency.
	type CancellationOrigin = EnsureRoot<AccountId>;
	// Origin from which any referendum may be cancelled in an emergency.
	type CancelProposalOrigin = EnsureRoot<AccountId>;
	// Origin from which proposals may be blacklisted.
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Origin for anyone able to veto proposals.
	// The number of Vetoers for a proposal must be small, extrinsics are weighted according to MAX_VETOERS
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>; 
	// Period in blocks where an external proposal may not be re-submitted after being vetoed.
	type CooloffPeriod = CooloffPeriod;
	// The amount of balance that must be deposited per byte of preimage stored.
	type PreimageByteDeposit = PreimageByteDeposit;
	// An origin that can provide a preimage using operational extrinsics.
	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
	// Handler for the unbalanced reduction when slashing a preimage deposit.
	type Slash = ();
	// The Scheduler handler.
	type Scheduler = Scheduler;
	// Overarching type of all pallets origins.
	type PalletsOrigin = OriginCaller;
	// The maximum number of votes for an account.
    // Also used to compute weight, an overly big value can lead to extrinsic with very big weight: see delegate for instance.
	type MaxVotes = MaxVotes;
	// Weight information for extrinsics in this pallet used for gas fees calculation
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
	// The maximum number of public proposals that can exist at any time.
	type MaxProposals = MaxProposals;
}
// scheduler pallet
parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
}
// end pallet scheduler


// pallet staking (used for validators and nominators)
// reward curve configuration
pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_750_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// Six sessions in an era (24 hours).
	pub const SessionsPerEra: SessionIndex = 6;
	// 28 eras for unbonding (28 days).
	pub const BondingDuration: pallet_staking::EraIndex = 28;
	pub const SlashDeferDuration: pallet_staking::EraIndex = 27;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
}
// set the accounts allowed to cancel a slash of funds
/*type SlashCancelOrigin = EnsureOneOf<
	AccountId,
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_3, _4, AccountId, CouncilCollective>
>;*/
// implementation types
impl pallet_staking::Config for Runtime {
	//const MAX_NOMINATIONS: u32 = 16;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
	type RewardRemainder = ();
	type Event = Event;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	// A majority of the council can cancel the slash.
	type SlashCancelOrigin = frame_system::EnsureNever<()>;
	type SessionInterface = Self;
	//type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type RewardCurve =  RewardCurve;
	type ElectionLookahead = ();
	type MinSolutionScoreBump = ();
	type OffchainSolutionWeightLimit = ();
	type Call = Call;
	type MaxIterations = ();
	type UnsignedPriority = ();
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type NextNewSession = Session;
	//type ElectionProvider = frame_election_provider_support::onchain::OnChainSequentialPhragmen<Self>;
	type WeightInfo = (); //pallet_staking::weights::SubstrateWeight<Runtime>;
}
//end staking pallet

// session pallet
// The Session module allows validators to manage their session keys, provides a function for changing the session length, and handles session rotation.
// Terminology:
// - Session: A session is a period of time that has a constant set of validators. 
//   Validators can only join or exit the validator set at a session change. It is measured in block numbers. 
//   The block where a session is ended is determined by the ShouldEndSession trait. 
//   When the session is ending, a new validator set can be chosen by OnSessionEnding implementations.
// - Session key: A session key is actually several keys kept together that provide the various signing functions 
//   required by network authorities/validators in pursuit of their duties.
// - Validator ID: Every account has an associated validator ID. For some simple staking systems, this may just be the same as the account ID. 
//   For staking systems using a stash/controller model, the validator ID would be the stash account ID of the controller.
// - Session key configuration process: Session keys are set using set_keys for use not in the next session, but the session after next. 
//   They are stored in NextKeys, a mapping between the caller's ValidatorId and the session keys provided. 
//   set_keys allows users to set their session key prior to being selected as validator. 
//   It is a public call since it uses ensure_signed, which checks that the origin is a signed account. 
//   As such, the account ID of the origin stored in NextKeys may not necessarily be associated with a block author or a validator. 
//   The session keys of accounts are removed once their account balance is zero.
// - Session length: This pallet does not assume anything about the length of each session. 
//   Rather, it relies on an implementation of ShouldEndSession to dictate a new session's start. 
//   This pallet provides the PeriodicSessions struct for simple periodic sessions.
// - Session rotation configuration: Configure as either a 'normal' (rewardable session where rewards are applied) 
//	 or 'exceptional' (slashable) session rotation.
// - Session rotation process: At the beginning of each block, the on_initialize function queries the provided 
//   implementation of ShouldEndSession. If the session is to end the newly activated validator IDs and session 
//   keys are taken from storage and passed to the SessionHandler. The validator set supplied by SessionManager::new_session
//   and the corresponding session keys, which may have been registered via set_keys during the previous session, 
//   are written to storage where they will wait one session before being passed to the SessionHandler themselves.
impl_opaque_keys! {
	pub struct SessionKeys {
		pub grandpa: Grandpa,
		pub babe: Babe,
	}
}
parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}
impl pallet_session::Config for Runtime {
	// The overarching event type.
	type Event = Event;
	// A stable ID for a validator.
	type ValidatorId = AccountId;
	//A conversion from account ID to validator ID. Its cost must be at most one storage read.
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	//Indicator for when to end the session.
	type ShouldEndSession = Babe;
	//Something that can predict the next session rotation. This should typically come from the same logical unit that provides ShouldEndSession, 
	// yet, it gives a best effort estimate. It is helpful to implement EstimateNextNewSession. We delegate Babe.
	type NextSessionRotation = Babe;
	// Handler for managing new session.
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	// Handler when a session has changed.
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	// The keys type
	type Keys = SessionKeys;
	// The fraction of validators set that is safe to be disabled.
   // After the threshold is reached disabled method starts to return true, which in combination with pallet_staking forces a new era.
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	// Weight information for gas fees on extrinsics for this pallet.
	type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}
// This is used when implementing blockchains that require accountable safety where validators from some amount f prior sessions must remain slashable.
// Rather than store the full session data for any given session, we instead commit to the roots of merkle tries containing the session data.
// These roots and proofs of inclusion can be generated at any time during the current session. Afterwards, the proofs can be fed to a consensus module when reporting misbehavior.
impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}
// end session pallet

// babe pallet for NPOS consensus
parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}
impl pallet_babe::Config for Runtime {
	//The amount of time, in slots, that each epoch should last. 
	//Currently it is not possible to change the epoch duration after the chain has started. Attempting to do so will brick block production.
	type EpochDuration = EpochDuration; 
	// The expected average block time at which BABE should be creating blocks. 
	// Since BABE is probabilistic it is not trivial to figure out what the expected average block time should be based on the slot duration 
	// and the security parameter c (where 1 - c represents the probability of a slot being empty).
	type ExpectedBlockTime = ExpectedBlockTime;
	// BABE requires some logic to be triggered on every block to query for whether an epoch has ended and to perform the transition to the next epoch.
	// Typically, the ExternalTrigger type should be used. An internal trigger should only be used when no other module is responsible for changing authority set.
	// Session module is the trigger for epoch change
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	// The proof of key ownership, used for validating equivocation reports. 
	// The proof must include the session index and validator count of the session at which the equivocation occurred.
	// We use the historical session
	type KeyOwnerProofSystem = Historical;
	// A system for proving ownership of keys, i.e. that a given key was part of a validator set, needed for validating equivocation reports.
	type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::Proof;
	// The identification of a key owner, used when reporting equivocations.
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::IdentificationTuple;
	//The equivocation handling subsystem, defines methods to report an offence (after the equivocation has been validated) 
	// and for submitting a transaction to report an equivocation (from an offchain context). 
	// When enabling equivocation handling (i.e. this type isn't set to ()) you must use this pallet's ValidateUnsigned in the runtime definition.
	type HandleEquivocation =
		pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	type WeightInfo = ();
}
// end pallet babe

// Asset pallet
parameter_types! {
	pub const ASSETDEPOSITBASE: Balance = 1 * DOLLARS;
	pub const ASSETDEPOSITPERZOMBIE: Balance = 1 * DOLLARS;
	pub const STRINGLIMIT: u32 = 8192;	// max metadata size in bytes
	pub const METADATADEPOSITBASE: Balance= 1 * DOLLARS; 
	pub const METADATADEPOSITPERBYTE: Balance = 1 * CENTS;
}
impl pallet_assets::Config for Runtime {
	type Event = Event;
	type Balance = u128;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDepositBase = ASSETDEPOSITBASE;
	type AssetDepositPerZombie = ASSETDEPOSITPERZOMBIE;
	type StringLimit= STRINGLIMIT;
	type MetadataDepositBase = METADATADEPOSITBASE;
	type MetadataDepositPerByte = METADATADEPOSITPERBYTE;
	type WeightInfo = (); //pallet_assets::weights::SubstrateWeight<Self>;
}
//end asset pallet
// pallet orml-nft
impl orml_nft::Config for Runtime {
	type ClassId = u32;
	type TokenId =u32;
	type ClassData = Vec<u8>;
	type TokenData = Vec<u8>;
}
// Claim pallet, to claim deposits from previous blockchain
impl pallet_claim::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
}
// end Claim pallet

// SendtransactionType Implementation
impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime where Call: From<C> {
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = Call;
}
// end SendtransactionType Implementation

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Babe: pallet_babe::{Module, Call, Storage, Config, ValidateUnsigned},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Aura: pallet_aura::{Module, Config<T>},
		Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Storage},
		Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
		//custom logic for assets
		Assets: pallet_assets::{Module, Call, Storage, Event<T>},
		// custom logic for smartc contracts pallet (!ink)
		Contracts: pallet_contracts::{Module, Call, Config<T>, Storage, Event<T>},
		// custom logic for democracy pallets
		Democracy: pallet_democracy::{Module, Call, Storage, Config, Event<T>},
		Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
        TechnicalCommittee: pallet_collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		Scheduler: pallet_scheduler::{Module, Call, Storage, Event<T>},
		// custom logic for staking pallets
		Staking: pallet_staking::{Module, Call, Storage, Config<T>, Event<T>},
		Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
		Historical: session_historical::{Module},
		Offences: pallet_offences::{Module, Call, Storage, Event},
		Authorship: pallet_authorship::{Module, Call, Storage},
		ImOnline: pallet_im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
		// Nft Pallet
		Nft: orml_nft::{Module, Call, Storage, Config<T>},
		// Claim Pallet
		Claim: pallet_claim::{Module, Call, Storage, Event<T>},
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllModules,
>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) ->
			Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}
	// implementation for contract pallet
	impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
    for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult {
            Contracts::bare_call(origin, dest, value, gas_limit, input_data)
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            address: AccountId,
        ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
            Contracts::rent_projection(address)
        }
    }
	//end implementation for contracts pallet

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
		for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use frame_system_benchmarking::Module as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
					.to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
					.to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
					.to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
					.to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
					.to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}
