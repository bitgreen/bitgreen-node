// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Autogenerated weights for pallet_vesting_contract
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bitgreen-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --pallet=pallet-vesting-contract
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/vesting-contract/src/weights.rs
// --template=./.maintain/bitg-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vesting_contract.
pub trait WeightInfo {
	fn add_new_contract() -> Weight;
	fn remove_contract() -> Weight;
	fn bulk_add_new_contracts(i: u32, ) -> Weight;
	fn bulk_remove_contracts(i: u32, ) -> Weight;
	fn withdraw_vested() -> Weight;
	fn force_withdraw_vested() -> Weight;
}

/// Weights for pallet_vesting_contract using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn add_new_contract() -> Weight {
		Weight::from_ref_time(25_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	fn remove_contract() -> Weight {
		Weight::from_ref_time(22_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn bulk_add_new_contracts(_i: u32, ) -> Weight {
		Weight::from_ref_time(26_044_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	fn bulk_remove_contracts(_i: u32, ) -> Weight {
		Weight::from_ref_time(22_383_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn withdraw_vested() -> Weight {
		Weight::from_ref_time(45_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn force_withdraw_vested() -> Weight {
		Weight::from_ref_time(46_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn add_new_contract() -> Weight {
		Weight::from_ref_time(25_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	fn remove_contract() -> Weight {
		Weight::from_ref_time(22_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn bulk_add_new_contracts(_i: u32, ) -> Weight {
		Weight::from_ref_time(26_044_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	fn bulk_remove_contracts(_i: u32, ) -> Weight {
		Weight::from_ref_time(22_383_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn withdraw_vested() -> Weight {
		Weight::from_ref_time(45_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: VestingContract VestingContracts (r:1 w:1)
	// Storage: VestingContract VestingBalance (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn force_withdraw_vested() -> Weight {
		Weight::from_ref_time(46_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
