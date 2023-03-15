// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use frame_support::{assert_noop, assert_ok, traits::Currency, PalletId};
use frame_system::RawOrigin;
use sp_runtime::traits::AccountIdConversion;

use crate::{mock::*, ContractDetail, Error, VestingBalance, VestingContracts};

pub type VestingContractEvent = crate::Event<Test>;

fn load_initial_pallet_balance(amount: u32) {
	// send some initial balance to vesting pallet
	let vesting_contract_pallet_acccount: u64 = PalletId(*b"bitg/vcp").into_account_truncating();
	Balances::make_free_balance_be(&vesting_contract_pallet_acccount, amount.into());
}

#[test]
fn add_new_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		let authorised_account_one = 1;
		let authorised_account_two = 2;
		let authorised_account_three = 3;
		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));

		assert_eq!(
			last_event(),
			VestingContractEvent::AuthorizedAccountAdded { account_id: authorised_account_one }
				.into()
		);

		assert_eq!(VestingContract::authorized_accounts().first(), Some(&authorised_account_one));

		assert_noop!(
			VestingContract::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account_one,
			),
			Error::<Test>::AuthorizedAccountAlreadyExists
		);

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_two,
		));

		assert_noop!(
			VestingContract::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account_three,
			),
			Error::<Test>::TooManyAuthorizedAccounts
		);

		assert_eq!(
			last_event(),
			VestingContractEvent::AuthorizedAccountAdded { account_id: authorised_account_two }
				.into()
		);
	});
}

#[test]
fn force_remove_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		let authorised_account_one = 1;
		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));
		assert_eq!(VestingContract::authorized_accounts().first(), Some(&authorised_account_one));

		assert_ok!(VestingContract::force_remove_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));

		assert_eq!(
			last_event(),
			VestingContractEvent::AuthorizedAccountRemoved { account_id: authorised_account_one }
				.into()
		);

		assert_eq!(VestingContract::authorized_accounts().len(), 0);
	});
}

#[test]
fn add_contract_fails_if_expiry_in_past() {
	new_test_ext().execute_with(|| {
		// Adding new contract fails since expiry is current block
		let expiry_block = 1;
		let vesting_amount = 1u32;
		let recipient = 1;
		let authorised_account = 10;

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		assert_noop!(
			VestingContract::add_new_contract(
				RawOrigin::Signed(authorised_account).into(),
				recipient,
				expiry_block,
				vesting_amount.into()
			),
			Error::<Test>::ExpiryInThePast
		);
	});
}

#[test]
fn add_contract_fails_if_caller_not_authorized() {
	new_test_ext().execute_with(|| {
		// can only add new contract if ForceOrigin
		let expiry_block = 10;
		let vesting_amount = 1u32;
		let recipient = 1;
		assert_noop!(
			VestingContract::add_new_contract(
				RawOrigin::Signed(1).into(),
				recipient,
				expiry_block,
				vesting_amount.into()
			),
			Error::<Test>::NotAuthorised
		);
	});
}

#[test]
fn add_contract_fails_if_pallet_out_of_funds() {
	new_test_ext().execute_with(|| {
		let expiry_block = 10;
		let vesting_amount = 1u32;
		let recipient = 1;
		let authorised_account = 10;

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		assert_noop!(
			VestingContract::add_new_contract(
				RawOrigin::Signed(authorised_account).into(),
				recipient,
				expiry_block,
				vesting_amount.into()
			),
			Error::<Test>::PalletOutOfFunds
		);
	});
}

#[test]
fn add_contract_works() {
	new_test_ext().execute_with(|| {
		let expiry_block = 10;
		let recipient = 1;
		let pallet_intial_balance = 200u32;
		let vesting_amount = pallet_intial_balance / 2u32;
		let authorised_account = 10;

		load_initial_pallet_balance(pallet_intial_balance);

		// Should fail if unauthorised account
		assert_noop!(
			VestingContract::add_new_contract(
				RawOrigin::Signed(20).into(),
				recipient,
				expiry_block,
				vesting_amount.into()
			),
			Error::<Test>::NotAuthorised
		);

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		// Adding new contract works
		assert_ok!(VestingContract::add_new_contract(
			RawOrigin::Signed(authorised_account).into(),
			recipient,
			expiry_block,
			vesting_amount.into()
		));

		// new contract is added in storage
		assert_eq!(
			VestingContracts::<Test>::get(recipient).unwrap(),
			ContractDetail { expiry: expiry_block, amount: vesting_amount.into() }
		);
		// ensure accounting worked correctly
		assert_eq!(VestingBalance::<Test>::get(), vesting_amount.into());

		assert_eq!(
			last_event(),
			VestingContractEvent::ContractAdded {
				recipient,
				expiry: expiry_block,
				amount: vesting_amount.into()
			}
			.into()
		);

		// Adding again failes with already exists error
		load_initial_pallet_balance(pallet_intial_balance);
		assert_noop!(
			VestingContract::add_new_contract(
				RawOrigin::Signed(authorised_account).into(),
				recipient,
				expiry_block,
				(vesting_amount * 2_u32).into()
			),
			Error::<Test>::ContractAlreadyExists
		);
	});
}

