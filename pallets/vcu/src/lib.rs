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

// SBP M1 review: missing documentation

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
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::traits::Zero;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};
    use sp_std::convert::TryInto;

    pub type AuthorizedAccountsListOf<T> = BoundedVec<
        <T as frame_system::Config>::AccountId,
        <T as Config>::MaxAuthorizedAccountCount,
    >;

    pub type BundleListOf<T> = BoundedVec<VcuId, <T as Config>::MaxBundleSize>;
    pub type VCUDetailOf<T> = VCUDetail<
        <T as frame_system::Config>::AccountId,
        <T as Config>::Balance,
        <T as Config>::AssetId,
        BundleListOf<T>,
    >;
    pub type VCUTypeOf<T> = VCUType<BundleListOf<T>>;
    pub type VCUCreationParamsOf<T> = VCUCreationParams<
        <T as frame_system::Config>::AccountId,
        <T as Config>::Balance,
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

        // Asset manager config
        type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
            + Mutate<Self::AccountId>;
        /// Maximum amount of authorised accounts permitted
        type MaxAuthorizedAccountCount: Get<u32>;
        /// Maximum amount of vcus in a bundle
        type MaxBundleSize: Get<u32>;
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
    pub type NextAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

    #[pallet::storage]
    /// The details of a VCU
    pub(super) type VCUs<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, ProjectId, Blake2_128Concat, VcuId, VCUDetailOf<T>>;

    #[pallet::storage]
    /// The retired vcu record
    pub(super) type RetiredVCUs<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, ProjectId>,
            NMapKey<Blake2_128Concat, VcuId>,
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
            /// The ProjectId of the created VCU
            project_id: ProjectId,
            /// The VcuId of the created VCU
            vcu_id: VcuId,
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
            /// The ProjectId of the retired VCU
            project_id: ProjectId,
            /// the VcuId of the retired VCU
            vcu_id: VcuId,
            /// The AccountId that retired the VCU
            account: T::AccountId,
            /// The VCUType of the retired VCU
            vcu_type: VCUTypeOf<T>,
            /// The amount of VCU units retired
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
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new account to the list of authorised Accounts
        // The caller must be from a permitted origin
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_authorized_account(
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
        pub fn remove_authorized_account(
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

        /// Create a new vcu and mint `amount` of vcus to the recipient account
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_vcu(
            origin: OriginFor<T>,
            project_id: ProjectId,
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

        /// Retire existing vcus from owner
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn retire_vcu(
            origin: OriginFor<T>,
            project_id: ProjectId,
            vcu_id: VcuId,
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


        // TODO : Mint into existing vcu
    }
}
