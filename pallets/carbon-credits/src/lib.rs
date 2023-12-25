// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! CarbonCredits Pallet
//! The CarbonCredits pallet creates and retires CarbonCredits units that represent the Carbon
//! Credits. These onchain CarbonCredits units can represent a single type of CarbonCredits or can
//! build to represent a combination of different types of Carbon Credits.
//!
//! The CarbonCredits units are created by an account that controls CarbonCredit units, represented
//! in the pallet as the originator. The creation process will store the CarbonCredits details on
//! the pallet storage and then mint the given amount of CarbonCredits units using the Asset Handler
//! like pallet-assets. These newly minted CarbonCredits units will be transferred to the recipient,
//! this can be any address. These units can then be sold/transferred to a buyer of carbon credits,
//! these transactions can take place multiple times but the final goal of purchasing a
//! CarbonCredits unit is to retire them. The current holder of the CarbonCredits units can call the
//! `retire` extrinsic to burn these tokens (erase from storage), this process will store a
//! reference of the tokens burned.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create`: Creates a new project onchain with details of batches of credits
//! * `resubmit`: Resubmit data for a project that has not been approved
//! * `mint`: Mint a specified amount of token credits
//! * `retire`: Burn a specified amount of token credits
//!
//! ### Permissioned Functions
//!
//! * `force_add_authorized_account`: Adds a new_authorized_account to the list
//! * `force_remove_authorized_account`: Removes an authorized_account from the list
//! * `force_set_next_asset_id`: Set the NextAssetId in storage
//! * `approve_project`: Set the project status to approved so minting can be executed
//! * `force_set_project_storage` : Set the project storage
//! * `force_set_next_item_id` : Set the NextItemId storage
//! * `force_set_retired_carbon_credit` : Set the RetiredCarbonCredits storage
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
pub use types::*;

mod functions;
pub mod migration;
pub use functions::*;

