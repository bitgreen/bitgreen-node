// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::{mock::*, Error, Orders};
use frame_support::{assert_noop, assert_ok, PalletId};
use orml_traits::MultiCurrency;
use sp_runtime::{traits::AccountIdConversion, Percent};

#[test]
fn basic_create_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(Origin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(Origin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);
		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(Origin::signed(seller), asset_id, 5, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 1);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);
	});
}

#[test]
fn create_sell_order_less_than_minimum_should_fail() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(Origin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(Origin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// sell order with less than minimum units
		assert_noop!(
			Dex::create_sell_order(Origin::signed(seller), asset_id, 0, 1),
			Error::<Test>::BelowMinimumUnits
		);

		// sell order with less than minimum price
		assert_noop!(
			Dex::create_sell_order(Origin::signed(seller), asset_id, 5, 0),
			Error::<Test>::BelowMinimumPrice
		);
	});
}

#[test]
fn cancel_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(Origin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(Origin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(Origin::signed(seller), asset_id, 5, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 1);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		// non existing order should fail
		assert_noop!(Dex::cancel_sell_order(Origin::signed(4), 10), Error::<Test>::InvalidOrderId);

		// only owner can cancel sell order
		assert_noop!(
			Dex::cancel_sell_order(Origin::signed(4), 0),
			Error::<Test>::InvalidOrderOwner
		);

		// owner should be able to cancel a sell order
		assert_ok!(Dex::cancel_sell_order(Origin::signed(seller), 0));

		// storage should be updated correctly
		assert!(Orders::<Test>::get(0).is_none());

		// Balance should be returned correctly
		assert_eq!(Assets::balance(asset_id, seller), 100);
		assert_eq!(Assets::balance(asset_id, dex_account), 0);
	});
}

#[test]
fn buy_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(Origin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(Origin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(Origin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(Origin::root(), Percent::from_percent(10)));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(Origin::signed(seller), asset_id, 5, 10));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 10);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		// non existing order should fail
		assert_noop!(
			Dex::buy_order(Origin::signed(buyer), 10, 0, 1),
			Error::<Test>::InvalidOrderId
		);

		// non matching asset_id should fail
		assert_noop!(
			Dex::buy_order(Origin::signed(buyer), 0, 10, 1),
			Error::<Test>::InvalidAssetId
		);

		// more than listed volume should fail
		assert_noop!(
			Dex::buy_order(Origin::signed(buyer), 0, asset_id, 1000),
			Error::<Test>::OrderUnitsOverflow
		);

		// should fail if the user does not have enough balance
		assert_noop!(
			Dex::buy_order(Origin::signed(5), 0, asset_id, 1),
			orml_tokens::Error::<Test>::BalanceTooLow
		);

		// use should be able to purchase
		assert_ok!(Dex::buy_order(Origin::signed(buyer), 0, asset_id, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 4);
		assert_eq!(sell_order_storage.price_per_unit, 10);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Asset balance should be set correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, buyer), 1);
		assert_eq!(Assets::balance(asset_id, dex_account), 4);

		// Token balance should be set correctly
		// seller gets the price_per_unit
		assert_eq!(Tokens::free_balance(AUSD, &seller), 10);
		// buyer spends price_per_unit + fees (10 + 1 + 1)
		assert_eq!(Tokens::free_balance(AUSD, &buyer), 88);
		// pallet gets fees (1 + 1)
		assert_eq!(Tokens::free_balance(AUSD, &dex_account), 2);
	});
}
