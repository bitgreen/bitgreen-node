// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Tests for CarbonCredits pallet
use crate::{
	mock::*, BatchGroupListOf, BatchGroupOf, BatchOf, Config, Error, NextItemId,
	ProjectCreateParams, Projects, RegistryListOf, RetiredCredits, SDGTypesListOf,
};
use frame_support::{
	assert_noop, assert_ok,
	traits::tokens::fungibles::{metadata::Inspect as MetadataInspect, Inspect},
	BoundedVec, PalletId,
};
use frame_system::RawOrigin;
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType};
use sp_runtime::{traits::AccountIdConversion, Percent, TokenError::FundsUnavailable};
use sp_std::convert::TryInto;

pub type CarbonCreditsEvent = crate::Event<Test>;

/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryListOf<T> {
	let registry_details = RegistryDetails {
		reg_name: RegistryName::Verra,
		name: "reg_name".as_bytes().to_vec().try_into().unwrap(),
		id: "reg_id".as_bytes().to_vec().try_into().unwrap(),
		summary: "reg_summary".as_bytes().to_vec().try_into().unwrap(),
	};
	vec![registry_details].try_into().unwrap()
}

/// helper function to generate standard sdg details
fn get_default_sdg_details<T: Config>() -> SDGTypesListOf<T> {
	let sdg_details: SDGTypesListOf<T> = vec![SDGDetails {
		sdg_type: SdgType::LifeOnLand,
		description: "sdg_desp".as_bytes().to_vec().try_into().unwrap(),
		references: "sdg_ref".as_bytes().to_vec().try_into().unwrap(),
	}]
	.try_into()
	.unwrap();

	sdg_details
}

fn get_single_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![Batch {
		name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
		issuance_year: 2020_u16,
		start_date: 2020_u16,
		end_date: 2020_u16,
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
	}]
	.try_into()
	.unwrap()
}

fn get_multiple_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![
		Batch {
			name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2020_u16,
			start_date: 2020_u16,
			end_date: 2020_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
		Batch {
			name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2021_u16,
			start_date: 2021_u16,
			end_date: 2021_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
	]
	.try_into()
	.unwrap()
}

/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_single_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate multiple batch details
fn get_multiple_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_multiple_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to create and approve tokens
fn create_and_approve_project(originator_account: u64, authorised_account: u64) {
	// create the project to approve
	let creation_params = get_default_creation_params::<Test>();
	assert_ok!(CarbonCredits::create(
		RawOrigin::Signed(originator_account).into(),
		creation_params
	));

	// approve project so minting can happen
	assert_ok!(CarbonCredits::force_add_authorized_account(
		RawOrigin::Root.into(),
		authorised_account
	));

	assert_ok!(CarbonCredits::approve_project(
		RawOrigin::Signed(authorised_account).into(),
		0u32,
		true
	),);
}

/// helper function to add authorised account
fn add_authorised_account(authorised_account: u64) {
	// authorise the account
	assert_ok!(CarbonCredits::force_add_authorized_account(
		RawOrigin::Root.into(),
		authorised_account
	));
}

/// helper function to create and approve tokens in batch config
fn create_and_approve_project_batch(originator_account: u64, authorised_account: u64) {
	let project_id = 0u32;
	// create the project to approve
	let mut creation_params = get_default_creation_params::<Test>();
	// replace the default with mutiple batches
	let created_batch_group = get_multiple_batch_group::<Test>();
	creation_params.batch_groups = created_batch_group;

	assert_ok!(CarbonCredits::create(
		RawOrigin::Signed(originator_account).into(),
		creation_params
	));

	// approve project so minting can happen
	assert_ok!(CarbonCredits::force_add_authorized_account(
		RawOrigin::Root.into(),
		authorised_account
	));
	assert_ok!(CarbonCredits::approve_project(
		RawOrigin::Signed(authorised_account).into(),
		project_id,
		true
	),);
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	let royalty = Royalty::<T::AccountId> {
		account_id: 1_u32.into(),
		percent_of_fees: Percent::from_percent(0),
	};

	let creation_params = ProjectCreateParams {
		name: "name".as_bytes().to_vec().try_into().unwrap(),
		description: "description".as_bytes().to_vec().try_into().unwrap(),
		location: "(1, 1), (2, 2), (3, 3), (4, 4)".as_bytes().to_vec().try_into().unwrap(),
		images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
			.try_into()
			.unwrap(),
		registry_details: get_default_registry_details::<T>(),
		sdg_details: get_default_sdg_details::<T>(),
		royalties: Some(vec![royalty].try_into().unwrap()),
		batch_groups: get_default_batch_group::<T>(),
		project_type: None,
	};

	creation_params
}

#[test]
fn add_new_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		let authorised_account_one = 1;
		let authorised_account_two = 2;
		let authorised_account_three = 3;
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::AuthorizedAccountAdded { account_id: authorised_account_one }
				.into()
		);

		assert_eq!(CarbonCredits::authorized_accounts().first(), Some(&authorised_account_one));

		assert_noop!(
			CarbonCredits::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account_one,
			),
			Error::<Test>::AuthorizedAccountAlreadyExists
		);

		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_two,
		));

		assert_noop!(
			CarbonCredits::force_add_authorized_account(
				RawOrigin::Root.into(),
				authorised_account_three,
			),
			Error::<Test>::TooManyAuthorizedAccounts
		);

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::AuthorizedAccountAdded { account_id: authorised_account_two }
				.into()
		);
	});
}