mod weights;
use frame_support::{pallet_prelude::DispatchResult, traits::Contains};
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use codec::HasCompact;
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::{
			fungibles::{metadata::Mutate as MetadataMutate, Create, Destroy, Mutate},
			nonfungibles::{Create as NFTCreate, Mutate as NFTMutate},
		},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, One};
	use sp_std::{convert::TryInto, vec::Vec};

	use super::*;

	/// The parameters the CarbonCredits pallet depends on
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The units in which we record balances.
		type Balance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// Identifier for the class of asset.
		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ sp_std::fmt::Display
			+ sp_std::cmp::PartialOrd
			+ sp_std::cmp::Ord
			+ CheckedAdd;

		/// Identifier for a project
		type ProjectId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ sp_std::fmt::Display
			+ sp_std::cmp::PartialOrd
			+ sp_std::cmp::Ord
			+ CheckedAdd;

		/// Identifier for a group
		type GroupId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ sp_std::fmt::Display
			+ sp_std::cmp::PartialOrd
			+ sp_std::cmp::Ord
			+ CheckedAdd;

		/// Identifier for the individual instances of NFT
		type ItemId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ CheckedAdd
			+ One
			+ From<u32>
			+ Into<u32>
			+ CheckedAdd;

		/// The CarbonCredits pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		// Asset manager config
		type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
			+ Mutate<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ MetadataMutate<Self::AccountId>;

		// NFT handler config
		type NFTHandler: NFTCreate<Self::AccountId, CollectionId = Self::AssetId, ItemId = Self::ItemId>
			+ NFTMutate<Self::AccountId>;

		/// KYC provider config
		type KYCProvider: Contains<Self::AccountId>;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Marketplace Escrow provider
		type MarketplaceEscrow: Get<Self::AccountId>;
		/// Maximum amount of authorised accounts permitted
		type MaxAuthorizedAccountCount: Get<u32>;
		/// Maximum amount of royalty recipient accounts permitted
		type MaxRoyaltyRecipients: Get<u32>;
		/// Maximum length of short string types
		type MaxShortStringLength: Get<u32>;
		/// Maximum length of long string types
		type MaxLongStringLength: Get<u32>;
		/// Maximum length of ipfs reference data
		type MaxIpfsReferenceLength: Get<u32>;
		/// Maximum amount of location cordinates to store
		type MaxCoordinatesLength: Get<u32>;
		/// Maximum count of documents for one type
		type MaxDocumentCount: Get<u32>;
		/// Maximum amount of carbon credits in a bundle
		type MaxGroupSize: Get<u32> + TypeInfo + Clone + Parameter;
		/// Minimum value of AssetId for CarbonCredits
		type MinProjectId: Get<Self::AssetId>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]

	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_item_id)]
	// NextItemId for NFT tokens to be created by retiring `AssetId` CarbonCredits tokens
	pub type NextItemId<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::ItemId>;

	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	// NextAssetId for CC tokens to be created for every project
	pub type NextAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_project_id)]
	// NextAssetId for CC tokens to be created for every project
	pub type NextProjectId<T: Config> = StorageValue<_, T::ProjectId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn authorized_accounts)]
	// List of AuthorizedAccounts for the pallet
	pub type AuthorizedAccounts<T: Config> =
		StorageValue<_, AuthorizedAccountsListOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	/// The details of a CarbonCredits
	pub(super) type Projects<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ProjectId, ProjectDetail<T>>;

	#[pallet::storage]
	#[pallet::getter(fn asset_id_lookup)]
	/// AssetId details for project/group
	pub(super) type AssetIdLookup<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, (T::ProjectId, T::GroupId)>;

	#[pallet::storage]
	#[pallet::getter(fn retired_carbon_credits)]
	/// The retired CarbonCredits record
	pub type RetiredCredits<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Blake2_128Concat,
		T::ItemId,
		RetiredCarbonCreditsData<T>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new AuthorizedAccount has been added
		AuthorizedAccountAdded { account_id: T::AccountId },
		/// An AuthorizedAccount has been removed
		AuthorizedAccountRemoved { account_id: T::AccountId },
		/// A new CarbonCredits has been created
		ProjectCreated {
			/// The ProjectId of the created project
			project_id: T::ProjectId,
		},
		/// A project details has been resubmitted
		ProjectResubmitted {
			/// The ProjectId of the created project
			project_id: T::ProjectId,
		},
		/// Project has been approved
		ProjectApproved {
			/// The ProjectId of the approved project
			project_id: T::ProjectId,
			/// The AssetIds created by the project
			asset_ids: Vec<T::AssetId>,
		},
		/// Project has been rejected
		ProjectRejected {
			/// The ProjectId of the approved project
			project_id: T::ProjectId,
		},
		// An amount of Carbon Credits was minted
		CarbonCreditMinted {
			/// The ProjectId of the minted CarbonCredits
			project_id: T::ProjectId,
			/// The GroupId of the minted CarbonCredits
			group_id: T::GroupId,
			/// The AccountId that received the minted CarbonCredits
			recipient: T::AccountId,
			/// The amount of CarbonCredits units minted
			amount: T::Balance,
		},
		// An existing CarbonCredits was retired
		CarbonCreditRetired {
			/// The ProjectId of the retired CarbonCredits
			project_id: T::ProjectId,
			/// The GroupId of the CarbonCredits retired
			group_id: T::GroupId,
			/// The AssetId of the CarbonCredits retired
			asset_id: T::AssetId,
			/// The AccountId that retired the CarbonCredits
			account: T::AccountId,
			/// The amount of CarbonCredits units retired
			amount: T::Balance,
			/// Details of the retired token
			retire_data: BatchRetireDataList<T>,
			/// reason for retirement
			reason: ShortStringOf<T>,
		},
		/// A project details has been updated
		ProjectUpdated {
			/// The ProjectId of the updated project
			project_id: T::ProjectId,
		},
		/// A new batch group was added to the project
		BatchGroupAdded {
			/// The ProjectId of the updated project
			project_id: T::ProjectId,
			/// GroupId of the new batch group
			group_id: T::GroupId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Adding a new authorized account failed
		TooManyAuthorizedAccounts,
		/// Cannot add a duplicate authorised account
		AuthorizedAccountAlreadyExists,
		/// Cannot create duplicate Projects
		ProjectAlreadyExists,
		/// Account failed KYC checks
		KYCAuthorisationFailed,
		/// The account is not authorised
		NotAuthorised,
		/// The project cannot be created without credits
		CannotCreateProjectWithoutCredits,
		/// The given Project was not found in storage
		ProjectNotFound,
		/// The Amount of CarbonCredits units is greater than supply
		AmountGreaterThanSupply,
		/// Calculcation triggered an Overflow
		Overflow,
		/// The token accounting generated an error
		SupplyAmountMismatch,
		/// The unit price for CarbonCredits cannot be zero
		UnitPriceIsZero,
		/// The project is not approved
		ProjectNotApproved,
		/// Cannot generate asset id
		CannotGenerateAssetId,
		/// ProjectId is lower than permitted
		ProjectIdLowerThanPermitted,
		/// Cannot resubmit an approved project
		CannotModifyApprovedProject,
		/// group max exceeded
		TooManyGroups,
		/// the group does not exist
		GroupNotFound,
		/// Can only update an approved project, use resubmit for rejected projects
		CannotUpdateUnapprovedProject,
		/// The project approval status has been processed
		ApprovalAlreadyProcessed,
		/// Retirement reason out of bounds
		RetirementReasonOutOfBounds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new project onchain
		/// This new project can mint tokens after approval from an authorised account
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(origin: OriginFor<T>, params: ProjectCreateParams<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_kyc_approval(&sender)?;
			let project_id = Self::create_project(sender, params)?;
			// emit event
			Self::deposit_event(Event::ProjectCreated { project_id });
			Ok(())
		}

		/// Resubmit a approval rejected project data onchain
		/// An approved project data cannot be resubmitted
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::create())]
		pub fn resubmit(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			params: ProjectCreateParams<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_kyc_approval(&sender)?;
			Self::resubmit_project(sender, project_id, params)
		}

		/// Set the project status to approve/reject
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::approve_project())]
		pub fn approve_project(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			is_approved: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;
			Self::do_approve_project(project_id, is_approved)
		}

		/// Mint tokens for an approved project
		/// The tokens are always minted in the ascending order of credits, for example, if the
		/// `amount_to_mint` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we
		/// mint 100 from 2019 and 50 from 2020.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn mint(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			group_id: T::GroupId,
			amount_to_mint: T::Balance,
			list_to_marketplace: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_authorized_account(&sender)?;
			// Self::check_kyc_approval(&sender)?;
			Self::mint_carbon_credits(
				sender,
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace,
			)
		}

		/// Retire existing credits from owner
		/// The tokens are always retired in the ascending order of credits, for example, if the
		/// `amount` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we retire
		/// 100 from 2019 and 50 from 2020.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::retire())]
		pub fn retire(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			group_id: T::GroupId,
			amount: T::Balance,
			reason: Option<Vec<u8>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_kyc_approval(&sender)?;
			Self::retire_carbon_credits(sender, project_id, group_id, amount, reason)
		}

		/// Add a new account to the list of authorised Accounts
		/// The caller must be from a permitted origin
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::force_add_authorized_account())]
		pub fn force_add_authorized_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			// add the account_id to the list of authorized accounts
			AuthorizedAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
				ensure!(
					!account_list.contains(&account_id),
					Error::<T>::AuthorizedAccountAlreadyExists
				);

				account_list
					.try_push(account_id.clone())
					.map_err(|_| Error::<T>::TooManyAuthorizedAccounts)?;
				Ok(())
			})?;

			Self::deposit_event(Event::AuthorizedAccountAdded { account_id });
			Ok(())
		}

		/// Remove an account from the list of authorised accounts
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::force_remove_authorized_account())]
		pub fn force_remove_authorized_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			// remove the account_id from the list of authorized accounts if already exists
			AuthorizedAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
				if let Some(index) = account_list.iter().position(|a| a == &account_id) {
					account_list.swap_remove(index);
					Self::deposit_event(Event::AuthorizedAccountRemoved { account_id });
				}

				Ok(())
			})
		}

		/// Force modify a project storage
		/// Can only be called by ForceOrigin
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::force_set_project_storage())]
		pub fn force_set_project_storage(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			detail: ProjectDetail<T>,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Projects::<T>::insert(project_id, detail);
			Ok(())
		}

		/// Force modify NextItemId storage
		/// Can only be called by ForceOrigin
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::force_set_next_item_id())]
		pub fn force_set_next_item_id(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			item_id: T::ItemId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			NextItemId::<T>::insert(asset_id, item_id);
			Ok(())
		}

		/// Force modify NextAssetId storage
		/// Can only be called by ForceOrigin
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::force_set_next_item_id())]
		pub fn force_set_next_asset_id(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			NextAssetId::<T>::set(asset_id);
			Ok(())
		}

		/// Force modify retired CarbonCredits storage
		/// Can only be called by ForceOrigin
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::force_set_retired_carbon_credit())]
		pub fn force_set_retired_carbon_credit(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			item_id: T::ItemId,
			credits_data: RetiredCarbonCreditsData<T>,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			RetiredCredits::<T>::insert(asset_id, item_id, credits_data);
			Ok(())
		}

		/// Single function to approve project and mint credits
		/// Can only be called by ForceOrigin
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn force_approve_and_mint_credits(
			origin: OriginFor<T>,
			sender: T::AccountId,
			project_id: T::ProjectId,
			amount_to_mint: T::Balance,
			list_to_marketplace: bool,
			group_id: T::GroupId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::check_kyc_approval(&sender)?;
			Self::do_approve_project(project_id, true)?;
			Self::mint_carbon_credits(
				sender,
				project_id,
				group_id,
				amount_to_mint,
				list_to_marketplace,
			)?;
			Ok(())
		}

		/// Force remove an project asset from storage, can be used by ForceOrigin to remove
		/// unapproved projects Can only be called by ForceOrigin
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::force_set_project_storage())]
		pub fn force_remove_project(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			let project = Projects::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;
			// remove all assets connected to this project
			for (_group_id, group) in project.batch_groups.iter() {
				// the asset is newly created and not distributed, so we can call finish destory
				// without removing accounts
				T::AssetHandler::start_destroy(group.asset_id, None)?;
				T::AssetHandler::finish_destroy(group.asset_id)?;
			}
			// remove project from storage
			Projects::<T>::take(project_id);
			Ok(())
		}

		/// Modify the details of an approved project
		/// Can only be called by the ProjectOwner
		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::create())]
		pub fn update_project_details(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			params: ProjectCreateParams<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_kyc_approval(&sender)?;
			Self::update_project(sender, project_id, params)
		}

		/// Add a new batch group to the project
		/// Can only be called by the ProjectOwner
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::create())]
		pub fn add_batch_group(
			origin: OriginFor<T>,
			project_id: T::ProjectId,
			batch_group: BatchGroupOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_kyc_approval(&sender)?;
			Self::do_add_batch_group(sender, project_id, batch_group)
		}
	}
}

/// Struct to verify if a given asset_id is representing a carbon credit project
impl<T: Config> primitives::CarbonCreditsValidator for Pallet<T> {
	type ProjectId = T::ProjectId;
	type Address = T::AccountId;
	type GroupId = T::GroupId;
	type AssetId = T::AssetId;
	type Amount = T::Balance;

	fn get_project_details(asset_id: &Self::AssetId) -> Option<(Self::ProjectId, Self::GroupId)> {
		AssetIdLookup::<T>::get(asset_id)
	}

	fn retire_credits(
		sender: Self::Address,
		project_id: Self::ProjectId,
		group_id: Self::GroupId,
		amount: Self::Amount,
		reason: Option<sp_std::vec::Vec<u8>>,
	) -> DispatchResult {
		Self::retire_carbon_credits(sender, project_id, group_id, amount, reason)
	}
}
