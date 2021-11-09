use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn create_proxy_settings_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::create_proxy_settings(Origin::root(), r#"{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()));
	});
}

#[test]
fn create_proxy_settings_should_not_work_for_invalid_json() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_proxy_settings(Origin::root(), r#"{"accounts":[ehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidJson
		);
	});
}

#[test]
fn create_proxy_settings_should_not_work_for_existing_key() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::create_proxy_settings(Origin::root(), r#"{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()));

		assert_noop!(
			VCU::create_proxy_settings(Origin::root(), r#"{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()),
			Error::<Test>::SettingsKeyExists
		);
	});
}

#[test]
fn create_proxy_settings_should_not_work_for_too_short_json() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_proxy_settings(Origin::root(), r#"{}"#.as_bytes().to_vec()),
			Error::<Test>::SettingsJsonTooShort
		);
	});
}

#[test]
fn destroy_proxy_settings_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::create_proxy_settings(Origin::root(), r#"{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()));

		assert_ok!(VCU::destroy_proxy_settings(Origin::root()));
	});
}

#[test]
fn destroy_proxy_settings_should_not_work_for_non_existing_key() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_proxy_settings(Origin::root()),
			Error::<Test>::SettingsKeyNotFound
		);
	});
}

#[test]
fn add_new_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::add_authorized_account(Origin::root(), 1, b"Verra".to_vec()));
		assert_eq!(VCU::get_authorized_accounts(1), b"Verra".to_vec());
	});
}

#[test]
fn update_existing_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::add_authorized_account(Origin::root(), 1, b"Verra".to_vec()));
		assert_eq!(VCU::get_authorized_accounts(1), b"Verra".to_vec());

		assert_ok!(VCU::add_authorized_account(Origin::root(), 1, b"Verra22".to_vec()));
		assert_eq!(VCU::get_authorized_accounts(1), b"Verra22".to_vec());

	});
}

#[test]
fn add_authorized_accounts_should_not_work_for_invalid_description() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::add_authorized_account(Origin::root(), 1, b"".to_vec()),
			Error::<Test>::InvalidDescription
		);
	});
}

#[test]
fn destroy_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::add_authorized_account(Origin::root(), 1, b"Verra".to_vec()));
		assert_eq!(VCU::get_authorized_accounts(1), b"Verra".to_vec());

		assert_ok!(VCU::destroy_authorized_account(Origin::root(), 1));
		assert_eq!(VCU::get_authorized_accounts(1), b"".to_vec());
	});
}

#[test]
fn destroy_authorized_accounts_should_not_work_for_non_existing_account() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_authorized_account(Origin::root(), 1),
			Error::<Test>::AuthorizedAccountsAGVNotFound
		);
	});
}

#[test]
fn create_asset_generating_vcu_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":10000}"#.as_bytes().to_vec();
		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		assert_ok!(VCU::add_authorized_account(Origin::root(), 11, b"Verra".to_vec()));
		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input));
	});
}

#[test]
fn create_asset_generating_vcu_should_not_work_if_not_valid_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_asset_generating_vcu(Origin::root(), 1, 1, r#"{"description":"", "proofOwnership":"ipfslink", "numberOfShares":10000}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidDescription
		);

		assert_noop!(
			VCU::create_asset_generating_vcu(Origin::root(), 1, 1, r#"{"description":"description", "proofOwnership":"", "numberOfShares":10000}"#.as_bytes().to_vec()),
			Error::<Test>::ProofOwnershipNotFound
		);

		assert_noop!(
			VCU::create_asset_generating_vcu(Origin::root(), 1, 1, r#"{"description":"description", "proofOwnership":"proofOwnership", "numberOfShares":""}"#.as_bytes().to_vec()),
			Error::<Test>::NumberofSharesNotFound
		);

		assert_noop!(
			VCU::create_asset_generating_vcu(Origin::root(), 1, 1, r#"{"description":"description", "proofOwnership":"proofOwnership", "numberOfShares":10001}"#.as_bytes().to_vec()),
			Error::<Test>::TooManyNumberofShares
		);
	});
}


#[test]
fn create_asset_generating_vcu_should_not_work_if_not_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, b"Verra".to_vec()),
			BadOrigin
		);
	});
}