#[test]
fn remove_contract_works() {
	new_test_ext().execute_with(|| {
		let expiry_block = 10;
		let recipient = 1;
		let pallet_intial_balance = 100u32;
		let vesting_amount = 1u32;
		load_initial_pallet_balance(pallet_intial_balance);

		let authorised_account = 10;

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		assert_ok!(VestingContract::add_new_contract(
			RawOrigin::Signed(authorised_account).into(),
			recipient,
			expiry_block,
			vesting_amount.into()
		));

		assert_eq!(VestingBalance::<Test>::get(), vesting_amount.into());
		assert_ok!(VestingContract::remove_contract(
			RawOrigin::Signed(authorised_account).into(),
			recipient
		));

		// contract removed from storage
		assert_eq!(VestingContracts::<Test>::get(recipient), None);
		assert_eq!(last_event(), VestingContractEvent::ContractRemoved { recipient }.into());
	});
}

#[test]
fn withdraw_contract_works() {
	new_test_ext().execute_with(|| {
		let expiry_block = 10;
		let recipient = 1;
		let pallet_intial_balance = 100u32;
		let vesting_amount = 1u32;
		load_initial_pallet_balance(pallet_intial_balance);

		// cannot withdraw on non existent contract
		assert_noop!(
			VestingContract::withdraw_vested(RawOrigin::Signed(recipient).into(),),
			Error::<Test>::ContractNotFound
		);

		let authorised_account = 10;

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		assert_ok!(VestingContract::add_new_contract(
			RawOrigin::Signed(authorised_account).into(),
			recipient,
			expiry_block,
			vesting_amount.into()
		));

		// cannot withdraw before expiry
		assert_noop!(
			VestingContract::withdraw_vested(RawOrigin::Signed(recipient).into(),),
			Error::<Test>::ContractNotExpired
		);

		// time travel to after expiry block to withdraw vested amount
		System::set_block_number(expiry_block + 1);
		assert_ok!(VestingContract::withdraw_vested(RawOrigin::Signed(recipient).into(),));

		// the user balance should be updated
		assert_eq!(Balances::free_balance(recipient), vesting_amount.into());

		// the storage should be removed
		assert_eq!(VestingContracts::<Test>::get(recipient), None);
		assert_eq!(
			last_event(),
			VestingContractEvent::ContractWithdrawn {
				recipient,
				expiry: expiry_block,
				amount: vesting_amount.into()
			}
			.into()
		);

		// the pallet vesting balance should be updated
		assert_eq!(VestingBalance::<Test>::get(), 0);
	});
}

#[test]
fn force_withdraw_contract_works() {
	new_test_ext().execute_with(|| {
		let expiry_block = 10;
		let recipient = 1;
		let pallet_intial_balance = 100u32;
		let vesting_amount = 1u32;
		let authorised_account = 10;
		load_initial_pallet_balance(pallet_intial_balance);

		assert_ok!(VestingContract::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		// cannot withdraw on non existent contract
		assert_noop!(
			VestingContract::force_withdraw_vested(
				RawOrigin::Signed(authorised_account).into(),
				recipient
			),
			Error::<Test>::ContractNotFound
		);

		assert_ok!(VestingContract::add_new_contract(
			RawOrigin::Signed(authorised_account).into(),
			recipient,
			expiry_block,
			vesting_amount.into()
		));

		// cannot withdraw before expiry
		assert_noop!(
			VestingContract::force_withdraw_vested(
				RawOrigin::Signed(authorised_account).into(),
				recipient
			),
			Error::<Test>::ContractNotExpired
		);

		// time travel to after expiry block to withdraw vested amount
		System::set_block_number(expiry_block + 1);
		assert_ok!(VestingContract::force_withdraw_vested(
			RawOrigin::Signed(authorised_account).into(),
			recipient
		));

		// the user balance should be updated
		assert_eq!(Balances::free_balance(recipient), vesting_amount.into());

		// the storage should be removed
		assert_eq!(VestingContracts::<Test>::get(recipient), None);
		assert_eq!(
			last_event(),
			VestingContractEvent::ContractWithdrawn {
				recipient,
				expiry: expiry_block,
				amount: vesting_amount.into()
			}
			.into()
		);

		// the pallet vesting balance should be updated
		assert_eq!(VestingBalance::<Test>::get(), 0);
	});
}
