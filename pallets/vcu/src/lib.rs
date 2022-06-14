// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! VCU Pallet
//! The VCU pallet creates and retires VCU units that represent the VCUs on the Verra registry. These onchain vcu units can represent a
//! single type of VCU or can build to represent a combination of different types of VCUs.
//!
//! The VCU units are created by an account that controls VCU units on the Verra registry, represented in the pallet as the originator.
//! The creation process will store the VCU details on the pallet storage and then mint the given amount of Vcu units using the Asset Handler
//! like pallet-assets. These newly minted vcu units will be transferred to the recipient, this can be any address.
//! These units can then be sold/transferred to a buyer of carbon credits, these transactions can take place multiple times but the final goal
//! of purchasing a Vcu unit is to retire them. The current holder of the vcu units can call the `retire` extrinsic to burn these
//! tokens (erase from storage), this process will store a reference of the tokens burned.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create`: Creates a new project onchain with details of batches of credits
//! * `mint`: Mint a specified amount of token credits
//! * `retire`: Burn a specified amount of token credits
//!
//! ### Permissioned Functions
//!
//! * `force_add_authorized_account`: Adds a new_authorized_account to the list
//! * `force_remove_authorized_account`: Removes an authorized_account from the list
//! * `force_set_next_asset_id`: Set the NextAssetId in storage
//! * `approve_project`: Set the project status to approved so minting can be executed
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
pub mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

mod types;
pub use types::*;

