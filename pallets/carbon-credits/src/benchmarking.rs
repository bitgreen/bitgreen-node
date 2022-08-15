// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Carbon Credits pallet benchmarking
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Event;
use crate::Pallet as CarbonCredits;
use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;
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
    T::AssetId: From<u32>,
    T::ItemId: From<u32>,
    T: pallet_membership::Config
}

    create {
        let caller : T::AccountId = account("account_id", 0, 0);
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
    }: _(RawOrigin::Signed(caller.into()), project_id, creation_params.into())
    verify {
        assert!(Projects::<T>::get(project_id).is_some());
    }

    approve_project {
        let caller : T::AccountId = account("account_id", 0, 0);
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
        CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), project_id, creation_params)?;
    }: _(RawOrigin::Signed(caller.into()), project_id, true)
    verify {
        assert_last_event::<T>(Event::ProjectApproved { project_id }.into());
    }

    mint {
        let caller : T::AccountId = account("account_id", 0, 0);
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
        CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
        CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), project_id, creation_params)?;
        CarbonCredits::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
    }: _(RawOrigin::Signed(caller.clone()), project_id, 100_u32.into(), false)
    verify {
        assert_last_event::<T>(Event::VCUMinted { project_id, recipient : caller, amount : 100_u32.into() }.into());
    }

    retire {
        let caller : T::AccountId = account("account_id", 0, 0);
        let project_id = 10_000_u32.into();
        let creation_params = get_default_creation_params::<T>();
        pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), caller.clone())?;
        CarbonCredits::<T>::force_add_authorized_account(RawOrigin::Root.into(), caller.clone().into())?;
        CarbonCredits::<T>::create(RawOrigin::Signed(caller.clone()).into(), project_id, creation_params)?;
        CarbonCredits::<T>::approve_project(RawOrigin::Signed(caller.clone()).into(), project_id, true)?;
        CarbonCredits::<T>::mint(RawOrigin::Signed(caller.clone()).into(), project_id, 100_u32.into(), false)?;
    }: _(RawOrigin::Signed(caller.clone()), project_id, 10_u32.into())
    verify {
        let item_id : T::ItemId = 0_u32.into();
        let retire_data = RetiredCredits::<T>::get(project_id, item_id).unwrap();
        assert_last_event::<T>(Event::VCURetired { project_id, account : caller, amount : 10_u32.into(), retire_data :retire_data.retire_data }.into());
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
        let project_id : T::AssetId = 1000_u32.into();
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
            batches: params.batches,
            created: 1_u32.into(),
            updated: None,
            approved: false,
            total_supply: 0_u32.into(),
            minted: 0_u32.into(),
            retired: 0_u32.into(),
            unit_price: params.unit_price,
        };
    }: _(RawOrigin::Root, project_id, new_project)
    verify {
        assert!(Projects::<T>::get(project_id).is_some());
    }

    force_set_next_item_id {
        let project_id : T::AssetId = 1000_u32.into();
        let next_item_id : T::ItemId = 100_u32.into();
    }: _(RawOrigin::Root, project_id, next_item_id)
    verify {
        assert_eq!(NextItemId::<T>::get(project_id).unwrap(), next_item_id);
    }

    force_set_retired_vcu {
        let account : T::AccountId = account("account_id", 0, 0);
        let project_id : T::AssetId = 1000_u32.into();
        let item_id : T::ItemId = 100_u32.into();
        let batch_data : BatchRetireDataOf::<T> = Default::default();
        let new_retire_data = RetiredVcuData::<T> {
            account,
            retire_data : vec![batch_data].try_into().unwrap(),
            timestamp : 1_u32.into(),
            count : 100_u32.into()
        };
    }: _(RawOrigin::Root, project_id, item_id, new_retire_data)
    verify {
        assert!(RetiredCredits::<T>::get(project_id, item_id).is_some());
    }

    impl_benchmark_test_suite!(CarbonCredits, crate::mock::new_test_ext(), crate::mock::Test);
}
