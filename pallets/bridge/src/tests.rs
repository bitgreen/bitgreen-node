use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_settings_should_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(Assets::force_create(Origin::root(), 1, 1, 1, 1));
		assert_ok!(Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()));
	});
}

#[test]
fn create_settings_should_not_work_for_invalid_json() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{"description":[ehXCPcNoHGKutQY"]}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidJson
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"kyc".to_vec(), b"[]".to_vec()),
			Error::<Test>::SettingsJsonTooShort
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"1".to_vec(), r#"{"description":"xxxxxxxxxx"}"#.as_bytes().to_vec()),
			Error::<Test>::SettingsKeyTooShort
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BITGBITGBITG".to_vec(), r#"{"description":"xxxxxxxxxx"}"#.as_bytes().to_vec()),
			Error::<Test>::SettingsKeyTooLong
		);

		assert_ok!(Assets::force_create(Origin::root(), 1, 1, 1, 1));
		assert_ok!(Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()));

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{"description":"xxxxxxxxxx"}"#.as_bytes().to_vec()),
			Error::<Test>::SettingsKeyExists
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{"chainid":4,"description":"xxxxxxxxxx"}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidChainId
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{"chainid":3,"description":""}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidDescription
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{"chainid":3,"description":"xxxxxxxxxx","address":""}"#.as_bytes().to_vec()),
			Error::<Test>::EmptyAddress
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":2,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::AssetDoesNotExist
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":"",
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalThresholdNotFound
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":101,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalThresholdInvalid
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":"",
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalThresholdNotFound
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":101,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalThresholdInvalid
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQ","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalKeepersAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":[],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalKeepersNotConfigured
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":3,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalKeepersNotMatchingThreshold
		);


		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalKeepersAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":[],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalKeepersNotConfigured
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":3,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalKeepersNotMatchingThreshold
		);


		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQp","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalWhatchDogsAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":[],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalWatchdogsNotConfigured
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57Y","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalWatchddogsAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":[],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalWatchdogsNotConfigured
		);



		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9r","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalWhatchCatsAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":[],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::InternalWatchcatsNotConfigured
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26F","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalWhatchCatsAccountIsWrong
		);

		assert_noop!(
			Bridge::create_settings(Origin::root(), b"BIT".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":[]
		}"#.as_bytes().to_vec()),
			Error::<Test>::ExternalWatchcatsNotConfigured
		);

	});
}

#[test]
fn destroy_settings_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::force_create(Origin::root(), 1, 1, 1, 1));
		assert_ok!(Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()));
		assert_ok!(Bridge::destroy_settings(Origin::root(), b"BITG".to_vec()));
	});
}

#[test]
fn destroy_settings_should_not_work_for_non_existing_key() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bridge::destroy_settings(Origin::root(), b"BITG".to_vec()),
			Error::<Test>::SettingsKeyNotFound
		);
	});
}

#[test]
fn mint_should_not_work_if_siger_is_not_keeper() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::force_create(Origin::root(), 1, 1, 1, 1));
		assert_ok!(Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()));
		assert_noop!(
			Bridge::mint(Origin::signed(1), b"BITG".to_vec(), 2, b"a123".to_vec(), 1),
			Error::<Test>::SignerIsNotKeeper
		);
	});
}

#[test]
fn burn_should_not_work_if_siger_is_not_keeper() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::force_create(Origin::root(), 1, 1, 1, 1));
		assert_ok!(Bridge::create_settings(Origin::root(), b"BITG".to_vec(), r#"{
		"chainid":1,
		"description":"xxxxxxxxxx",
		"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid":1,
		"internalthreshold":2,
		"externathreshold":2,
		"internalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalkeepers":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchdogs":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"internalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
		"externalwatchcats":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
		}"#.as_bytes().to_vec()));
		assert_noop!(
			Bridge::burn(Origin::signed(1), b"BITG".to_vec(), 2, b"a123".to_vec(), 1),
			Error::<Test>::SignerIsNotKeeper
		);
	});
}

#[test]
fn set_lockdown_should_not_work_for_non_existing_key() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bridge::set_lockdown(Origin::signed(1), b"BITG".to_vec()),
			Error::<Test>::SettingsKeyNotFound
		);
	});
}

#[test]
fn set_unlockdown_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bridge::set_unlockdown(Origin::root()));
		assert_eq!(Bridge::lockdown(), false);
	});
}