mod functions;
pub use functions::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::alloc::string::ToString;
    use codec::HasCompact;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            tokens::fungibles::{metadata::Mutate as MetadataMutate, Create, Mutate},
            tokens::nonfungibles::{Create as NFTCreate, Mutate as NFTMutate},
        },
        transactional, PalletId,
    };
    use frame_system::{pallet_prelude::*, WeightInfo};
    use primitives::BatchRetireData;
    use sp_runtime::traits::{
        AccountIdConversion, AtLeast32Bit, AtLeast32BitUnsigned, CheckedAdd, Scale, Zero,
    };
    use sp_std::{cmp, convert::TryInto, vec, vec::Vec};

    /// The parameters the VCU pallet depends on
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
            + sp_std::cmp::Ord;

        /// Identifier for the individual instances of NFT
        type ItemId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo
            + From<u32>
            + Into<u32>;

        /// The vcu pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        // Asset manager config
        type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
            + Mutate<Self::AccountId>
            + MetadataMutate<Self::AccountId>;

        // NFT handler config
        type NFTHandler: NFTCreate<Self::AccountId, CollectionId = Self::AssetId, ItemId = Self::ItemId>
            + NFTMutate<Self::AccountId>;

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
        /// Maximum count of documents for one type
        type MaxDocumentCount: Get<u32>;
        /// Maximum amount of vcus in a bundle
        type MaxGroupSize: Get<u32>;
        /// Maximum amount of location cordinates to store
        type MaxCoordinatesLength: Get<u32>;
        /// Minimum value of AssetId for VCU
        type MinProjectId: Get<Self::AssetId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_item_id)]
    // NextItemId for NFT tokens to be created by retiring `AssetId` vcu tokens
    pub type NextItemId<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::ItemId>;

    #[pallet::storage]
    #[pallet::getter(fn authorized_accounts)]
    // List of AuthorizedAccounts for the pallet
    pub type AuthorizedAccounts<T: Config> =
        StorageValue<_, AuthorizedAccountsListOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn projects)]
    /// The details of a VCU
    pub(super) type Projects<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AssetId, ProjectDetail<T>>;

    #[pallet::storage]
    #[pallet::getter(fn retired_vcus)]
    /// The retired vcu record
    pub(super) type RetiredVCUs<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AssetId>, // classid of the NFT
            NMapKey<Blake2_128Concat, T::ItemId>,  // item id of the NFT
        ),
        RetiredVcuData<T>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new AuthorizedAccount has been added
        AuthorizedAccountAdded { account_id: T::AccountId },
        /// An AuthorizedAccount has been removed
        AuthorizedAccountRemoved { account_id: T::AccountId },
        /// A new VCU has been created
        ProjectCreated {
            /// The T::AssetId of the created project
            project_id: T::AssetId,
            /// The details of the created project
            details: ProjectDetail<T>,
        },
        ProjectApproved {
            /// The T::AssetId of the approved project
            project_id: T::AssetId,
        },
        // An amount of VCUs was minted
        VCUMinted {
            /// The T::AssetId of the minted VCU
            project_id: T::AssetId,
            /// The AccountId that received the minted VCU
            recipient: T::AccountId,
            /// The amount of VCU units minted
            amount: T::Balance,
        },
        // An existing VCU was retired
        VCURetired {
            /// The T::AssetId of the retired VCU
            project_id: T::AssetId,
            /// The AccountId that retired the VCU
            account: T::AccountId,
            /// The amount of VCU units retired
            amount: T::Balance,
            /// Details of the retired token
            retire_data: BatchRetireDataList<T>,
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
        /// The account is not authorised
        NotAuthorised,
        /// The given Project was not found in storage
        ProjectNotFound,
        /// The Amount of VCU units is greater than supply
        AmountGreaterThanSupply,
        /// Calculcation triggered an Overflow
        Overflow,
        /// The token accounting generated an error
        SupplyAmountMismatch,
        /// The unit price for vcu cannot be zero
        UnitPriceIsZero,
        /// The project is not approved
        ProjectNotApproved,
        /// Cannot generate asset id
        CannotGenerateAssetId,
        /// ProjectId is lower than permitted
        ProjectIdLowerThanPermitted,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new project onchain
        /// This new project can mint tokens after approval from an authorised account
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            params: ProjectCreateParams<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let now = frame_system::Pallet::<T>::block_number();

            ensure!(
                project_id >= T::MinProjectId::get(),
                Error::<T>::ProjectIdLowerThanPermitted
            );

            // the unit price should not be zero
            ensure!(
                params.unit_price > Zero::zero(),
                Error::<T>::UnitPriceIsZero
            );

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

                let new_project = ProjectDetail {
                    originator: sender,
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

        /// Set the project status to approve/reject
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn approve_project(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            is_approved: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let authorized_accounts = AuthorizedAccounts::<T>::get();
            ensure!(
                authorized_accounts.contains(&sender),
                Error::<T>::NotAuthorised
            );

            Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
                // ensure the Project exists
                let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

                project.approved = is_approved;

                // emit event
                Self::deposit_event(Event::ProjectApproved { project_id });

                Ok(())
            })
        }

        /// TODO : Need an ext to resubmit

        /// Mint tokens for an approved project
        /// The tokens are always minted in the ascending order of credits, for example, if the
        /// `amount_to_mint` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we mint
        /// 100 from 2019 and 50 from 2020.
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn mint(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            amount_to_mint: T::Balance,
            list_to_marketplace: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Projects::<T>::try_mutate(project_id, |project| -> DispatchResult {
                // ensure the project exists
                let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;

                // ensure the project is approved
                ensure!(project.approved, Error::<T>::ProjectNotApproved);

                // ensure the caller is the originator
                ensure!(
                    sender == project.originator.clone(),
                    Error::<T>::NotAuthorised
                );

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
                // sort by issuance year so we mint from oldest batch
                batch_list.sort_by(|x, y| x.issuance_year.cmp(&y.issuance_year));
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
                    remaining = remaining - actual;
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

                // // create the asset if not already existing
                // if project.asset_id.is_none() {
                //     let asset_id = NextAssetId::<T>::get();
                //     // create the asset
                //     T::AssetHandler::create(asset_id, Self::account_id(), true, 1_u32.into())?;
                //     // set metadata for the asset
                //     T::AssetHandler::set(
                //         asset_id,
                //         &Self::account_id(),
                //         project.name.clone().into_inner(), // asset name
                //         asset_id.to_string().as_bytes().to_vec(), // asset symbol
                //         0,
                //     )?;

                //     //increment assetId counter
                //     let next_asset_id: u32 = asset_id.into() + 1_u32;
                //     NextAssetId::<T>::set(next_asset_id.into());

                //     // set the new asset_id in storage
                //     project.asset_id = Some(asset_id);
                // }

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

        /// Retire existing vcus from owner
        /// The tokens are always retired in the ascending order of credits, for example, if the
        /// `amount` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we retire
        /// 100 from 2019 and 50 from 2020.
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn retire(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::retire_vcus(sender, project_id, amount)
        }

        /// Add a new account to the list of authorised Accounts
        /// The caller must be from a permitted origin
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn force_add_authorized_account(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            // check for SUDO
            // TODO : Remove tight coupling with sudo, make configurable from config
            ensure_root(origin)?;
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
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn force_remove_authorized_account(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            // check for SUDO
            // TODO : Remove tight coupling with sudo, make configurable from config
            ensure_root(origin)?;
            // remove the account_id from the list of authorized accounts if already exists
            AuthorizedAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
                match account_list.binary_search(&account_id) {
                    Ok(index) => {
                        account_list.swap_remove(index);
                        Self::deposit_event(Event::AuthorizedAccountRemoved { account_id });
                    }
                    Err(_) => {}
                }
                Ok(())
            })
        }

        // TODO : Ext to forceset/clear storage
    }
}
