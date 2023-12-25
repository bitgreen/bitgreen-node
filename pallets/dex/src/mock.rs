// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use frame_support::{
	pallet_prelude::DispatchResult,
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, Contains, Nothing},
	PalletId,
};

use frame_system::{EnsureRoot, EnsureSigned};
use orml_traits::parameter_type_with_key;
use primitives::{Amount, Balance, CarbonCreditsValidator, CurrencyId};
use sp_core::{ConstU128, ConstU64, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, Percent,
};
use sp_std::convert::{TryFrom, TryInto};

pub type AccountId = u64;
pub const USDT: CurrencyId = CurrencyId::USDT;

use crate as pallet_dex;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
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
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type MaxReserves = ();
	type CurrencyHooks = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

pub struct DummyValidator;
impl CarbonCreditsValidator for DummyValidator {
	type ProjectId = u32;
	type Amount = Balance;
	type Address = AccountId;
	type AssetId = u32;
	type GroupId = u32;
	fn get_project_details(_asset_id: &Self::AssetId) -> Option<(Self::ProjectId, Self::GroupId)> {
		Some((0, 0))
	}

	fn retire_credits(
		_sender: Self::Address,
		_project_id: Self::ProjectId,
		_group_id: Self::GroupId,
		_amount: Self::Amount,
	) -> DispatchResult {
		Ok(())
	}
}

pub struct MockKycProvider;
impl Contains<u64> for MockKycProvider {
	fn contains(value: &u64) -> bool {
		// special account to test negative kyc
		if value == &20 {
			return false
		}

		true
	}
}

parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"bitg/dex");
	pub const MinUnitsToCreateSellOrder : u32 = 2;
	pub const MinPricePerUnit : u32 = 1;
	pub const MaxPaymentFee : Percent = Percent::from_percent(50);
	pub const MaxPurchaseFee : u128 = 100u128;
	#[derive(Clone, scale_info::TypeInfo)]
	pub const MaxValidators : u32 = 10;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxTxHashLen : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo)]
	pub const BuyOrderExpiryTime : u32 = 2;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxAddressLen : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxOrderIds : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxPayoutsToStore : u32 = 1000;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxOpenOrdersPerUser : u32 = 2;
}

impl pallet_dex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Asset = Assets;
	type Currency = Tokens;
	type CurrencyBalance = u128;
	type AssetBalance = u128;
	type PalletId = DexPalletId;
	type KYCProvider = MockKycProvider;
	type MinPricePerUnit = MinPricePerUnit;
	type AssetValidator = DummyValidator;
	type MaxValidators = MaxValidators;
	type MaxTxHashLen = MaxTxHashLen;
	type BuyOrderExpiryTime = BuyOrderExpiryTime;
	type MaxAddressLen = MaxAddressLen;
	type MaxOrderIds = MaxOrderIds;
	type MaxOpenOrdersPerUser = MaxOpenOrdersPerUser;
	type MinUnitsToCreateSellOrder = MinUnitsToCreateSellOrder;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxPaymentFee = MaxPaymentFee;
	type MaxPurchaseFee = MaxPurchaseFee;
	type MaxPayoutsToStore = MaxPayoutsToStore;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	orml_tokens::GenesisConfig::<Test> { balances: vec![(4, USDT, 100), (10, USDT, 10000)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	// need to set block number to 1 to test events
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}
