use crate::{
    BatchRetireDataList, BatchRetireDataOf, Config, Error, Event, NextItemId, Pallet, Projects,
    RetiredVCUs, RetiredVcuData,
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
        T::PalletId::get().into_account()
    }

    // /// Get the project details from AssetId
    // pub fn get_project_details(asset_id : T::AssetId) -> Option<ProjectDetail<T>> {

    // }

    /// Retire vcus for given asset_id
    pub fn retire_vcus(
        from: T::AccountId,
        project_id: T::ProjectId,
        amount: T::Balance,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();

        Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
            // ensure the project exists
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

            let asset_id = project.asset_id.as_ref().ok_or(Error::<T>::VCUNotMinted)?;

            // attempt to burn the tokens from the caller
            T::AssetHandler::burn_from(*asset_id, &from.clone(), amount)?;

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
            let mut batch_list: Vec<_> = project.batches.clone().into_iter().collect();
            // sort by issuance year so we retire from oldest batch
            batch_list.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
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
            let maybe_item_id = NextItemId::<T>::get(&asset_id);

            // handle the case of first retirement of proejct
            let item_id = match maybe_item_id {
                None => {
                    // If the item-id does not exist it implies this is the first retirement of project tokens
                    // create a collection and use default item-id
                    T::NFTHandler::create_class(
                        asset_id,
                        &Self::account_id(),
                        &Self::account_id(),
                    )?;
                    Default::default()
                }
                Some(x) => x,
            };

            // mint the NFT to caller
            T::NFTHandler::mint_into(asset_id, &item_id, &from)?;
            // Increment the NextItemId storage
            let next_item_id: u32 = item_id.into() + 1_u32;
            NextItemId::<T>::insert::<T::AssetId, T::ItemId>(*asset_id, next_item_id.into());

            // form the retire vcu data
            let retired_vcu_data = RetiredVcuData::<T> {
                account: from.clone(),
                retire_data: batch_retire_data_list.clone(),
                timestamp: now,
                count: amount,
            };

            //Store the details of retired batches in storage
            RetiredVCUs::<T>::insert((asset_id, item_id), retired_vcu_data);

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
