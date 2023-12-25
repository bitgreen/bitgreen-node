use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn add_member_works() {
	new_test_ext().execute_with(|| {
		let authorised_account = 1;
		assert_ok!(Membership::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account,
		));
		assert_noop!(
			Membership::add_member(RuntimeOrigin::signed(5), 15, UserLevel::KYCLevel1),
			crate::Error::<Test, _>::NotAuthorised
		);
		assert_noop!(
			Membership::add_member(
				RuntimeOrigin::signed(authorised_account),
				10,
				UserLevel::KYCLevel1
			),
			Error::<Test, _>::AlreadyMember
		);
		assert_ok!(Membership::add_member(
			RuntimeOrigin::signed(authorised_account),
			15,
			UserLevel::KYCLevel1
		));

		for member in [10, 15, 20, 30] {
			assert_eq!(Membership::members(member), Some(UserLevel::KYCLevel1));
		}
	});
}

#[test]
fn add_member_airdrop_works() {
	new_test_ext().execute_with(|| {
		let authorised_account = 1;
		assert_ok!(Membership::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account,
		));

		// set the airdrop amount
		let airdrop_amount = 10;
		assert_ok!(
			Membership::force_set_kyc_airdrop(RawOrigin::Root.into(), Some(airdrop_amount),)
		);

		// set some balance to the pallet account
		let kyc_pallet_account: u64 = PalletId(*b"bitg/kyc").into_account_truncating();
		Balances::make_free_balance_be(&kyc_pallet_account, 100);

		let balance_before_kyc = Balances::free_balance(15);
		assert_ok!(Membership::add_member(
			RuntimeOrigin::signed(authorised_account),
			15,
			UserLevel::KYCLevel1
		));
		assert_eq!(Balances::free_balance(15), balance_before_kyc + airdrop_amount);
	});
}

#[test]
fn remove_member_works() {
	new_test_ext().execute_with(|| {
		let authorised_account = 1;
		assert_ok!(Membership::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account,
		));
		assert_noop!(
			Membership::remove_member(RuntimeOrigin::signed(5), 20),
			Error::<Test, _>::NotAuthorised
		);
		assert_noop!(
			Membership::remove_member(RuntimeOrigin::signed(authorised_account), 15),
			Error::<Test, _>::NotMember
		);
		assert_ok!(Membership::remove_member(RuntimeOrigin::signed(authorised_account), 20));
		assert_eq!(Membership::members(20), None);
	});
}

#[test]
fn modify_member_works() {
	new_test_ext().execute_with(|| {
		let authorised_account = 1;
		assert_ok!(Membership::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account,
		));

		assert_noop!(
			Membership::modify_member(RuntimeOrigin::signed(5), 15, UserLevel::KYCLevel1),
			crate::Error::<Test, _>::NotAuthorised
		);

		assert_noop!(
			Membership::modify_member(
				RuntimeOrigin::signed(authorised_account),
				100,
				UserLevel::KYCLevel1
			),
			Error::<Test, _>::NotMember
		);

		assert_ok!(Membership::modify_member(
			RuntimeOrigin::signed(authorised_account),
			10,
			UserLevel::KYCLevel2
		));

		for member in [20, 30] {
			assert_eq!(Membership::members(member), Some(UserLevel::KYCLevel1));
		}

		assert_eq!(Membership::members(10), Some(UserLevel::KYCLevel2));
	});
}
