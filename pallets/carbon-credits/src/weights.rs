// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Autogenerated weights for pallet_carbon_credits
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bitg-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --pallet=pallet-carbon-credits
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/CarbonCredits/src/weights.rs
// --template=./.maintain/bitg-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_carbon_credits.
pub trait WeightInfo {
	fn create() -> Weight;
	fn approve_project() -> Weight;
	fn mint() -> Weight;
	fn retire() -> Weight;
	fn force_add_authorized_account() -> Weight;
	fn force_remove_authorized_account() -> Weight;
	fn force_set_project_storage() -> Weight;
	fn force_set_next_item_id() -> Weight;
	fn force_set_retired_carbon_credit() -> Weight;
}

/// Weights for pallet_carbon_credits using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(69_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:0)
	// Storage: VCU Projects (r:1 w:1)
	fn approve_project() -> Weight {
		Weight::from_ref_time(40_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: VCU NextItemId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(117_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:1)
	fn force_add_authorized_account() -> Weight {
		Weight::from_ref_time(22_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:1)
	fn force_remove_authorized_account() -> Weight {
		Weight::from_ref_time(24_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: VCU Projects (r:0 w:1)
	fn force_set_project_storage() -> Weight {
		Weight::from_ref_time(11_000_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: VCU NextItemId (r:0 w:1)
	fn force_set_next_item_id() -> Weight {
		Weight::from_ref_time(7_000_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn force_set_retired_carbon_credit() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(69_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:0)
	// Storage: VCU Projects (r:1 w:1)
	fn approve_project() -> Weight {
		Weight::from_ref_time(40_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: VCU Projects (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: VCU NextItemId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(117_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:1)
	fn force_add_authorized_account() -> Weight {
		Weight::from_ref_time(22_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: VCU AuthorizedAccounts (r:1 w:1)
	fn force_remove_authorized_account() -> Weight {
		Weight::from_ref_time(24_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: VCU Projects (r:0 w:1)
	fn force_set_project_storage() -> Weight {
		Weight::from_ref_time(11_000_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: VCU NextItemId (r:0 w:1)
	fn force_set_next_item_id() -> Weight {
		Weight::from_ref_time(7_000_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn force_set_retired_carbon_credit() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