#[test]
fn force_remove_authorized_accounts_should_work() {
	new_test_ext().execute_with(|| {
		let authorised_account_one = 1;
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));
		assert_eq!(CarbonCredits::authorized_accounts().first(), Some(&authorised_account_one));

		assert_ok!(CarbonCredits::force_remove_authorized_account(
			RawOrigin::Root.into(),
			authorised_account_one,
		));

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::AuthorizedAccountRemoved { account_id: authorised_account_one }
				.into()
		);

		assert_eq!(CarbonCredits::authorized_accounts().len(), 0);
	});
}

#[test]
fn create_works_for_single_batch() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;

		let creation_params = get_default_creation_params::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();

		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert!(!stored_data.approved.is_approved());

		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
		assert_eq!(group_data.batches, get_single_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 100_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		assert_eq!(last_event(), CarbonCreditsEvent::ProjectCreated { project_id }.into());
	});
}

#[test]
fn create_works_for_multiple_batch() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the default with mutiple batches
		creation_params.batch_groups = get_multiple_batch_group::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();

		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert!(!stored_data.approved.is_approved());

		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		assert_eq!(last_event(), CarbonCreditsEvent::ProjectCreated { project_id }.into());
	});
}

#[test]
fn create_fails_for_multiple_batch_with_single_batch_supply_zero() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the default with mutiple batches
		let batches: BoundedVec<BatchOf<Test>, <Test as Config>::MaxGroupSize> = vec![
			Batch {
				name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
				uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
				issuance_year: 2020_u16,
				start_date: 2020_u16,
				end_date: 2020_u16,
				total_supply: 0_u32.into(), // this should be rejected
				minted: 0_u32.into(),
				retired: 0_u32.into(),
			},
			Batch {
				name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
				uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
				issuance_year: 2021_u16,
				start_date: 2021_u16,
				end_date: 2021_u16,
				total_supply: 100_u32.into(),
				minted: 0_u32.into(),
				retired: 0_u32.into(),
			},
		]
		.try_into()
		.unwrap();

		let batch_groups = vec![BatchGroupOf::<Test> {
			name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
			asset_id: 0_u32,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
			batches,
		}]
		.try_into()
		.unwrap();

		creation_params.batch_groups = batch_groups;

		assert_noop!(
			CarbonCredits::create(RawOrigin::Signed(originator_account).into(), creation_params),
			Error::<Test>::CannotCreateProjectWithoutCredits
		);
	});
}

#[test]
fn create_fails_for_empty_batch_group() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the batch value with empty
		creation_params.batch_groups = Default::default();

		assert_noop!(
			CarbonCredits::create(RawOrigin::Signed(originator_account).into(), creation_params),
			Error::<Test>::CannotCreateProjectWithoutCredits
		);
	});
}

#[test]
fn create_fails_for_empty_batches() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;

		let mut creation_params = get_default_creation_params::<Test>();
		let batch_groups = vec![BatchGroupOf::<Test> {
			name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
			asset_id: 0_u32,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
			batches: Default::default(), // empty batches
		}]
		.try_into()
		.unwrap();

		creation_params.batch_groups = batch_groups;

		assert_noop!(
			CarbonCredits::create(RawOrigin::Signed(originator_account).into(), creation_params),
			Error::<Test>::CannotCreateProjectWithoutCredits
		);
	});
}

#[test]
fn resubmit_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the default with mutiple batches
		creation_params.batch_groups = get_multiple_batch_group::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params.clone()
		));

		// only originator can resubmit
		assert_noop!(
			CarbonCredits::resubmit(
				RawOrigin::Signed(10).into(),
				project_id,
				creation_params.clone()
			),
			Error::<Test>::NotAuthorised
		);

		creation_params.name = "Newname".as_bytes().to_vec().try_into().unwrap();
		assert_ok!(CarbonCredits::resubmit(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			creation_params.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert!(!stored_data.approved.is_approved());

		// the supply of both batches should be added correctly
		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		assert_eq!(last_event(), CarbonCreditsEvent::ProjectResubmitted { project_id }.into());

		// authorise the account
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));
		assert_ok!(CarbonCredits::approve_project(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			true
		),);

		// approved project cannot be resubmitted
		assert_noop!(
			CarbonCredits::resubmit(
				RawOrigin::Signed(originator_account).into(),
				project_id,
				creation_params
			),
			Error::<Test>::CannotModifyApprovedProject
		);
	});
}

