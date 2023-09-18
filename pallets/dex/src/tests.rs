// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::{
	mock::*, types::UserLevel, BuyOrders, BuyOrdersByUser, Error, Event, Orders, SellerReceivables,
};
use frame_support::{
	assert_noop, assert_ok, traits::OnIdle, weights::Weight, BoundedVec, PalletId,
};
use frame_system::RawOrigin;
use sp_runtime::{traits::AccountIdConversion, Percent};

/// helper function to add authorised account
fn add_validator_account(validator_account: u64) {
	// authorise the account
	assert_ok!(Dex::force_add_validator_account(RawOrigin::Root.into(), validator_account));
}

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

		assert_eq!(last_event(), Event::SellOrderCancelled { order_id: 0, seller }.into());
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

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

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
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 10, 0, 1, 100),
			Error::<Test>::InvalidOrderId
		);

		// non kyc buyer should fail
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(20), 0, 10, 1, 100),
			Error::<Test>::KYCAuthorisationFailed
		);

		// non matching asset_id should fail
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, 10, 1, 100),
			Error::<Test>::InvalidAssetId
		);

		// more than listed volume should fail
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1000, 100),
			Error::<Test>::OrderUnitsOverflow
		);

		// should fail if the buyer and seller are same
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(seller), 0, asset_id, 1, 100),
			Error::<Test>::SellerAndBuyerCannotBeSame
		);

		// should fail if the fee is zero
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 0),
			Error::<Test>::FeeExceedsUserLimit
		);

		// should fail if the fee is less than expected
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 0),
			Error::<Test>::FeeExceedsUserLimit
		);

		// use should be able to purchase
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		// sell order storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 4);
		assert_eq!(sell_order_storage.price_per_unit, 10);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Asset balance should be set correctly
		// no transfers until payment is validated
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, buyer), 0);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		// buy order storage should be updated correctly
		let buy_order_storage = BuyOrders::<Test>::get(0).unwrap();
		assert_eq!(buy_order_storage.buyer, buyer);
		assert_eq!(buy_order_storage.units, 1);
		assert_eq!(buy_order_storage.price_per_unit, 10);
		assert_eq!(buy_order_storage.asset_id, asset_id);
		assert_eq!(buy_order_storage.total_fee, 11);
		assert_eq!(buy_order_storage.total_amount, 21);
		assert!(buy_order_storage.payment_info.is_none());

		assert_eq!(
			last_event(),
			Event::BuyOrderCreated {
				order_id: 0,
				sell_order_id: 0,
				units: 1,
				price_per_unit: 10,
				seller,
				buyer,
				fees_paid: 11u128,
				total_amount: 21u128,
				project_id: 0,
				group_id: 0,
			}
			.into()
		);
	});
}

#[test]
fn validate_buy_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let validator = 10;
		let buy_order_id = 0;

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 10));

		// create a new buy order
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		let tx_proof: BoundedVec<_, _> = vec![].try_into().unwrap();

		// non validator cannot validate
		assert_noop!(
			Dex::validate_buy_order(
				RuntimeOrigin::signed(buyer),
				buy_order_id,
				0u32,
				vec![].try_into().unwrap()
			),
			Error::<Test>::NotAuthorised
		);

		// validator can validate a payment order
		add_validator_account(validator);
		assert_ok!(Dex::validate_buy_order(
			RuntimeOrigin::signed(validator),
			buy_order_id,
			0u32,
			tx_proof.clone()
		));

		// buy order storage should be updated correctly
		let buy_order_storage = BuyOrders::<Test>::get(0).unwrap();
		assert_eq!(buy_order_storage.buyer, buyer);
		assert_eq!(buy_order_storage.units, 1);
		assert_eq!(buy_order_storage.price_per_unit, 10);
		assert_eq!(buy_order_storage.asset_id, asset_id);
		assert_eq!(buy_order_storage.total_fee, 11);
		assert_eq!(buy_order_storage.total_amount, 21);

		let payment_info = buy_order_storage.payment_info.unwrap();
		assert_eq!(payment_info.chain_id, 0u32);
		assert_eq!(payment_info.tx_proof, tx_proof);
		assert_eq!(payment_info.validators.len(), 1);
		assert_eq!(payment_info.validators.first().unwrap(), &validator);

		assert_eq!(
			last_event(),
			Event::BuyOrderPaymentValidated { order_id: 0, chain_id: 0u32, validator }.into()
		);
	});
}

