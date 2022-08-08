// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! CarbonCredits Pools pallet benchmarking
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as VCUPools;
use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;
use pallet_carbon_credits::{BatchGroupOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf};
use primitives::{Batch, RegistryDetails, RegistryName, SDGDetails, SdgType};
use sp_std::convert::TryInto;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryListOf<T> {
    let registry_details = RegistryDetails {
        registry: RegistryName::Verra,
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

/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupOf<T> {
    let batches: BatchGroupOf<T> = vec![Batch {
        name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
        uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
        issuance_year: 2020_u32,
        start_date: 2020_u32,
        end_date: 2020_u32,
        total_supply: 100_u32.into(),
        minted: 0_u32.into(),
        retired: 0_u32.into(),
    }]
    .try_into()
    .unwrap();

    batches
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
{
    let creation_params = ProjectCreateParams {
        name: "name".as_bytes().to_vec().try_into().unwrap(),
        description: "description".as_bytes().to_vec().try_into().unwrap(),
        location: vec![(1, 1), (2, 2), (3, 3), (4, 4)].try_into().unwrap(),
        images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        registry_details: get_default_registry_details::<T>(),
        sdg_details: get_default_sdg_details::<T>(),
        batches: get_default_batch_group::<T>(),
        royalties: None,
        unit_price: 100_u32.into(),
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

    }: _(RawOrigin::Signed(account_id), pool_id, Default::default(), None, asset_symbol)
    verify {
        assert!(
            VCUPools::<T>::pools(pool_id).is_some()
        );
    }

    deposit {
        let caller : T::AccountId = account("account_id", 0, 0);
        // create a project and mint tokens
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
        pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
        pallet_carbon_credits::Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), project_id, creation_params)?;
        pallet_carbon_credits::Pallet::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
        pallet_carbon_credits::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, 100_u32.into(), false)?;

        // create a pool
        let pool_id = 10_001_u32.into();
        let asset_symbol =  "pool_xyz".as_bytes().to_vec().try_into().unwrap();
        VCUPools::<T>::create(RawOrigin::Signed(caller.clone()).into(), pool_id, Default::default(), None, asset_symbol).unwrap();
    }: _(RawOrigin::Signed(caller.clone()), pool_id, project_id, 1_u32.into())
    verify {
        assert_last_event::<T>(Event::Deposit { project_id, who : caller, amount : 1_u32.into(), pool_id }.into());
    }

    retire {
        let caller : T::AccountId = account("account_id", 0, 0);
        // create a project and mint tokens
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
        pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
        pallet_carbon_credits::Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), project_id, creation_params)?;
        pallet_carbon_credits::Pallet::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
        pallet_carbon_credits::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, 100_u32.into(), false)?;

        // create a pool and deposit tokens
        let pool_id = 10_001_u32.into();
        let asset_symbol =  "pool_xyz".as_bytes().to_vec().try_into().unwrap();
        VCUPools::<T>::create(RawOrigin::Signed(caller.clone()).into(), pool_id, Default::default(), None, asset_symbol).unwrap();
        VCUPools::<T>::deposit(RawOrigin::Signed(caller.clone()).into(), pool_id, project_id, 10_u32.into()).unwrap();
    }: _(RawOrigin::Signed(caller.clone()), pool_id, 1_u32.into())
    verify {
        assert_last_event::<T>(Event::Retired { pool_id, who : caller, amount : 1_u32.into() }.into());
    }

    impl_benchmark_test_suite!(VCUPools, crate::mock::new_test_ext(), crate::mock::Test);
}
