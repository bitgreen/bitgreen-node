//! Tests for vcu pallet
use crate::{mock::*, Error, NextAssetId, VCUCreationParams, VCUDetail, VCUType, VCUs};
use frame_support::{assert_noop, assert_ok, traits::tokens::fungibles::Inspect};
use frame_system::RawOrigin;
use sp_std::convert::TryInto;

#[test]
fn add_new_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let authorised_account_two = 2;
        let authorised_account_three = 3;
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        assert_eq!(
            VCU::authorized_accounts().first(),
            Some(&authorised_account_one)
        );

        assert_noop!(
            VCU::force_add_authorized_account(RawOrigin::Root.into(), authorised_account_one,),
            Error::<Test>::AuthorizedAccountAlreadyExists
        );

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_two,
        ));

        assert_noop!(
            VCU::force_add_authorized_account(RawOrigin::Root.into(), authorised_account_three,),
            Error::<Test>::TooManyAuthorizedAccounts
        );
    });
}

#[test]
fn force_remove_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));
        assert_eq!(
            VCU::authorized_accounts().first(),
            Some(&authorised_account_one)
        );

        assert_ok!(VCU::force_remove_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        assert_eq!(VCU::authorized_accounts().len(), 0);
    });
}

#[test]
fn create_fails_for_unauthorized() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_id = 1;
        let vcu_type = VCUType::Single(vcu_id);
        let owner = 10;
        let recipient = owner;
        let amount = 100;

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_noop!(
            VCU::create(
                RawOrigin::Signed(authorised_account_one).into(),
                project_id,
                creation_params
            ),
            Error::<Test>::NotAuthorised
        );
    });
}

#[test]
fn create_works_for_single() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_id = 1;
        let vcu_type = VCUType::Single(vcu_id);
        let owner = 10;
        let recipient = owner;
        let amount = 100;
        let expected_asset_id = 0;

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_ok!(VCU::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            creation_params
        ));

        // Ensure the storage updated correctly
        assert_eq!(
            VCUs::<Test>::get(project_id, vcu_id).unwrap(),
            VCUDetail {
                originator: owner,
                supply: amount,
                retired: 0,
                asset_id: expected_asset_id,
                vcu_type
            }
        );

        assert_eq!(NextAssetId::<Test>::get(), 1);

        // Ensure the asset is created and minted correctly
        assert_eq!(Assets::total_issuance(expected_asset_id), amount);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), amount);
    });
}

#[test]
fn create_works_for_bundle() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_ids = vec![1, 2, 3];
        let vcu_type = VCUType::Bundle(vcu_ids.clone().try_into().unwrap());
        let owner = 10;
        let recipient = owner;
        let amount = 100;
        let expected_asset_id = 0;

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_ok!(VCU::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            creation_params
        ));

        // Ensure the storage updated correctly
        // ethe vcu_id is the first vcu_id in the bundle
        assert_eq!(
            VCUs::<Test>::get(project_id, 1).unwrap(),
            VCUDetail {
                originator: owner,
                supply: amount,
                retired: 0,
                asset_id: expected_asset_id,
                vcu_type
            }
        );

        // Ensure the asset is created and minted correctly
        assert_eq!(Assets::total_issuance(expected_asset_id), amount);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), amount);
    });
}

#[test]
fn retire_vcu_works_for_single() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_id = 1;
        let vcu_type = VCUType::Single(vcu_id);
        let owner = 10;
        let recipient = owner;
        let amount = 100;
        let expected_asset_id = 0;

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_ok!(VCU::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            creation_params
        ));

        // Ensure the storage updated correctly
        assert_eq!(
            VCUs::<Test>::get(project_id, vcu_id).unwrap(),
            VCUDetail {
                originator: owner,
                supply: amount,
                retired: 0,
                asset_id: expected_asset_id,
                vcu_type: vcu_type.clone()
            }
        );

        // Ensure the asset is created and minted correctly
        assert_eq!(Assets::total_issuance(expected_asset_id), amount);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), amount);

        assert_ok!(VCU::retire(
            RawOrigin::Signed(recipient).into(),
            project_id,
            vcu_id,
            amount
        ));

        assert_eq!(
            VCUs::<Test>::get(project_id, vcu_id).unwrap(),
            VCUDetail {
                originator: owner,
                supply: 0,
                retired: amount,
                asset_id: expected_asset_id,
                vcu_type
            }
        );

        assert_eq!(Assets::total_issuance(expected_asset_id), 0);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), 0);
    });
}

#[test]
fn retire_non_existent_vcu_should_fail() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_id = 1;
        let vcu_type = VCUType::Single(vcu_id);
        let owner = 10;
        let recipient = owner;
        let amount = 100;

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_ok!(VCU::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            creation_params
        ));

        assert_noop!(
            VCU::retire(
                RawOrigin::Signed(authorised_account_one).into(),
                project_id,
                vcu_id,
                amount
            ),
            pallet_assets::Error::<Test>::NoAccount
        );
    });
}

#[test]
fn mint_into_works() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let project_id = 1;
        let vcu_id = 1;
        let vcu_type = VCUType::Single(vcu_id);
        let owner = 10;
        let recipient = owner;
        let amount = 100;
        let expected_asset_id = 0;

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        let creation_params = VCUCreationParams {
            originator: owner,
            amount,
            recipient,
            vcu_type: vcu_type.clone(),
        };

        assert_ok!(VCU::create(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            creation_params
        ));

        // Ensure the asset is created and minted correctly
        assert_eq!(Assets::total_issuance(expected_asset_id), amount);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), amount);

        assert_ok!(VCU::mint_into(
            RawOrigin::Signed(authorised_account_one).into(),
            project_id,
            vcu_id,
            recipient,
            amount
        ));

        // Ensure the storage updated correctly
        assert_eq!(
            VCUs::<Test>::get(project_id, vcu_id).unwrap(),
            VCUDetail {
                originator: owner,
                supply: amount * 2,
                retired: 0,
                asset_id: expected_asset_id,
                vcu_type
            }
        );

        // Ensure the asset is created and minted correctly
        assert_eq!(Assets::total_issuance(expected_asset_id), amount * 2);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, recipient), amount * 2);
    });
}
