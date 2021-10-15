use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_support::error::BadOrigin;

#[test]
fn create_change_settings_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"[{'document':'Profit&Loss Previous year'}]".to_vec()));
	});
}

#[test]
fn create_change_settings_does_not_work_for_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::signed(1), b"kyc".to_vec(), b"[{'document':'Profit&Loss Previous year'}]".to_vec()),
			BadOrigin
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_json() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"[]".to_vec()),
			Error::<Test>::SettingsJsonTooShort
		);

		// This should work
		// assert_noop!(
		// 	Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"[{'document':'Profit&Loss Previous year']".to_vec()),
		// 	Error::<Test>::InvalidJson
		// );

	});
}
