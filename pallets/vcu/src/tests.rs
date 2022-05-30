//! Tests for vcu pallet
use crate::{
    mock::*, Batch, BatchGroupOf, Config, Error, Event, NextAssetId, ProjectCreateParams, Projects,
    RegistryDetails, Royalty, SDGDetails, SDGTypesListOf, SdgType, ShortStringOf,
};
use frame_support::{assert_noop, assert_ok, traits::tokens::fungibles::Inspect};
use frame_system::RawOrigin;
use sp_runtime::Percent;
use sp_std::convert::TryInto;

pub type VCUEvent = crate::Event<Test>;

/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryDetails<ShortStringOf<T>> {
    let registry_details = RegistryDetails {
        name: "reg_name".as_bytes().to_vec().try_into().unwrap(),
        id: "reg_id".as_bytes().to_vec().try_into().unwrap(),
        summary: "reg_summary".as_bytes().to_vec().try_into().unwrap(),
    };
    registry_details
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

/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupOf<T> {
    let batches: BatchGroupOf<T> = vec![Batch {
        name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
        uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
        issuance_year: 2020_u32,
        start_date: 2020_u32,
        end_date: 2020_u32,
        total_supply: 100_u32.into(),
        minted: 0_u32.into(),
        retired: 0_u32.into(),
    }]
    .try_into()
    .unwrap();

    batches
}

/// helper function to generate multiple batch details
fn get_multiple_batch_group<T: Config>() -> BatchGroupOf<T> {
    let batches: BatchGroupOf<T> = vec![
        Batch {
            name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
            uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
            issuance_year: 2021_u32,
            start_date: 2021_u32,
            end_date: 2021_u32,
            total_supply: 100_u32.into(),
            minted: 0_u32.into(),
            retired: 0_u32.into(),
        },
        Batch {
            name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
            uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
            issuance_year: 2020_u32,
            start_date: 2020_u32,
            end_date: 2020_u32,
            total_supply: 100_u32.into(),
            minted: 0_u32.into(),
            retired: 0_u32.into(),
        },
    ]
    .try_into()
    .unwrap();

    batches
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
        location: [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ],
        images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
            .try_into()
            .unwrap(),
        registry_details: get_default_registry_details::<T>(),
        sdg_details: get_default_sdg_details::<T>(),
        batches: get_default_batch_group::<T>(),
        royalties: Some(vec![royalty].try_into().unwrap()),
        unit_price: 100_u32.into(),
    };

    creation_params
}

#[test]
fn add_new_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        let authorised_account_two = 2;
        let authorised_account_three = 3;
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        assert_eq!(
            last_event(),
            VCUEvent::AuthorizedAccountAdded {
                account_id: authorised_account_one
            }
            .into()
        );

        assert_eq!(
            VCU::authorized_accounts().first(),
            Some(&authorised_account_one)
        );

        assert_noop!(
            VCU::force_add_authorized_account(RawOrigin::Root.into(), authorised_account_one,),
            Error::<Test>::AuthorizedAccountAlreadyExists
        );

        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_two,
        ));

        assert_noop!(
            VCU::force_add_authorized_account(RawOrigin::Root.into(), authorised_account_three,),
            Error::<Test>::TooManyAuthorizedAccounts
        );

        assert_eq!(
            last_event(),
            VCUEvent::AuthorizedAccountAdded {
                account_id: authorised_account_two
            }
            .into()
        );
    });
}

#[test]
fn force_remove_authorized_accounts_should_work() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));
        assert_eq!(
            VCU::authorized_accounts().first(),
            Some(&authorised_account_one)
        );

        assert_ok!(VCU::force_remove_authorized_account(
            RawOrigin::Root.into(),
            authorised_account_one,
        ));

        assert_eq!(
            last_event(),
            VCUEvent::AuthorizedAccountRemoved {
                account_id: authorised_account_one
            }
            .into()
        );

        assert_eq!(VCU::authorized_accounts().len(), 0);
    });
}