#[test]
fn payment_is_processed_after_validator_threshold_reached() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let validator = 10;
		let validator_two = 11;
		let buy_order_id = 0;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 10));

		add_validator_account(validator);
		add_validator_account(validator_two);

		// create a new buy order
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		let tx_proof: BoundedVec<_, _> = vec![].try_into().unwrap();

		// non validator cannot validate
		assert_noop!(
			Dex::validate_buy_order(
				RuntimeOrigin::signed(buyer),
				buy_order_id,
				0u32,
				vec![].try_into().unwrap()
			),
			Error::<Test>::NotAuthorised
		);

		// validator can validate a payment order
		assert_ok!(Dex::validate_buy_order(
			RuntimeOrigin::signed(validator),
			buy_order_id,
			0u32,
			tx_proof.clone()
		));

		// buy order storage should be updated correctly
		let buy_order_storage = BuyOrders::<Test>::get(0).unwrap();
		assert_eq!(buy_order_storage.buyer, buyer);
		assert_eq!(buy_order_storage.units, 1);
		assert_eq!(buy_order_storage.price_per_unit, 10);
		assert_eq!(buy_order_storage.asset_id, asset_id);
		assert_eq!(buy_order_storage.total_fee, 11);
		assert_eq!(buy_order_storage.total_amount, 21);

		let payment_info = buy_order_storage.payment_info.unwrap();
		assert_eq!(payment_info.chain_id, 0u32);
		assert_eq!(payment_info.tx_proof, tx_proof);
		assert_eq!(payment_info.validators.len(), 1);
		assert_eq!(payment_info.validators.first().unwrap(), &validator);

		assert_eq!(
			last_event(),
			Event::BuyOrderPaymentValidated { order_id: 0, chain_id: 0u32, validator }.into()
		);

		//	same validator cannot validate again
		assert_noop!(
			Dex::validate_buy_order(
				RuntimeOrigin::signed(validator),
				buy_order_id,
				0u32,
				tx_proof.clone()
			),
			Error::<Test>::DuplicateValidation
		);

		// ensure the storage is updated correctly
		let order_storage_by_user = BuyOrdersByUser::<Test>::get(buyer);
		let expected_storage = vec![(0, 1)];
		assert_eq!(order_storage_by_user.unwrap().into_inner(), expected_storage);

		//	next validator validates
		assert_ok!(Dex::validate_buy_order(
			RuntimeOrigin::signed(validator_two),
			buy_order_id,
			0u32,
			tx_proof
		));

		assert_eq!(
			last_event(),
			Event::BuyOrderFilled {
				order_id: 0,
				sell_order_id: 0,
				buyer,
				seller,
				fees_paid: buy_order_storage.total_fee,
				units: buy_order_storage.units,
				group_id: 0,
				price_per_unit: buy_order_storage.price_per_unit,
				project_id: 0,
			}
			.into()
		);

		// seller receivable should be updated with the correct amount
		let seller_receivables = SellerReceivables::<Test>::get(seller).unwrap();
		assert_eq!(seller_receivables, 10);

		// buy order storage should be cleared since payment is done
		let buy_order_storage = BuyOrders::<Test>::get(0);
		assert!(buy_order_storage.is_none());

		// Asset balance should be set correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, buyer), 1);
		assert_eq!(Assets::balance(asset_id, dex_account), 4);

		// ensure the storage is updated correctly
		let order_storage_by_user = BuyOrdersByUser::<Test>::get(buyer);
		let expected_storage = vec![];
		assert_eq!(order_storage_by_user.unwrap().into_inner(), expected_storage);
	});
}

// #[test]
// fn partial_fill_and_cancel_works() {
// 	new_test_ext().execute_with(|| {
// 		let asset_id = 0;
// 		let seller = 1;
// 		let buyer = 4;
// 		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

// 		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
// 		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
// 		assert_eq!(Assets::balance(asset_id, seller), 100);

// 		// set fee values
// 		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
// 		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

// 		// should be able to create a sell order
// 		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 50, 10));

// 		// user should be able to purchase
// 		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 5, 100));

// 		// cancel sell order should return the remaining units
// 		assert_ok!(Dex::cancel_sell_order(RuntimeOrigin::signed(seller), 0));

// 		// Balance should be returned correctly
// 		assert_eq!(Assets::balance(asset_id, seller), 95);
// 		assert_eq!(Assets::balance(asset_id, dex_account), 0);

// 		assert_eq!(last_event(), Event::SellOrderCancelled { order_id: 0, seller }.into());

