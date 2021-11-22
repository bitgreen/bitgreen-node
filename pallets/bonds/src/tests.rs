use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_support::error::BadOrigin;

#[test]
fn create_change_settings_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"{'manager':'5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY','supervisor':'5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY','operators':['5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY']}".to_vec()));
	});

	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::create_change_settings(Origin::root(), b"infodocuments".to_vec(), r#"{"documents":[{"document":"Profit&LossPreviousyear"}]}"#.as_bytes().to_vec()));
	});
}

#[test]
fn create_change_settings_does_not_work_for_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::signed(1), b"kyc".to_vec(), b"[{'document':'Profit&Loss Previous year'}]".to_vec()),
			BadOrigin
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_key_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"[]".to_vec()),
			Error::<Test>::SettingsJsonTooShort
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), r#"[{"document":"Profit&Loss Previous year"]"#.as_bytes().to_vec()),
			Error::<Test>::InvalidJson
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc1".to_vec(), b"{'manager':'5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY','supervisor':'5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY','operators':['5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY']}".to_vec()),
			Error::<Test>::SettingsKeyIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::KycManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","supervisor":"234"}"#.as_bytes().to_vec()),
			Error::<Test>::KycSupervisorAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","supervisor":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","operators":[]}"#.as_bytes().to_vec()),
			Error::<Test>::KycOperatorsNotConfigured
		);


	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_bondapproval_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"bondapproval".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::BondApprovalManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"bondapproval".to_vec(), r#"{"committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::BondApprovalCommitteeIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"bondapproval".to_vec(), r#"{"committee":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],"mandatoryunderwriting":""}"#.as_bytes().to_vec()),
			Error::<Test>::BondApprovalMandatoryUnderwritingIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"bondapproval".to_vec(), r#"{"committee":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],"mandatoryunderwriting":"Y","mandatorycreditrating":""}"#.as_bytes().to_vec()),
			Error::<Test>::BondApprovalMandatoryCreditRatingIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"bondapproval".to_vec(), r#"{"committee":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],"mandatoryunderwriting":"Y","mandatorycreditrating":"Y","mandatorylegalopinion":""}"#.as_bytes().to_vec()),
			Error::<Test>::BondApprovalMandatoryLegalOpinionIsWrong
		);


	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_underwriterssubmission_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"underwriterssubmission".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::UnderWritersSubmissionManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"underwriterssubmission".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::UnderwritersSubmissionCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_insurerssubmission_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"insurerssubmission".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::InsurerSubmissionManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"insurerssubmission".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::InsurerSubmissionCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_creditratingagencies_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"creditratingagencies".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::CreditRatingAgenciesSubmissionManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"creditratingagencies".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::CreditRatingAgenciesSubmissionCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_lawyerssubmission_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"lawyerssubmission".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::LawyersSubmissionManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"lawyerssubmission".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::LawyersSubmissionCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_collateralsverification_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"collateralsverification".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::CollateralVerificationManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"collateralsverification".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::CollateralVerificationCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_fundapproval_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"fundapproval".to_vec(), r#"{"manager":"Profit&Loss Previous year"}"#.as_bytes().to_vec()),
			Error::<Test>::FundApprovalManagerAccountIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"fundapproval".to_vec(), r#"{"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":[]}"#.as_bytes().to_vec()),
			Error::<Test>::FundApprovalCommitteeIsWrong
		);
	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_infodocuments_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"infodocuments".to_vec(), r#"{"documents":[]}"#.as_bytes().to_vec()),
			Error::<Test>::InfoDocumentsIsWrong
		);

	});
}

#[test]
fn create_change_settings_does_not_work_for_invalid_insuranceminreserve_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"insuranceminreserve".to_vec(), r#"{"currency":12}"#.as_bytes().to_vec()),
			Error::<Test>::InsuranceCurrencyIsWrong
		);

		assert_noop!(
			Bonds::create_change_settings(Origin::root(), b"insuranceminreserve".to_vec(), r#"{"currency":123,"reserve":0}"#.as_bytes().to_vec()),
			Error::<Test>::InsuranceMinReserveCannotBeZero
		);

	});
}

#[test]
fn create_change_kyc_does_not_work_for_invalid_input() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::create_change_settings(Origin::root(), b"kyc".to_vec(), b"{'manager':'512345675123264591234567571234567891234567891234','supervisor':'5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY','operators':['5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY']}".to_vec()));

		assert_noop!(
			Bonds::create_change_kyc(Origin::signed(11), 11, r#"{"name":"Smith and Wesson Inc","address":"103, Paris Boulevard","city":"London","zip":"00100","state":"England","country":"Great Britain","phone":"+441232322332","website":"https://www.smith.co.uk","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}]}"#.as_bytes().to_vec()),
			Error::<Test>::SignerIsNotAuthorizedForKycApproval
		);
	});

}

