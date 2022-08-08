// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! CarbonCredits pallet helper functions
use crate::{
    AuthorizedAccounts, BatchRetireDataList, BatchRetireDataOf, Config, Error, Event, NextItemId,
    Pallet, ProjectCreateParams, ProjectDetail, Projects, RetiredCredits, RetiredVcuData,
};
use codec::alloc::string::ToString;
use frame_support::{
    ensure,
    pallet_prelude::*,
    traits::{
        tokens::fungibles::{metadata::Mutate as MetadataMutate, Create, Mutate},
        tokens::nonfungibles::{Create as NFTCreate, Mutate as NFTMutate},
        Contains, Get,
    },
};
use primitives::BatchRetireData;
use sp_runtime::traits::{AccountIdConversion, CheckedAdd, One, Zero};
use sp_std::{cmp, convert::TryInto, vec::Vec};

impl<T: Config> Pallet<T> {
    /// The account ID of the CarbonCredits pallet
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account_truncating()
    }

    /// Get the project details from AssetId
    pub fn get_project_details(project_id: T::AssetId) -> Option<ProjectDetail<T>> {
        Projects::<T>::get(project_id)
    }

    /// Checks if given account is kyc approved
    pub fn check_kyc_approval(account_id: &T::AccountId) -> DispatchResult {
        if !T::KYCProvider::contains(account_id) {
            Err(Error::<T>::KYCAuthorisationFailed.into())
        } else {
            Ok(())
        }
    }

    /// Checks if the given account_id is part of authorized account list
    pub fn check_authorized_account(account_id: &T::AccountId) -> DispatchResult {
        let authorized_accounts = AuthorizedAccounts::<T>::get();
        if !authorized_accounts.contains(account_id) {
            Err(Error::<T>::NotAuthorised.into())
        } else {
            Ok(())
        }
    }

    /// Approve/reject a project
    pub fn do_approve_project(project_id: T::AssetId, is_approved: bool) -> DispatchResult {
        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            // ensure the Project exists
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            project.approved = is_approved;

            // emit event
            // TODO : Emit rejected event if rejected?
            Self::deposit_event(Event::ProjectApproved { project_id });

            Ok(())
        })
    }

    /// Calculate the issuance year for a project
    /// For a project with a single batch it's the issuance year of that batch
    /// For a project with multiple batches, its the issuance year of the oldest batch
    pub fn calculate_issuance_year(project: ProjectDetail<T>) -> Option<u32> {
        // the data is stored sorted in ascending order of issuance year, hence first() will always return oldest batch
        project.batches.first().map(|x| x.issuance_year)
    }

    /// Create a new project with `params`
    pub fn create_project(
        admin: T::AccountId,
        project_id: T::AssetId,
        mut params: ProjectCreateParams<T>,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();

        ensure!(
            project_id >= T::MinProjectId::get(),
            Error::<T>::ProjectIdLowerThanPermitted
        );

        // the unit price should not be zero
        ensure!(!params.unit_price.is_zero(), Error::<T>::UnitPriceIsZero);

        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            ensure!(project.is_none(), Error::<T>::ProjectAlreadyExists);

            // the total supply of project must match the supply of all batches
            let batch_total_supply =
                params
                    .batches
                    .iter()
                    .fold(Zero::zero(), |mut sum: T::Balance, batch| {
                        sum += batch.total_supply;
                        sum
                    });

            ensure!(
                batch_total_supply > Zero::zero(),
                Error::<T>::CannotCreateProjectWithoutCredits
            );

            // sort batch data in ascending order of issuance year
            params
                .batches
                .sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));

            let new_project = ProjectDetail {
                originator: admin,
                name: params.name,
                description: params.description,
                location: params.location,
                images: params.images,
                videos: params.videos,
                documents: params.documents,
                registry_details: params.registry_details,
                sdg_details: params.sdg_details,
                royalties: params.royalties,
                batches: params.batches,
                created: now,
                updated: None,
                approved: false,
                total_supply: batch_total_supply,
                minted: Zero::zero(),
                retired: Zero::zero(),
                unit_price: params.unit_price,
            };

            *project = Some(new_project.clone());

            // create the asset
            T::AssetHandler::create(project_id, Self::account_id(), true, 1_u32.into())?;

            // set metadata for the asset
            T::AssetHandler::set(
                project_id,
                &Self::account_id(),
                new_project.name.clone().into_inner(), // asset name
                project_id.to_string().as_bytes().to_vec(), // asset symbol
                0,
            )?;

            // emit event
            Self::deposit_event(Event::ProjectCreated {
                project_id,
                details: new_project,
            });

            Ok(())
        })
    }

    /// Resubmit a project after approval is rejected
    pub fn resubmit_project(
        admin: T::AccountId,
        project_id: T::AssetId,
        mut params: ProjectCreateParams<T>,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();

        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            // approved projects cannot be modified
            ensure!(!project.approved, Error::<T>::CannotModifyApprovedProject);
            // only originator can resubmit
            ensure!(project.originator == admin, Error::<T>::NotAuthorised);

            // the unit price should not be zero
            ensure!(!params.unit_price.is_zero(), Error::<T>::UnitPriceIsZero);

            // the total supply of project must match the supply of all batches
            let batch_total_supply =
                params
                    .batches
                    .iter()
                    .fold(Zero::zero(), |mut sum: T::Balance, batch| {
                        sum += batch.total_supply;
                        sum
                    });

            // sort batch data in ascending order of issuance year
            params
                .batches
                .sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));

            let new_project = ProjectDetail {
                originator: admin,
                name: params.name,
                description: params.description,
                location: params.location,
                images: params.images,
                videos: params.videos,
                documents: params.documents,
                registry_details: params.registry_details,
                sdg_details: params.sdg_details,
                royalties: params.royalties,
                batches: params.batches,
                created: project.created,
                updated: Some(now),
                approved: false,
                total_supply: batch_total_supply,
                minted: Zero::zero(),
                retired: Zero::zero(),
                unit_price: params.unit_price,
            };

            *project = new_project.clone();

            // emit event
            Self::deposit_event(Event::ProjectResubmitted {
                project_id,
                details: new_project,
            });

            Ok(())
        })
    }

    pub fn mint_vcus(
        _sender: T::AccountId,
        project_id: T::AssetId,
        amount_to_mint: T::Balance,
        list_to_marketplace: bool,
    ) -> DispatchResult {
        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            // ensure the project exists
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            // ensure the project is approved
            ensure!(project.approved, Error::<T>::ProjectNotApproved);

            // ensure the amount_to_mint does not exceed limit
            ensure!(
                amount_to_mint + project.minted <= project.total_supply,
                Error::<T>::AmountGreaterThanSupply
            );

            let recipient = match list_to_marketplace {
                true => T::MarketplaceEscrow::get(),
                false => project.originator.clone(),
            };

            // Mint in the individual batches too
            let mut batch_list: Vec<_> = project.batches.clone().into_iter().collect();

            let mut remaining = amount_to_mint;
            for batch in batch_list.iter_mut() {
                // lets mint from the older batches as much as possible
                let available_to_mint = batch.total_supply - batch.minted;
                let actual = cmp::min(available_to_mint, remaining);

                batch.minted = batch
                    .minted
                    .checked_add(&actual)
                    .ok_or(Error::<T>::Overflow)?;

                // this is safe since actual is <= remaining
                remaining -= actual;
                if remaining <= Zero::zero() {
                    break;
                }
            }

            // this should not happen since total_supply = batches supply but
            // lets be safe
            ensure!(
                remaining == Zero::zero(),
                Error::<T>::AmountGreaterThanSupply
            );

            project.batches = batch_list
                .try_into()
                .expect("This should not fail since we did not change the size. qed");

            // increase the minted count
            project.minted = project
                .minted
                .checked_add(&amount_to_mint)
                .ok_or(Error::<T>::Overflow)?;

            // another check to ensure accounting is correct
            ensure!(
                project.minted <= project.total_supply,
                Error::<T>::AmountGreaterThanSupply
            );

            // mint the asset to the recipient
            T::AssetHandler::mint_into(project_id, &recipient, amount_to_mint)?;

            // emit event
            Self::deposit_event(Event::VCUMinted {
                project_id,
                recipient,
                amount: amount_to_mint,
            });

            Ok(())
        })
    }

    /// Retire vcus for given project_id
    pub fn retire_carbon_credits(
        from: T::AccountId,
        project_id: T::AssetId,
        amount: T::Balance,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();

        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            // ensure the project exists
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            // attempt to burn the tokens from the caller
            T::AssetHandler::burn_from(project_id, &from, amount)?;

            // reduce the supply of the CarbonCredits
            project.retired = project
                .retired
                .checked_add(&amount)
                .ok_or(Error::<T>::AmountGreaterThanSupply)?;

            // another check to ensure accounting is correct
            ensure!(
                project.retired <= project.total_supply,
                Error::<T>::AmountGreaterThanSupply
            );

            // Retire in the individual batches too
            let mut batch_list: Vec<_> = project.batches.clone().into_iter().collect();

            // list to store retirement data
            let mut batch_retire_data_list: BatchRetireDataList<T> = Default::default();
            let mut remaining = amount;
            for batch in batch_list.iter_mut() {
                // lets retire from the older batches as much as possible
                // this is safe since we ensure minted >= retired
                let available_to_retire = batch.minted - batch.retired;
                let actual = cmp::min(available_to_retire, remaining);

                batch.retired = batch
                    .retired
                    .checked_add(&actual)
                    .ok_or(Error::<T>::Overflow)?;

                // create data of retired batch
                let batch_retire_data: BatchRetireDataOf<T> = BatchRetireData {
                    name: batch.name.clone(),
                    uuid: batch.uuid.clone(),
                    issuance_year: batch.issuance_year,
                    count: actual,
                };

                // add to retired list
                batch_retire_data_list
                    .try_push(batch_retire_data)
                    .map_err(|_| Error::<T>::Overflow)?;

                // this is safe since actual is <= remaining
                remaining -= actual;
                if remaining <= Zero::zero() {
                    break;
                }
            }

            // this should not happen since total_retired = batches supply but
            // lets be safe
            ensure!(
                remaining == Zero::zero(),
                Error::<T>::AmountGreaterThanSupply
            );

            // sanity checks to ensure accounting is correct
            ensure!(
                project.minted <= project.total_supply,
                Error::<T>::AmountGreaterThanSupply
            );
            ensure!(
                project.retired <= project.minted,
                Error::<T>::AmountGreaterThanSupply
            );

            project.batches = batch_list
                .try_into()
                .expect("This should not fail since the size is unchanged. qed");

            // Get the item-id of the NFT to mint
            let maybe_item_id = NextItemId::<T>::get(&project_id);

            // handle the case of first retirement of proejct
            let item_id = match maybe_item_id {
                None => {
                    // If the item-id does not exist it implies this is the first retirement of project tokens
                    // create a collection and use default item-id
                    T::NFTHandler::create_collection(
                        &project_id,
                        &Self::account_id(),
                        &Self::account_id(),
                    )?;
                    Default::default()
                }
                Some(x) => x,
            };

            // mint the NFT to caller
            T::NFTHandler::mint_into(&project_id, &item_id, &from)?;
            // Increment the NextItemId storage
            let next_item_id: T::ItemId = item_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::Overflow)?;
            NextItemId::<T>::insert::<T::AssetId, T::ItemId>(project_id, next_item_id);

            // form the retire CarbonCredits data
            let retired_vcu_data = RetiredVcuData::<T> {
                account: from.clone(),
                retire_data: batch_retire_data_list.clone(),
                timestamp: now,
                count: amount,
            };

            //Store the details of retired batches in storage
            RetiredCredits::<T>::insert(project_id, item_id, retired_vcu_data);

            // emit event
            Self::deposit_event(Event::VCURetired {
                project_id,
                account: from,
                amount,
                retire_data: batch_retire_data_list,
            });

            Ok(())
        })
    }
}
