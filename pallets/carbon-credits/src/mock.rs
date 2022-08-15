// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate as pallet_carbon_credits;
use frame_support::{
    bounded_vec, parameter_types,
    traits::{AsEnsureOriginWithArg, ConstU128, ConstU32, Everything, GenesisBuild},
    PalletId,
};
use frame_system as system;
use frame_system::EnsureRoot;
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
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u128;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
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
    type Event = Event;
    type Balance = u128;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type AssetDeposit = AssetDepositBase;
    type AssetAccountDeposit = AssetDepositBase;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = AssetDepositBase;
    type StringLimit = ConstU32<50>;
    type Freezer = ();
    type WeightInfo = ();
    type Extra = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
  pub const MarketplaceEscrowAccount : u64 = 10;
  pub const VCUPalletId: PalletId = PalletId(*b"bitg/vcu");
  pub VCUPalletAcccount : u64 = PalletId(*b"bitg/vcu").into_account_truncating();
}

impl pallet_carbon_credits::Config for Test {
    type Event = Event;
    type Balance = u128;
    type AssetId = u32;
    type PalletId = VCUPalletId;
    type AssetHandler = Assets;
    type KYCProvider = KYCMembership;
    type MarketplaceEscrow = MarketplaceEscrowAccount;
    type MaxAuthorizedAccountCount = ConstU32<2>;
    type MaxShortStringLength = ConstU32<20>;
    type MaxLongStringLength = ConstU32<100>;
    type MaxIpfsReferenceLength = ConstU32<20>;
    type MaxDocumentCount = ConstU32<2>;
    type MaxRoyaltyRecipients = ConstU32<5>;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type ItemId = u32;
    type NFTHandler = Uniques;
    type MaxGroupSize = ConstU32<5>;
    type MaxCoordinatesLength = ConstU32<8>;
    type MinProjectId = ConstU32<1000>;
    type WeightInfo = ();
}

impl pallet_uniques::Config for Test {
    type Event = Event;
    type CollectionId = u32;
    type ItemId = u32;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type Locker = ();
    type CollectionDeposit = ConstU128<0>;
    type ItemDeposit = ConstU128<0>;
    type MetadataDepositBase = ConstU128<1>;
    type AttributeDepositBase = ConstU128<1>;
    type DepositPerByte = ConstU128<1>;
    type StringLimit = ConstU32<50>;
    type KeyLimit = ConstU32<50>;
    type ValueLimit = ConstU32<50>;
    type WeightInfo = ();
}

impl pallet_membership::Config for Test {
    type Event = Event;
    type AddOrigin = EnsureRoot<u64>;
    type RemoveOrigin = EnsureRoot<u64>;
    type SwapOrigin = EnsureRoot<u64>;
    type ResetOrigin = EnsureRoot<u64>;
    type PrimeOrigin = EnsureRoot<u64>;
    type MembershipInitialized = ();
    type MembershipChanged = ();
    type MaxMembers = ConstU32<10>;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

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

pub fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}
