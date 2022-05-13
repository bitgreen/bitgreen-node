use crate::{
    mock::*, AssetGeneratingVCUContent, AssetGeneratingVCUContentOf,
    AssetsGeneratingVCUScheduleContent, Error,
};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use frame_system::RawOrigin;
use sp_std::convert::TryInto;

#[test]
fn add_new_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(VCU::add_authorized_account(
            RawOrigin::Root.into(),
            1,
            b"Verra".to_vec().try_into().unwrap()
        ));
        assert_eq!(
            VCU::get_authorized_accounts(1),
            Some(b"Verra".to_vec().try_into().unwrap())
        );
    });
}

#[test]
fn update_existing_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(VCU::add_authorized_account(
            RawOrigin::Root.into(),
            1,
            b"Verra".to_vec().try_into().unwrap()
        ));
        assert_eq!(
            VCU::get_authorized_accounts(1),
            Some(b"Verra".to_vec().try_into().unwrap())
        );

        assert_ok!(VCU::add_authorized_account(
            RawOrigin::Root.into(),
            1,
            b"Verra22".to_vec().try_into().unwrap()
        ));
        assert_eq!(
            VCU::get_authorized_accounts(1),
            Some(b"Verra22".to_vec().try_into().unwrap())
        );
    });
}

#[test]
fn add_authorized_accounts_should_not_work_for_invalid_description() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::add_authorized_account(
                RawOrigin::Root.into(),
                1,
                b"".to_vec().try_into().unwrap()
            ),
            Error::<Test>::InvalidDescription
        );
    });
}

#[test]
fn destroy_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(VCU::add_authorized_account(
            RawOrigin::Root.into(),
            1,
            b"Verra".to_vec().try_into().unwrap()
        ));
        assert_eq!(
            VCU::get_authorized_accounts(1),
            Some(b"Verra".to_vec().try_into().unwrap())
        );

        assert_ok!(VCU::destroy_authorized_account(RawOrigin::Root.into(), 1));
        assert_eq!(VCU::get_authorized_accounts(1), None);
    });
}

#[test]
fn create_asset_generating_vcu_should_work_if_signed_by_root_or_authorized_user() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));
        assert_eq!(VCU::asset_generating_vcu(1, 1), Some(input.clone()));

        assert_ok!(VCU::add_authorized_account(
            RawOrigin::Root.into(),
            11,
            b"Verra".to_vec().try_into().unwrap()
        ));

        assert_ok!(VCU::create_asset_generating_vcu(
            Origin::signed(11),
            1,
            1,
            input
        ));
    });
}

#[test]
fn create_asset_generating_vcu_should_not_work_if_not_valid_input() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 0,
            other_documents: None,
            expiry: None,
        };

        assert_noop!(
            VCU::create_asset_generating_vcu(RawOrigin::Root.into(), 1, 1, input.clone()),
            Error::<Test>::NumberofSharesCannotBeZero
        );

        assert_noop!(
            VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input),
            Error::<Test>::NotAuthorised
        );
    });
}

#[test]
fn destroy_asset_generating_vcu_should_work_if_signed_by_root_or_authorized_user() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));
        assert_eq!(VCU::asset_generating_vcu(1, 1), Some(input));

        assert_ok!(VCU::destroy_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1
        ));
        assert_eq!(VCU::asset_generating_vcu(1, 1), None);
    });
}

#[test]
fn destroy_asset_generating_vcu_should_not_work_if_not_signed_by_root_or_authorized_user() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::destroy_asset_generating_vcu(Origin::signed(11), 1, 1),
            Error::<Test>::NotAuthorised
        );
    });
}

#[test]
fn destroy_asset_generating_vcu_should_not_work_if_not_exists() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::destroy_asset_generating_vcu(RawOrigin::Root.into(), 1, 1),
            Error::<Test>::AssetGeneratingVCUNotFound
        );
    });
}

