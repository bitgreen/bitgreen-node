// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::{mock::*, Error, Event, Orders};
use frame_support::{assert_noop, assert_ok, PalletId};
use orml_traits::MultiCurrency;
use sp_runtime::{traits::AccountIdConversion, Percent};

#[test]
fn basic_create_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);
		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 1);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		assert_eq!(
			last_event(),
			Event::SellOrderCreated {
				order_id: 0,
				asset_id,
				project_id: 0,
				group_id: 0,
				units: 5,
				price_per_unit: 1,
				owner: seller
			}
			.into()
		);
	});
}

#[test]
fn create_sell_order_less_than_minimum_should_fail() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// sell order with less than minimum units
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 0, 1),
			Error::<Test>::BelowMinimumUnits
		);

		// sell order with less than minimum price
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 0),
			Error::<Test>::BelowMinimumPrice
		);
	});
}

#[test]
fn create_sell_order_should_fail_if_caller_does_not_have_asset_balance() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);
		// should not be able to create a sell order since the amount is greater than seller balance
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 101, 1),
			pallet_assets::Error::<Test>::BalanceLow
		);
	});
}

#[test]
fn cancel_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 1));

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
		assert_noop!(
			Dex::cancel_sell_order(RuntimeOrigin::signed(4), 10),
			Error::<Test>::InvalidOrderId
		);

		// only owner can cancel sell order
		assert_noop!(
			Dex::cancel_sell_order(RuntimeOrigin::signed(4), 0),
			Error::<Test>::InvalidOrderOwner
		);

		// owner should be able to cancel a sell order
		assert_ok!(Dex::cancel_sell_order(RuntimeOrigin::signed(seller), 0));

		// storage should be updated correctly
		assert!(Orders::<Test>::get(0).is_none());

		// Balance should be returned correctly
		assert_eq!(Assets::balance(asset_id, seller), 100);
		assert_eq!(Assets::balance(asset_id, dex_account), 0);

		assert_eq!(last_event(), Event::SellOrderCancelled { order_id: 0 }.into());
	});
}

#[test]
fn buy_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 10));

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
			Dex::buy_order(RuntimeOrigin::signed(buyer), 10, 0, 1, 100),
			Error::<Test>::InvalidOrderId
		);

		// non matching asset_id should fail
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(buyer), 0, 10, 1, 100),
			Error::<Test>::InvalidAssetId
		);

		// more than listed volume should fail
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1000, 100),
			Error::<Test>::OrderUnitsOverflow
		);

		// should fail if the user does not have enough balance
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(5), 0, asset_id, 1, 100),
			orml_tokens::Error::<Test>::BalanceTooLow
		);

		// should fail if the buyer and seller are same
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(1), 0, asset_id, 1, 100),
			Error::<Test>::SellerAndBuyerCannotBeSame
		);

		// should fail if the fee is zero
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 0),
			Error::<Test>::FeeExceedsUserLimit
		);

		// should fail if the fee is less than expected
		assert_noop!(
			Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 0),
			Error::<Test>::FeeExceedsUserLimit
		);

		// use should be able to purchase
		assert_ok!(Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

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
		assert_eq!(Tokens::free_balance(USDT, &seller), 10);
		// buyer spends price_per_unit + fees (10 + 1 + 10)
		assert_eq!(Tokens::free_balance(USDT, &buyer), 79);
		// pallet gets fees (1 + 10)
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 11);

		assert_eq!(
			last_event(),
			Event::BuyOrderFilled {
				order_id: 0,
				units: 1,
				price_per_unit: 10,
				seller,
				buyer,
				fees_paid: 0u128,
				project_id: 0,
				group_id: 0,
			}
			.into()
		);
	});
}

#[test]
fn sell_order_is_removed_if_all_units_bought() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 10));

		// user should be able to purchase
		assert_ok!(Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 5, 100));

		// sell order should be removed since all units have been bought
		assert!(Orders::<Test>::get(0).is_none());

		// Token balance should be set correctly
		// seller gets the price_per_unit
		assert_eq!(Tokens::free_balance(USDT, &seller), 50);
		// buyer spends price_per_unit + fees (50 + 5 + 10)
		assert_eq!(Tokens::free_balance(USDT, &buyer), 35);
		// pallet gets fees (5 + 10)
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 15);
	});
}

#[test]
fn partial_fill_and_cancel_works() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 50, 10));

		// user should be able to purchase
		assert_ok!(Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 5, 100));

		// cancel sell order should return the remaining units
		assert_ok!(Dex::cancel_sell_order(RuntimeOrigin::signed(seller), 0));

		// Balance should be returned correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 0);

		assert_eq!(last_event(), Event::SellOrderCancelled { order_id: 0 }.into());

		// Token balance should be set correctly
		// seller gets the price_per_unit
		assert_eq!(Tokens::free_balance(USDT, &seller), 50);
		// buyer spends price_per_unit + fees (50 + 5 + 10)
		assert_eq!(Tokens::free_balance(USDT, &buyer), 35);
		// pallet gets fees (5 + 10)
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 15);
	});
}

#[test]
fn cannot_set_more_than_max_fee() {
	new_test_ext().execute_with(|| {
		// set fee values
		assert_noop!(
			Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(51)),
			Error::<Test>::CannotSetMoreThanMaxPaymentFee
		);
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
	});
}

#[test]
fn fee_is_more_expensive_when_order_is_split() {
	new_test_ext().execute_with(|| {
		// Assuming a 75 price, and fees at 10%; if a user buys 50 units they’ll pay 750 in fees
		// If they instead have 50 separate orders of 1 unit each, they should pay 775 in fees
		// Here we assume purchase fee is zero, since this is to test the payment fee calculation
		let asset_id = 0;
		let seller = 1;
		let buyer = 10;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 0u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 100, 75));

		// Let the user make a single purchase of 50 units
		assert_ok!(Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 50, 1000));
		// pallet gets fees (10%)
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 375);

		// Let the user make a purchse of 1 unit 50 times
		for _i in 0..50 {
			assert_ok!(Dex::buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 1000));
		}

		// pallet gets more than 20% (750)
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 775);
	});
}