// 		// Token balance should be set correctly
// 		// seller gets the price_per_unit
// 		assert_eq!(Tokens::free_balance(USDT, &seller), 50);
// 		// buyer spends price_per_unit + fees (50 + 5 + 10)
// 		assert_eq!(Tokens::free_balance(USDT, &buyer), 35);
// 		// pallet gets fees (5 + 10)
// 		assert_eq!(Tokens::free_balance(USDT, &dex_account), 15);
// 	});
// }

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

// #[test]
// fn fee_is_more_expensive_when_order_is_split() {
// 	new_test_ext().execute_with(|| {
// 		// Assuming a 75 price, and fees at 10%; if a user buys 50 units they’ll pay 750 in fees
// 		// If they instead have 50 separate orders of 1 unit each, they should pay 775 in fees
// 		// Here we assume purchase fee is zero, since this is to test the payment fee calculation
// 		let asset_id = 0;
// 		let seller = 1;
// 		let buyer = 10;
// 		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

// 		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
// 		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
// 		assert_eq!(Assets::balance(asset_id, seller), 100);

// 		// set fee values
// 		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
// 		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 0u32.into()));

// 		// should be able to create a sell order
// 		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 100, 75));

// 		// Let the user make a single purchase of 50 units
// 		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 50, 1000));
// 		// pallet gets fees (10%)
// 		assert_eq!(Tokens::free_balance(USDT, &dex_account), 375);

// 		// Let the user make a purchse of 1 unit 50 times
// 		for _i in 0..50 {
// 			assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 1000));
// 		}

// 		// pallet gets more than 20% (750)
// 		assert_eq!(Tokens::free_balance(USDT, &dex_account), 775);
// 	});
// }

#[test]
fn buy_order_handle_expiry_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let validator = 10;

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, 10));

		add_validator_account(validator);

		// use should be able to purchase
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		// sell order storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 4);
		assert_eq!(sell_order_storage.price_per_unit, 10);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// buy order storage should be updated correctly
		let buy_order_storage = BuyOrders::<Test>::get(0).unwrap();
		assert_eq!(buy_order_storage.buyer, buyer);
		assert_eq!(buy_order_storage.units, 1);
		assert_eq!(buy_order_storage.expiry_time, 3);

		Dex::on_idle(3, Weight::MAX);

		// the order should not be cleared before expiry
		assert!(BuyOrders::<Test>::get(0).is_some());

		Dex::on_idle(6, Weight::MAX);

		// the order should be cleared
		assert!(BuyOrders::<Test>::get(0).is_none());

		// sell order storage should be restored correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 10);
		assert_eq!(sell_order_storage.asset_id, asset_id);
	});
}

#[test]
fn force_set_min_validator_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Dex::force_set_min_validations(RuntimeOrigin::root(), 5));
		assert_eq!(crate::MinPaymentValidations::<Test>::get(), 5);
	});
}

#[test]
fn force_set_seller_payout_authority_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Dex::force_set_seller_payout_authority(RuntimeOrigin::root(), 5));
		assert_eq!(crate::SellerPayoutAuthority::<Test>::get(), Some(5));
	});
}

#[test]
fn set_seller_payout_preference_should_work() {
	// Set up the test environment using the new_test_ext function
	new_test_ext().execute_with(|| {
		// Define test data
		let seller = 1;
		let preference = crate::types::SellerPayoutPreference {
			chain_id: 0,
			recipient_address: vec![].try_into().unwrap(),
		};

		// Call the set_seller_payout_preference extrinsic and assert that it succeeds
		assert_ok!(Dex::set_seller_payout_preference(
			RuntimeOrigin::signed(seller),
			Some(preference.clone())
		));

		// Assert that the payout preference is set correctly in the SellerPayoutPreferences storage
		// map
		assert_eq!(crate::SellerPayoutPreferences::<Test>::get(seller).unwrap(), preference);

		// Call the set_seller_payout_preference extrinsic to clear and assert that it succeeds
		assert_ok!(Dex::set_seller_payout_preference(RuntimeOrigin::signed(seller), None));

		// Assert that the payout preference is set correctly in the SellerPayoutPreferences storage
		// map
		assert_eq!(crate::SellerPayoutPreferences::<Test>::get(seller), None);
	});
}