#[test]
fn create_works_for_single_batch() {
    new_test_ext().execute_with(|| {
        let originator_account = 1;
        let project_id = 1001;

        let creation_params = get_default_creation_params::<Test>();

        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));

        // ensure the storage is populated correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();

        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.name, creation_params.name);
        assert_eq!(
            stored_data.registry_details,
            get_default_registry_details::<Test>()
        );
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.batches, get_default_batch_group::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 100_u32.into());
        assert_eq!(stored_data.minted, 0_u32.into());
        assert_eq!(stored_data.retired, 0_u32.into());
        assert_eq!(stored_data.approved, false);

        assert_eq!(
            last_event(),
            VCUEvent::ProjectCreated {
                project_id,
                details: stored_data
            }
            .into()
        );
    });
}

#[test]
fn create_works_for_multiple_batch() {
    new_test_ext().execute_with(|| {
        let originator_account = 1;
        let project_id = 1001;

        let mut creation_params = get_default_creation_params::<Test>();
        // replace the default with mutiple batches
        creation_params.batches = get_multiple_batch_group::<Test>();

        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));

        // ensure the storage is populated correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();

        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.name, creation_params.name);
        assert_eq!(
            stored_data.registry_details,
            get_default_registry_details::<Test>()
        );
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.batches, get_multiple_batch_group::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        // the supply of both batches should be added correctly
        assert_eq!(stored_data.total_supply, 200_u32.into());
        assert_eq!(stored_data.minted, 0_u32.into());
        assert_eq!(stored_data.retired, 0_u32.into());
        assert_eq!(stored_data.approved, false);

        assert_eq!(
            last_event(),
            VCUEvent::ProjectCreated {
                project_id,
                details: stored_data
            }
            .into()
        );
    });
}

#[test]
fn approve_project_works() {
    new_test_ext().execute_with(|| {
        let originator_account = 1;
        let authorised_account = 10;
        let project_id = 1001;

        // non authorised account should trigger an error
        assert_noop!(
            VCU::approve_project(
                RawOrigin::Signed(authorised_account).into(),
                project_id,
                true
            ),
            Error::<Test>::NotAuthorised
        );

        // authorise the account
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account
        ));

        // non existent project should throw error
        assert_noop!(
            VCU::approve_project(RawOrigin::Signed(authorised_account).into(), 1234, true),
            Error::<Test>::ProjectNotFound
        );

        // create the project to approve
        let creation_params = get_default_creation_params::<Test>();
        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));

        // ensure the storage is populated correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();

        // sanity check
        assert_eq!(stored_data.approved, false);

        // approve should work now
        assert_ok!(VCU::approve_project(
            RawOrigin::Signed(authorised_account).into(),
            project_id,
            true
        ),);

        // ensure storage changed correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.approved, true);

        assert_eq!(
            last_event(),
            VCUEvent::ProjectApproved { project_id }.into()
        );
    });
}

