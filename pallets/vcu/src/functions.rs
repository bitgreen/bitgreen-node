// SBP M2 review: compilation warnings.

// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! VCU pallet helper functions
use crate::{
    BatchRetireDataList, BatchRetireDataOf, Config, Error, Event, NextItemId, Pallet,
    ProjectDetail, Projects, RetiredVCUs, RetiredVcuData,
};
use frame_support::pallet_prelude::DispatchResult;
use frame_support::traits::fungibles::Mutate;
use frame_support::traits::tokens::nonfungibles::{Create, Mutate as NFTMutate};
use frame_support::{ensure, traits::Get};
use primitives::BatchRetireData;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::traits::CheckedAdd;
use sp_runtime::traits::Zero;
use sp_std::{cmp, convert::TryInto, vec, vec::Vec};

impl<T: Config> Pallet<T> {
    /// The account ID of the vcu pallet
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account_truncating()
    }

    /// Get the project details from AssetId
    pub fn get_project_details(project_id: T::AssetId) -> Option<ProjectDetail<T>> {
        Projects::<T>::get(project_id)
    }

    /// Calculate the issuance year for a project
    /// For a project with a single batch it's the issuance year of that batch
    /// For a project with multiple batches, its the issuance year of the oldest batch
    // SBP M2 review: you can always sort a vector
    // Then take the first value if EXISTS
    // How about a situation when `batches` is empty?
    // How about sorting batches on insert?
    pub fn calculate_issuance_year(project: ProjectDetail<T>) -> u32 {
        // single batch
        if project.batches.len() == 1 {
            return project.batches.first().unwrap().issuance_year;
        } else {
            let mut batch_list = project.batches.clone();
            batch_list.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
            batch_list.first().unwrap().issuance_year
        }
    }

    /// Retire vcus for given project_id
    // SBP M2 review: I suggest returning `DispatchResult` only in extrinsics
    // IMHO non extrinsic function should return Result<_, Error>
    // Also too long function, refactor needed
    pub fn retire_vcus(
        from: T::AccountId,
        project_id: T::AssetId,
        amount: T::Balance,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();

        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            // ensure the project exists
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            // let project_id = project.project_id.as_ref().ok_or(Error::<T>::VCUNotMinted)?;

            // attempt to burn the tokens from the caller
            T::AssetHandler::burn_from(project_id, &from.clone(), amount)?;

            // reduce the supply of the vcu
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
            // SBP M2 review: try to avoid cloning vectors
            // Maybe sorting on insert can help?
            let mut batch_list: Vec<_> = project.batches.clone().into_iter().collect();
            // sort by issuance year so we retire from oldest batch
            // SBP M2 review: how about impl Ord trait
            // like: https://stackoverflow.com/questions/29884402/how-do-i-implement-ord-for-a-struct
            batch_list.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
            // list to store retirement data
            let mut batch_retire_data_list: BatchRetireDataList<T> = Default::default();
            let mut remaining = amount;
            // SBP M2 review: try to avoid iterations as much as possible
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
                    // SBP M2 review: add better error handling
                    .expect("this should not fail");

                // this is safe since actual is <= remaining
                remaining = remaining - actual;
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
            // SBP M2 review: take care of overflows
            // Check https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add
            let next_item_id: u32 = item_id.into() + 1_u32;
            // SBP M2 review: use try_mutate
            // https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageMap.html#tymethod.try_mutate
            NextItemId::<T>::insert::<T::AssetId, T::ItemId>(project_id, next_item_id.into());

            // form the retire vcu data
            let retired_vcu_data = RetiredVcuData::<T> {
                account: from.clone(),
                retire_data: batch_retire_data_list.clone(),
                timestamp: now,
                count: amount,
            };

            //Store the details of retired batches in storage
            RetiredVCUs::<T>::insert((project_id, item_id), retired_vcu_data);

            // emit event
            // SBP M2 review: I suggest emitting events only in extrinsics
            // not in helper functions
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
