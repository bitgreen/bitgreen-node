// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! VCU Pallet
//! The VCU pallet creates and retires VCU units that represent the VCUs on the Verra registry. These onchain vcu units can represent a
//! single type of VCU or can build to represent a combination of different types of VCUs.  The VCUs are represented onchain as follows:
//!
//! pub struct VCUDetail<AccountId, Balance, AssetId, VcuId, BundleList> {
//!     // The account that owns/controls the VCU class
//!     pub originator: AccountId,
//!     // Count of current active units of VCU
//!     pub supply: Balance,
//!     // Count of retired units of VCU
//!     pub retired: Balance,
//!     // The AssetId that represents the Fungible class of VCU
//!     pub asset_id: AssetId,
//!     // The type of VCU [Bundle, Single]
//!     pub vcu_type: VCUType<VcuId, BundleList>,
//! }
//! The VCU units are created by an account that controls VCU units on the Verra registry, represented in the pallet as the originator.
//! The creation process will store the VCU details on the pallet storage and then mint the given amount of Vcu units using the Asset Handler
//! like pallet-assets. These newly minted vcu units will be transferred to the recipient, this can be any address.
//! These units can then be sold/transferred to a buyer of carbon credits, these transactions can take place multiple times but the final goal
//! of purchasing a Vcu unit is to retire them. The current holder of the vcu units can call the `retire_vcu` extrinsic to burn these
//! tokens (erase from storage), this process will store a reference of the tokens burned.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create`: Creates a new VCU, minting an amount of tokens
//! * `retire`: Burns an amount of VCU tokens
//! * `mint_into`: Create more units of already existing VCU
//!
//! ### Permissioned Functions
//!
//! * `force_add_authorized_account`: Adds a new_authorized_account to the list
//! * `force_remove_authorized_account`: Removes an authorized_account from the list
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::HasCompact;
    use frame_support::{
        pallet_prelude::*,
        traits::tokens::fungibles::{Create, Mutate},
        transactional,
    };
    use frame_system::{pallet_prelude::*, WeightInfo};
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::traits::Zero;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};
    use sp_std::convert::TryInto;

    /// AuthorizedAccounts type of pallet
    pub type AuthorizedAccountsListOf<T> = BoundedVec<
        <T as frame_system::Config>::AccountId,
        <T as Config>::MaxAuthorizedAccountCount,
    >;

    /// BundleList type of pallet
    pub type BundleListOf<T> = BoundedVec<<T as Config>::VcuId, <T as Config>::MaxBundleSize>;
    /// VCUDetail type of pallet
    pub type VCUDetailOf<T> = VCUDetail<
        <T as frame_system::Config>::AccountId,
        <T as Config>::Balance,
        <T as Config>::AssetId,
        <T as Config>::VcuId,
        BundleListOf<T>,
    >;
    /// VCUType type of pallet
    pub type VCUTypeOf<T> = VCUType<<T as Config>::VcuId, BundleListOf<T>>;

    /// VCUCreationParams type of pallet
    pub type VCUCreationParamsOf<T> = VCUCreationParams<
        <T as frame_system::Config>::AccountId,
        <T as Config>::Balance,
        <T as Config>::VcuId,
        BundleListOf<T>,
    >;

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
            + Into<u32>;

        /// Identifier for a project
        type ProjectId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Identifier for a VCU
        type VcuId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        // Asset manager config
        type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
            + Mutate<Self::AccountId>;
        /// Maximum amount of authorised accounts permitted
        type MaxAuthorizedAccountCount: Get<u32>;
        /// Maximum amount of vcus in a bundle
        type MaxBundleSize: Get<u32>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn authorized_accounts)]
    // List of AuthorizedAccounts for the pallet
    pub type AuthorizedAccounts<T: Config> =
        StorageValue<_, AuthorizedAccountsListOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_token_id)]
    // NextAssetId for the generated vcu_tokens, start from 1000
    // TODO : Ensure starts from 1000
    pub type NextAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vcus)]
    /// The details of a VCU
    pub(super) type VCUs<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProjectId,
        Blake2_128Concat,
        T::VcuId,
        VCUDetailOf<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn retired_vcus)]
    /// The retired vcu record
    pub(super) type RetiredVCUs<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::ProjectId>,
            NMapKey<Blake2_128Concat, T::VcuId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
        ),
        T::Balance,
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new AuthorizedAccount has been added
        AuthorizedAccountAdded { account_id: T::AccountId },
        /// An AuthorizedAccount has been removed
        AuthorizedAccountRemoved { account_id: T::AccountId },
        /// A new VCU has been created
        VCUCreated {
            /// The T::ProjectId of the created VCU
            project_id: T::ProjectId,
            /// The T::VcuId of the created VCU
            vcu_id: T::VcuId,
            /// The VCUType of the created VCU
            vcu_type: VCUTypeOf<T>,
            /// The AccountId that controls the VCU asset
            originator: T::AccountId,
            /// The AccountId that received the amount
            recipient: T::AccountId,
            /// The amount of VCU units created
            amount: T::Balance,
        },
        // An existing VCU was retired
        VCURetired {
            /// The T::ProjectId of the retired VCU
            project_id: T::ProjectId,
            /// the T::VcuId of the retired VCU
            vcu_id: T::VcuId,
            /// The AccountId that retired the VCU
            account: T::AccountId,
            /// The VCUType of the retired VCU
            vcu_type: VCUTypeOf<T>,
            /// The amount of VCU units retired
            amount: T::Balance,
        },
        // An existing VCU was retired
        VCUMinted {
            /// The T::ProjectId of the minted VCU
            project_id: T::ProjectId,
            /// the T::VcuId of the minted VCU
            vcu_id: T::VcuId,
            /// The AccountId that received the minted VCU
            recipient: T::AccountId,
            /// The amount of VCU units minted
            amount: T::Balance,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Adding a new authorized account failed
        TooManyAuthorizedAccounts,
        /// Cannot add a duplicate authorised account
        AuthorizedAccountAlreadyExists,
        /// Cannot create duplicate VCUs
        VCUAlreadyExists,
        /// The account is not authorised
        NotAuthorised,
        /// The given VCU was not found in storage
        VCUNotFound,
        /// The Amount of VCU units is greater than supply
        AmountGreaterThanSupply,
        /// Calculcation triggered an Overflow
        Overflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new vcu and mint `amount` of vcus to the recipient account
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            project_id: T::ProjectId,
            params: VCUCreationParamsOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let authorized_accounts = AuthorizedAccounts::<T>::get();
            ensure!(
                authorized_accounts.contains(&sender),
                Error::<T>::NotAuthorised
            );

            // get the vcu_id depending on the type
            let vcu_id = match params.vcu_type {
                // for bundle, we select the first id
                // TODO : Better error handling
                VCUType::Bundle(ref vcu_ids) => vcu_ids.first().unwrap().clone(),
                VCUType::Single(vcu_id) => vcu_id,
            };

            VCUs::<T>::try_mutate(project_id, vcu_id, |vcu| -> DispatchResult {
                ensure!(vcu.is_none(), Error::<T>::VCUAlreadyExists);

                let asset_id = NextAssetId::<T>::get();

                // TODO : We could add metadata to the created asset class
                // create the token
                T::AssetHandler::create(asset_id, params.originator.clone(), true, 1_u32.into())?;

                // mint the asset to the recipient
                T::AssetHandler::mint_into(asset_id, &params.recipient, params.amount)?;

                // update the storage
                let new_vcu = VCUDetailOf::<T> {
                    originator: params.originator.clone(),
                    supply: params.amount,
                    retired: Zero::zero(),
                    asset_id,
                    vcu_type: params.vcu_type.clone(),
                };

                //increment assetId counter
                let next_asset_id: u32 = asset_id.into() + 1_u32;

                NextAssetId::<T>::set(next_asset_id.into());

                // emit event
                Self::deposit_event(Event::VCUCreated {
                    originator: params.originator,
                    project_id,
                    vcu_id: vcu_id,
                    vcu_type: params.vcu_type,
                    recipient: params.recipient,
                    amount: params.amount,
                });

                *vcu = Some(new_vcu);

                Ok(())
            })
        }

        /// Mint amount of tokens to already existing VCU
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn mint_into(
            origin: OriginFor<T>,
            project_id: T::ProjectId,
            vcu_id: T::VcuId,
            recipient: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let authorized_accounts = AuthorizedAccounts::<T>::get();
            ensure!(
                authorized_accounts.contains(&sender),
                Error::<T>::NotAuthorised
            );

            VCUs::<T>::try_mutate(project_id, vcu_id, |vcu| -> DispatchResult {
                // ensure the VCU exists
                let vcu = vcu.as_mut().ok_or(Error::<T>::VCUNotFound)?;

                // mint the asset to the recipient
                T::AssetHandler::mint_into(vcu.asset_id, &recipient, amount)?;

                // reduce the supply of the vcu
                vcu.supply = vcu
                    .supply
                    .checked_add(&amount)
                    .ok_or(Error::<T>::Overflow)?;

                // emit event
                Self::deposit_event(Event::VCUMinted {
                    project_id,
                    vcu_id,
                    recipient,
                    amount,
                });

                Ok(())
            })
        }

        /// Retire existing vcus from owner
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn retire(
            origin: OriginFor<T>,
            project_id: T::ProjectId,
            vcu_id: T::VcuId,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            VCUs::<T>::try_mutate(project_id, vcu_id, |vcu| -> DispatchResult {
                // ensure the VCU exists
                let vcu = vcu.as_mut().ok_or(Error::<T>::VCUNotFound)?;

                // attempt to burn the tokens from the caller
                T::AssetHandler::burn_from(vcu.asset_id, &sender.clone(), amount)?;

                // reduce the supply of the vcu
                vcu.supply = vcu
                    .supply
                    .checked_sub(&amount)
                    .ok_or(Error::<T>::AmountGreaterThanSupply)?;

                vcu.retired = vcu
                    .retired
                    .checked_add(&amount)
                    .ok_or(Error::<T>::AmountGreaterThanSupply)?;

                // increment the retired vcus count
                RetiredVCUs::<T>::try_mutate(
                    (project_id, vcu_id, sender.clone()),
                    |retired_vcu| -> DispatchResult {
                        retired_vcu.checked_add(&amount);
                        Ok(())
                    },
                )?;

                // TODO : When the supply reaches zero we can delete the VCU storage from memory

                // TODO : Mint an NFT with the burned vcu details

                // emit event
                Self::deposit_event(Event::VCURetired {
                    account: sender,
                    project_id,
                    vcu_id,
                    vcu_type: vcu.vcu_type.clone(),
                    amount,
                });

                Ok(())
            })
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
    }
}