#[test]
fn approve_project_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;
		let asset_id = 0;

		// non authorised account should trigger an error
		assert_noop!(
			CarbonCredits::approve_project(
				RawOrigin::Signed(authorised_account).into(),
				project_id,
				true
			),
			Error::<Test>::NotAuthorised
		);

		// authorise the account
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		// non existent project should throw error
		assert_noop!(
			CarbonCredits::approve_project(
				RawOrigin::Signed(authorised_account).into(),
				1234,
				true
			),
			Error::<Test>::ProjectNotFound
		);

		// create the project to approve
		let creation_params = get_default_creation_params::<Test>();
		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();

		// sanity check
		assert!(!stored_data.approved.is_approved());

		// approve should work now
		assert_ok!(CarbonCredits::approve_project(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			true
		),);

		// ensure storage changed correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert!(stored_data.approved.is_approved());
		// the asset_id should be set correctly
		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(group_data.asset_id, asset_id);
		assert_eq!(Assets::total_issuance(asset_id), 0);

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::ProjectApproved { project_id, asset_ids: vec![0u32] }.into()
		);
	});
}

#[test]
fn cleanup_after_project_reject_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;

		// authorise the account
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));

		// create the project to approve
		let creation_params = get_default_creation_params::<Test>();
		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params
		));

		// approve the project to create asset
		assert_ok!(CarbonCredits::approve_project(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			true
		),);

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::ProjectApproved { project_id, asset_ids: vec![0u32] }.into()
		);

		// remove the project from storage
		assert_ok!(CarbonCredits::force_remove_project(RawOrigin::Root.into(), project_id,),);

		// ensure storage is cleaned
		assert_eq!(CarbonCredits::get_project_details(project_id), None);
		assert_eq!(Assets::total_issuance(0), 0);
	});
}

#[test]
fn mint_non_authorised_account_should_fail() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			CarbonCredits::mint(RawOrigin::Signed(1).into(), 1001, 100, 100, false),
			Error::<Test>::NotAuthorised
		);
	});
}

#[test]
fn mint_non_existent_project_should_fail() {
	new_test_ext().execute_with(|| {
		add_authorised_account(1);

		// minting a non existent project should fail
		assert_noop!(
			CarbonCredits::mint(RawOrigin::Signed(1).into(), 1001, 100, 100, false),
			Error::<Test>::ProjectNotFound
		);
	});
}

#[test]
fn mint_non_approved_project_should_fail() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let amount_to_mint = 50;
		let list_to_marketplace = false;

		// create the project to approve
		let creation_params = get_default_creation_params::<Test>();
		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params
		));

		add_authorised_account(10);
		assert_noop!(
			CarbonCredits::mint(
				RawOrigin::Signed(10).into(),
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace
			),
			Error::<Test>::ProjectNotApproved
		);
	});
}

// #[test]
// fn test_only_project_originator_can_mint_tokens() {
//     new_test_ext().execute_with(|| {
//         let originator_account = 1;
//         let project_id = 1001;
//         // token minting params
//         let amount_to_mint = 50;
//         let authorised_account = 10;
//         let list_to_marketplace = false;

//         // create the project to approve
//         create_and_approve_project(originator_account, authorised_account);

//         // only originator can mint tokens
//         assert_noop!(
//             CarbonCredits::mint(
//                 RawOrigin::Signed(authorised_account).into(),
//                 project_id,
//                 amount_to_mint,
//                 list_to_marketplace
//             ),
//             Error::<Test>::NotAuthorised
//         );
//     });
// }

#[test]
fn test_cannot_mint_more_than_supply() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let authorised_account = 10;
		let list_to_marketplace = false;

		// create the project to approve
		create_and_approve_project(originator_account, authorised_account);

		// cannot mint more than supply
		assert_noop!(
			CarbonCredits::mint(
				RawOrigin::Signed(authorised_account).into(),
				project_id,
				group_id,
				10_000,
				list_to_marketplace
			),
			Error::<Test>::AmountGreaterThanSupply
		);
	});
}

