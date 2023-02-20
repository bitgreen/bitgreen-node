// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate as pallet_carbon_credits;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	bounded_vec, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU32, Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;
use frame_system::EnsureRoot;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
};
use sp_std::convert::{TryFrom, TryInto};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		KYCMembership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		CarbonCredits: pallet_carbon_credits::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type AccountData = pallet_balances::AccountData<u128>;
	type AccountId = u64;
	type BaseCallFilter = Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type RuntimeEvent = RuntimeEvent;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type RuntimeOrigin = RuntimeOrigin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
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
}

parameter_types! {
	pub const AssetDepositBase: u64 = 0;
	pub const AssetDepositPerZombie: u64 = 0;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u64 = 0;
	pub const MetadataDepositPerByte: u64 = 0;
}

impl pallet_assets::Config for Test {
	type ApprovalDeposit = AssetDepositBase;
	type AssetAccountDeposit = AssetDepositBase;
	type AssetDeposit = AssetDepositBase;
	type AssetId = u32;
	type Balance = u128;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Extra = ();
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type Freezer = ();
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type StringLimit = ConstU32<50>;
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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_membership::GenesisConfig::<Test> {
		members: bounded_vec![1, 3, 10],
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