#[test]
fn record_payment_to_seller_should_work() {
	// Set up the test environment using the new_test_ext function
	new_test_ext().execute_with(|| {
		// Define test data
		let seller = 1;
		let authority = 5;

		let payment = crate::types::PayoutExecutedToSeller {
			order_id: vec![1, 2].try_into().unwrap(),
			chain_id: 0,
			recipient_address: vec![].try_into().unwrap(),
			tx_hash: vec![].try_into().unwrap(),
			amount: 100,
		};

		// Ensure the extrinsic fails with SellerPayoutAuthorityNotSet error
		assert_noop!(
			Dex::record_payment_to_seller(RuntimeOrigin::signed(seller), seller, payment.clone()),
			Error::<Test>::SellerPayoutAuthorityNotSet
		);

		// Set the seller payout authority using the force_set_seller_payout_authority extrinsic
		assert_ok!(Dex::force_set_seller_payout_authority(RuntimeOrigin::root(), authority));

		// Ensure the extrinsic fails with NotSellerPayoutAuthority error
		assert_noop!(
			Dex::record_payment_to_seller(RuntimeOrigin::signed(seller), seller, payment.clone()),
			Error::<Test>::NotSellerPayoutAuthority
		);

		// Ensure the extrinsic fails with NoReceivables error
		assert_noop!(
			Dex::record_payment_to_seller(
				RuntimeOrigin::signed(authority),
				seller,
				payment.clone()
			),
			Error::<Test>::NoReceivables
		);

		// Set the seller's receivables to 90
		crate::SellerReceivables::<Test>::set(seller, Some(90));

		// Ensure the extrinsic fails with ReceivableLessThanPayment error
		assert_noop!(
			Dex::record_payment_to_seller(
				RuntimeOrigin::signed(authority),
				seller,
				payment.clone()
			),
			Error::<Test>::ReceivableLessThanPayment
		);

		// Set the seller's receivables to 100
		crate::SellerReceivables::<Test>::set(seller, Some(100));

		// Call the record_payment_to_seller extrinsic and assert that it succeeds
		assert_ok!(Dex::record_payment_to_seller(
			RuntimeOrigin::signed(authority),
			seller,
			payment.clone()
		));

		// Assert that the seller's receivables is updated to 0
		assert_eq!(crate::SellerReceivables::<Test>::get(seller).unwrap(), 0);
	});
}

#[test]
fn buy_order_limits_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 100, 10));

		// should fail if the limits are not set
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11),
			Error::<Test>::UserOpenOrderUnitsLimtNotFound
		);

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			10
		));

		// use should not be able to purchase above limit
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 11, 110),
			Error::<Test>::UserOpenOrderUnitsAllowedExceeded
		);

		// use should be able to purchase below limit
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		// use should not be able exceed purchase limit with another order
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 10, 110),
			Error::<Test>::UserOpenOrderUnitsAllowedExceeded
		);

		// use should be able to create another order if its below limit
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 5, 110),);

		// use should not be able to create another order if its above number of total open orders
		// allowed
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 110),
			Error::<Test>::OpenOrderLimitExceeded
		);

		// ensure the storage is updated correctly
		let order_storage_by_user = BuyOrdersByUser::<Test>::get(buyer);
		let expected_storage = vec![(0, 1), (1, 5)];
		assert!(order_storage_by_user.unwrap().into_inner() == expected_storage);
	});
}

#[test]
fn buy_order_limits_are_reset_correctly() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;

		assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 100, 10));

		// should fail if the limits are not set
		assert_noop!(
			Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11),
			Error::<Test>::UserOpenOrderUnitsLimtNotFound
		);

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			10
		));

		// use should be able to purchase below limit
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 11));

		// use should be able to create another order if its below limit
		assert_ok!(Dex::create_buy_order(RuntimeOrigin::signed(buyer), 0, asset_id, 5, 110),);

		// ensure the storage is updated correctly
		let order_storage_by_user = BuyOrdersByUser::<Test>::get(buyer);
		let expected_storage = vec![(0, 1), (1, 5)];
		assert!(order_storage_by_user.unwrap().into_inner() == expected_storage);

		let buy_order_storage = BuyOrders::<Test>::get(0).unwrap();
		assert_eq!(buy_order_storage.buyer, buyer);
		assert_eq!(buy_order_storage.units, 1);
		assert_eq!(buy_order_storage.expiry_time, 3);

		// lets expire the first buy order
		Dex::on_idle(4, Weight::MAX);

		// the order should be cleared
		assert!(BuyOrders::<Test>::get(1).is_none());

		// ensure the storage is updated correctly
		let order_storage_by_user = BuyOrdersByUser::<Test>::get(buyer);
		let expected_storage = vec![(0, 1)];
		assert_eq!(order_storage_by_user.unwrap().into_inner(), expected_storage);
	});
}
