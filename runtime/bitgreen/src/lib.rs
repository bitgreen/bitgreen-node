// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod impls;
mod weights;
pub mod xcm_config;
use codec::{Decode, Encode, MaxEncodedLen};
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
// Polkadot Imports
use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, Contains, Currency, InstanceFilter, Nothing, PrivilegeCmp,
	},
	weights::{
		constants::WEIGHT_PER_SECOND, ConstantMultiplier, Weight, WeightToFeeCoefficient,
		WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot, EnsureSigned,
};
use orml_traits::parameter_type_with_key;
use pallet_contracts::{
	weights::{SubstrateWeight, WeightInfo},
	Config, DefaultAddressGenerator, Frame, Schedule,
};
use polkadot_runtime_common::{BlockHashCount, SlowAdjustingFeeUpdate};
pub use primitives::{
	currency::*,
	time::{
		AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO,
		SLOT_DURATION,
	},
	AccountId, Address, Amount, Balance, BlockNumber, Hash, Header, Index, Signature,
};
use scale_info::TypeInfo;
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, One},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Percent,
};
pub use sp_runtime::{
	traits::{AccountIdConversion, Zero},
	MultiAddress, Perbill, Permill,
};
use sp_std::{cmp::Ordering, convert::TryInto, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};
// XCM Imports
use xcm::latest::prelude::BodyId;
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};
use xcm_executor::XcmExecutor;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	pallet_vesting_contract::PrevalidateVestingWithdraw<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
		// in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
		let p = MILLIUNIT / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	use sp_runtime::{generic, traits::BlakeTwo256};

	use super::*;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("bitgreen-parachain"),
	impl_name: create_runtime_str!("bitgreen-parachain"),
	authoring_version: 1,
	spec_version: 1001, // v1.0.1
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = impls::BaseFilter;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Runtime version.
	type Version = Version;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	type EventHandler = (ParachainStaking,);
	type FilterUncle = ();
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MICROUNIT;
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, impls::DealWithFees<Runtime>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = WEIGHT_PER_SECOND.saturating_div(4);
	pub const ReservedDmpWeight: Weight = WEIGHT_PER_SECOND.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
	type DmpMessageHandler = DmpQueue;
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type OutboundXcmpMessageSource = XcmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type XcmpMessageHandler = XcmpQueue;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type ChannelInfo = ParachainSystem;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type RuntimeEvent = RuntimeEvent;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type VersionWrapper = ();
	type WeightInfo = ();
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
	pub const Period: u32 = 2 * HOURS;
	pub const Offset: u32 = 0;
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Keys = SessionKeys;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type SessionManager = ParachainStaking;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_parachain_staking::IdentityCollator;
	type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const MinCandidates: u32 = 5;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	#[derive(Clone, TypeInfo)]
	pub const MaxDelegators : u32 = 20;
	pub const MinDelegationAmount : u32 = 100;
	pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type ParachainStakingUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_parachain_staking::Config for Runtime {
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type MaxCandidates = MaxCandidates;
	type MaxDelegators = MaxDelegators;
	type MaxInvulnerables = MaxInvulnerables;
	type MinCandidates = MinCandidates;
	type MinDelegationAmount = MinDelegationAmount;
	type PotId = PotId;
	type UpdateOrigin = ParachainStakingUpdateOrigin;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_parachain_staking::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

// orml pallets
parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: primitives::CurrencyId| -> Balance {
		One::one()
	};
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(_a: &AccountId) -> bool {
		false
	}
}

impl orml_tokens::Config for Runtime {
	type Amount = Amount;
	type Balance = Balance;
	type CurrencyId = primitives::CurrencyId;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxLocks;
	type ReserveIdentifier = [u8; 8];
	type CurrencyHooks = ();
	type WeightInfo = ();
}