#[test]
fn mint_without_list_to_marketplace_works_for_single_batch() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let amount_to_mint: u128 = 50;
		let list_to_marketplace = false;
		let expected_asset_id = project_id;

		create_and_approve_project(originator_account, authorised_account);

		// mint should work with all params correct
		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			group_id,
			amount_to_mint,
			list_to_marketplace
		));

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::CarbonCreditMinted {
				project_id,
				group_id,
				recipient: originator_account,
				amount: amount_to_mint
			}
			.into()
		);

		// ensure minting worked correctly
		let stored_data = CarbonCredits::get_project_details(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);

		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 100_u32.into());
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, 0_u32.into());
		assert!(stored_data.approved.is_approved());

		// the batch should also be updated with minted count
		let batch_detail = group_data.batches.first().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, amount_to_mint);
		assert_eq!(batch_detail.retired, 0);

		// the originator should have the minted tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(Assets::balance(expected_asset_id, originator_account), amount_to_mint);

		// the minted token metadata should be set correctly
		assert_eq!(Assets::name(expected_asset_id), "0".as_bytes().to_vec());
		assert_eq!(Assets::symbol(expected_asset_id), "0".as_bytes().to_vec());
		assert_eq!(Assets::decimals(expected_asset_id), 0_u8);

		// the originator can freely transfer the tokens
		assert_ok!(Assets::transfer(
			RawOrigin::Signed(originator_account).into(),
			expected_asset_id,
			2,
			amount_to_mint - 1
		));
		assert_eq!(Assets::balance(expected_asset_id, originator_account), 1);
		assert_eq!(Assets::balance(expected_asset_id, 2), amount_to_mint - 1);

		// the originator cannot burn the tokens or mint more tokens
		assert_noop!(
			Assets::mint(
				RawOrigin::Signed(originator_account).into(),
				expected_asset_id,
				2,
				amount_to_mint
			),
			pallet_assets::Error::<Test>::NoPermission
		);

		assert_noop!(
			Assets::burn(
				RawOrigin::Signed(originator_account).into(),
				expected_asset_id,
				2,
				amount_to_mint
			),
			pallet_assets::Error::<Test>::NoPermission
		);
	});
}

#[test]
fn mint_without_list_to_marketplace_works_for_multiple_batches() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;
		let group_id = 0;
		// the amount will consume full of first batch and half of second batch
		let amount_to_mint: u128 = 150;
		let list_to_marketplace = false;
		let expected_asset_id = project_id;

		create_and_approve_project_batch(originator_account, authorised_account);

		// mint should work with all params correct
		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			group_id,
			amount_to_mint,
			list_to_marketplace
		));

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::CarbonCreditMinted {
				project_id,
				group_id,
				recipient: originator_account,
				amount: amount_to_mint
			}
			.into()
		);

		// ensure minting worked correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);

		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, 0_u32.into());
		assert!(stored_data.approved.is_approved());

		// the batch should also be updated with minted count
		// we have a total supply of 200, with 100 in each batch
		// we minted 150 tokens so 100 should be minted from the oldest batch
		// and the rest 50 should be minted from the next batch
		let mut stored_batches: Vec<Batch<_, _>> = group_data.batches.clone().into_iter().collect();
		// this should have been sorted so arranged in the ascending order of issuance date
		let newest_batch = stored_batches.pop().unwrap();
		assert_eq!(newest_batch.issuance_year, 2021);
		assert_eq!(newest_batch.total_supply, 100);
		assert_eq!(newest_batch.minted, 50);
		assert_eq!(newest_batch.retired, 0);

		let oldest_batch = stored_batches.pop().unwrap();
		assert_eq!(oldest_batch.issuance_year, 2020);
		assert_eq!(oldest_batch.total_supply, 100);
		assert_eq!(oldest_batch.minted, 100);
		assert_eq!(oldest_batch.retired, 0);

		// the originator should have the minted tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(Assets::balance(expected_asset_id, originator_account), amount_to_mint);

		// the originator can freely transfer the tokens
		assert_ok!(Assets::transfer(
			RawOrigin::Signed(originator_account).into(),
			expected_asset_id,
			2,
			amount_to_mint - 1
		));
		assert_eq!(Assets::balance(expected_asset_id, originator_account), 1);
		assert_eq!(Assets::balance(expected_asset_id, 2), amount_to_mint - 1);

		// the originator cannot burn the tokens or mint more tokens
		assert_noop!(
			Assets::mint(
				RawOrigin::Signed(originator_account).into(),
				expected_asset_id,
				2,
				amount_to_mint
			),
			pallet_assets::Error::<Test>::NoPermission
		);

		assert_noop!(
			Assets::burn(
				RawOrigin::Signed(originator_account).into(),
				expected_asset_id,
				2,
				amount_to_mint
			),
			pallet_assets::Error::<Test>::NoPermission
		);

		// mint another 150, should fail with no supply error
		assert_noop!(
			CarbonCredits::mint(
				RawOrigin::Signed(authorised_account).into(),
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace
			),
			Error::<Test>::AmountGreaterThanSupply
		);

		// mint remaining 50 to exhaust supply
		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			group_id,
			50,
			list_to_marketplace
		));

		// ensure minting worked correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 200_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());
		assert!(stored_data.approved.is_approved());

		// the batch should also be updated with minted count
		// we have a total supply of 200, with 100 in each batch
		// we minted 150 tokens in the previous run, 100 from oldest batch and 50 from newest batch
		// so the new 50 tokens should be minted from the newest batch
		let mut stored_batches: Vec<Batch<_, _>> = group_data.batches.clone().into_iter().collect();
		// this should have been sorted so arranged in the ascending order of issuance date
		let newest_batch = stored_batches.pop().unwrap();
		assert_eq!(newest_batch.issuance_year, 2021);
		assert_eq!(newest_batch.total_supply, 100);
		assert_eq!(newest_batch.minted, 100);
		assert_eq!(newest_batch.retired, 0);

		let oldest_batch = stored_batches.pop().unwrap();
		assert_eq!(oldest_batch.issuance_year, 2020);
		assert_eq!(oldest_batch.total_supply, 100);
		assert_eq!(oldest_batch.minted, 100);
		assert_eq!(oldest_batch.retired, 0);
	});
}

