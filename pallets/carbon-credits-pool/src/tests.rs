// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! Tests for CarbonCredits pool pallet
use crate::{mock::*, Config, Error, Pools};
use frame_support::{
    assert_noop, assert_ok,
    traits::tokens::fungibles::{metadata::Inspect as MetadataInspect, Inspect},
};
use frame_system::RawOrigin;
use pallet_carbon_credits::{BatchGroupOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf};
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType};
use sp_runtime::Percent;
use sp_std::convert::TryInto;

pub type VCUPoolEvent = crate::Event<Test>;

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

/// helper function to generate multiple batch details
fn get_multiple_batch_group<T: Config>() -> BatchGroupOf<T> {
    let batches: BatchGroupOf<T> = vec![
        Batch {
            name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
            uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
            issuance_year: 2021_u32,
            start_date: 2021_u32,
            end_date: 2021_u32,
            total_supply: 100_u32.into(),
            minted: 0_u32.into(),
            retired: 0_u32.into(),
        },
        Batch {
            name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
            uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
            issuance_year: 2020_u32,
            start_date: 2020_u32,
            end_date: 2020_u32,
            total_supply: 100_u32.into(),
            minted: 0_u32.into(),
            retired: 0_u32.into(),
        },
    ]
    .try_into()
    .unwrap();

    batches
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
    <T as frame_system::Config>::AccountId: From<u32>,
{
    let royalty = Royalty::<T::AccountId> {
        account_id: 1_u32.into(),
        percent_of_fees: Percent::from_percent(0),
    };
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
        royalties: Some(vec![royalty].try_into().unwrap()),
        unit_price: 100_u32.into(),
    };

    creation_params
}

pub fn create_project_and_mint<T: Config>(
    project_id: u32,
    originator_account: u64,
    amount_to_mint: u32,
    batch: bool,
) {
    let mut creation_params = get_default_creation_params::<Test>();
    if batch {
        // replace the default with mutiple batches
        let created_batch_list = get_multiple_batch_group::<Test>();
        creation_params.batches = created_batch_list;
    }

    let authorised_account = 10;

    assert_ok!(CarbonCredits::create(
        RawOrigin::Signed(originator_account).into(),
        project_id,
        creation_params
    ));

    // approve project so minting can happen
    assert_ok!(CarbonCredits::force_add_authorized_account(
        RawOrigin::Root.into(),
        authorised_account
    ));
    assert_ok!(CarbonCredits::approve_project(
        RawOrigin::Signed(authorised_account).into(),
        project_id,
        true
    ),);

    // mint should work with all params correct
    assert_ok!(CarbonCredits::mint(
        RawOrigin::Signed(authorised_account).into(),
        project_id,
        amount_to_mint.into(),
        false
    ));
}

#[test]
fn test_cannot_create_pools_below_min_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCUPools::create(
                RawOrigin::Signed(1).into(),
                10,
                Default::default(),
                None,
                "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
            ),
            Error::<Test>::PoolIdBelowExpectedMinimum
        );
    });
}

#[test]
fn create_new_pools() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 10_000;

        assert_ok!(VCUPools::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            Default::default(),
            None,
            "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
        ));

        assert_eq!(
            last_event(),
            VCUPoolEvent::PoolCreated {
                admin: authorised_account_one,
                id: project_id,
                config: Default::default()
            }
            .into()
        );

        // Ensure asset is created
        assert_eq!(Assets::total_issuance(project_id), 0);
        assert_eq!(Assets::minimum_balance(project_id), 1);
        assert_eq!(Assets::name(project_id), "pool_xyz".as_bytes().to_vec());
        assert_eq!(Assets::symbol(project_id), "pool_xyz".as_bytes().to_vec());
        assert_eq!(Assets::decimals(project_id), 0_u8);

        assert_noop!(
            VCUPools::create(
                RawOrigin::Signed(authorised_account_one).into(),
                10_000,
                Default::default(),
                None,
                "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
            ),
            Error::<Test>::PoolIdInUse
        );
    });
}

#[test]
fn deposit_works() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1_000;
        let pool_id = 10_000;
        let project_tokens_to_mint = 100;
        let project_tokens_to_deposit = 99;

        assert_ok!(VCUPools::create(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            Default::default(),
            None,
            "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
        ));

        create_project_and_mint::<Test>(
            project_id,
            authorised_account_one,
            project_tokens_to_mint,
            false,
        );

        // deposit to pool should work
        assert_ok!(VCUPools::deposit(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            project_id,
            project_tokens_to_deposit
        ));

        assert_eq!(
            last_event(),
            VCUPoolEvent::Deposit {
                who: authorised_account_one,
                project_id,
                pool_id,
                amount: project_tokens_to_deposit
            }
            .into()
        );

        // The pool account should have the balance
        assert_eq!(
            Assets::total_issuance(project_id),
            project_tokens_to_mint.into()
        );
        assert_eq!(Assets::minimum_balance(project_id), 1);
        //assert_eq!(Assets::balance(project_id, ), 1);

        // The depositor should have lost the balance
        assert_eq!(Assets::balance(project_id, authorised_account_one), 1_u128);

        // The depositor should have gained equal pool tokens
        assert_eq!(
            Assets::balance(pool_id, authorised_account_one),
            project_tokens_to_deposit
        );

        // ensure storage updated correctly
        let stored_pool = Pools::<Test>::get(pool_id).unwrap();
        let stored_issuance_map = stored_pool.credits.get(&2020).unwrap();
        let amount = stored_issuance_map.get(&project_id).unwrap();
        assert_eq!(amount, &project_tokens_to_deposit);
    });
}