#[test]
fn mint_without_list_to_marketplace_works_for_single_batch() {
    new_test_ext().execute_with(|| {
        let originator_account = 1;
        let authorised_account = 10;
        let project_id = 1001;
        // token minting params
        let amount_to_mint = 50;
        let list_to_marketplace = false;
        let expected_asset_id = 1000;

        // minting a non existent project should fail
        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::ProjectNotFound
        );

        // create the project to approve
        let creation_params = get_default_creation_params::<Test>();
        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));

        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::ProjectNotApproved
        );

        // approve project so minting can happen
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account
        ));
        assert_ok!(VCU::approve_project(
            RawOrigin::Signed(authorised_account).into(),
            project_id,
            true
        ),);

        // only originator can mint tokens
        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(authorised_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::NotAuthorised
        );

        // cannot mint more than supply
        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                10_000,
                list_to_marketplace
            ),
            Error::<Test>::AmountGreaterThanSupply
        );

        // mint should work with all params correct
        assert_ok!(VCU::mint(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            amount_to_mint,
            list_to_marketplace
        ));

        assert_eq!(
            last_event(),
            VCUEvent::VCUMinted {
                project_id,
                recipient: originator_account,
                amount: amount_to_mint
            }
            .into()
        );

        // ensure minting worked correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 100_u32.into());
        assert_eq!(stored_data.minted, amount_to_mint);
        assert_eq!(stored_data.retired, 0_u32.into());
        assert_eq!(stored_data.approved, true);
        assert_eq!(stored_data.asset_id.unwrap(), expected_asset_id);

        // the batch should also be updated with minted count
        let batch_detail = stored_data.batches.first().unwrap();
        assert_eq!(batch_detail.total_supply, 100_u32.into());
        assert_eq!(batch_detail.minted, amount_to_mint);
        assert_eq!(batch_detail.retired, 0);

        // the next asset-id should be set correctly
        assert_eq!(NextAssetId::<Test>::get(), 1001);

        // the originator should have the minted tokens
        assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(
            Assets::balance(expected_asset_id, originator_account),
            amount_to_mint
        );

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
        let project_id = 1001;
        // the amount will consume full of first batch and half of second batch
        let amount_to_mint = 150;
        let list_to_marketplace = false;
        let expected_asset_id = 1000;

        // create the project to approve
        let mut creation_params = get_default_creation_params::<Test>();
        creation_params.batches = get_multiple_batch_group::<Test>();
        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));
        // approve project so minting can happen
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account
        ));
        assert_ok!(VCU::approve_project(
            RawOrigin::Signed(authorised_account).into(),
            project_id,
            true
        ),);

        // mint should work with all params correct
        assert_ok!(VCU::mint(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            amount_to_mint,
            list_to_marketplace
        ));

        assert_eq!(
            last_event(),
            VCUEvent::VCUMinted {
                project_id,
                recipient: originator_account,
                amount: amount_to_mint
            }
            .into()
        );

        // ensure minting worked correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 200_u32.into());
        assert_eq!(stored_data.minted, amount_to_mint);
        assert_eq!(stored_data.retired, 0_u32.into());
        assert_eq!(stored_data.approved, true);
        assert_eq!(stored_data.asset_id.unwrap(), expected_asset_id);

        // the batch should also be updated with minted count
        // we have a total supply of 200, with 100 in each batch
        // we minted 150 tokens so 100 should be minted from the oldest batch
        // and the rest 50 should be minted from the next batch
        let mut stored_batches: Vec<Batch<Test>> = stored_data.batches.into_iter().collect();
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

        // the next asset-id should be set correctly
        assert_eq!(NextAssetId::<Test>::get(), 1001);

        // the originator should have the minted tokens
        assert_eq!(Assets::total_issuance(expected_asset_id), amount_to_mint);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(
            Assets::balance(expected_asset_id, originator_account),
            amount_to_mint
        );

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
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::AmountGreaterThanSupply
        );

        // mint remaining 50 to exhaust supply
        assert_ok!(VCU::mint(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            50,
            list_to_marketplace
        ));

        // ensure minting worked correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 200_u32.into());
        assert_eq!(stored_data.minted, 200_u32.into());
        assert_eq!(stored_data.retired, 0_u32.into());
        assert_eq!(stored_data.approved, true);
        assert_eq!(stored_data.asset_id.unwrap(), expected_asset_id);

        // the batch should also be updated with minted count
        // we have a total supply of 200, with 100 in each batch
        // we minted 150 tokens in the previous run, 100 from oldest batch and 50 from newest batch
        // so the new 50 tokens should be minted from the newest batch
        let mut stored_batches: Vec<Batch<Test>> = stored_data.batches.into_iter().collect();
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

// TODO : Add tests for list_marketplace true