// // TODO : Add tests for list_marketplace true

#[test]
fn retire_non_existent_project_should_fail() {
	new_test_ext().execute_with(|| {
		// retire a non existent project should fail
		assert_noop!(
			CarbonCredits::retire(RawOrigin::Signed(10).into(), 1001, 100, 100, Default::default()),
			Error::<Test>::ProjectNotFound
		);
	});
}

#[test]
fn test_retire_non_minted_project_should_fail() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;
		let group_id = 0;

		// create the project
		create_and_approve_project_batch(originator_account, 10);

		// calling retire from a non minted project should fail
		assert_noop!(
			CarbonCredits::retire(
				RawOrigin::Signed(3).into(),
				project_id,
				group_id,
				100u128,
				Default::default()
			),
			FundsUnavailable
		);
	});
}

#[test]
fn test_retire_for_single_batch() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let amount_to_mint: u128 = 100;
		let amount_to_retire: u128 = 50;
		let list_to_marketplace = false;
		let expected_asset_id = 0;

		create_and_approve_project(originator_account, authorised_account);

		// mint should work with all params correct
		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			group_id,
			amount_to_mint,
			list_to_marketplace
		));

		// calling retire from an account that holds no token should fail
		assert_noop!(
			CarbonCredits::retire(
				RawOrigin::Signed(3).into(),
				project_id,
				group_id,
				amount_to_mint,
				Default::default()
			),
			FundsUnavailable
		);

		// cannot retire more than holdings
		assert_noop!(
			CarbonCredits::retire(
				RawOrigin::Signed(originator_account).into(),
				project_id,
				group_id,
				amount_to_mint + 1,
				Default::default()
			),
			FundsUnavailable
		);

		// should work when amount less than holding
		assert_ok!(CarbonCredits::retire(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			group_id,
			amount_to_retire,
			b"reason".to_vec().try_into().unwrap()
		));

		// Ensure the retirement happend correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 100_u32.into());
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, amount_to_retire);

		// the batch should also be updated with retired count
		let batch_detail = group_data.batches.first().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, amount_to_mint);
		assert_eq!(batch_detail.retired, amount_to_retire);

		// the originator should have lost the supply of retired tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint - amount_to_retire);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(
			Assets::balance(expected_asset_id, originator_account),
			amount_to_mint - amount_to_retire
		);

		// Ensure the NFT is minted correctly
		let carbon_credits_pallet_account_id: u64 =
			PalletId(*b"bitg/ccp").into_account_truncating();
		// the collection owner should be pallet
		assert_eq!(
			Uniques::collection_owner(expected_asset_id).unwrap(),
			carbon_credits_pallet_account_id
		);
		// the originator should have received the item
		assert_eq!(Uniques::owner(expected_asset_id, 0).unwrap(), originator_account);

		// Then NextItemId storage should be set correctly
		assert_eq!(NextItemId::<Test>::get(expected_asset_id).unwrap(), 1);

		// The retired data storage should be set correctly
		let creation_params = get_default_creation_params::<Test>();
		let stored_retired_data = RetiredCredits::<Test>::get(expected_asset_id, 0).unwrap();
		assert_eq!(stored_retired_data.account, originator_account);
		assert_eq!(stored_retired_data.reason.into_inner(), b"reason".to_vec());
		assert_eq!(stored_retired_data.retire_data.len(), 1);
		let retired_batch = stored_retired_data.retire_data.first().unwrap();
		assert_eq!(
			retired_batch.name,
			creation_params.batch_groups.first().unwrap().batches.first().unwrap().name
		);
		assert_eq!(
			retired_batch.uuid,
			creation_params.batch_groups.first().unwrap().batches.first().unwrap().uuid
		);
		assert_eq!(retired_batch.issuance_year, 2020);
		assert_eq!(retired_batch.count, amount_to_retire);
		assert_eq!(stored_retired_data.timestamp, 1);

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::CarbonCreditRetired {
				project_id,
				group_id,
				asset_id: expected_asset_id,
				account: originator_account,
				amount: amount_to_retire,
				retire_data: stored_retired_data.retire_data,
				reason: b"reason".to_vec().try_into().unwrap()
			}
			.into()
		);

		// retire the remaining tokens
		assert_ok!(CarbonCredits::retire(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			group_id,
			amount_to_mint - amount_to_retire,
			Default::default()
		));

		// Ensure the retirement happend correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 100_u32.into());
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, amount_to_mint);

		// the batch should also be updated with retired count
		let batch_detail = group_data.batches.first().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, amount_to_mint);
		assert_eq!(batch_detail.retired, amount_to_mint);

		// the originator should have lost the supply of retired tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), 0);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(Assets::balance(expected_asset_id, originator_account), 0);

		// originator cannot mint more since the supply is exhausted
		assert_noop!(
			CarbonCredits::mint(
				RawOrigin::Signed(authorised_account).into(),
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace
			),
			Error::<Test>::AmountGreaterThanSupply
		);

		// the collection owner should be pallet
		assert_eq!(
			Uniques::collection_owner(expected_asset_id).unwrap(),
			carbon_credits_pallet_account_id
		);
		// the originator should have received the item
		assert_eq!(Uniques::owner(expected_asset_id, 1).unwrap(), originator_account);

		// Then NextItemId storage should be set correctly
		assert_eq!(NextItemId::<Test>::get(expected_asset_id).unwrap(), 2);
		// The retired data storage should be set correctly
		let stored_retired_data = RetiredCredits::<Test>::get(expected_asset_id, 1).unwrap();
		assert_eq!(stored_retired_data.account, originator_account);
		assert_eq!(stored_retired_data.retire_data.len(), 1);
		let retired_batch = stored_retired_data.retire_data.first().unwrap();
		assert_eq!(
			retired_batch.name,
			creation_params.batch_groups.first().unwrap().batches.first().unwrap().name
		);
		assert_eq!(
			retired_batch.uuid,
			creation_params.batch_groups.first().unwrap().batches.first().unwrap().uuid
		);
		assert_eq!(retired_batch.issuance_year, 2020);
		assert_eq!(retired_batch.count, amount_to_retire);
		assert_eq!(stored_retired_data.timestamp, 1);
	});
}

