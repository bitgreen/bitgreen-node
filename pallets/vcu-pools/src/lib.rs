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

//! ## VCU Pools Pallet
//! The VCU Pools pallet lets users create and manage vcu pools. A vcu pool is a collection of vcu tokens of different types represented by a
//! common pool token. A user holding any vcu tokens (subject to the VCU pool config) can deposit vcu tokens to the pool and receive equivalent
//! pool tokens in return. These pool tokens can be transferred freely and can be retired. When retire function is called, the underlying vcu credits
//! are retired starting from the oldest in the pool.
//!
//! ### Pool Config
//! A pool creator can setup configs, these configs determine which type of tokens are accepted into the pool. Currently the owner can setup two configs for a pool
//! 1. Registry List : This limits the pool to accept vcu's issued by the given registry's only
//! 2. Project List : This limits the pool to accepts vcu's issued by specific project's only
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create`: Creates a new pool with given config
//! * `deposit`: Deposit some vcu tokens to generate pool tokens
//! * `retire`: Burn a specified amount of pool tokens
//!
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::HasCompact;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::tokens::fungibles::{metadata::Mutate as MetadataMutate, Create, Mutate, Transfer},
        transactional, PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
    use sp_std::convert::TryFrom;
    use sp_std::convert::TryInto;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_vcu::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The PoolId type for the pallet
        type PoolId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo
            + Into<Self::AssetId>
            + From<Self::AssetId>
            + sp_std::cmp::PartialOrd;

        // Asset manager config
        type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
            + Mutate<Self::AccountId>
            + MetadataMutate<Self::AccountId>
            + Transfer<Self::AccountId>;

        /// Maximum registrys allowed in the pool config
        type MaxRegistryListCount: Get<u32>;
        /// Maximum issuance years allowed in the pool config
        type MaxIssuanceYearCount: Get<u32>;
        /// Maximum projectIds allowed in the pool config
        type MaxProjectIdList: Get<u32>;
        /// Max length of pool asset symbol
        type MaxAssetSymbolLength: Get<u32>;
        /// Min permitted value for PoolId
        type MinPoolId: Get<Self::PoolId>;
        /// The vcu-pools pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn pools)]
    pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new pool was created
        PoolCreated {
            admin: T::AccountId,
            id: T::PoolId,
            config: PoolConfigOf<T>,
        },
        /// A new deposit was added to pool
        Deposit {
            who: T::AccountId,
            pool_id: T::PoolId,
            project_id: T::AssetId,
            amount: T::Balance,
        },
        /// Pool tokens were retired
        Retired {
            who: T::AccountId,
            pool_id: T::PoolId,
            amount: T::Balance,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// PoolId is already being used
        PoolIdInUse,
        /// The max limit supplied is greater than allowd
        MaxLimitGreaterThanPermitted,
        /// The given PoolId does not exist
        InvalidPoolId,
        /// The given project was not found
        ProjectNotFound,
        /// The pool does not allow this registry projects
        RegistryNotPermitted,
        /// The projectId is not whitelisted
        ProjectIdNotWhitelisted,
        /// PoolId should be above min limit
        PoolIdBelowExpectedMinimum,
        /// Overflow happened during retire
        UnexpectedOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new vcu pool with given params
        ///
        /// Params:
        /// id : Id of the new pool
        /// config : Config values for new pool
        /// max_limit : Limit of maximum project-ids the pool can support, default to T::MaxProjectIdLIst
        /// asset_symbol : Symbol for asset created for the pool
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            id: T::PoolId,
            config: PoolConfigOf<T>,
            max_limit: Option<u32>,
            asset_symbol: SymbolStringOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(
                id >= T::MinPoolId::get(),
                Error::<T>::PoolIdBelowExpectedMinimum
            );

            // TODO : Check if the user is authorised to create pools
            // TODO : add more checks for asset symbol

            ensure!(!Pools::<T>::contains_key(id), Error::<T>::PoolIdInUse);

            // use default limit if limit not given by project owner
            let actual_max_limit = match max_limit {
                Some(limit) => {
                    ensure!(
                        limit <= T::MaxProjectIdList::get(),
                        Error::<T>::MaxLimitGreaterThanPermitted
                    );
                    limit
                }
                None => T::MaxProjectIdList::get(),
            };

            // insert to storage
            <Pools<T>>::insert(
                id,
                Pool {
                    admin: who.clone(),
                    config: config.clone(),
                    max_limit: actual_max_limit,
                    credits: Default::default(),
                },
            );

            // create an asset collection to reserve asset-id
            <T as pallet::Config>::AssetHandler::create(
                id.into(),
                Self::account_id(),
                true,
                1_u32.into(),
            )?;

            // set metadata for the asset
            <T as pallet::Config>::AssetHandler::set(
                id.into(),
                &Self::account_id(),
                asset_symbol.clone().into(), // asset name
                asset_symbol.into(),         // asset symbol
                0,
            )?;

            // Emit an event.
            Self::deposit_event(Event::PoolCreated {
                admin: who,
                id,
                config,
            });

            Ok(().into())
        }

        /// Deposit VCU tokens to pool with `id`
        ///
        /// Params:
        /// pool_id : Id of the pool to deposit into
        /// project_id : The project_id of the vcu being deposited
        /// amount: The amount of VCU to deposit
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn deposit(
            origin: OriginFor<T>,
            pool_id: T::PoolId,
            project_id: T::AssetId,
            amount: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Pools::<T>::try_mutate(pool_id, |pool| -> DispatchResultWithPostInfo {
                let pool = pool.as_mut().ok_or(Error::<T>::InvalidPoolId)?;

                // get the details of project
                let project_details: pallet_vcu::ProjectDetail<T> =
                    pallet_vcu::Pallet::get_project_details(project_id)
                        .ok_or(Error::<T>::ProjectNotFound)?;

                // ensure the project_id passes the pool config
                if let Some(registry_list) = &pool.config.registry_list {
                    ensure!(
                        registry_list.contains(&project_details.registry_details.registry),
                        Error::<T>::RegistryNotPermitted
                    );
                }

                if let Some(project_id_list) = &pool.config.project_id_list {
                    ensure!(
                        project_id_list.contains(&project_id),
                        Error::<T>::ProjectIdNotWhitelisted
                    )
                }

                // calculate the issuance year for the project
                let project_issuance_year =
                    pallet_vcu::Pallet::calculate_issuance_year(project_details);

                // TODO : transfer the tokens to pool account

                // create an asset collection to reserve asset-id
                <T as pallet::Config>::AssetHandler::transfer(
                    project_id,
                    &who,
                    &Self::account_id(),
                    amount,
                    true,
                )?;

                // add the project to the credits pool
                let issuance_year_map = pool.credits.get_mut(&project_issuance_year);

                if let Some(project_map) = issuance_year_map {
                    let project_details = project_map.get_mut(&project_id);
                    if let Some(existing_amount) = project_details {
                        let new_amount = existing_amount.clone() + amount;
                        project_map
                            .try_insert(project_id, new_amount)
                            .map_err(|_| Error::<T>::UnexpectedOverflow)?;
                    } else {
                        project_map
                            .try_insert(project_id, amount)
                            .map_err(|_| Error::<T>::UnexpectedOverflow)?;
                    }
                } else {
                    let mut project_map: ProjectDetails<T> = Default::default();
                    project_map
                        .try_insert(project_id, amount)
                        .map_err(|_| Error::<T>::UnexpectedOverflow)?;
                    pool.credits
                        .try_insert(project_issuance_year, project_map)
                        .map_err(|_| Error::<T>::UnexpectedOverflow)?;
                }

                // Mint new pool tokens and transfer to caller
                <T as pallet::Config>::AssetHandler::mint_into(pool_id.into(), &who, amount)?;

                // Emit an event.
                Self::deposit_event(Event::Deposit {
                    who,
                    pool_id,
                    project_id,
                    amount,
                });

                Ok(().into())
            })
        }

        /// Retire Pool Tokens - A user can retire pool tokens, this will look at the available vcu token supply in the pool and retire tokens
        /// starting from the oldest issuance until the entire amount is retired.
        ///
        /// Params:
        /// pool_id : Id of the pooltokens to retire
        /// amount: The amount of VCU to deposit
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn retire(
            origin: OriginFor<T>,
            pool_id: T::PoolId,
            amount: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Pools::<T>::try_mutate(pool_id, |pool| -> DispatchResultWithPostInfo {
                let pool = pool.as_mut().ok_or(Error::<T>::InvalidPoolId)?;

                // Burn the amount of pool tokens from caller
                <T as pallet::Config>::AssetHandler::burn_from(pool_id.into(), &who, amount)?;

                let mut remaining = amount;

                let mut pool_credits_temp = pool.credits.clone().into_inner();

                // Retire tokens starting from oldest until `amount` is retired
                for (_year, project_map) in pool_credits_temp.iter_mut() {
                    // the iterator is sorted by key (year), so retire all from year before moving to next year
                    // we dont care about the project order
                    for (project_id, available_amount) in
                        project_map.clone().into_inner().iter_mut()
                    {
                        let actual: T::Balance;

                        if remaining <= *available_amount {
                            actual = remaining;
                            *available_amount = *available_amount - actual;
                        } else {
                            actual = *available_amount;
                            *available_amount = 0_u32.into();
                        }

                        // transfer the vcu tokens to caller
                        <T as pallet::Config>::AssetHandler::transfer(
                            *project_id,
                            &Self::account_id(),
                            &who,
                            actual,
                            true,
                        )?;
                        // Retire the transferred tokens
                        pallet_vcu::Pallet::<T>::retire_vcus(who.clone(), *project_id, actual)?;

                        // Update value in storage
                        // TODO : Remove entry if value is zero
                        project_map
                            .try_insert(*project_id, *available_amount)
                            .map_err(|_| Error::<T>::UnexpectedOverflow)?;

                        // this is safe since actual is <= remaining
                        remaining = remaining - actual;
                        if remaining <= Zero::zero() {
                            break;
                        }
                    }
                }

                pool.credits = CreditsMap::<T>::try_from(pool_credits_temp)
                    .map_err(|_| Error::<T>::UnexpectedOverflow)?;

                // Emit an event.
                Self::deposit_event(Event::Retired {
                    who,
                    pool_id,
                    amount,
                });

                Ok(().into())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// The account ID of the vcu pallet
        pub fn account_id() -> T::AccountId {
            <T as pallet::Config>::PalletId::get().into_account()
        }
    }
}
