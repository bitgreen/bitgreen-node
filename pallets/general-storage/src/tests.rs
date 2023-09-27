use crate::{mock::*, Error, KeyOf, StoredData};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn store_data_should_work() {
	new_test_ext().execute_with(|| {
		let key = "test";
		let value = "somedatatostore";

		// should fail if given empty key
		assert_noop!(
			GeneralStorage::store_data(
				RuntimeOrigin::signed(1),
				Default::default(),
				value.as_bytes().to_vec().try_into().unwrap()
			),
			Error::<Test>::EmptyInput
		);

		// should fail if given empty value
		assert_noop!(
			GeneralStorage::store_data(
				RuntimeOrigin::signed(1),
				key.as_bytes().to_vec().try_into().unwrap(),
				Default::default(),
			),
			Error::<Test>::EmptyInput
		);

		// should fail if cannot pay deposit
		assert_noop!(
			GeneralStorage::store_data(
				RuntimeOrigin::signed(1),
				key.as_bytes().to_vec().try_into().unwrap(),
				value.as_bytes().to_vec().try_into().unwrap(),
			),
			pallet_balances::Error::<Test>::InsufficientBalance
		);

		// give some balance to pay deposit
		Balances::make_free_balance_be(&1, 1000);
		assert_ok!(GeneralStorage::store_data(
			RuntimeOrigin::signed(1),
			key.as_bytes().to_vec().try_into().unwrap(),
			value.as_bytes().to_vec().try_into().unwrap(),
		));

		// ensure the storage is updated
		let key: KeyOf<Test> = key.as_bytes().to_vec().try_into().unwrap();
		assert_eq!(
			StoredData::<Test>::get(1, key),
			Some(value.as_bytes().to_vec().try_into().unwrap())
		);
	});
}

#[test]
fn clear_data_should_work() {
	new_test_ext().execute_with(|| {
		let key = "test";
		let value = "somedatatostore";

		// should fail if given empty key
		assert_noop!(
			GeneralStorage::clear_data(RuntimeOrigin::signed(1), Default::default(),),
			Error::<Test>::EmptyInput
		);

		// should fail if no data stored
		assert_noop!(
			GeneralStorage::clear_data(
				RuntimeOrigin::signed(1),
				key.as_bytes().to_vec().try_into().unwrap(),
			),
			Error::<Test>::NoDataStored
		);

		// give some balance to pay deposit
		Balances::make_free_balance_be(&1, 1000);
		assert_ok!(GeneralStorage::store_data(
			RuntimeOrigin::signed(1),
			key.as_bytes().to_vec().try_into().unwrap(),
			value.as_bytes().to_vec().try_into().unwrap(),
		));

		// ensure the storage is updated
		let key: KeyOf<Test> = key.as_bytes().to_vec().try_into().unwrap();
		assert_eq!(
			StoredData::<Test>::get(1, key.clone()),
			Some(value.as_bytes().to_vec().try_into().unwrap())
		);

		// deletion should work now
		assert_ok!(GeneralStorage::clear_data(RuntimeOrigin::signed(1), key.clone(),));

		assert_eq!(StoredData::<Test>::get(1, key.clone()), None);

		// trying again should fail since already deleted
		assert_noop!(
			GeneralStorage::clear_data(RuntimeOrigin::signed(1), key,),
			Error::<Test>::NoDataStored
		);
	});
}
