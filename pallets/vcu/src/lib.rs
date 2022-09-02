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
//! * `force_set_retired_vcu` : Set the RetiredVCU storage
//!
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
pub use functions::*;

mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::HasCompact;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            tokens::fungibles::{metadata::Mutate as MetadataMutate, Create, Destroy, Mutate},
            tokens::nonfungibles::{Create as NFTCreate, Mutate as NFTMutate},
            Contains,
        },
        transactional, PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, One};
    use sp_std::convert::TryInto;

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
            + CheckedAdd
            + One
            + From<u32>
            + Into<u32>;

        /// The vcu pallet id
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
        type ForceOrigin: EnsureOrigin<Self::Origin>;
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
    pub(super) type RetiredVCUs<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AssetId,
        Blake2_128Concat,
        T::ItemId,
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
        /// A project details has been resubmitted
        ProjectResubmitted {
            /// The T::AssetId of the created project
            project_id: T::AssetId,
            /// The details of the created project
            details: ProjectDetail<T>,
        },
        /// Project has been approved
        ProjectApproved {
            /// The T::AssetId of the approved project
            project_id: T::AssetId,
        },
        /// Project has been rejected
        ProjectRejected {
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
        /// Account failed KYC checks
        KYCAuthorisationFailed,
        /// The account is not authorised
        NotAuthorised,
        /// The project cannot be created without vcus
        CannotCreateProjectWithoutCredits,
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
        /// Cannot resubmit an approved project
        CannotModifyApprovedProject,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new project onchain
        /// This new project can mint tokens after approval from an authorised account
        #[transactional]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            params: ProjectCreateParams<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_kyc_approval(&sender)?;
            Self::create_project(sender, project_id, params)
        }

        /// Resubmit a approval rejected project data onchain
        /// An approved project data cannot be resubmitted
        #[transactional]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn resubmit(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            params: ProjectCreateParams<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_kyc_approval(&sender)?;
            Self::resubmit_project(sender, project_id, params)
        }

        /// Set the project status to approve/reject
        #[transactional]
        #[pallet::weight(T::WeightInfo::approve_project())]
        pub fn approve_project(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            is_approved: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_authorized_account(&sender)?;
            Self::do_approve_project(project_id, is_approved)
        }

        /// Mint tokens for an approved project
        /// The tokens are always minted in the ascending order of credits, for example, if the
        /// `amount_to_mint` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we mint
        /// 100 from 2019 and 50 from 2020.
        #[transactional]
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            amount_to_mint: T::Balance,
            list_to_marketplace: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_authorized_account(&sender)?;
            // Self::check_kyc_approval(&sender)?;
            Self::mint_vcus(sender, project_id, amount_to_mint, list_to_marketplace)
        }

        /// Retire existing vcus from owner
        /// The tokens are always retired in the ascending order of credits, for example, if the
        /// `amount` is 150 and the project has 100 tokens of 2019 and 2020 year. Then we retire
        /// 100 from 2019 and 50 from 2020.
        #[transactional]
        #[pallet::weight(T::WeightInfo::retire())]
        pub fn retire(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_kyc_approval(&sender)?;
            Self::retire_vcus(sender, project_id, amount)
        }

        /// Add a new account to the list of authorised Accounts
        /// The caller must be from a permitted origin
        #[transactional]
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
        #[transactional]
        #[pallet::weight(T::WeightInfo::force_remove_authorized_account())]
        pub fn force_remove_authorized_account(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            // remove the account_id from the list of authorized accounts if already exists
            AuthorizedAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
                if let Ok(index) = account_list.binary_search(&account_id) {
                    account_list.swap_remove(index);
                    Self::deposit_event(Event::AuthorizedAccountRemoved { account_id });
                }

                Ok(())
            })
        }

        /// Force modify a project storage
        /// Can only be called by ForceOrigin
        #[transactional]
        #[pallet::weight(T::WeightInfo::force_set_project_storage())]
        pub fn force_set_project_storage(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            detail: ProjectDetail<T>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Projects::<T>::insert(project_id, detail);
            Ok(())
        }

        /// Force modify NextItemId storage
        /// Can only be called by ForceOrigin
        #[transactional]
        #[pallet::weight(T::WeightInfo::force_set_next_item_id())]
        pub fn force_set_next_item_id(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            item_id: T::ItemId,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            NextItemId::<T>::insert(project_id, item_id);
            Ok(())
        }

        /// Force modify retired vcu storage
        /// Can only be called by ForceOrigin
        #[transactional]
        #[pallet::weight(T::WeightInfo::force_set_retired_vcu())]
        pub fn force_set_retired_vcu(
            origin: OriginFor<T>,
            project_id: T::AssetId,
            item_id: T::ItemId,
            vcu_data: RetiredVcuData<T>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            RetiredVCUs::<T>::insert(project_id, item_id, vcu_data);
            Ok(())
        }

        /// Single function to create project, approve and mint vcus
        /// Can only be called by ForceOrigin
        #[transactional]
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn force_approve_and_mint_vcu(
            origin: OriginFor<T>,
            sender: T::AccountId,
            project_id: T::AssetId,
            params: ProjectCreateParams<T>,
            amount_to_mint: T::Balance,
            list_to_marketplace: bool,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::check_kyc_approval(&sender)?;
            Self::create_project(sender.clone(), project_id, params)?;
            Self::do_approve_project(project_id, true)?;
            Self::mint_vcus(sender, project_id, amount_to_mint, list_to_marketplace)?;
            Ok(())
        }

        /// Force remove an project asset from storage, can be used by ForceOrigin to remove unapproved projects
        /// Can only be called by ForceOrigin
        #[transactional]
        #[pallet::weight(T::WeightInfo::force_set_project_storage())]
        pub fn force_remove_project(
            origin: OriginFor<T>,
            project_id: T::AssetId,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            let destroy_witness = T::AssetHandler::get_destroy_witness(&project_id)
                .ok_or(Error::<T>::ProjectNotFound)?;
            T::AssetHandler::destroy(project_id, destroy_witness, None)?;
            Projects::<T>::take(project_id);
            Ok(())
        }
    }
}