#[test]
fn deposit_works_for_batch_vcus() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1_000;
        let pool_id = 10_000;
        let project_tokens_to_mint = 100;
        let project_tokens_to_deposit = 99;

        assert_ok!(VCUPools::create(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            Default::default(),
            None,
            "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
        ));

        create_project_and_mint::<Test>(
            project_id,
            authorised_account_one,
            project_tokens_to_mint,
            true,
        );

        // deposit to pool should work
        assert_ok!(VCUPools::deposit(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            project_id,
            project_tokens_to_deposit
        ));

        assert_eq!(
            last_event(),
            VCUPoolEvent::Deposit {
                who: authorised_account_one,
                project_id,
                pool_id,
                amount: project_tokens_to_deposit
            }
            .into()
        );

        // The pool account should have the balance
        assert_eq!(
            Assets::total_issuance(project_id),
            project_tokens_to_mint.into()
        );
        assert_eq!(Assets::minimum_balance(project_id), 1);

        // The depositor should have lost the balance
        assert_eq!(Assets::balance(project_id, authorised_account_one), 1_u128);

        // The depositor should have gained equal pool tokens
        assert_eq!(
            Assets::balance(pool_id, authorised_account_one),
            project_tokens_to_deposit
        );

        // ensure storage updated correctly
        let stored_pool = Pools::<Test>::get(pool_id).unwrap();
        // the issuance date is the issuance date of oldest batch
        let stored_issuance_map = stored_pool.credits.get(&2020).unwrap();
        let amount = stored_issuance_map.get(&project_id).unwrap();
        assert_eq!(amount, &project_tokens_to_deposit);
    });
}

#[test]
fn retire_works() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1_000;
        let pool_id = 10_000;
        let project_tokens_to_mint = 100;
        let project_tokens_to_deposit = 99;

        assert_ok!(VCUPools::create(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            Default::default(),
            None,
            "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
        ));

        create_project_and_mint::<Test>(
            project_id,
            authorised_account_one,
            project_tokens_to_mint,
            false,
        );

        // deposit to pool should work
        assert_ok!(VCUPools::deposit(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            project_id,
            project_tokens_to_deposit
        ));

        // The pool account should have the balance
        assert_eq!(
            Assets::total_issuance(project_id),
            project_tokens_to_mint.into()
        );
        assert_eq!(Assets::minimum_balance(project_id), 1);
        //assert_eq!(Assets::balance(project_id, ), 1);

        // The depositor should have lost the balance
        assert_eq!(Assets::balance(project_id, authorised_account_one), 1_u128);

        // The depositor should have gained equal pool tokens
        assert_eq!(
            Assets::balance(pool_id, authorised_account_one),
            project_tokens_to_deposit
        );

        // retire more than balance should fail
        assert_noop!(
            VCUPools::retire(
                RawOrigin::Signed(authorised_account_one).into(),
                pool_id,
                10_000
            ),
            pallet_assets::Error::<Test>::BalanceLow
        );

        // retire should work
        assert_ok!(VCUPools::retire(
            RawOrigin::Signed(authorised_account_one).into(),
            pool_id,
            90
        ));

        assert_eq!(
            last_event(),
            VCUPoolEvent::Retired {
                who: authorised_account_one,
                pool_id,
                amount: 90
            }
            .into()
        );

        // the caller should have lost equivalent pool tokens
        assert_eq!(Assets::balance(pool_id, authorised_account_one), 9);
        assert_eq!(Assets::total_issuance(pool_id), 9);

        // ensure accounting worked correctly
        let stored_pool = Pools::<Test>::get(pool_id).unwrap();
        let stored_issuance_map = stored_pool.credits.get(&2020).unwrap();
        let amount = stored_issuance_map.get(&project_id).unwrap();
        assert_eq!(amount, &9_u128);

        // the equivalent project tokens should have been retired
        let stored_data =
            pallet_carbon_credits::Pallet::<Test>::get_project_details(project_id).unwrap();
        assert_eq!(stored_data.minted, 100_u32.into());
        assert_eq!(stored_data.retired, 90_u32.into());
    });
}
