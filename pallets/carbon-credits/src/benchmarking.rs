// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Carbon Credits pallet benchmarking
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks, vec};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use primitives::{Batch, RegistryDetails, RegistryName, SDGDetails, SdgType};
use sp_std::convert::TryInto;

use super::*;
use crate::{Event, Pallet as CarbonCredits};
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
		location: "location".as_bytes().to_vec().try_into().unwrap(),
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
	T::AssetId: From<u32>,
	T::ItemId: From<u32>,
	T: pallet_membership::Config
}

	create {
		let caller : T::AccountId = account("account_id", 0, 0);
		let project_id : T::ProjectId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();
		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;
	}: _(RawOrigin::Signed(caller.into()), creation_params.into())
	verify {
		assert!(Projects::<T>::get(project_id).is_some());
	}

	approve_project {
		let caller : T::AccountId = account("account_id", 0, 0);
		let project_id : T::ProjectId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();
		CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;
		CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), creation_params)?;
	}: _(RawOrigin::Signed(caller.into()), project_id, true)
	verify {
		assert_last_event::<T>(Event::ProjectApproved { project_id, asset_ids: vec![0u32.into()] }.into());
	}

	mint {
		let caller : T::AccountId = account("account_id", 0, 0);
		let project_id : T::ProjectId = 0_u32.into();
		let group_id : T::GroupId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();

		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;

		CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
		CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), creation_params)?;
		CarbonCredits::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
	}: _(RawOrigin::Signed(caller.clone()), project_id, group_id, 100_u32.into(), false)
	verify {
		assert_last_event::<T>(Event::CarbonCreditMinted { project_id, group_id, recipient : caller, amount : 100_u32.into() }.into());
	}

	retire {
		let caller : T::AccountId = account("account_id", 0, 0);
		let project_id : T::ProjectId = 0_u32.into();
		let group_id : T::GroupId = 0_u32.into();
		let asset_id : T::AssetId = 0_u32.into();
		let creation_params = get_default_creation_params::<T>();

		let caller_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(caller.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller_lookup)?;

		CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
		CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), creation_params)?;
		CarbonCredits::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
		CarbonCredits::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, group_id, 100_u32.into(), false)?;
	}: _(RawOrigin::Signed(caller.clone()), project_id, group_id, 10_u32.into())
	verify {
		let item_id : T::ItemId = 0_u32.into();
		let retire_data = RetiredCredits::<T>::get(asset_id, item_id).unwrap();
		assert_last_event::<T>(Event::CarbonCreditRetired { project_id, group_id, asset_id, account : caller, amount : 10_u32.into(), retire_data :retire_data.retire_data }.into());
	}

	force_add_authorized_account {
		let account_id : T::AccountId = account("account_id", 0, 0);
	}: _(RawOrigin::Root, account_id.clone().into())
	verify {
		assert_eq!(
			CarbonCredits::<T>::authorized_accounts().len(),
			1
		);
	}

	force_remove_authorized_account {
		let account_id : T::AccountId = account("account_id", 0, 0);
		CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), account_id.clone().into())?;
	}: _(RawOrigin::Root, account_id.clone().into())
	verify {
		assert_eq!(
			CarbonCredits::<T>::authorized_accounts().len(),
			0
		);
	}

	force_set_project_storage {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let project_id : T::ProjectId = 1000_u32.into();
		let params = get_default_creation_params::<T>();
		let new_project = ProjectDetail::<T> {
			originator: account_id,
			name: params.name,
			description: params.description,
			location: params.location,
			images: params.images,
			videos: params.videos,
			documents: params.documents,
			registry_details: params.registry_details,
			sdg_details: params.sdg_details,
			royalties: params.royalties,
			batch_groups: Default::default(),
			created: 1_u32.into(),
			updated: None,
			approved: false,
			project_type: None
		};
	}: _(RawOrigin::Root, project_id, new_project)
	verify {
		assert!(Projects::<T>::get(project_id).is_some());
	}

	force_set_next_item_id {
		let asset_id : T::AssetId = 1000_u32.into();
		let next_item_id : T::ItemId = 100_u32.into();
	}: _(RawOrigin::Root, asset_id, next_item_id)
	verify {
		assert_eq!(NextItemId::<T>::get(asset_id).unwrap(), next_item_id);
	}

	force_set_retired_carbon_credit {
		let account : T::AccountId = account("account_id", 0, 0);
		let asset_id : T::AssetId = 1000_u32.into();
		let item_id : T::ItemId = 100_u32.into();
		let batch_data : BatchRetireDataOf::<T> = Default::default();
		let new_retire_data = RetiredCarbonCreditsData::<T> {
			account,
			retire_data : vec![batch_data].try_into().unwrap(),
			timestamp : 1_u32.into(),
			count : 100_u32.into()
		};
	}: _(RawOrigin::Root, asset_id, item_id, new_retire_data)
	verify {
		assert!(RetiredCredits::<T>::get(asset_id, item_id).is_some());
	}

	impl_benchmark_test_suite!(CarbonCredits, crate::mock::new_test_ext(), crate::mock::Test);
}
