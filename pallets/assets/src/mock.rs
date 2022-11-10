// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//

//! Test environment for Assets pallet.

use frame_support::{
	construct_runtime,
	traits::{ConstU32, ConstU64, GenesisBuild},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use super::*;
use crate as pallet_assets;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = ConstU64<250>;
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
	type MaxConsumers = ConstU32<2>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU64<1>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

impl Config for Test {
	type ApprovalDeposit = ConstU64<1>;
	type AssetAccountDeposit = ConstU64<10>;
	type AssetDeposit = ConstU64<1>;
	type AssetId = u32;
	type Balance = u64;
	type Currency = Balances;
	type Event = Event;
	type Extra = ();
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type Freezer = TestFreezer;
	type MetadataDepositBase = ConstU64<1>;
	type MetadataDepositPerByte = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type WeightInfo = ();
}

use std::{cell::RefCell, collections::HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Hook {
	Died(u32, u64),
}
thread_local! {
	static FROZEN: RefCell<HashMap<(u32, u64), u64>> = RefCell::new(Default::default());
	static HOOKS: RefCell<Vec<Hook>> = RefCell::new(Default::default());
}

pub struct TestFreezer;
impl FrozenBalance<u32, u64, u64> for TestFreezer {
	fn frozen_balance(asset: u32, who: &u64) -> Option<u64> {
		FROZEN.with(|f| f.borrow().get(&(asset, *who)).cloned())
	}

	fn died(asset: u32, who: &u64) {
		HOOKS.with(|h| h.borrow_mut().push(Hook::Died(asset, *who)));
		// Sanity check: dead accounts have no balance.
		assert!(Assets::balance(asset, *who).is_zero());
	}
}

pub(crate) fn set_frozen_balance(asset: u32, who: u64, amount: u64) {
	FROZEN.with(|f| f.borrow_mut().insert((asset, who), amount));
}

pub(crate) fn clear_frozen_balance(asset: u32, who: u64) {
	FROZEN.with(|f| f.borrow_mut().remove(&(asset, who)));
}

pub(crate) fn hooks() -> Vec<Hook> { HOOKS.with(|h| h.borrow().clone()) }

pub(crate) fn take_hooks() -> Vec<Hook> { HOOKS.with(|h| h.take()) }

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	let config: pallet_assets::GenesisConfig<Test> = pallet_assets::GenesisConfig {
		assets: vec![
			// id, owner, is_sufficient, min_balance
			(999, 0, true, 1),
		],
		metadata: vec![
			// id, name, symbol, decimals
			(999, "Token Name".into(), "TOKEN".into(), 10),
		],
		accounts: vec![
			// id, account_id, balance
			(999, 1, 100),
		],
	};

	config.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	// Clear thread local vars for https://github.com/paritytech/substrate/issues/10479.
	ext.execute_with(take_hooks);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