#[test]
fn retire_for_multiple_batch() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let amount_to_mint: u128 = 200;
		let amount_to_retire: u128 = 50;
		let list_to_marketplace = false;
		let expected_asset_id = 0;

		create_and_approve_project_batch(originator_account, authorised_account);

		// mint should work with all params correct
		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			group_id,
			amount_to_mint,
			list_to_marketplace
		));

		// cannot retire more than holdings
		assert_noop!(
			CarbonCredits::retire(
				RawOrigin::Signed(originator_account).into(),
				project_id,
				group_id,
				amount_to_mint + 1,
				Default::default()
			),
			FundsUnavailable
		);

		// should work when amount less than holding
		assert_ok!(CarbonCredits::retire(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			group_id,
			amount_to_retire,
			Default::default()
		));

		// Ensure the retirement happend correctly
		let mut stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		let group_data = stored_data.batch_groups.get_mut(&group_id).unwrap();
		assert_eq!(group_data.total_supply, amount_to_mint);
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, amount_to_retire);

		// the batch should be udpated correctly, should be retired from the oldest batch
		// this should have been sorted so arranged in the ascending order of issuance date
		// the newest should not have any retired
		let batch_detail = group_data.batches.pop().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, 100);
		assert_eq!(batch_detail.retired, 0);
		assert_eq!(batch_detail.issuance_year, 2021);

		// the oldest batch should have retired the amount
		let batch_detail = group_data.batches.pop().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, 100);
		assert_eq!(batch_detail.retired, amount_to_retire);
		assert_eq!(batch_detail.issuance_year, 2020);

		// the originator should have lost the supply of retired tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint - amount_to_retire);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(
			Assets::balance(expected_asset_id, originator_account),
			amount_to_mint - amount_to_retire
		);

		// Ensure the NFT is minted correctly
		let carbon_credits_pallet_account_id: u64 =
			PalletId(*b"bitg/ccp").into_account_truncating();
		// the collection owner should be pallet
		assert_eq!(
			Uniques::collection_owner(expected_asset_id).unwrap(),
			carbon_credits_pallet_account_id
		);
		// the originator should have received the item
		assert_eq!(Uniques::owner(expected_asset_id, 0).unwrap(), originator_account);

		// Then NextItemId storage should be set correctly
		assert_eq!(NextItemId::<Test>::get(expected_asset_id).unwrap(), 1);

		// The retired data storage should be set correctly
		let mut stored_retired_data = RetiredCredits::<Test>::get(expected_asset_id, 0).unwrap();
		assert_eq!(stored_retired_data.account, originator_account);
		assert_eq!(stored_retired_data.retire_data.len(), 1);

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::CarbonCreditRetired {
				project_id,
				group_id,
				asset_id: expected_asset_id,
				account: originator_account,
				amount: amount_to_retire,
				retire_data: stored_retired_data.retire_data.clone(),
				reason: Default::default()
			}
			.into()
		);

		let retired_batch = stored_retired_data.retire_data.pop().unwrap();
		assert_eq!(retired_batch.issuance_year, 2020);
		assert_eq!(retired_batch.count, amount_to_retire);
		assert_eq!(stored_retired_data.timestamp, 1);
		assert_eq!(stored_retired_data.count, amount_to_retire);

		// retire the remaining tokens
		assert_ok!(CarbonCredits::retire(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			group_id,
			amount_to_mint - amount_to_retire,
			Default::default()
		));

		// Ensure the retirement happend correctly
		let mut stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		let group_data = stored_data.batch_groups.get_mut(&group_id).unwrap();
		assert_eq!(group_data.total_supply, amount_to_mint);
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, amount_to_mint);

		// the batch should be udpated correctly, should be retired from the oldest batch
		// this should have been sorted so arranged in the ascending order of issuance date
		let batch_detail = group_data.batches.pop().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, 100);
		assert_eq!(batch_detail.retired, 100);
		assert_eq!(batch_detail.issuance_year, 2021);

		// the oldest batch should have retired the amount
		let batch_detail = group_data.batches.pop().unwrap();
		assert_eq!(batch_detail.total_supply, 100_u32.into());
		assert_eq!(batch_detail.minted, 100);
		assert_eq!(batch_detail.retired, 100);
		assert_eq!(batch_detail.issuance_year, 2020);

		// the originator should have lost the supply of retired tokens
		assert_eq!(Assets::total_issuance(expected_asset_id), 0);
		assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
		assert_eq!(Assets::balance(expected_asset_id, originator_account), 0);

		// originator cannot mint more since the supply is exhausted
		assert_noop!(
			CarbonCredits::mint(
				RawOrigin::Signed(authorised_account).into(),
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace
			),
			Error::<Test>::AmountGreaterThanSupply
		);

		// the collection owner should be pallet
		assert_eq!(
			Uniques::collection_owner(expected_asset_id).unwrap(),
			carbon_credits_pallet_account_id
		);
		// the originator should have received the item
		assert_eq!(Uniques::owner(expected_asset_id, 1).unwrap(), originator_account);

		// Then NextItemId storage should be set correctly
		assert_eq!(NextItemId::<Test>::get(expected_asset_id).unwrap(), 2);

		// The retired data storage should be set correctly
		let mut stored_retired_data = RetiredCredits::<Test>::get(expected_asset_id, 1).unwrap();
		assert_eq!(stored_retired_data.account, originator_account);
		assert_eq!(stored_retired_data.retire_data.len(), 2);
		// We retired a total of 150 tokens in the call, 50 of 2020 batch had been retired
		// previously So in this retirement, we have 50 from 2020 and 100 from 2021
		let retired_batch = stored_retired_data.retire_data.pop().unwrap();
		assert_eq!(retired_batch.issuance_year, 2021);
		assert_eq!(retired_batch.count, 100);
		assert_eq!(stored_retired_data.timestamp, 1);
		let retired_batch = stored_retired_data.retire_data.pop().unwrap();
		assert_eq!(retired_batch.issuance_year, 2020);
		assert_eq!(retired_batch.count, 50);
		assert_eq!(stored_retired_data.timestamp, 1);
	});
}