// Asset pallet
parameter_types! {
	pub const ASSETDEPOSIT: Balance = DOLLARS;
	pub const ASSETACCOUNTDEPOSIT: Balance = DOLLARS;
	pub const STRINGLIMIT: u32 = 8192;	// max metadata size in bytes
	pub const METADATADEPOSITBASE: Balance= DOLLARS;
	pub const METADATADEPOSITPERBYTE: Balance =CENTS;
	pub const APPROVALDEPOSIT: Balance = DOLLARS;
}

pub struct TestFreezer;
impl pallet_assets::FrozenBalance<u32, AccountId, u128> for TestFreezer {
	fn frozen_balance(_asset: u32, _who: &AccountId) -> Option<u128> {
		None
	}

	fn died(_asset: u32, _who: &AccountId) {}
}

impl pallet_assets::Config for Runtime {
	type ApprovalDeposit = APPROVALDEPOSIT;
	type AssetAccountDeposit = ASSETACCOUNTDEPOSIT;
	type AssetDeposit = ASSETDEPOSIT;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type AssetId = u32;
	type Balance = u128;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = TestFreezer;
	type MetadataDepositBase = METADATADEPOSITBASE;
	type MetadataDepositPerByte = METADATADEPOSITPERBYTE;
	type StringLimit = STRINGLIMIT;
	type WeightInfo = ();
}

impl pallet_membership::Config for Runtime {
	type AddOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type MaxMembers = ConstU32<100_000>;
	type MembershipChanged = ();
	type MembershipInitialized = ();
	type PrimeOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

parameter_types! {
  pub MarketplaceEscrowAccount : AccountId =  PalletId(*b"bitg/mkp").into_account_truncating();
  pub const VCUPalletId: PalletId = PalletId(*b"bitg/vcu");
  pub const MaxAuthorizedAccountCount : u32 = 10;
  pub const MaxCoordinatesLength : u32 = 10;
  pub const MaxDocumentCount : u32 = 10;
  pub const MaxIpfsReferenceLength : u32 = 1024;
  pub const MaxLongStringLength : u32 = 3072;
  pub const MaxRoyaltyRecipients : u32 = 10;
  pub const MaxShortStringLength : u32 = 1024;
  pub const MinProjectId : u32 = 1000;
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
  pub const MaxGroupSize : u32 = 10;
}

impl pallet_carbon_credits::Config for Runtime {
	type AssetHandler = Assets;
	type AssetId = u32;
	type ProjectId = u32;
	type GroupId = u32;
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = EnsureRoot<AccountId>;
	type ItemId = u32;
	type KYCProvider = KYCMembership;
	type MarketplaceEscrow = MarketplaceEscrowAccount;
	type MaxAuthorizedAccountCount = MaxAuthorizedAccountCount;
	type MaxCoordinatesLength = MaxCoordinatesLength;
	type MaxDocumentCount = MaxDocumentCount;
	type MaxGroupSize = MaxGroupSize;
	type MaxIpfsReferenceLength = MaxIpfsReferenceLength;
	type MaxLongStringLength = MaxLongStringLength;
	type MaxRoyaltyRecipients = MaxRoyaltyRecipients;
	type MaxShortStringLength = MaxShortStringLength;
	type MinProjectId = MinProjectId;
	type NFTHandler = Uniques;
	type PalletId = VCUPalletId;
	type WeightInfo = ();
}

parameter_types! {
	pub const VCUPoolPalletId: PalletId = PalletId(*b"bit/vcup");
	pub const MaxAssetSymbolLength : u32 = 10;
	pub const MaxIssuanceYearCount : u32 = 20;
	pub const MaxProjectIdList : u32 = 100;
	pub const MaxRegistryListCount : u32 = 10;
	pub const MinPoolId : u32 = 10000;
}

impl pallet_carbon_credits_pool::Config for Runtime {
	type AssetHandler = Assets;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxAssetSymbolLength = MaxAssetSymbolLength;
	type MaxIssuanceYearCount = MaxIssuanceYearCount;
	type MaxProjectIdList = MaxProjectIdList;
	type MaxRegistryListCount = MaxRegistryListCount;
	type MinPoolId = MinPoolId;
	type PalletId = VCUPoolPalletId;
	type PoolId = u32;
	type WeightInfo = ();
}

// TODO : Ensure sensible values
impl pallet_uniques::Config for Runtime {
	type AttributeDepositBase = ConstU128<1>;
	type CollectionDeposit = ConstU128<0>;
	type CollectionId = u32;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Currency = Balances;
	type DepositPerByte = ConstU128<1>;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type ItemDeposit = ConstU128<0>;
	type ItemId = u32;
	type KeyLimit = ConstU32<50>;
	type Locker = ();
	type MetadataDepositBase = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 100 * DOLLARS;
	pub const ProposalBondMaximum: Balance = 500 * DOLLARS;
	pub const SpendPeriod: BlockNumber = 24 * DAYS;
	pub const Burn: Permill = Permill::from_percent(1);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");

	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = DOLLARS;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const MaxApprovals: u32 = 100;
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

type ApproveOrigin = EnsureRoot<AccountId>;

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = ApproveOrigin;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ProposalBondMaximum;
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = ApproveOrigin;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = ();
}

parameter_types! {
  pub const NativeTokenId: u32 = primitives::BBB_TOKEN;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_transaction_pause::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UpdateOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxContractInputLength : u32 = 32;
	pub const VestingContractPalletId: PalletId = PalletId(*b"bitg/vcp");
}

impl pallet_vesting_contract::Config for Runtime {
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxContractInputLength = MaxContractInputLength;
	type PalletId = VestingContractPalletId;
	type WeightInfo = ();
}

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: bool = true;

parameter_types! {
	pub const DepositPerItem: Balance = deposit(1, 0);
	pub const DepositPerByte: Balance = deposit(0, 1);
	// The lazy deletion runs inside on_initialize.
	pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
		RuntimeBlockWeights::get().max_block;
	// The weight needed for decoding the queue should be less or equal than a fifth
	// of the overall weight dedicated to the lazy deletion.
	pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get().ref_time() / (
			<Runtime as Config>::WeightInfo::on_initialize_per_queue_item(1).ref_time() -
			<Runtime as Config>::WeightInfo::on_initialize_per_queue_item(0).ref_time()
		)) / 5) as u32;
	pub MySchedule: Schedule<Runtime> = Default::default();
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = SubstrateWeight<Self>;
	type ChainExtension = ();
	type DeletionQueueDepth = DeletionQueueDepth;
	type DeletionWeightLimit = DeletionWeightLimit;
	type Schedule = MySchedule;
	type CallStack = [Frame<Self>; 31];
	type AddressGenerator = DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 128 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32);
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = ConstU32<100>;
	type WeightInfo = ();
}

