// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU32},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};
use scale_info::TypeInfo;
use sp_core::{ConstU16, ConstU64, H256};
use sp_runtime::{
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use sp_std::convert::{TryFrom, TryInto};

use crate as pallet_carbon_credits_pool;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		KYCMembership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		CarbonCredits: pallet_carbon_credits::{Pallet, Call, Storage, Event<T>},
		CarbonCreditPools: pallet_carbon_credits_pool::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Block = Block;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;

	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<0>;
	type AssetAccountDeposit = ConstU128<0>;
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type ApprovalDeposit = ConstU128<0>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
	type MinimumPeriod = MinimumPeriod;
	type Moment = u64;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

parameter_types! {
  pub const MarketplaceEscrowAccount : u64 = 10;
  pub const CarbonCreditsPalletId: PalletId = PalletId(*b"bitg/ccp");
  pub CarbonCreditsPalletAcccount : u64 = PalletId(*b"bitg/ccp").into_account_truncating();
  #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
  pub const MaxGroupSize: u32 = 10;
}

impl pallet_carbon_credits::Config for Test {
	type AssetHandler = Assets;
	type AssetId = u32;
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type ItemId = u32;
	type ProjectId = u32;
	type MaxCoordinatesLength = ConstU32<8>;
	type GroupId = u32;
	type KYCProvider = KYCMembership;
	type MarketplaceEscrow = MarketplaceEscrowAccount;
	type MaxAuthorizedAccountCount = ConstU32<2>;
	type MaxDocumentCount = ConstU32<2>;
	type MaxGroupSize = MaxGroupSize;
	type MaxIpfsReferenceLength = ConstU32<20>;
	type MaxLongStringLength = ConstU32<100>;
	type MaxRoyaltyRecipients = ConstU32<5>;
	type MaxShortStringLength = ConstU32<20>;
	type MinProjectId = ConstU32<1000>;
	type NFTHandler = Uniques;
	type PalletId = CarbonCreditsPalletId;
	type WeightInfo = ();
}

impl pallet_uniques::Config for Test {
	type AttributeDepositBase = ConstU128<1>;
	type CollectionDeposit = ConstU128<0>;
	type CollectionId = u32;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type Currency = Balances;
	type DepositPerByte = ConstU128<1>;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type ItemDeposit = ConstU128<0>;
	type ItemId = u32;
	type KeyLimit = ConstU32<50>;
	type Locker = ();
	type MetadataDepositBase = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
}

impl pallet_membership::Config for Test {
	type AddOrigin = EnsureRoot<u64>;
	type RuntimeEvent = RuntimeEvent;
	type MaxMembers = ConstU32<10>;
	type MembershipChanged = ();
	type MembershipInitialized = ();
	type PrimeOrigin = EnsureRoot<u64>;
	type RemoveOrigin = EnsureRoot<u64>;
	type ResetOrigin = EnsureRoot<u64>;
	type SwapOrigin = EnsureRoot<u64>;
	type WeightInfo = ();
}

parameter_types! {
	pub const CarbonCreditPoolsPalletId: PalletId = PalletId(*b"bit/ccpp");
}

impl pallet_carbon_credits_pool::Config for Test {
	type AssetHandler = Assets;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type MaxAssetSymbolLength = ConstU32<20>;
	type MaxIssuanceYearCount = ConstU32<20>;
	type MaxProjectIdList = ConstU32<100>;
	type MaxRegistryListCount = ConstU32<2>;
	type MinPoolId = ConstU32<10000>;
	type PalletId = CarbonCreditPoolsPalletId;
	type PoolId = u32;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_membership::GenesisConfig::<Test> {
		members: sp_core::bounded_vec![1, 3, 10],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	// set to block 1 to test events
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}