#[test]
fn destroy_asset_generating_vcu_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();

		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		assert_ok!(VCU::destroy_asset_generating_vcu(Origin::root(), 1, 1));
		assert_eq!(VCU::asset_generating_vcu(1, 1), b"".to_vec());
	});
}

#[test]
fn destroy_asset_generating_vcu_should_not_work_if_not_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_asset_generating_vcu(Origin::signed(11), 1, 1),
			BadOrigin
		);
	});
}

#[test]
fn destroy_asset_generating_vcu_should_not_work_if_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_asset_generating_vcu(Origin::root(), 1, 1),
			Error::<Test>::AssetGeneratedVCUNotFound
		);
	});
}

#[test]
fn create_asset_generating_vcu_schedule_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();
		let j =r#"{"period_days":1,"amount_vcu":1,"token_id":1}"#;
		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		assert_ok!(VCU::create_asset_generating_vcu_schedule(Origin::root(), 1, 1, 1, 1, 1));

		let v: Vec<u8> = VCU::asset_generating_vcu_schedule(1, 1);
		assert_eq!(sp_std::str::from_utf8(&v).unwrap(), j);
	});
}

#[test]
fn create_asset_generating_vcu_schedule_should_not_work_if_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_asset_generating_vcu_schedule(Origin::root(), 1, 1, 1, 1, 1),
			Error::<Test>::AssetGeneratedVCUNotFound
		);
	});
}

#[test]
fn create_asset_generating_vcu_schedule_should_not_work_if_amount_is_zero() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();
		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_noop!(
			VCU::create_asset_generating_vcu_schedule(Origin::root(), 1, 1, 1, 0, 1),
			Error::<Test>::InvalidVCUAmount
		);
	});
}

#[test]
fn destroy_asset_generating_vcu_schedule_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();
		let j =r#"{"period_days":1,"amount_vcu":1,"token_id":1}"#;
		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		assert_ok!(VCU::create_asset_generating_vcu_schedule(Origin::root(), 1, 1, 1, 1, 1));

		let v: Vec<u8> = VCU::asset_generating_vcu_schedule(1, 1);
		assert_eq!(sp_std::str::from_utf8(&v).unwrap(), j);

		assert_ok!(VCU::destroy_asset_generating_vcu_schedule(Origin::root(), 1, 1));
		assert_eq!(VCU::asset_generating_vcu_schedule(1, 1), b"".to_vec());
	});
}

#[test]
fn destroy_asset_generating_vcu_schedule_should_not_work_if_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_asset_generating_vcu_schedule(Origin::root(), 1, 1),
			Error::<Test>::AssetGeneratedVCUSchedule
		);
	});
}

#[test]
fn mint_scheduled_vcu_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {

		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();
		assert_ok!(VCU::add_authorized_account(Origin::root(), 11, b"Verra".to_vec()));
		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		let token_id:u32 = 1;
		let amount_vcu: u128 = 1000;

		assert_ok!(VCU::create_asset_generating_vcu_schedule(Origin::signed(11), 1, 1, 0, amount_vcu, token_id));

		assert_eq!(Assets::total_supply(token_id), 0);

		assert_ok!(VCU::mint_scheduled_vcu(Origin::signed(11), 1, 1));

		assert_eq!(Assets::total_supply(token_id), amount_vcu);

	});
}

#[test]
fn mint_scheduled_vcu_should_not_work_if_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::mint_scheduled_vcu(Origin::root(), 1, 1),
			Error::<Test>::AssetGeneratedVCUSchedule
		);
	});
}

#[test]
fn mint_scheduled_vcu_should_not_mint_if_schedule_has_been_expired() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();
		assert_ok!(VCU::add_authorized_account(Origin::root(), 11, b"Verra".to_vec()));
		assert_ok!(VCU::create_asset_generating_vcu(Origin::signed(11), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		let token_id:u32 = 1;
		let amount_vcu: u128 = 1000;

		assert_ok!(VCU::create_asset_generating_vcu_schedule(Origin::signed(11), 1, 1, 1, amount_vcu, token_id));

		assert_eq!(Assets::total_supply(token_id), 0);

		assert_noop!(
			VCU::mint_scheduled_vcu(Origin::signed(11), 1, 1),
			Error::<Test>::AssetGeneratedScheduleExpired
		);

		assert_eq!(Assets::total_supply(token_id), 0);
	});
}