#[test]
fn force_approve_and_mint_credits_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let project_id = 0;
		let group_id = 0;
		// token minting params
		let amount_to_mint: u128 = 50;
		let list_to_marketplace = false;

		let creation_params = get_default_creation_params::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params
		));

		// mint should work with all params correct
		assert_ok!(CarbonCredits::force_approve_and_mint_credits(
			RawOrigin::Root.into(),
			originator_account,
			project_id,
			amount_to_mint,
			list_to_marketplace,
			group_id,
		));

		assert_eq!(
			last_event(),
			CarbonCreditsEvent::CarbonCreditMinted {
				project_id,
				group_id,
				recipient: originator_account,
				amount: amount_to_mint
			}
			.into()
		);

		// ensure minting worked correctly
		let stored_data = CarbonCredits::get_project_details(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		let group_data = stored_data.batch_groups.get(&group_id).unwrap();
		assert_eq!(group_data.total_supply, 100_u32.into());
		assert_eq!(group_data.minted, amount_to_mint);
		assert_eq!(group_data.retired, 0_u32.into());
		assert!(stored_data.approved.is_approved());
	});
}

#[test]
fn update_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the default with mutiple batches
		creation_params.batch_groups = get_multiple_batch_group::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params.clone()
		));

		// unapproved project cannot be updated
		assert_noop!(
			CarbonCredits::update_project_details(
				RawOrigin::Signed(originator_account).into(),
				project_id,
				creation_params.clone()
			),
			Error::<Test>::CannotUpdateUnapprovedProject
		);

		// authorise the account
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));
		assert_ok!(CarbonCredits::approve_project(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			true
		),);

		// only originator can update
		assert_noop!(
			CarbonCredits::update_project_details(
				RawOrigin::Signed(10).into(),
				project_id,
				creation_params.clone()
			),
			Error::<Test>::NotAuthorised
		);

		creation_params.name = "Newname".as_bytes().to_vec().try_into().unwrap();
		// update the minted count
		creation_params.batch_groups = vec![BatchGroupOf::<Test> {
			name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
			asset_id: 0_u32,
			total_supply: 100_u32.into(),
			minted: 10_000_u32.into(),
			retired: 0_u32.into(),
			batches: get_multiple_batch_list::<Test>(),
		}]
		.try_into()
		.unwrap();

		assert_ok!(CarbonCredits::update_project_details(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			creation_params.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert!(stored_data.approved.is_approved());

		// the batch group should not be updated
		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 200_u32.into());
		// the minted amount should not be updated
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		assert_eq!(last_event(), CarbonCreditsEvent::ProjectUpdated { project_id }.into());
	});
}