#[test]
fn create_asset_generating_vcu_schedule_should_work_if_signed_by_root_or_authorized_user() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        let expected_schedule = AssetsGeneratingVCUScheduleContent {
            period_days: 1_u64,
            amount_vcu: 1_u128,
            token_id: 10000_u32,
        };

        Balances::make_free_balance_be(&1, 1000);
        Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));
        assert_eq!(VCU::asset_generating_vcu(1, 1), Some(input));

        assert_ok!(VCU::create_asset_generating_vcu_schedule(
            RawOrigin::Root.into(),
            1,
            1,
            1,
            1,
            10000
        ));

        let stored_schedule: AssetsGeneratingVCUScheduleContent =
            VCU::asset_generating_vcu_schedule(1, 1).unwrap();
        assert_eq!(expected_schedule, stored_schedule);
    });
}

#[test]
fn create_asset_generating_vcu_schedule_should_not_work_if_not_exists() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::create_asset_generating_vcu_schedule(RawOrigin::Root.into(), 1, 1, 1, 1, 1),
            Error::<Test>::AssetGeneratingVCUNotFound
        );
    });
}

#[test]
fn create_asset_generating_vcu_schedule_should_not_work_if_amount_is_zero() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));
        assert_noop!(
            VCU::create_asset_generating_vcu_schedule(RawOrigin::Root.into(), 1, 1, 1, 0, 1),
            Error::<Test>::InvalidVCUAmount
        );
    });
}

#[test]
fn destroy_asset_generating_vcu_schedule_should_work_if_signed_by_root_or_authorized_user() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        let expected_schedule = AssetsGeneratingVCUScheduleContent {
            period_days: 1_u64,
            amount_vcu: 1_u128,
            token_id: 10000_u32,
        };

        Balances::make_free_balance_be(&1, 1000);
        Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));
        assert_eq!(VCU::asset_generating_vcu(1, 1), Some(input));

        assert_ok!(VCU::create_asset_generating_vcu_schedule(
            RawOrigin::Root.into(),
            1,
            1,
            1,
            1,
            10000
        ));

        let stored_schedule: AssetsGeneratingVCUScheduleContent =
            VCU::asset_generating_vcu_schedule(1, 1).unwrap();
        assert_eq!(expected_schedule, stored_schedule);

        assert_ok!(VCU::destroy_asset_generating_vcu_schedule(
            RawOrigin::Root.into(),
            1,
            1
        ));
        assert_eq!(VCU::asset_generating_vcu_schedule(1, 1), None);
    });
}

#[test]
fn destroy_asset_generating_vcu_schedule_should_not_work_if_not_exists() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::destroy_asset_generating_vcu_schedule(RawOrigin::Root.into(), 1, 1),
            Error::<Test>::AssetGeneratedVCUScheduleNotFound
        );
    });
}

// TODO : Fix time elapsed calculation
// #[test]
// fn mint_scheduled_vcu_should_work_if_signed_by_root_or_authorized_user() {
//     new_test_ext().execute_with(|| {
//
//         let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
//             description: b"Description".to_vec().try_into().unwrap(),
//             proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
//             number_of_shares: 1000,
//             other_documents: None,
//             expiry: None,
//         };
//
//         let token_id:u32 = 1;
//         let amount_vcu: u128 = 1000;
//
//         Balances::make_free_balance_be(&1, 1000);
//         Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();
//
// 		assert_ok!(VCU::add_authorized_account(RawOrigin::Root.into(), 11, b"Verra".to_vec().try_into().unwrap()));
// 		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input.clone()));
// 		assert_eq!(VCU::asset_generating_vcu(1, 1), Some(input));
//
//         assert_ok!(VCU::create_asset_generating_vcu_schedule(
//             RawOrigin::Root.into(),
//             1,
//             1,
//             1,
//             1,
//             10000
//         ));
//
// 		assert_eq!(Assets::total_supply(token_id), 0);
// 		assert_ok!(VCU::mint_scheduled_vcu(Origin::signed(11), 1,1));
// 		assert_eq!(Assets::total_supply(token_id), amount_vcu);
//
// 	});
// }

#[test]
fn mint_scheduled_vcu_should_not_work_if_not_exists() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::mint_scheduled_vcu(RawOrigin::Root.into(), 1, 1),
            Error::<Test>::AssetGeneratedVCUScheduleNotFound
        );
    });
}

