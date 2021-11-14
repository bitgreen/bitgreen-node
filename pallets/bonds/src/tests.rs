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