#[test]
fn retire_for_single_batch() {
    new_test_ext().execute_with(|| {
        let originator_account = 1;
        let authorised_account = 10;
        let project_id = 1001;
        // token minting params
        let amount_to_mint = 100;
        let amount_to_retire = 50;
        let list_to_marketplace = false;
        let expected_asset_id = 1000;

        // retire a non existent project should fail
        assert_noop!(
            VCU::retire(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
            ),
            Error::<Test>::ProjectNotFound
        );

        // create the project to approve
        let creation_params = get_default_creation_params::<Test>();
        assert_ok!(VCU::create(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            creation_params.clone()
        ));

        // calling retire from a non minted project should fail
        assert_noop!(
            VCU::retire(RawOrigin::Signed(3).into(), project_id, amount_to_mint,),
            Error::<Test>::VCUNotMinted
        );

        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::ProjectNotApproved
        );

        // approve project so minting can happen
        assert_ok!(VCU::force_add_authorized_account(
            RawOrigin::Root.into(),
            authorised_account
        ));
        assert_ok!(VCU::approve_project(
            RawOrigin::Signed(authorised_account).into(),
            project_id,
            true
        ),);

        // mint should work with all params correct
        assert_ok!(VCU::mint(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            amount_to_mint,
            list_to_marketplace
        ));

        // calling retire from an account that holds no token should fail
        assert_noop!(
            VCU::retire(RawOrigin::Signed(3).into(), project_id, amount_to_mint,),
            pallet_assets::Error::<Test>::NoAccount
        );

        // cannot retire more than holdings
        assert_noop!(
            VCU::retire(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint + 1,
            ),
            pallet_assets::Error::<Test>::BalanceLow
        );

        // should work when amount less than holding
        assert_ok!(VCU::retire(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            amount_to_retire
        ));

        assert_eq!(
            last_event(),
            VCUEvent::VCURetired {
                project_id,
                account: originator_account,
                amount: amount_to_retire
            }
            .into()
        );

        // Ensure the retirement happend correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 100_u32.into());
        assert_eq!(stored_data.minted, amount_to_mint);
        assert_eq!(stored_data.retired, amount_to_retire);
        assert_eq!(stored_data.approved, true);
        assert_eq!(stored_data.asset_id.unwrap(), expected_asset_id);

        // the batch should also be updated with retired count
        let batch_detail = stored_data.batches.first().unwrap();
        assert_eq!(batch_detail.total_supply, 100_u32.into());
        assert_eq!(batch_detail.minted, amount_to_mint);
        assert_eq!(batch_detail.retired, amount_to_retire);

        // the originator should have lost the supply of retired tokens
        assert_eq!(
            Assets::total_issuance(expected_asset_id),
            amount_to_mint - amount_to_retire
        );
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(
            Assets::balance(expected_asset_id, originator_account),
            amount_to_mint - amount_to_retire
        );

        // mint the remaining tokens
        assert_ok!(VCU::retire(
            RawOrigin::Signed(originator_account).into(),
            project_id,
            amount_to_mint - amount_to_retire
        ));

        // Ensure the retirement happend correctly
        let stored_data = Projects::<Test>::get(project_id).unwrap();
        assert_eq!(stored_data.originator, originator_account);
        assert_eq!(stored_data.sdg_details, get_default_sdg_details::<Test>());
        assert_eq!(stored_data.unit_price, 100_u32.into());
        assert_eq!(stored_data.total_supply, 100_u32.into());
        assert_eq!(stored_data.minted, amount_to_mint);
        assert_eq!(stored_data.retired, amount_to_mint);
        assert_eq!(stored_data.approved, true);
        assert_eq!(stored_data.asset_id.unwrap(), expected_asset_id);

        // the batch should also be updated with retired count
        let batch_detail = stored_data.batches.first().unwrap();
        assert_eq!(batch_detail.total_supply, 100_u32.into());
        assert_eq!(batch_detail.minted, amount_to_mint);
        assert_eq!(batch_detail.retired, amount_to_mint);

        // the originator should have lost the supply of retired tokens
        assert_eq!(Assets::total_issuance(expected_asset_id), 0);
        assert_eq!(Assets::minimum_balance(expected_asset_id), 1);
        assert_eq!(Assets::balance(expected_asset_id, originator_account), 0);

        // originator cannot mint more since the supply is exhausted
        assert_noop!(
            VCU::mint(
                RawOrigin::Signed(originator_account).into(),
                project_id,
                amount_to_mint,
                list_to_marketplace
            ),
            Error::<Test>::AmountGreaterThanSupply
        );
    });
}