// TODO : Fix after time elapsed fixes
// #[test]
// fn mint_scheduled_vcu_should_not_mint_if_schedule_has_been_expired() {
//     new_test_ext().execute_with(|| {
//         let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
//             description: b"Description".to_vec().try_into().unwrap(),
//             proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
//             number_of_shares: 1000,
//             other_documents: None,
//             expiry: None,
//         };
//
//         let expected_schedule = AssetsGeneratingVCUScheduleContent {
//             period_days: 1_u64,
//             amount_vcu: 1_u128,
//             token_id: 10000_u32,
//         };
//
//         Balances::make_free_balance_be(&1, 1000);
//         Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();
//
// 		assert_ok!(VCU::add_authorized_account(RawOrigin::Root.into(), 11, b"Verra".to_vec().try_into().unwrap()));
// 		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input.clone()));
//
//
// 		let token_id:u32 = 1;
// 		let amount_vcu: u128 = 1000;
//
//         assert_ok!(VCU::create_asset_generating_vcu_schedule(
//             RawOrigin::Root.into(),
//             1,
//             1,
//             1,
//             1,
//             10000
//         ));
//
// 		assert_eq!(Assets::total_supply(token_id), 0);
//
//
// 		assert_noop!(
// 		VCU::mint_scheduled_vcu(Origin::signed(11), 1, 1),
// 			Error::<Test>::AssetGeneratedScheduleExpired
// 		);
//
// 		assert_eq!(Assets::total_supply(token_id), 0);
// 	});
// }
//
#[test]
fn create_oracle_account_minting_vcu_should_work() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        let expected_schedule = AssetsGeneratingVCUScheduleContent {
            period_days: 1_u64,
            amount_vcu: 1_u128,
            token_id: 10000_u32,
        };

        Balances::make_free_balance_be(&1, 1000);
        Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));

        assert_ok!(VCU::create_oracle_account_minting_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            10,
            10_000
        ));
    });
}

#[test]
fn destroy_oracle_account_minting_vcu_should_work() {
    new_test_ext().execute_with(|| {
        let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 1000,
            other_documents: None,
            expiry: None,
        };

        let expected_schedule = AssetsGeneratingVCUScheduleContent {
            period_days: 1_u64,
            amount_vcu: 1_u128,
            token_id: 10000_u32,
        };

        Balances::make_free_balance_be(&1, 1000);
        Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();

        assert_ok!(VCU::create_asset_generating_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            input.clone()
        ));

        assert_ok!(VCU::create_oracle_account_minting_vcu(
            RawOrigin::Root.into(),
            1,
            1,
            10,
            10000
        ));
        assert_ok!(VCU::destroy_oracle_account_minting_vcu(
            RawOrigin::Root.into(),
            1,
            1
        ));
    });
}

#[test]
fn destroy_oracle_account_minting_vcu_not_work_for_non_existing_key() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            VCU::destroy_oracle_account_minting_vcu(RawOrigin::Root.into(), 1, 1),
            Error::<Test>::OraclesAccountMintingVCUNotFound
        );
    });
}

// #[test]
// fn mint_vcu_from_oracle_should_work() {
//     new_test_ext().execute_with(|| {
//         let input: AssetGeneratingVCUContentOf<Test> = AssetGeneratingVCUContent {
//             description: b"Description".to_vec().try_into().unwrap(),
//             proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
//             number_of_shares: 1000,
//             other_documents: None,
//             expiry: None,
//         };
//
//         let expected_schedule = AssetsGeneratingVCUScheduleContent {
//             period_days: 1_u64,
//             amount_vcu: 1_u128,
//             token_id: 10000_u32,
//         };
//
//         Balances::make_free_balance_be(&1, 1000);
//         Assets::create(Origin::signed(1), 10000, 1, 1_u32.into()).unwrap();
//
//
// 		assert_ok!(VCU::add_authorized_account(RawOrigin::Root.into(), 11, b"Verra".to_vec().try_into().unwrap()));
// 		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 11, 1, input.clone()));
//
//
// 		let asset_id:u32 = 1;
// 		let amount_vcu: u128 = 1000;
//
// 		assert_ok!(VCU::create_oracle_account_minting_vcu(RawOrigin::Root.into(), 1, 1, 10,10_000));
// 		assert_ok!(VCU::mint_vcu_from_oracle(Origin::signed(1), 1,1, amount_vcu));
// 		assert_eq!(Assets::total_supply(asset_id), amount_vcu);
//
// 	});
// }
