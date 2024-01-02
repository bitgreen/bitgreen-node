// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! CarbonCredits pallet helper functions
use codec::alloc::string::ToString;
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{
		tokens::{
			fungibles::{metadata::Mutate as MetadataMutate, Create, Mutate},
			nonfungibles::{Create as NFTCreate, Mutate as NFTMutate},
			Fortitude::Polite,
			Precision::Exact,
		},
		Contains, Get,
	},
	BoundedBTreeMap,
};
use primitives::BatchRetireData;
use sp_runtime::traits::{AccountIdConversion, CheckedAdd, CheckedSub, One, Zero};
use sp_std::{cmp, convert::TryInto, vec::Vec};

use crate::{
	AssetIdLookup, AuthorizedAccounts, BatchGroupOf, BatchRetireDataList, BatchRetireDataOf,
	Config, Error, Event, NextAssetId, NextItemId, NextProjectId, Pallet, ProjectApprovalStatus,
	ProjectCreateParams, ProjectDetail, Projects, RetiredCarbonCreditsData, RetiredCredits,
	ShortStringOf,
};

impl<T: Config> Pallet<T> {
	/// The account ID of the CarbonCredits pallet
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	/// Get the project details from AssetId
	pub fn get_project_details(project_id: T::ProjectId) -> Option<ProjectDetail<T>> {
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
	pub fn do_approve_project(project_id: T::ProjectId, is_approved: bool) -> DispatchResult {
		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			// ensure the project exists
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			ensure!(
				project.approved == ProjectApprovalStatus::Pending,
				Error::<T>::ApprovalAlreadyProcessed
			);

			// if approved, create assets
			if is_approved {
				project.approved = ProjectApprovalStatus::Approved;

				let mut created_asset_ids: Vec<T::AssetId> = Default::default();

				for (group_id, group) in project.batch_groups.iter_mut() {
					let asset_id = Self::next_asset_id();
					let next_asset_id =
						asset_id.checked_add(&1u32.into()).ok_or(Error::<T>::Overflow)?;
					NextAssetId::<T>::put(next_asset_id);

					// create the asset
					T::AssetHandler::create(asset_id, Self::account_id(), true, 1_u32.into())?;

					// set metadata for the asset
					T::AssetHandler::set(
						asset_id,
						&Self::account_id(),
						project_id.to_string().as_bytes().to_vec(), // asset name
						project_id.to_string().as_bytes().to_vec(), // asset symbol
						0,
					)?;

					// set the asset id
					group.asset_id = asset_id;

					AssetIdLookup::<T>::insert(group.asset_id, (project_id, group_id));

					// add the assetId for event updation
					created_asset_ids.push(asset_id);
				}

				Self::deposit_event(Event::ProjectApproved {
					project_id,
					asset_ids: created_asset_ids,
				});
			} else {
				project.approved = ProjectApprovalStatus::Rejected;
				Self::deposit_event(Event::ProjectRejected { project_id });
			}

			Ok(())
		})
	}

	/// Calculate the issuance year for a group
	/// For a project with a single batch it's the issuance year of that batch
	/// For a project with multiple batches, its the issuance year of the oldest batch
	pub fn calculate_issuance_year(project: ProjectDetail<T>, group_id: T::GroupId) -> Option<u16> {
		// the data is stored sorted in ascending order of issuance year, hence first() will always
		// return oldest batch
		if let Some(group) = project.batch_groups.get(&group_id) {
			group.batches.first().map(|x| x.issuance_year)
		} else {
			None
		}
	}

	/// Create a new project with `params`
	pub fn create_project(
		admin: T::AccountId,
		params: ProjectCreateParams<T>,
	) -> Result<T::ProjectId, DispatchError> {
		let now = frame_system::Pallet::<T>::block_number();

		let project_id = Self::next_project_id();
		let next_project_id = project_id.checked_add(&1u32.into()).ok_or(Error::<T>::Overflow)?;
		NextProjectId::<T>::put(next_project_id);

		Projects::<T>::try_mutate(project_id, |project| -> Result<T::ProjectId, DispatchError> {
			ensure!(project.is_none(), Error::<T>::ProjectAlreadyExists);

			// cannot create a new project with empty batch_groups
			ensure!(!params.batch_groups.is_empty(), Error::<T>::CannotCreateProjectWithoutCredits);

			let mut batch_group_map: BoundedBTreeMap<_, _, _> = Default::default();
			let mut group_id: T::GroupId = 0u32.into();

			// ensure the groups are formed correctly and convert to BTreeMap
			for mut group in params.batch_groups.into_iter() {
				let mut group_total_supply: T::Balance = Zero::zero();

				for batch in group.batches.iter() {
					ensure!(
						batch.total_supply > Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					ensure!(
						batch.minted == Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					ensure!(
						batch.retired == Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					group_total_supply = group_total_supply
						.checked_add(&batch.total_supply)
						.ok_or(Error::<T>::Overflow)?;
				}

				ensure!(
					group_total_supply > Zero::zero(),
					Error::<T>::CannotCreateProjectWithoutCredits
				);

				// sort batch data in ascending order of issuance year
				group.batches.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
				group.total_supply = group_total_supply;

				// insert the group to BTreeMap
				batch_group_map
					.try_insert(group_id, group.clone())
					.map_err(|_| Error::<T>::TooManyGroups)?;

				group_id = group_id.checked_add(&1u32.into()).ok_or(Error::<T>::Overflow)?;
			}

			let new_project = ProjectDetail {
				originator: admin,
				name: params.name,
				description: params.description,
				location: params.location,
				images: params.images,
				videos: params.videos,
				documents: params.documents,
				registry_details: params.registry_details,
				batch_groups: batch_group_map,
				sdg_details: params.sdg_details,
				royalties: params.royalties,
				created: now,
				updated: None,
				approved: ProjectApprovalStatus::Pending,
				project_type: params.project_type,
			};

			*project = Some(new_project);

			Ok(project_id)
		})
	}

	/// Resubmit a project after approval is rejected
	pub fn resubmit_project(
		admin: T::AccountId,
		project_id: T::ProjectId,
		params: ProjectCreateParams<T>,
	) -> DispatchResult {
		let now = frame_system::Pallet::<T>::block_number();

		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			// only originator can resubmit
			ensure!(project.originator == admin, Error::<T>::NotAuthorised);

			// approved projects cannot be modified
			ensure!(!project.approved.is_approved(), Error::<T>::CannotModifyApprovedProject);

			let mut batch_group_map: BoundedBTreeMap<_, _, _> = Default::default();
			let mut group_id: T::GroupId = 0u32.into();

			// ensure the groups are formed correctly and convert to BTreeMap
			for mut group in params.batch_groups.into_iter() {
				let mut group_total_supply: T::Balance = Zero::zero();
				for batch in group.batches.iter() {
					ensure!(
						batch.total_supply > Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					ensure!(
						batch.minted == Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					ensure!(
						batch.retired == Zero::zero(),
						Error::<T>::CannotCreateProjectWithoutCredits
					);

					group_total_supply = group_total_supply
						.checked_add(&batch.total_supply)
						.ok_or(Error::<T>::Overflow)?;
				}

				ensure!(
					group_total_supply > Zero::zero(),
					Error::<T>::CannotCreateProjectWithoutCredits
				);

				// sort batch data in ascending order of issuance year
				group.batches.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
				group.total_supply = group_total_supply;

				// insert the group to BTreeMap
				batch_group_map
					.try_insert(group_id, group.clone())
					.map_err(|_| Error::<T>::TooManyGroups)?;

				group_id = group_id.checked_add(&1u32.into()).ok_or(Error::<T>::Overflow)?;
			}

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
				batch_groups: batch_group_map,
				created: project.created,
				updated: Some(now),
				approved: ProjectApprovalStatus::Pending,
				project_type: params.project_type,
			};

			*project = new_project;

			// emit event
			Self::deposit_event(Event::ProjectResubmitted { project_id });

			Ok(())
		})
	}

	/// Update a project that has already been approved, this function only allows the owner to
	/// update certain fields of the project description, once approved the project cannot modify
	/// the batch groups data.
	pub fn update_project(
		admin: T::AccountId,
		project_id: T::ProjectId,
		params: ProjectCreateParams<T>,
	) -> DispatchResult {
		let now = frame_system::Pallet::<T>::block_number();

		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			// non approved project needs to be resubmitted
			ensure!(project.approved.is_approved(), Error::<T>::CannotUpdateUnapprovedProject);

			// only originator can resubmit
			ensure!(project.originator == admin, Error::<T>::NotAuthorised);

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
				// we don't allow editing of the project batch data
				batch_groups: project.batch_groups.clone(),
				created: project.created,
				updated: Some(now),
				approved: project.approved,
				project_type: params.project_type,
			};

			*project = new_project;

			// emit event
			Self::deposit_event(Event::ProjectUpdated { project_id });

			Ok(())
		})
	}

	/// Add a new batch group to the project, this can only be done by the originator
	pub fn do_add_batch_group(
		admin: T::AccountId,
		project_id: T::ProjectId,
		mut batch_group: BatchGroupOf<T>,
	) -> DispatchResult {
		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			// non approved project needs to be resubmitted
			ensure!(project.approved.is_approved(), Error::<T>::CannotUpdateUnapprovedProject);

			// only originator can resubmit
			ensure!(project.originator == admin, Error::<T>::NotAuthorised);

			let mut batch_group_map = project.batch_groups.clone();

			let group_id: u32 = batch_group_map.len() as u32;

			let mut group_total_supply: T::Balance = Zero::zero();

			for batch in batch_group.batches.iter() {
				ensure!(
					batch.total_supply > Zero::zero(),
					Error::<T>::CannotCreateProjectWithoutCredits
				);

				ensure!(
					batch.minted == Zero::zero(),
					Error::<T>::CannotCreateProjectWithoutCredits
				);

				ensure!(
					batch.retired == Zero::zero(),
					Error::<T>::CannotCreateProjectWithoutCredits
				);

				group_total_supply = group_total_supply
					.checked_add(&batch.total_supply)
					.ok_or(Error::<T>::Overflow)?;
			}

			ensure!(
				group_total_supply > Zero::zero(),
				Error::<T>::CannotCreateProjectWithoutCredits
			);

			// sort batch data in ascending order of issuance year
			batch_group.batches.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
			batch_group.total_supply = group_total_supply;

			// insert the group to BTreeMap
			batch_group_map
				.try_insert(group_id.into(), batch_group.clone())
				.map_err(|_| Error::<T>::TooManyGroups)?;

			project.batch_groups = batch_group_map;

			// emit event
			Self::deposit_event(Event::BatchGroupAdded { project_id, group_id: group_id.into() });

			Ok(())
		})
	}

	pub fn mint_carbon_credits(
		_sender: T::AccountId,
		project_id: T::ProjectId,
		group_id: T::GroupId,
		amount_to_mint: T::Balance,
		list_to_marketplace: bool,
	) -> DispatchResult {
		if amount_to_mint.is_zero() {
			return Ok(())
		}

		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			// ensure the project exists
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			// ensure the project is approved
			ensure!(project.approved.is_approved(), Error::<T>::ProjectNotApproved);

			// ensure the group exists
			let group = project.batch_groups.get_mut(&group_id).ok_or(Error::<T>::GroupNotFound)?;

			// ensure the amount_to_mint does not exceed limit
			let projected_total_supply =
				amount_to_mint.checked_add(&group.minted).ok_or(Error::<T>::Overflow)?;

			ensure!(
				projected_total_supply <= group.total_supply,
				Error::<T>::AmountGreaterThanSupply
			);

			let recipient = match list_to_marketplace {
				// TODO : Support marketplace escrow
				true => project.originator.clone(),
				false => project.originator.clone(),
			};

			// Mint in the individual batches too
			let mut batch_list: Vec<_> = group.batches.clone().into_iter().collect();

			let mut remaining = amount_to_mint;
			for batch in batch_list.iter_mut() {
				// lets mint from the older batches as much as possible
				let available_to_mint =
					batch.total_supply.checked_sub(&batch.minted).ok_or(Error::<T>::Overflow)?;

				let actual = cmp::min(available_to_mint, remaining);

				batch.minted = batch.minted.checked_add(&actual).ok_or(Error::<T>::Overflow)?;

				// this is safe since actual is <= remaining
				remaining = remaining.checked_sub(&actual).ok_or(Error::<T>::Overflow)?;
				if remaining <= Zero::zero() {
					break
				}
			}

			// this should not happen since total_supply = batches supply but
			// lets be safe
			ensure!(remaining == Zero::zero(), Error::<T>::AmountGreaterThanSupply);

			group.batches = batch_list.try_into().map_err(|_| Error::<T>::Overflow)?;

			// increase the minted count
			group.minted = group.minted.checked_add(&amount_to_mint).ok_or(Error::<T>::Overflow)?;

			// another check to ensure accounting is correct
			ensure!(group.minted <= group.total_supply, Error::<T>::AmountGreaterThanSupply);

			// mint the asset to the recipient
			T::AssetHandler::mint_into(group.asset_id, &recipient, amount_to_mint)?;

			// emit event
			Self::deposit_event(Event::CarbonCreditMinted {
				project_id,
				group_id,
				recipient,
				amount: amount_to_mint,
			});

			Ok(())
		})
	}

	/// Retire carbon credits for given project_id
	pub fn retire_carbon_credits(
		from: T::AccountId,
		project_id: T::ProjectId,
		group_id: T::GroupId,
		amount: T::Balance,
		reason: Option<Vec<u8>>,
	) -> DispatchResult {
		let now = frame_system::Pallet::<T>::block_number();

		if amount.is_zero() {
			return Ok(())
		}

		Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
			// ensure the project exists
			let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

			// ensure the project is approved
			ensure!(project.approved.is_approved(), Error::<T>::ProjectNotApproved);

			// ensure the group exists
			let group = project.batch_groups.get_mut(&group_id).ok_or(Error::<T>::GroupNotFound)?;

			// attempt to burn the tokens from the caller
			T::AssetHandler::burn_from(group.asset_id, &from, amount, Exact, Polite)?;

			// reduce the supply of the CarbonCredits
			group.retired =
				group.retired.checked_add(&amount).ok_or(Error::<T>::AmountGreaterThanSupply)?;

			// another check to ensure accounting is correct
			ensure!(group.retired <= group.minted, Error::<T>::AmountGreaterThanSupply);

			// Retire in the individual batches too
			let mut batch_list: Vec<_> = group.batches.clone().into_iter().collect();

			// list to store retirement data
			let mut batch_retire_data_list: BatchRetireDataList<T> = Default::default();
			let mut remaining = amount;
			for batch in batch_list.iter_mut() {
				// lets retire from the older batches as much as possible
				// this is safe since we ensure minted >= retired
				let available_to_retire =
					batch.minted.checked_sub(&batch.retired).ok_or(Error::<T>::Overflow)?;

				let actual = cmp::min(available_to_retire, remaining);

				if actual.is_zero() {
					continue
				}

				batch.retired = batch.retired.checked_add(&actual).ok_or(Error::<T>::Overflow)?;

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
				remaining = remaining.checked_sub(&actual).ok_or(Error::<T>::Overflow)?;
				if remaining <= Zero::zero() {
					break
				}
			}

			// this should not happen since total_retired = batches supply but
			// lets be safe
			ensure!(remaining == Zero::zero(), Error::<T>::AmountGreaterThanSupply);

			// sanity checks to ensure accounting is correct
			ensure!(group.minted <= group.total_supply, Error::<T>::AmountGreaterThanSupply);
			ensure!(group.retired <= group.minted, Error::<T>::AmountGreaterThanSupply);

			group.batches = batch_list.try_into().map_err(|_| Error::<T>::Overflow)?;

			// Get the item-id of the NFT to mint
			let maybe_item_id = NextItemId::<T>::get(group.asset_id);

			// handle the case of first retirement of proejct
			let item_id = match maybe_item_id {
				None => {
					// If the item-id does not exist it implies this is the first retirement of
					// project tokens create a collection and use default item-id
					T::NFTHandler::create_collection(
						&group.asset_id,
						&Self::account_id(),
						&Self::account_id(),
					)?;
					Default::default()
				},
				Some(x) => x,
			};

			// mint the NFT to caller
			T::NFTHandler::mint_into(&group.asset_id, &item_id, &from)?;
			// Increment the NextItemId storage
			let next_item_id: T::ItemId =
				item_id.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			NextItemId::<T>::insert::<T::AssetId, T::ItemId>(group.asset_id, next_item_id);

			let ret_reason: ShortStringOf<T> = if reason.is_none() {
				Default::default()
			} else {
				reason
					.expect("Checked above!")
					.try_into()
					.map_err(|_| Error::<T>::RetirementReasonOutOfBounds)?
			};

			// form the retire CarbonCredits data
			let retired_carbon_credit_data = RetiredCarbonCreditsData::<T> {
				account: from.clone(),
				retire_data: batch_retire_data_list.clone(),
				timestamp: now,
				count: amount,
				reason: ret_reason.clone(),
			};

			//Store the details of retired batches in storage
			RetiredCredits::<T>::insert(group.asset_id, item_id, retired_carbon_credit_data);

			// emit event
			Self::deposit_event(Event::CarbonCreditRetired {
				project_id,
				group_id,
				asset_id: group.asset_id,
				account: from,
				amount,
				retire_data: batch_retire_data_list,
				reason: ret_reason,
			});

			Ok(())
		})
	}
}
