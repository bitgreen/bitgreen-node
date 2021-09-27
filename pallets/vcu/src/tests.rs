use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_vcu_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCU::create_vcu(Origin::signed(1), 42, "Veera".as_bytes().to_vec(), 10, b"QmXbTtSAPJ545YRnLt7n7ngMa4ZTmizmznshZZjXDRhYih".to_vec()));

		assert_eq!(VCU::get_vcu(1).serial_number, 42);
	});
}

#[test]
fn create_vcu_should_not_work_for_invalid_ipfs_hash() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_vcu(Origin::signed(1), 42, "Veera".as_bytes().to_vec(), 10, b"test".to_vec()),
			Error::<Test>::InvalidIPFSHash
		);
	});
}

#[test]
fn create_vcu_should_not_work_for_invalid_project_name() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCU::create_vcu(Origin::signed(1), 42, "".as_bytes().to_vec(), 10, b"QmXbTtSAPJ545YRnLt7n7ngMa4ZTmizmznshZZjXDRhYih".to_vec()),
			Error::<Test>::ProjectNameIsTooShort
		);
	});
}
