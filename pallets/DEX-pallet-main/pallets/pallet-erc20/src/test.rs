use crate::mock::*;
use crate::Error;
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;
use sp_core::U256;
use sp_runtime::DispatchError::BadOrigin;

fn assert_last_event(_event: crate::Event<Test>) {
    // let last_event = frame_system::Pallet::<Test>::events()
    //     .pop()
    //     .expect("Event expected")
    //     .event;
    // assert_eq!(last_event, mock::Event::Erc20Token(event));
}

#[test]
fn mint() {
    new_test_ext().execute_with(|| {
        let account_id = 1;

        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            account_id,
            U256::from(100)
        ));
        assert_eq!(Erc20Token::total_supply(), U256::from(100));
        assert_eq!(Erc20Token::balance(account_id), U256::from(100));
        assert_last_event(crate::Event::Mint(
            account_id,
            U256::from(100),
            U256::from(100),
        ));
    });
}

#[test]
fn mint_should_fail_when_no_root_rights() {
    new_test_ext().execute_with(|| {
        let account_id = 1;

        assert_err!(
            Erc20Token::mint(
                RawOrigin::Signed(account_id).into(),
                account_id,
                U256::from(100)
            ),
            BadOrigin
        );
    });
}

#[test]
fn mint_should_fail_when_overflow() {
    new_test_ext().execute_with(|| {
        let account_id = 1;

        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            account_id,
            U256::MAX
        ));
        assert_eq!(Erc20Token::total_supply(), U256::MAX);
        assert_err!(
            Erc20Token::mint(RawOrigin::Root.into(), account_id, U256::from(1)),
            Error::<Test>::ArithmeticError
        );
    });
}

#[test]
fn burn() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            account_id,
            U256::from(100)
        ));

        assert_ok!(Erc20Token::burn(
            RawOrigin::Root.into(),
            account_id,
            U256::from(10)
        ));
        assert_eq!(Erc20Token::balance(account_id), U256::from(90));
        assert_eq!(Erc20Token::total_supply(), U256::from(90));
        assert_last_event(crate::Event::Burn(
            account_id,
            U256::from(10),
            U256::from(90),
        ));
    });
}

#[test]
fn burn_should_fail_when_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let account_id = 1;

        assert_err!(
            Erc20Token::burn(RawOrigin::Root.into(), account_id, U256::from(1)),
            Error::<Test>::BalanceNotEnough
        );
    });
}

#[test]
fn burn_should_fail_when_no_root_user() {
    new_test_ext().execute_with(|| {
        let account_id = 1;

        assert_err!(
            Erc20Token::burn(
                RawOrigin::Signed(account_id).into(),
                account_id,
                U256::from(1)
            ),
            BadOrigin
        );
    });
}

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        let source_account_id = 1;
        let dest_account_id = 2;

        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            source_account_id,
            U256::from(25)
        ));
        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            dest_account_id,
            U256::from(36)
        ));
        assert_eq!(Erc20Token::total_supply(), U256::from(61));
        assert_eq!(Erc20Token::balance(source_account_id), U256::from(25));
        assert_eq!(Erc20Token::balance(dest_account_id), U256::from(36));

        assert_ok!(Erc20Token::transfer(
            RawOrigin::Signed(source_account_id).into(),
            dest_account_id,
            U256::from(11)
        ));

        assert_eq!(Erc20Token::balance(source_account_id), U256::from(14));
        assert_eq!(Erc20Token::balance(dest_account_id), U256::from(47));
        assert_eq!(Erc20Token::total_supply(), U256::from(61));

        assert_last_event(crate::Event::Transfer(
            source_account_id,
            dest_account_id,
            U256::from(11),
        ));
    });
}

#[test]
fn transfer_should_fail_when_not_enough_balance() {
    new_test_ext().execute_with(|| {
        let source_account_id = 1;
        let dest_account_id = 2;

        assert_eq!(Erc20Token::balance(source_account_id), U256::zero());
        assert_eq!(Erc20Token::balance(dest_account_id), U256::zero());

        assert_err!(
            Erc20Token::transfer(
                RawOrigin::Signed(source_account_id).into(),
                dest_account_id,
                U256::from(100)
            ),
            Error::<Test>::BalanceNotEnough
        );
    });
}

#[test]
fn approve() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;

        assert_ok!(Erc20Token::approve(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(15)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(15)
        );
        assert_last_event(crate::Event::Approval(
            account_id,
            spender_account_id,
            U256::from(15),
        ));
    });
}

#[test]
fn increase_allowance() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;

        assert_ok!(Erc20Token::increase_allowance(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(17)
        );

        assert_ok!(Erc20Token::increase_allowance(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(2)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(19)
        );

        assert_last_event(crate::Event::Approval(
            account_id,
            spender_account_id,
            U256::from(19),
        ));
    });
}

#[test]
fn increase_allowance_should_fail_when_overflow_error() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;

        assert_ok!(Erc20Token::increase_allowance(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(17)
        );

        assert_err!(
            Erc20Token::increase_allowance(
                RawOrigin::Signed(account_id).into(),
                spender_account_id,
                U256::MAX
            ),
            Error::<Test>::ArithmeticError
        );
    });
}

#[test]
fn decrease_allowance() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;

        assert_ok!(Erc20Token::approve(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(17)
        );

        assert_ok!(Erc20Token::decrease_allowance(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(3)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(14)
        );

        assert_last_event(crate::Event::Approval(
            account_id,
            spender_account_id,
            U256::from(14),
        ));
    });
}

#[test]
fn decrease_allowance_should_fail_when_arithmetic_error() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;

        assert_ok!(Erc20Token::approve(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(17)
        );

        assert_err!(
            Erc20Token::decrease_allowance(
                RawOrigin::Signed(account_id).into(),
                spender_account_id,
                U256::from(18)
            ),
            Error::<Test>::ArithmeticError
        );
    });
}

#[test]
fn transfer_from() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;
        let dest_account_id = 3;

        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            account_id,
            U256::from(25)
        ));
        assert_ok!(Erc20Token::approve(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));

        assert_ok!(Erc20Token::transfer_from(
            RawOrigin::Signed(spender_account_id).into(),
            account_id,
            dest_account_id,
            U256::from(10)
        ));
        assert_eq!(Erc20Token::balance(spender_account_id), U256::zero());
        assert_eq!(Erc20Token::balance(account_id), U256::from(15));
        assert_eq!(Erc20Token::balance(dest_account_id), U256::from(10));
        assert_eq!(
            Erc20Token::allowance(account_id, spender_account_id),
            U256::from(7)
        );

        assert_last_event(crate::Event::Transfer(
            account_id,
            dest_account_id,
            U256::from(10),
        ));
    });
}

#[test]
fn transfer_from_should_fail_when_insufficient_allowance() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        let spender_account_id = 2;
        let destination_account_id = 3;

        assert_ok!(Erc20Token::mint(
            RawOrigin::Root.into(),
            account_id,
            U256::from(25)
        ));
        assert_ok!(Erc20Token::approve(
            RawOrigin::Signed(account_id).into(),
            spender_account_id,
            U256::from(17)
        ));

        assert_err!(
            Erc20Token::transfer_from(
                RawOrigin::Signed(spender_account_id).into(),
                account_id,
                destination_account_id,
                U256::from(18)
            ),
            Error::<Test>::InsufficientAllowance
        );
    });
}