#[test]
fn create_change_kyc_does_not_work_for_invalid_settings() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_kyc(Origin::signed(11), 11, r#"{"name":"Smith and Wesson Inc","address":"103, Paris Boulevard","city":"London","zip":"00100","state":"England","country":"Great Britain","phone":"+441232322332","website":"https://www.smith.co.uk","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}]}"#.as_bytes().to_vec()),
			Error::<Test>::KycSettingsNotConfigured
		);
	});
}

#[test]
fn kyc_approve_does_not_work_if_kyc_id_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::kyc_approve(Origin::signed(11), 11),
			Error::<Test>::KycIdNotFound
		);
	});
}

#[test]
fn create_change_fund_does_not_work_if_setting_does_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_fund(Origin::signed(11), 11, b"kyc".to_vec()),
			Error::<Test>::SettingsDoesNotExist
		);
	});
}

#[test]
fn fund_approve_does_not_work_if_kyc_id_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::fund_approve(Origin::signed(11), 11),
			Error::<Test>::KycIdNotFound
		);
	});
}

#[test]
fn bond_create_does_not_work_if_bond_id_zero() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::bond_create(Origin::signed(11), 0, b"kyc".to_vec()),
			Error::<Test>::BondIdIsWrongCannotBeZero
		);
	});
}

#[test]
fn bond_create_does_not_work_if_kyc_id_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::bond_create(Origin::signed(11), 1, b"kyc".to_vec()),
			Error::<Test>::MissingKycForSigner
		);
	});
}

#[test]
fn bond_approve_does_not_work_if_bond_id_zero() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::bond_approve(Origin::signed(11), 1),
			Error::<Test>::BondsIdNotFound
		);
	});
}

#[test]
fn create_change_credit_rating_agency_does_not_work_if_settings_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_change_credit_rating_agency(Origin::signed(11), 1, b"kyc".to_vec()),
			Error::<Test>::SettingsDoesNotExist
		);
	});
}

#[test]
fn create_credit_rating_does_not_work_if_signer_not_authorized() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_credit_rating(Origin::signed(11), 1, b"kyc".to_vec()),
			Error::<Test>::SignerIsNotAuthorizedAsCreditRatingAgency
		);
	});
}

#[test]
fn create_collaterals_does_not_work_if_bond_id_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::create_collaterals(Origin::signed(11), 1, 1, b"kyc".to_vec()),
			Error::<Test>::BondsIdNotFound
		);
	});
}

#[test]
fn confirm_collaterals_does_not_work_if_setting_does_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::confirm_collaterals(Origin::signed(11), 1, 1, b"collateralsverification".to_vec()),
			Error::<Test>::SettingsDoesNotExist
		);
	});
}

#[test]
fn iso_country_create_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::iso_country_create(Origin::root(), b"IN".to_vec(), b"India".to_vec()));

		assert_noop!(
			Bonds::iso_country_create(Origin::root(), b"IN".to_vec(), b"India".to_vec()),
			Error::<Test>::CountryCodeAlreadyPresent
		);
	});

}

#[test]
fn iso_country_create_does_not_work_if_invalid_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::iso_country_create(Origin::root(), b"IND".to_vec(), b"IN".to_vec()),
			Error::<Test>::WrongLengthCountryCode
		);

		assert_noop!(
			Bonds::iso_country_create(Origin::root(), b"IN".to_vec(), b"IN".to_vec()),
			Error::<Test>::CountryNameTooShort
		);
	});
}

#[test]
fn iso_country_destroy_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bonds::iso_country_create(Origin::root(), b"IN".to_vec(), b"India".to_vec()));

		assert_ok!(Bonds::iso_country_destroy(Origin::root(), b"IN".to_vec()));

	});
}

#[test]
fn iso_country_destroy_does_not_work_if_country_code_does_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::iso_country_destroy(Origin::root(), b"IN".to_vec()),
			Error::<Test>::CountryCodeNotFound
		);
	});
}

#[test]
fn currency_create_does_not_work_if_country_code_is_invalid() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bonds::currency_create(Origin::root(), b"IND".to_vec(), b"IN".to_vec()),
			Error::<Test>::WrongLengthCurrencyCode
		);
	});
}

#[test]
fn country_create_does_not_work_if_invalid_input() {
	new_test_ext().execute_with(|| {

		assert_noop!(
			Bonds::currency_create(Origin::root(), b"IN".to_vec(), r#"{name":"Bitcoin","category":"c","country":"AE","blockchain":"Bitcoin","address":"not applicable"}"#.as_bytes().to_vec()),
			Error::<Test>::InvalidJson
		);
	});
}