// TODO: test limits are safe
parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"bitg/dex");
	pub const StableCurrencyId: primitives::CurrencyId = primitives::CurrencyId::USDT;
	pub const MinUnitsToCreateSellOrder : u32 = 100;
	pub const MinPricePerUnit : u32 = 1;
	pub const MaxPaymentFee : Percent = Percent::from_percent(10);
}

impl pallet_dex::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Asset = Assets;
	type Currency = Tokens;
	type CurrencyBalance = u128;
	type AssetBalance = u128;
	type StableCurrencyId = StableCurrencyId;
	type PalletId = DexPalletId;
	type AssetValidator = pallet_carbon_credits::CarbonCreditsAssetValidator<Runtime>;
	type MinPricePerUnit = MinPricePerUnit;
	type MinUnitsToCreateSellOrder = MinUnitsToCreateSellOrder;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxPaymentFee = MaxPaymentFee;
	type WeightInfo = ();
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = NORMAL_DISPATCH_RATIO * RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

/// Used the compare the privilege of an origin inside the scheduler.
pub struct OriginPrivilegeCmp;

impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
	fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
		if left == right {
			return Some(Ordering::Equal)
		}

		match (left, right) {
			// Root is greater than anything.
			(OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
			// For every other origin we don't care, as they are not used for `ScheduleOrigin`.
			_ => None,
		}
	}
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeEvent = RuntimeEvent;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = ();
	type Preimages = Preimage;
	type OriginPrivilegeCmp = OriginPrivilegeCmp;
}

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub const PreimageBaseDeposit: Balance = deposit(2, 64);
	pub const PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
}

