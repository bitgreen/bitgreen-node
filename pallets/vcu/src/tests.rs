use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

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