#[test]
fn add_batch_group_works() {
	new_test_ext().execute_with(|| {
		let originator_account = 1;
		let authorised_account = 10;
		let project_id = 0;

		let mut creation_params = get_default_creation_params::<Test>();
		// replace the default with mutiple batches
		creation_params.batch_groups = get_multiple_batch_group::<Test>();

		assert_ok!(CarbonCredits::create(
			RawOrigin::Signed(originator_account).into(),
			creation_params.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert!(!stored_data.approved.is_approved());

		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		assert_eq!(last_event(), CarbonCreditsEvent::ProjectCreated { project_id }.into());

		// authorise the account
		assert_ok!(CarbonCredits::force_add_authorized_account(
			RawOrigin::Root.into(),
			authorised_account
		));
		assert_ok!(CarbonCredits::approve_project(
			RawOrigin::Signed(authorised_account).into(),
			project_id,
			true
		),);

		// add a new batch group to the project
		let new_batch = BatchGroupOf::<Test> {
			name: "new_batch_group_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "new_batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
			asset_id: 0_u32,
			total_supply: 200_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
			batches: get_multiple_batch_list::<Test>(),
		};

		assert_ok!(CarbonCredits::add_batch_group(
			RawOrigin::Signed(originator_account).into(),
			project_id,
			new_batch.clone()
		));

		// ensure the storage is populated correctly
		let stored_data = Projects::<Test>::get(project_id).unwrap();
		assert_eq!(stored_data.originator, originator_account);
		assert_eq!(stored_data.name, creation_params.name);
		assert_eq!(stored_data.registry_details, get_default_registry_details::<Test>());
		assert_eq!(stored_data.batch_groups.len(), 2);

		let group_data = stored_data.batch_groups.get(&0u32).unwrap();
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());

		let group_data = stored_data.batch_groups.get(&1u32).unwrap();
		assert_eq!(group_data.batches, get_multiple_batch_list::<Test>());
		assert_eq!(group_data.name, new_batch.name);
		assert_eq!(group_data.uuid, new_batch.uuid);
		assert_eq!(group_data.total_supply, 200_u32.into());
		assert_eq!(group_data.minted, 0_u32.into());
		assert_eq!(group_data.retired, 0_u32.into());
	});
}
