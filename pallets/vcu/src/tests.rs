use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn create_vcu_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::create_vcu(Origin::root(), 421, 10, b"QmXbTtSAPJ545YRnLt7n7ngMa4ZTmizmznshZZjXDRhYih".to_vec()));

		assert_eq!(VCU::get_vcu(421).is_empty(), false);
	});
}

#[test]
fn create_vcu_should_not_work_for_invalid_ipfs_hash() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_vcu(Origin::root(), 421, 10, b"test".to_vec()),
			Error::<Test>::InvalidIPFSHash
		);
	});
}

#[test]
fn create_vcu_should_not_work_for_invalid_project_name() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_vcu(Origin::root(), 0, 10, b"QmXbTtSAPJ545YRnLt7n7ngMa4ZTmizmznshZZjXDRhYih".to_vec()),
			Error::<Test>::InvalidPidLength
		);
	});
}

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
fn destroy_asset_generated_vcu_should_work_if_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		let input = r#"{"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":"1000"}"#.as_bytes().to_vec();

		assert_ok!(VCU::create_asset_generating_vcu(Origin::root(), 1, 1, input.clone()));
		assert_eq!(VCU::asset_generating_vcu(1, 1), input);

		assert_ok!(VCU::destroy_asset_generated_vcu(Origin::root(), 1, 1));
		assert_eq!(VCU::asset_generating_vcu(1, 1), b"".to_vec());
	});
}

#[test]
fn destroy_asset_generated_vcu_should_not_work_if_not_signed_by_root_or_authorized_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_asset_generated_vcu(Origin::signed(11), 1, 1),
			BadOrigin
		);
	});
}

#[test]
fn destroy_asset_generated_vcu_should_not_work_if_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::destroy_asset_generated_vcu(Origin::root(), 1, 1),
			Error::<Test>::AssetGeneratedVCUNotFound
		);
	});
}