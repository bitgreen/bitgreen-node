// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! CarbonCredits Pools pallet benchmarking
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;
use pallet_carbon_credits::{BatchGroupOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf, BatchOf, BatchGroupListOf};
use primitives::{Batch, RegistryDetails, RegistryName, SDGDetails, SdgType};
use sp_std::convert::TryInto;
use frame_support::BoundedVec;

use super::*;
use crate::Pallet as CarbonCreditPools;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryListOf<T> {
	let registry_details = RegistryDetails {
		reg_name: RegistryName::Verra,
		name: "reg_name".as_bytes().to_vec().try_into().unwrap(),
		id: "reg_id".as_bytes().to_vec().try_into().unwrap(),
		summary: "reg_summary".as_bytes().to_vec().try_into().unwrap(),
	};
	vec![registry_details].try_into().unwrap()
}

/// helper function to generate standard sdg details
fn get_default_sdg_details<T: Config>() -> SDGTypesListOf<T> {
	let sdg_details: SDGTypesListOf<T> = vec![SDGDetails {
		sdg_type: SdgType::LifeOnLand,
		description: "sdg_desp".as_bytes().to_vec().try_into().unwrap(),
		references: "sdg_ref".as_bytes().to_vec().try_into().unwrap(),
	}]
	.try_into()
	.unwrap();

	sdg_details
}

fn get_single_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![Batch {
		name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
		issuance_year: 2020_u16,
		start_date: 2020_u16,
		end_date: 2020_u16,
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
	}]
	.try_into()
	.unwrap()
}


/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupListOf<T> {
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_single_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
{
	let creation_params = ProjectCreateParams {
		name: "name".as_bytes().to_vec().try_into().unwrap(),
		description: "description".as_bytes().to_vec().try_into().unwrap(),
		location: vec![(1, 1), (2, 2), (3, 3), (4, 4)].try_into().unwrap(),
		images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
			.try_into()
			.unwrap(),
		registry_details: get_default_registry_details::<T>(),
		sdg_details: get_default_sdg_details::<T>(),
		batch_groups: get_default_batch_group::<T>(),
		royalties: None,
	};

	creation_params
}

benchmarks! {

	where_clause { where
	T::PoolId: From<u32>,
	T: pallet_membership::Config
}

	create {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let owner : T::AccountId = account("owner", 0, 1);
		let pool_id = 10_001_u32.into();
		let asset_symbol =  "pool_xyz".as_bytes().to_vec().try_into().unwrap();
	}: _(RawOrigin::Signed(account_id), pool_id, owner, Default::default(), None, asset_symbol)
	verify {
		assert!(
			CarbonCreditPools::<T>::pools(pool_id).is_some()
		);
	}

	deposit {
		let caller : T::AccountId = account("account_id", 0, 0);
		let owner : T::AccountId = account("owner", 0, 1);
		// create a project and mint tokens
		let project_id : T::ProjectId = 0_u32.into();
		let group_id  : T::GroupId = 0_u32.into();
		let asset_id : T::AssetId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();
		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;

		pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
		pallet_carbon_credits::Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), creation_params)?;
		pallet_carbon_credits::Pallet::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
		pallet_carbon_credits::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, group_id, 100_u32.into(), false)?;

		// create a pool
		let pool_id = 10_001_u32.into();
		let asset_symbol =  "pool_xyz".as_bytes().to_vec().try_into().unwrap();
		CarbonCreditPools::<T>::create(RawOrigin::Signed(caller.clone()).into(), pool_id, owner, Default::default(), None, asset_symbol).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), pool_id, asset_id, 1_u32.into())
	verify {
		assert_last_event::<T>(Event::Deposit { asset_id, who : caller, amount : 1_u32.into(), pool_id }.into());
	}

	retire {
		let caller : T::AccountId = account("account_id", 0, 0);
		let owner : T::AccountId = account("owner", 0, 1);
		// create a project and mint tokens
		let project_id : T::ProjectId = 0_u32.into();
		let group_id  : T::GroupId = 0_u32.into();
		let asset_id : T::AssetId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();
		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;

		pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
		pallet_carbon_credits::Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), creation_params)?;
		pallet_carbon_credits::Pallet::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
		pallet_carbon_credits::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, group_id, 100_u32.into(), false)?;

		// create a pool and deposit tokens
		let pool_id = 10_001_u32.into();
		let asset_symbol =  "pool_xyz".as_bytes().to_vec().try_into().unwrap();
		CarbonCreditPools::<T>::create(RawOrigin::Signed(caller.clone()).into(), pool_id, owner, Default::default(), None, asset_symbol).unwrap();
		CarbonCreditPools::<T>::deposit(RawOrigin::Signed(caller.clone()).into(), pool_id, asset_id, 10_u32.into()).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), pool_id, 1_u32.into())
	verify {
		assert_last_event::<T>(Event::Retired { pool_id, who : caller, amount : 1_u32.into() }.into());
	}

	impl_benchmark_test_suite!(CarbonCreditPools, crate::mock::new_test_ext(), crate::mock::Test);
}
