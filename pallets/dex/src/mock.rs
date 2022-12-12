// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains, Everything, GenesisBuild, Nothing},
	PalletId,
};
use frame_system as system;
use frame_system::EnsureRoot;
use orml_traits::parameter_type_with_key;
use primitives::{Amount, Balance, CurrencyId};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::{TryFrom, TryInto};

pub type AccountId = u64;
pub const AUSD: CurrencyId = CurrencyId::AUSD;

use crate as pallet_dex;
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
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>},
		Dex: pallet_dex::{Pallet, Call, Storage, Event<T>},
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
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
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
	type Event = Event;
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
	type Event = Event;
	type Extra = ();
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type Freezer = ();
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type StringLimit = ConstU32<50>;
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Test {
	type Amount = Amount;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = Nothing;
	type Event = Event;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type MaxReserves = ();
	type OnDust = ();
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

pub struct DummyValidator;
impl Contains<u32> for DummyValidator {
	fn contains(_asset_id: &u32) -> bool {
		true
	}
}

parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"bitg/dex");
	pub StableCurrencyId: CurrencyId = CurrencyId::AUSD;
	pub const MinUnitsToCreateSellOrder : u32 = 2;
	pub const MinPricePerUnit : u32 = 1;
}

impl pallet_dex::Config for Test {
	type Event = Event;
	type Asset = Assets;
	type Currency = Tokens;
	type StableCurrencyId = StableCurrencyId;
	type PalletId = DexPalletId;
	type MinPricePerUnit = MinPricePerUnit;
	type AssetValidator = DummyValidator;
	type MinUnitsToCreateSellOrder = MinUnitsToCreateSellOrder;
	type ForceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	orml_tokens::GenesisConfig::<Test> { balances: vec![(4, AUSD, 100)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	// need to set block number to 1 to test events
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn last_event() -> Event {
	System::events().pop().expect("Event expected").event
}
