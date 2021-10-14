use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_support::error::BadOrigin;

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }

#[test]
fn create_change_settings_does_not_work_for_non_root() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			Bonds::create_change_settings(Origin::signed(1), b"kyc".to_vec(), b"[{'document':'Profit&Loss Previous year'}]".to_vec()),
			BadOrigin
		);
	});
}