/// The type used to represent the kinds of proxying allowed.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::unnecessary_cast)]
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, MaxEncodedLen, TypeInfo,
)]
pub enum ProxyType {
	/// All calls can be proxied. This is the trivial/most permissive filter.
	Any = 0,
	/// Only extrinsics that do not transfer funds.
	NonTransfer = 1,
	/// Allow to veto an announced proxy call.
	CancelProxy = 2,
	/// Allow extrinsic related to Balances.
	Balances = 3,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => {
				matches!(
					c,
					RuntimeCall::System(..) |
						RuntimeCall::Timestamp(..) |
						RuntimeCall::ParachainStaking(..) |
						RuntimeCall::Utility(..) | RuntimeCall::Proxy(..)
				)
			},
			ProxyType::CancelProxy =>
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })),
			ProxyType::Balances => {
				matches!(c, RuntimeCall::Balances(..) | RuntimeCall::Utility(..))
			},
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	// One storage item; key size 32, value size 8
	type ProxyDepositBase = ConstU128<{ deposit(1, 8) }>;
	// Additional storage item size of 21 bytes (20 bytes AccountId + 1 byte sizeof(ProxyType)).
	type ProxyDepositFactor = ConstU128<{ deposit(0, 21) }>;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ConstU128<{ deposit(1, 8) }>;
	// Additional storage item size of 56 bytes:
	// - 20 bytes AccountId
	// - 32 bytes Hasher (Blake2256)
	// - 4 bytes BlockNumber (u32)
	type AnnouncementDepositFactor = ConstU128<{ deposit(0, 56) }>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// System support stuff.
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
		ParachainSystem: cumulus_pallet_parachain_system::{
			Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
		} = 1,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
		ParachainInfo: parachain_info::{Pallet, Storage, Config} = 4,

		// Monetary stuff.
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,

		// Collator support. The order of these 4 are important and shall not change.
		Authorship: pallet_authorship::{Pallet, Call, Storage} = 20,
		ParachainStaking: pallet_parachain_staking::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
		Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
		AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 24,
		Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>} = 25,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin, Config} = 31,
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
		DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,

		// orml pallets
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>, Config<T>} = 41,

		// Bitgreen pallets
		KYCMembership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>} = 50,
		Sudo: pallet_sudo::{Pallet, Call, Storage, Config<T>, Event<T>} = 51,
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>} = 52,
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>} = 53,
		VCU: pallet_carbon_credits::{Pallet, Call, Storage, Event<T>} = 54,
		VCUPools: pallet_carbon_credits_pool::{Pallet, Call, Storage, Event<T>} = 55,
		TransactionPause: pallet_transaction_pause::{Pallet, Call, Storage, Event<T>} = 56,
		VestingContract: pallet_vesting_contract::{Pallet, Call, Storage, Event<T>} = 57,
		Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>} = 58,
		Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 59,
		Dex: pallet_dex::{Pallet, Call, Storage, Event<T>} = 60,

		// Utility pallets
		Utility: pallet_utility::{Pallet, Call, Event} = 61,
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 62,
		Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 63,
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 64,
	}
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_session, SessionBench::<Runtime>]
		[pallet_timestamp, Timestamp]
		[pallet_parachain_staking, ParachainStaking]
		[cumulus_pallet_xcmp_queue, XcmpQueue]
		[pallet_carbon_credits, VCU]
		[pallet_vesting_contract, VestingContract]
		[pallet_dex, Dex]
	);
}

impl_runtime_apis! {
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

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
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
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

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}


	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			log::info!("try-runtime::on_runtime_upgrade parachain-template.");
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();
			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			impl cumulus_pallet_session_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
