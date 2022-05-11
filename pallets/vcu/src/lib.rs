// This file is part of BitGreen.

// Copyright (C) 2021 BitGreen.

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

// SBP M1 review: missing documentation & benchmarks.
// General remark: you should avoid using JSON as an internal format,
// and instead leverage the built-in suport for SCALE encoding.
// Perform JSON string parsing operations will make your runtime perform
// suboptimally, and bloat the chain's storage.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::alloc::string::ToString;
use codec::Decode;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::UnixTime;
use frame_support::{ensure, traits::Get};
pub use frame_system::pallet_prelude::*;
use frame_system::RawOrigin;
use frame_system::{ensure_root, ensure_signed};
use primitives::Balance;
use sp_runtime::traits::One;
use sp_runtime::traits::StaticLookup;
use sp_std::convert::TryInto;
use sp_std::vec;
use sp_std::vec::Vec;

mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

    pub type AssetGeneratingVCUContentOf<T> =
        AssetGeneratingVCUContent<<T as frame_system::Config>::BlockNumber>;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128>
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Veera project id minimum length
        type MinPIDLength: Get<u32>;
        /// Unix time
        type UnixTime: UnixTime;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Settings configuration, we define some administrator accounts for the pallet VCU without using the super user account.
    #[pallet::storage]
    #[pallet::getter(fn get_settings)]
    pub type Settings<T: Config> = StorageValue<_, Vec<T::AccountId>>;

    /// AuthorizedAccountsAGV, we define authorized accounts to store/change the Assets Generating VCU (Verified Carbon Credit).
    #[pallet::storage]
    #[pallet::getter(fn get_authorized_accounts)]
    pub type AuthorizedAccountsAGV<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    /// AssetsGeneratingVCU (Verified Carbon Credit) should be stored on chain from the authorized accounts.
    #[pallet::storage]
    #[pallet::getter(fn asset_generating_vcu)]
    pub type AssetsGeneratingVCU<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        AssetGeneratingVCUContentOf<T>,
    >;

    /// AssetsGeneratingVCUShares The AGV shares can be minted/burned from the Authorized account up to the maximum number set in the AssetsGeneratingVCU.
    #[pallet::storage]
    #[pallet::getter(fn asset_generating_vcu_shares)]
    pub type AssetsGeneratingVCUShares<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>, // agv account
            NMapKey<Blake2_128Concat, u32>,          // agv id
            NMapKey<Blake2_128Concat, T::AccountId>, // recipient
        ),
        u32,
        ValueQuery,
    >;

    /// AssetsGeneratingVCUSharesMinted the total AGV shares minted for a shareholder
    #[pallet::storage]
    #[pallet::getter(fn asset_generating_vcu_shares_minted)]
    pub type AssetsGeneratingVCUSharesMinted<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u32, ValueQuery>;

    /// AssetsGeneratingVCUSharesMintedTotal the total AGV shares minted for a specific AGV
    #[pallet::storage]
    #[pallet::getter(fn asset_generating_vcu_shares_minted_total)]
    pub type AssetsGeneratingVCUSharesMintedTotal<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u32, ValueQuery>;

    /// AssetsGeneratingVCUSchedule (Verified Carbon Credit) should be stored on chain from the authorized accounts.
    #[pallet::storage]
    #[pallet::getter(fn asset_generating_vcu_schedule)]
    pub type AssetsGeneratingVCUSchedule<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        AssetsGeneratingVCUScheduleContent,
    >;

    /// AssetsGeneratingVCUGenerated Minting of Scheduled VCU
    #[pallet::storage]
    #[pallet::getter(fn vcu_generated)]
    pub type AssetsGeneratingVCUGenerated<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u64, ValueQuery>;

    /// VCUsBurnedAccounts: store the burned vcu for each account
    #[pallet::storage]
    #[pallet::getter(fn vcu_burned_account)]
    pub type VCUsBurnedAccounts<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        u128,
        ValueQuery,
    >;

    /// VCUsBurned: store the burned VCU for each type of VCU token
    #[pallet::storage]
    #[pallet::getter(fn vcu_burned)]
    pub(super) type VCUsBurned<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

    /// OraclesAccountMintingVCU: allow to store the account of the Oracle to mint the VCU for its AGV
    #[pallet::storage]
    #[pallet::getter(fn oracle_account_generating_vcu)]
    pub type OraclesAccountMintingVCU<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, T::AccountId>;

    /// OraclesTokenMintingVCU: allows to store the tokenid of the Oracle to mint the VCU for its AGV
    #[pallet::storage]
    #[pallet::getter(fn oracle_tokenid_generating_vcu)]
    pub type OraclesTokenMintingVCU<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u32, ValueQuery>;

    /// BundleAssetsGeneratingVCU: a "bundle" of AGV
    #[pallet::storage]
    #[pallet::getter(fn bundle_asset_generating_vcu)]
    pub(super) type BundleAssetsGeneratingVCU<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

    /// A counter of burned tokens for the signer
    #[pallet::storage]
    #[pallet::getter(fn get_burn_count)]
    pub type BurnedCounter<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u32, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New proxy setting has been created.
        SettingsCreated(Vec<T::AccountId>),
        /// Proxy setting has been destroyed.
        SettingsDestroyed,
        /// Added authorized account.
        AuthorizedAccountAdded(T::AccountId),
        /// Destroyed authorized account.
        AuthorizedAccountsAGVDestroyed(T::AccountId),
        /// AssetsGeneratingVCU has been stored.
        AssetsGeneratingVCUCreated(u32),
        /// Destroyed AssetGeneratedVCU.
        AssetGeneratingVCUDestroyed(u32),
        /// Minted AssetGeneratedVCU.
        AssetsGeneratingVCUSharesMinted(T::AccountId, u32),
        /// Burned AssetGeneratedVCU.
        AssetsGeneratingVCUSharesBurned(T::AccountId, u32),
        /// Transferred AssetGeneratedVCU.
        AssetsGeneratingVCUSharesTransferred(T::AccountId),
        /// Added AssetsGeneratingVCUSchedule
        AssetsGeneratingVCUScheduleAdded(T::AccountId, u32),
        /// Destroyed AssetsGeneratingVCUSchedule
        AssetsGeneratingVCUScheduleDestroyed(T::AccountId, u32),
        /// Added AssetsGeneratingVCUGenerated.
        AssetsGeneratingVCUGenerated(T::AccountId, u32),
        /// Added VCUBurned.
        VCUsBurnedAdded(T::AccountId, u32, u32),
        /// Added OraclesAccountMintingVCU
        OraclesAccountMintingVCUAdded(T::AccountId, u32, T::AccountId),
        /// Destroyed OraclesAccountMintingVCUDestroyed
        OraclesAccountMintingVCUDestroyed(T::AccountId, u32),
        /// OracleAccountVCUMinted
        OracleAccountVCUMinted(T::AccountId, u32, T::AccountId),
        /// Added BundleAssetsGeneratingVCU
        AddedBundleAssetsGeneratingVCU(u32),
        /// Destroyed BundleAssetsGeneratingVCU
        DestroyedBundleAssetsGeneratingVCU(u32),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Settings Key already exists
        SettingsKeyExists,
        /// Settings Key has not been found on the blockchain
        SettingsKeyNotFound,
        /// Settings data is too short to be valid
        SettingsJsonTooShort,
        /// Settings data is too long to be valid
        SettingsJsonTooLong,
        /// Invalid Json structure
        InvalidJson,
        /// Invalid Description
        InvalidDescription,
        /// AuthorizedAccountsAGV has not been found on the blockchain
        AuthorizedAccountsAGVNotFound,
        /// NumberofShares not found
        NumberofSharesNotFound,
        /// Number of share cannot be zero
        NumberofSharesCannotBeZero,
        /// Too many NumberofShares
        TooManyShares,
        /// AssetGeneratedVCU has not been found on the blockchain
        AssetGeneratingVCUNotFound,
        /// Invalid AGVId
        InvalidAGVId,
        /// Too less NumberofShares
        TooLessShares,
        /// InsufficientShares
        InsufficientShares,
        /// Got an overflow after adding
        Overflow,
        /// AssetGeneratedShares has not been found on the blockchain
        AssetGeneratedSharesNotFound,
        /// Invalid VCU Amount
        InvalidVCUAmount,
        /// AssetGeneratedVCUSchedule has not been found on the blockchain
        AssetGeneratedVCUScheduleNotFound,
        /// Asset does not exist,
        AssetDoesNotExist,
        /// AssetGeneratingSchedule has been Expired
        AssetGeneratedScheduleExpired,
        /// AOraclesAccountMintingVCU Not Found
        OraclesAccountMintingVCUNotFound,
        /// BundleAssetsGeneratingVCU JSON is too short to be valid
        BundleAssetsGeneratingVCUJsonTooShort,
        /// BundleAssetsGeneratingVCU is too long to be valid
        BundleAssetsGeneratingVCUJsonTooLong,
        /// InvalidAGVs
        InvalidAGVs,
        /// Bundle does not exist,
        BundleDoesNotExist,
        /// BundleAssetIdNotSame
        BundleAssetIdNotSame,
        /// The recipient has not shares minted
        RecipientSharesNotFound,
        /// Recipient Shares are less of burning shares
        RecipientSharesLessOfBurningShares,
        /// Total shares are not enough to burn the amount requested
        TotalSharesNotEnough,
        /// Invalid period in days
        InvalidPeriodDays,
        /// The schedule is already present on chain
        ScheduleDuplicated,
        /// The minting time is not not yet arrived based on the schedule
        AssetGeneratedScheduleNotYetArrived,
        /// Token id not found in Assets Pallet
        TokenIdNotFound,
        /// The schedule is already present on chain
        AssetsGeneratingVCUScheduleAlreadyOnChain,
        /// The Oracle account is not matching the signer of the transaction
        OracleAccountNotMatchingSigner,
        /// Token for Oracle has not been found, inconsistency in stored data
        OraclesTokenMintingVCUNotFound,
        /// InsufficientVCUs
        InsufficientVCUs,
        /// Token id must have a value > 10000 because till 10000 is reserved for the Bridge pallet.
        ReservedTokenId,
        /// Asset Already In Use
        AssetAlreadyInUse,
        /// The AVG has not yet shares minted
        NoAVGSharesNotFound,
        /// Too many shares
        TooManyNumberofShares,
        /// AGV not found
        AssetGeneratedVCUNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // SBP M1 review: you could probably use Substrate FRAME's Proxy pallet for a more flexible & optimal call delegation system.
        // Note: all dispatchable calls should be benchmarked.

        /// Create new proxy setting that allow to define some accounts with administrator rights on the pallet.
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_proxy_settings(
            origin: OriginFor<T>,
            accounts: Vec<T::AccountId>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // check whether setting key already exists
            ensure!(!Settings::<T>::exists(), Error::<T>::SettingsKeyExists);

            Settings::<T>::set(Some(accounts.clone()));
            // Generate event
            Self::deposit_event(Event::SettingsCreated(accounts));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy proxy setting keys
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_proxy_settings(origin: OriginFor<T>) -> DispatchResult {
            // check for SUDO
            ensure_root(origin)?;

            // check whether setting key exists
            ensure!(Settings::<T>::exists(), Error::<T>::SettingsKeyNotFound);

            // remove the proxy settings
            Settings::<T>::kill();
            // Generate event
            Self::deposit_event(Event::SettingsDestroyed);
            // Return a successful DispatchResult
            Ok(())
        }

        /// Store/update an AuthorizedAccountsAGV
        /// This function allows to store the Accounts enabled to create Assets generating VCU (AGV).
        ///
        /// `add_authorized_accounts` will accept `account_id` and `description` as parameter
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn add_authorized_account(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            description: Vec<u8>,
        ) -> DispatchResult {
            // check for SUDO
            ensure_root(origin)?;
            // description is mandatory
            ensure!(!description.is_empty(), Error::<T>::InvalidDescription);
            //minimu lenght of 4 chars
            ensure!(description.len() > 4, Error::<T>::InvalidDescription);
            // add/replace the description for the account received
            AuthorizedAccountsAGV::<T>::try_mutate_exists(account_id.clone(), |desc| {
                *desc = Some(description);
                // Generate event
                Self::deposit_event(Event::AuthorizedAccountAdded(account_id));
                // Return a successful DispatchResult
                Ok(())
            })
        }

        /// Destroys an authorized account revekin its authorization
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_authorized_account(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            // check for SUDO
            ensure_root(origin)?;
            // check whether authorized account exists or not
            ensure!(
                AuthorizedAccountsAGV::<T>::contains_key(&account_id),
                Error::<T>::AuthorizedAccountsAGVNotFound
            );
            // remove the authorized account from the state
            AuthorizedAccountsAGV::<T>::remove(account_id.clone());
            // Generate event
            Self::deposit_event(Event::AuthorizedAccountsAGVDestroyed(account_id));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Create new Assets Generating VCU on chain
        ///
        /// `create_asset_generating_vcu` will accept `agv_account_id`, `agv_id` and `content` as parameter
        /// and create new Assets Generating VCU in system
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_asset_generating_vcu(
            origin: OriginFor<T>,
            agv_account_id: T::AccountId,
            agv_id: u32,
            content: AssetGeneratingVCUContentOf<T>,
        ) -> DispatchResult {
            // check for SUDO user or owner account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            ensure!(
                content.number_of_shares > 0,
                Error::<T>::NumberofSharesCannotBeZero
            );

            // store the asset
            AssetsGeneratingVCU::<T>::try_mutate_exists(agv_account_id, agv_id, |desc| {
                *desc = Some(content);
                // Generate event
                Self::deposit_event(Event::AssetsGeneratingVCUCreated(agv_id));
                // Return a successful DispatchResult
                Ok(())
            })
        }

        /// Destroy Assets Generating VCU from storage.
        ///
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_asset_generating_vcu(
            origin: OriginFor<T>,
            agv_account_id: T::AccountId,
            agv_id: u32,
        ) -> DispatchResult {
            // check for SUDO or authorized account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check whether asset generated VCU exists or not
            ensure!(
                AssetsGeneratingVCU::<T>::contains_key(&agv_account_id, &agv_id),
                Error::<T>::AssetGeneratingVCUNotFound
            );
            // TODO check for VCU already generated to avoid orphans or leave the decision to the administrator?
            // renove the assets generating VCU
            AssetsGeneratingVCU::<T>::remove(agv_account_id, agv_id);
            // Generate event
            Self::deposit_event(Event::AssetGeneratingVCUDestroyed(agv_id));
            // Return a successful DispatchResult
            Ok(())
        }

        /// The AGV shares can be minted from the Authorized account up to the maximum number set in the AssetsGeneratingVCU.
        ///
        /// ex: agvaccout: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn mint_shares_asset_generating_vcu(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            agv_account: T::AccountId,
            agv_id: u32,
            number_of_shares: u32,
        ) -> DispatchResult {
            // checking for SUDO or authorized account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check whether asset generating VCU (AGV) exists or not
            // read info about the AGV
            let content: AssetGeneratingVCUContentOf<T> =
                AssetsGeneratingVCU::<T>::get(&agv_account, &agv_id)
                    .ok_or(Error::<T>::AssetGeneratingVCUNotFound)?;

            // increase the total shares minted for the recipient/shareholder
            AssetsGeneratingVCUSharesMinted::<T>::try_mutate(
                &agv_account,
                &agv_id,
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_add(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    ensure!(
                        total_sh <= content.number_of_shares,
                        Error::<T>::TooManyShares
                    );
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // increase the total shares minted per AGV
            AssetsGeneratingVCUSharesMintedTotal::<T>::try_mutate(
                &agv_account,
                &agv_id,
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_add(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    ensure!(
                        total_sh <= content.number_of_shares,
                        Error::<T>::TooManyShares
                    );
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // increase the shares minted for the recipient
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &recipient),
                |share| -> DispatchResult {
                    let total_sha = share
                        .checked_add(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    *share = total_sha;
                    Ok(())
                },
            )?;

            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUSharesMinted(agv_account, agv_id));
            // Return a successful DispatchResult
            Ok(())
        }

        /// The AGV shares can be burned from the Authorized account in the AssetsGeneratingVCU.
        ///
        /// ex: agv_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn burn_shares_asset_generating_vcu(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            agv_account: T::AccountId,
            agv_id: u32,
            number_of_shares: u32,
        ) -> DispatchResult {
            // checking for SUDO or authorized account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check whether asset generated VCU exists or not
            ensure!(
                AssetsGeneratingVCU::<T>::contains_key(&agv_account, &agv_id),
                Error::<T>::AssetGeneratingVCUNotFound
            );
            // check for previously minted shares for the recipient
            ensure!(
                AssetsGeneratingVCUShares::<T>::contains_key((&agv_account, &agv_id, &recipient),),
                Error::<T>::RecipientSharesNotFound
            );
            // check  the number of burnable shares for the recipient
            let currentshares = AssetsGeneratingVCUShares::<T>::get((
                agv_account.clone(),
                agv_id,
                recipient.clone(),
            ));
            ensure!(
                currentshares >= number_of_shares,
                Error::<T>::RecipientSharesLessOfBurningShares
            );
            // check the number of burnable shares in total
            ensure!(
                AssetsGeneratingVCUSharesMinted::<T>::contains_key(&agv_account, &agv_id),
                Error::<T>::TotalSharesNotEnough
            );
            let totalcurrentshares =
                AssetsGeneratingVCUSharesMinted::<T>::get(&agv_account, &agv_id);
            ensure!(
                totalcurrentshares >= number_of_shares,
                Error::<T>::TotalSharesNotEnough
            );
            // decrease total shares minted
            AssetsGeneratingVCUSharesMinted::<T>::try_mutate(
                &agv_account,
                &agv_id,
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_sub(number_of_shares)
                        .ok_or(Error::<T>::InsufficientShares)?;
                    ensure!(total_sh > 0, Error::<T>::TooLessShares);
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // decrease shares minted for the recipient account
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &recipient),
                |share| -> DispatchResult {
                    let total_sha = share
                        .checked_sub(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    *share = total_sha;
                    Ok(())
                },
            )?;
            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUSharesBurned(agv_account, agv_id));
            // Return a successful DispatchResult
            Ok(())
        }
        /// The owner can transfer its own shares to a recipient
        ///
        /// ex: agv_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn transfer_shares_asset_generating_vcu(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            agv_account: T::AccountId,
            agv_id: u32,
            number_of_shares: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // check that the shares are present
            ensure!(
                AssetsGeneratingVCUShares::<T>::contains_key((&agv_account, &agv_id, &sender)),
                Error::<T>::AssetGeneratedSharesNotFound
            );
            // get the shares available
            let sender_shares =
                AssetsGeneratingVCUShares::<T>::get((&agv_account, &agv_id, &sender));
            // check whether shares are enough for the transfer
            ensure!(
                number_of_shares <= sender_shares,
                Error::<T>::NumberofSharesNotFound
            );
            // decrease the shares for the sender
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &sender),
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_sub(number_of_shares)
                        .ok_or(Error::<T>::TooLessShares)?;
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // increase the shares for the recipient for the same amount
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &recipient),
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_add(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUSharesTransferred(recipient));
            // Return a successful DispatchResult
            Ok(())
        }
        /// The administrator can force a transfer of shares from a sender to a recipient
        ///
        /// ex: agv_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn forcetransfer_shares_asset_generating_vcu(
            origin: OriginFor<T>,
            sender: T::AccountId,
            recipient: T::AccountId,
            agv_account: T::AccountId,
            agv_id: u32,
            number_of_shares: u32,
        ) -> DispatchResult {
            // chec for administrator access
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;
            // check that the shares are present
            ensure!(
                AssetsGeneratingVCUShares::<T>::contains_key((&agv_account, &agv_id, &sender)),
                Error::<T>::AssetGeneratedSharesNotFound
            );
            // get the shares available
            let sender_shares =
                AssetsGeneratingVCUShares::<T>::get((&agv_account, &agv_id, &sender));
            // check whether shares are enough for the transfer
            ensure!(
                number_of_shares <= sender_shares,
                Error::<T>::NumberofSharesNotFound
            );
            // decrease the shares for the sender
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &sender),
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_sub(number_of_shares)
                        .ok_or(Error::<T>::TooLessShares)?;
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // increase the shares for the recipient for the same amount
            AssetsGeneratingVCUShares::<T>::try_mutate(
                (&agv_account, &agv_id, &recipient),
                |share| -> DispatchResult {
                    let total_sh = share
                        .checked_add(number_of_shares)
                        .ok_or(Error::<T>::Overflow)?;
                    *share = total_sh;
                    Ok(())
                },
            )?;
            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUSharesTransferred(recipient));
            // Return a successful DispatchResult
            Ok(())
        }

        /// To store asset generating vcu schedule
        ///
        /// ex: agv_account: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_asset_generating_vcu_schedule(
            origin: OriginFor<T>,
            agv_account_id: T::AccountId,
            agv_id: u32,
            period_days: u64,
            amount_vcu: Balance,
            token_id: u32,
        ) -> DispatchResult {
            // check for Sudo or other admnistrator account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check whether asset generating VCU exists or not
            ensure!(
                AssetsGeneratingVCU::<T>::contains_key(&agv_account_id, &agv_id),
                Error::<T>::AssetGeneratingVCUNotFound
            );
            // check for VCU amount > 0
            ensure!(amount_vcu > 0, Error::<T>::InvalidVCUAmount);
            // check for days >0
            ensure!(period_days > 0, Error::<T>::InvalidPeriodDays);
            // check the schedule is not alreayd on chain
            ensure!(
                !AssetsGeneratingVCUSchedule::<T>::contains_key(&agv_account_id, &agv_id),
                Error::<T>::AssetsGeneratingVCUScheduleAlreadyOnChain
            );

            // check the token id is present on chain
            ensure!(
                pallet_assets::Pallet::<T>::maybe_total_supply(token_id).is_some(),
                Error::<T>::TokenIdNotFound
            );

            // check the token id > 10000 (because under 10000 reserver for the bridge)
            ensure!(token_id >= 10000, Error::<T>::ReservedTokenId);

            // store the schedule
            AssetsGeneratingVCUSchedule::<T>::insert(
                &agv_account_id,
                &agv_id,
                AssetsGeneratingVCUScheduleContent {
                    period_days,
                    amount_vcu,
                    token_id,
                },
            );
            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUScheduleAdded(
                agv_account_id,
                agv_id,
            ));
            // Return a successful DispatchResult
            Ok(())
        }

        /// To destroy asset generating vcu schedule
        ///
        /// ex: agv_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_asset_generating_vcu_schedule(
            origin: OriginFor<T>,
            agv_account_id: T::AccountId,
            agv_id: u32,
        ) -> DispatchResult {
            // check for Sudo or other admnistrator account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check whether asset generated VCU exists or not
            ensure!(
                AssetsGeneratingVCUSchedule::<T>::contains_key(&agv_account_id, &agv_id),
                Error::<T>::AssetGeneratedVCUScheduleNotFound
            );
            // remove the schedule
            AssetsGeneratingVCUSchedule::<T>::remove(&agv_account_id, &agv_id);
            // Generate event
            Self::deposit_event(Event::AssetsGeneratingVCUScheduleDestroyed(
                agv_account_id,
                agv_id,
            ));
            // Return a successful DispatchResult
            Ok(())
        }

        /// This function allows the minting of the VCU periodically. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.
        ///
        /// This function checks if it’s time to mint new VCU based on the schedule and the previous generated VCU stored in AssetsGeneratingVCUGenerated or
        /// if it’s time to generate new VCU, it mints the scheduled “Assets” (see Assets pallets), and stores in AssetsGeneratingVCUGenerated  a json structure with the following fields:
        /// ```json
        /// {
        /// “timestamp”: u32  (epoch time in seconds)
        /// “amountvcu”: i32,
        /// }
        /// ```
        /// The function must deny further minting once is done till the new schedule is expired.
        /// For example with a schedule every year, the minting will be executed only one time every 365 days.
        ///
        /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        /// the first minting can be done anytime, the  following minting not before the scheduled time
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn mint_scheduled_vcu(
            origin: OriginFor<T>,
            agv_account_id: T::AccountId,
            agv_id: u32,
        ) -> DispatchResultWithPostInfo {
            // check for Sudo or other admnistrator account
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;

            // check for AGV
            ensure!(
                AssetsGeneratingVCUSchedule::<T>::contains_key(&agv_account_id, &agv_id),
                Error::<T>::AssetGeneratedVCUScheduleNotFound
            );
            let content: AssetsGeneratingVCUScheduleContent =
                AssetsGeneratingVCUSchedule::<T>::get(agv_account_id.clone(), &agv_id).unwrap();

            let mut timestamp: u64 = 0;
            let now: u64 = T::UnixTime::now().as_secs();
            // check for the last minting done
            if AssetsGeneratingVCUGenerated::<T>::contains_key(&agv_account_id, &agv_id) {
                timestamp = AssetsGeneratingVCUGenerated::<T>::get(&agv_account_id, &agv_id);
            }
            let elapse: u64 = content.period_days * 24 * 60;
            ensure!(
                now + elapse <= timestamp,
                Error::<T>::AssetGeneratedScheduleNotYetArrived
            );
            // create token if it does not exists
            ensure!(content.token_id >= 10000, Error::<T>::ReservedTokenId);
            if let None = pallet_assets::Pallet::<T>::maybe_total_supply(content.token_id) {
                pallet_assets::Pallet::<T>::force_create(
                    RawOrigin::Root.into(),
                    content.token_id,
                    T::Lookup::unlookup(agv_account_id.clone()),
                    Default::default(),
                    One::one(),
                )?;
            }
            // check for existing shares
            ensure!(
                AssetsGeneratingVCUSharesMintedTotal::<T>::contains_key(
                    agv_account_id.clone(),
                    agv_id.clone()
                ),
                Error::<T>::NoAVGSharesNotFound
            );
            // read totals shares minted for the AGV
            let totalshares: u128 = AssetsGeneratingVCUSharesMintedTotal::<T>::get(
                agv_account_id.clone(),
                agv_id.clone(),
            )
            .into();
            // set the key of search
            let shareholdersc =
                AssetsGeneratingVCUShares::<T>::iter_prefix((agv_account_id.clone(), agv_id));
            let nshareholders = shareholdersc.count();
            // iter for the available shareholders
            let shareholders =
                AssetsGeneratingVCUShares::<T>::iter_prefix((agv_account_id.clone(), agv_id));
            let mut vcuminted: u128 = 0;
            let mut nshareholdersprocessed: usize = 0;
            for numsh in shareholders {
                let shareholder = numsh.0;
                let numshares: u128 = numsh.1.into();
                //compute VCU for the shareholder
                let mut nvcu = content.amount_vcu / totalshares * numshares;
                // increase counter shareholders processed
                nshareholdersprocessed = nshareholdersprocessed + 1;
                // manage overflow for rounding
                if nshareholdersprocessed == nshareholders && vcuminted + nvcu > content.amount_vcu
                {
                    nvcu = content.amount_vcu - vcuminted;
                }
                // manage underflow for rounding
                if nshareholdersprocessed == nshareholders && vcuminted + nvcu < content.amount_vcu
                {
                    nvcu = content.amount_vcu - vcuminted;
                }
                //mint the vcu in proportion to the shares owned
                pallet_assets::Pallet::<T>::mint(
                    RawOrigin::Signed(agv_account_id.clone()).into(),
                    content.token_id,
                    T::Lookup::unlookup(shareholder.clone()),
                    nvcu,
                )
                .unwrap();
                // increase counter minted
                vcuminted = vcuminted + nvcu;
            }
            // mint the assets
            // store the last minting time in AssetsGeneratingVCUGenerated
            if AssetsGeneratingVCUGenerated::<T>::contains_key(&agv_account_id, &agv_id) {
                AssetsGeneratingVCUGenerated::<T>::take(&agv_account_id, &agv_id);
            }
            AssetsGeneratingVCUGenerated::<T>::insert(&agv_account_id, &agv_id, now);
            // generate event
            Self::deposit_event(Event::AssetsGeneratingVCUGenerated(agv_account_id, agv_id));
            // return
            Ok(().into())
        }

        // /// The owner of the “VCUs”  can decide anytime to “retire”, basically burning them.
        // ///
        // /// The dispatch origin for this call must be `Signed` from the owner of the VCU
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn retire_vcu(
        //     origin: OriginFor<T>,
        //     agv_account_id: T::AccountId,
        //     agv_id: u32,
        //     amount: u128,
        // ) -> DispatchResultWithPostInfo {
        //     // check for a signed transaction
        //     let sender = ensure_signed(origin)?;
        //     // check for the schedule of the assetid
        //     ensure!(
        //         AssetsGeneratingVCUSchedule::<T>::contains_key(&agv_account_id, &agv_id),
        //         Error::<T>::AssetGeneratedVCUScheduleNotFound
        //     );
        //     let content: Vec<u8> =
        //         AssetsGeneratingVCUSchedule::<T>::get(agv_account_id.clone(), &agv_id);
        //     let token_id = Self::json_get_value(content.clone(), "token_id".as_bytes().to_vec());
        //     let token_id = str::parse::<u32>(sp_std::str::from_utf8(&token_id).unwrap()).unwrap();
        //     // check for enough balance
        //     let amount_vcu = pallet_assets::Pallet::<T>::balance(token_id, sender.clone());
        //     ensure!(amount_vcu >= amount, Error::<T>::InsufficientVCUs);
        //
        //     // burn the tokens on assets pallet for the requested amount
        //     pallet_assets::Pallet::<T>::burn(
        //         RawOrigin::Signed(sender.clone()).into(),
        //         token_id,
        //         T::Lookup::unlookup(agv_account_id.clone()),
        //         amount,
        //     )?;
        //     // increase the counter of burned VCU for the signer of th transaction
        //     BurnedCounter::<T>::try_mutate(&sender, &token_id, |count| -> DispatchResult {
        //         *count += 1;
        //         Ok(())
        //     })?;
        //     //increase burned VCU for the AGV
        //     VCUsBurnedAccounts::<T>::try_mutate(
        //         &agv_account_id,
        //         &agv_id,
        //         |vcu| -> DispatchResult {
        //             let total_vcu = vcu.checked_add(amount).ok_or(Error::<T>::Overflow)?;
        //             *vcu = total_vcu;
        //             Ok(())
        //         },
        //     )?;
        //     // increase global counter burned VCU
        //     VCUsBurned::<T>::try_mutate(&token_id, |vcu| -> DispatchResult {
        //         let total_vcu = vcu.checked_add(amount).ok_or(Error::<T>::Overflow)?;
        //         *vcu = total_vcu;
        //         Ok(())
        //     })?;
        //     // Generate event
        //     Self::deposit_event(Event::VCUsBurnedAdded(agv_account_id, agv_id, token_id));
        //     // Return a successful DispatchResult
        //     Ok(().into())
        // }
        //
        // /// The VCUs may be generated from Oracle collecting data from off-chain. For example a Solar Panel field may have an Oracle collecting the
        // /// output power and generating the VCUs periodically on Chain. We have allowed the account of the Oracle to mint the VCU for his AGV.
        // ///
        // /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn create_oracle_account_minting_vcu(
        //     origin: OriginFor<T>,
        //     agv_account_id: T::AccountId,
        //     agv_id: u32,
        //     oracle_account_id: T::AccountId,
        //     token_id: u32,
        // ) -> DispatchResult {
        //     // check for SUDO or administrator accounts
        //     match ensure_root(origin.clone()) {
        //         Ok(()) => Ok(()),
        //         Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
        //             if AuthorizedAccountsAGV::<T>::contains_key(&o) {
        //                 Ok(())
        //             } else {
        //                 Err(e)
        //             }
        //         }),
        //     }?;
        //     // check if the AGV exists or not
        //     ensure!(
        //         AssetsGeneratingVCU::<T>::contains_key(&agv_account_id, &agv_id),
        //         Error::<T>::AssetGeneratingVCUNotFound
        //     );
        //     // check token id >10000
        //     ensure!(token_id >= 10000, Error::<T>::ReservedTokenId);
        //     // store the token if assigned for the Oracle
        //     if OraclesTokenMintingVCU::<T>::contains_key(agv_account_id.clone(), agv_id.clone()) {
        //         OraclesTokenMintingVCU::<T>::take(agv_account_id.clone(), agv_id.clone());
        //     }
        //     OraclesTokenMintingVCU::<T>::insert(
        //         agv_account_id.clone(),
        //         agv_id.clone(),
        //         token_id.clone(),
        //     );
        //     //store the oracle or replace if already present, we allow only one oracle for each AGV
        //     OraclesAccountMintingVCU::<T>::try_mutate_exists(
        //         agv_account_id.clone(),
        //         agv_id,
        //         |oracle| {
        //             *oracle = Some(oracle_account_id.clone());
        //             // Generate event
        //             Self::deposit_event(Event::OraclesAccountMintingVCUAdded(
        //                 agv_account_id,
        //                 agv_id,
        //                 oracle_account_id,
        //             ));
        //             // Return a successful DispatchResult
        //             Ok(())
        //         },
        //     )
        // }
        //
        // /// Removes Oracles Generating VCU from storage.
        // ///
        // /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn destroy_oracle_account_minting_vcu(
        //     origin: OriginFor<T>,
        //     agv_account_id: T::AccountId,
        //     agv_id: u32,
        // ) -> DispatchResult {
        //     //store the oracle or replace if already present, we allow only one oracle for each AGV
        //     match ensure_root(origin.clone()) {
        //         Ok(()) => Ok(()),
        //         Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
        //             if AuthorizedAccountsAGV::<T>::contains_key(&o) {
        //                 Ok(())
        //             } else {
        //                 Err(e)
        //             }
        //         }),
        //     }?;
        //     // check for Oracle presence on chain
        //     ensure!(
        //         OraclesAccountMintingVCU::<T>::contains_key(&agv_account_id, &agv_id),
        //         Error::<T>::OraclesAccountMintingVCUNotFound
        //     );
        //     // remove the Oracle Account
        //     OraclesAccountMintingVCU::<T>::remove(agv_account_id.clone(), &agv_id);
        //     // remove the Oracle Token Id
        //     OraclesTokenMintingVCU::<T>::remove(agv_account_id.clone(), &agv_id);
        //     // Generate event
        //     Self::deposit_event(Event::OraclesAccountMintingVCUDestroyed(
        //         agv_account_id,
        //         agv_id,
        //     ));
        //     // Return a successful DispatchResult
        //     Ok(())
        // }
        //
        // /// Mints Oracles Generating VCUs
        // ///
        // /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn mint_vcu_from_oracle(
        //     origin: OriginFor<T>,
        //     agv_account: Vec<u8>,
        //     amount_vcu: Balance,
        // ) -> DispatchResultWithPostInfo {
        //     let sender = ensure_signed(origin)?;
        //     // get avg_account_id  and agv_id from the vector agv_account
        //     let agv_id_vec: Vec<&str> = sp_std::str::from_utf8(&agv_account)
        //         .unwrap()
        //         .split('-')
        //         .collect();
        //     ensure!(agv_id_vec.len() >= 1, Error::<T>::InvalidAGVId);
        //     let (str_account_id, agv_id): (&str, u32) =
        //         (agv_id_vec[0], str::parse::<u32>(agv_id_vec[1]).unwrap());
        //     let agv_account_id =
        //         T::AccountId::decode(&mut &str_account_id.as_bytes().to_vec()[1..33])
        //             .map_err(|_| Error::<T>::InvalidJson)?;
        //     // check for Oracle presence on chain
        //     // check for matching signer with Oracle Account
        //     let oracle_account: T::AccountId =
        //         OraclesAccountMintingVCU::<T>::get(&agv_account_id, &agv_id)
        //             .ok_or(Error::<T>::OraclesAccountMintingVCUNotFound)?;
        //     ensure!(
        //         oracle_account == sender,
        //         Error::<T>::OracleAccountNotMatchingSigner
        //     );
        //     // check for Token id in Oracle configuration
        //     ensure!(
        //         OraclesTokenMintingVCU::<T>::contains_key(&agv_account_id, &agv_id),
        //         Error::<T>::OraclesTokenMintingVCUNotFound
        //     );
        //     // get the token id
        //     let token_id = OraclesTokenMintingVCU::<T>::get(&agv_account_id, &agv_id);
        //     // create token if it does not exist yet
        //     if let None = pallet_assets::Pallet::<T>::maybe_total_supply(token_id) {
        //         pallet_assets::Pallet::<T>::force_create(
        //             RawOrigin::Root.into(),
        //             token_id,
        //             T::Lookup::unlookup(oracle_account.clone()),
        //             false,
        //             One::one(),
        //         )?;
        //     }
        //     // check for existing shares
        //     ensure!(
        //         AssetsGeneratingVCUSharesMintedTotal::<T>::contains_key(
        //             agv_account_id.clone(),
        //             agv_id.clone()
        //         ),
        //         Error::<T>::NoAVGSharesNotFound
        //     );
        //     // read totals shares minted for the AGV
        //     let totalshares: u128 = AssetsGeneratingVCUSharesMintedTotal::<T>::get(
        //         agv_account_id.clone(),
        //         agv_id.clone(),
        //     )
        //     .into();
        //     // set the key of search
        //     let shareholdersc = AssetsGeneratingVCUShares::<T>::iter_prefix(agv_account.clone());
        //     let nshareholders = shareholdersc.count();
        //     // iter for the available shareholders
        //     let shareholders = AssetsGeneratingVCUShares::<T>::iter_prefix(agv_account);
        //     let mut vcuminted: u128 = 0;
        //     let mut nshareholdersprocessed: usize = 0;
        //     for numsh in shareholders {
        //         let shareholder = numsh.0;
        //         let numshares: u128 = numsh.1.into();
        //         //compute VCU for the shareholder
        //         let mut nvcu = amount_vcu / totalshares * numshares;
        //         // increase counter shareholders processed
        //         nshareholdersprocessed = nshareholdersprocessed + 1;
        //         // manage overflow for rounding
        //         if nshareholdersprocessed == nshareholders && vcuminted + nvcu > amount_vcu {
        //             nvcu = amount_vcu - vcuminted;
        //         }
        //         // manage underflow for rounding
        //         if nshareholdersprocessed == nshareholders && vcuminted + nvcu < amount_vcu {
        //             nvcu = amount_vcu - vcuminted;
        //         }
        //         //mint the vcu in proportion to the shares owned
        //         pallet_assets::Pallet::<T>::mint(
        //             RawOrigin::Signed(oracle_account.clone()).into(),
        //             token_id,
        //             T::Lookup::unlookup(shareholder.clone()),
        //             nvcu,
        //         )
        //         .unwrap();
        //         // increase counter minted
        //         vcuminted = vcuminted + nvcu;
        //     }
        //     // here the total vcu minted should be exactly the amount received as parameter.
        //     // generate event
        //     Self::deposit_event(Event::OracleAccountVCUMinted(
        //         agv_account_id,
        //         agv_id,
        //         oracle_account,
        //     ));
        //     Ok(().into())
        // }
        //
        // /// To store a "bundle" of AGV that has the constraint of using the same "asset id"
        // /// but potentially different schedules or Oracle for the generation of the VCU.
        // ///
        // /// example: {"description":"xxxxxxx","agvs":[{"accountid","xxxxxxx","id":xx},{..}],assetid:xx}
        // /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn create_bundle_agv(
        //     origin: OriginFor<T>,
        //     bundle_id: u32,
        //     info: Vec<u8>,
        // ) -> DispatchResult {
        //     // check for SUDO or administrator user
        //     match ensure_root(origin.clone()) {
        //         Ok(()) => Ok(()),
        //         Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
        //             if AuthorizedAccountsAGV::<T>::contains_key(&o) {
        //                 Ok(())
        //             } else {
        //                 Err(e)
        //             }
        //         }),
        //     }?;
        //
        //     //check accounts json length
        //     ensure!(
        //         info.len() > 12,
        //         Error::<T>::BundleAssetsGeneratingVCUJsonTooShort
        //     );
        //     ensure!(
        //         info.len() < 8192,
        //         Error::<T>::BundleAssetsGeneratingVCUJsonTooLong
        //     );
        //
        //     // check json validity
        //     let js = info.clone();
        //     ensure!(Self::json_check_validity(js), Error::<T>::InvalidJson);
        //     // check for description validity
        //     let description = Self::json_get_value(info.clone(), "description".as_bytes().to_vec());
        //     ensure!(
        //         !description.is_empty() && description.len() <= 64,
        //         Error::<T>::InvalidDescription
        //     );
        //     // check for asset id
        //     let asset_id = Self::json_get_value(info.clone(), "assetid".as_bytes().to_vec());
        //     ensure!(asset_id.len() > 0, Error::<T>::AssetDoesNotExist);
        //     let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();
        //     // check whether asset exists or not
        //     ensure!(
        //         pallet_assets::Pallet::<T>::maybe_total_supply(asset_id).is_some(),
        //         Error::<T>::AssetDoesNotExist
        //     );
        //     // check the validity of the AGV in the array
        //     let agvs = Self::json_get_complexarray(info.clone(), "agvs".as_bytes().to_vec());
        //     let mut x = 0;
        //     if agvs.len() > 2 {
        //         loop {
        //             let w = Self::json_get_recordvalue(agvs.clone(), x);
        //             if w.is_empty() {
        //                 break;
        //             }
        //             let account_id =
        //                 Self::json_get_value(w.clone(), "accountid".as_bytes().to_vec());
        //             let id = Self::json_get_value(w.clone(), "id".as_bytes().to_vec());
        //
        //             let account_id = T::AccountId::decode(&mut &account_id[1..33])
        //                 .map_err(|_| Error::<T>::InvalidJson)?;
        //             let id = str::parse::<u32>(sp_std::str::from_utf8(&id).unwrap()).unwrap();
        //             // check whether asset generated VCU exists or not
        //             ensure!(
        //                 AssetsGeneratingVCU::<T>::contains_key(&account_id, &id),
        //                 Error::<T>::AssetGeneratingVCUNotFound
        //             );
        //             x += 1;
        //         }
        //     }
        //     ensure!(x > 0, Error::<T>::InvalidAGVs);
        //     BundleAssetsGeneratingVCU::<T>::insert(&bundle_id, &info);
        //     Self::deposit_event(Event::AddedBundleAssetsGeneratingVCU(bundle_id));
        //
        //     Ok(())
        // }

        /// Destroys an AGV bundle from storage.
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_bundle_agv(origin: OriginFor<T>, bundle_id: u32) -> DispatchResult {
            // check for SUDO or administrator user
            match ensure_root(origin.clone()) {
                Ok(()) => Ok(()),
                Err(e) => ensure_signed(origin).and_then(|o: T::AccountId| {
                    if AuthorizedAccountsAGV::<T>::contains_key(&o) {
                        Ok(())
                    } else {
                        Err(e)
                    }
                }),
            }?;
            // check if the bundle is on chain
            ensure!(
                BundleAssetsGeneratingVCU::<T>::contains_key(&bundle_id),
                Error::<T>::BundleDoesNotExist
            );
            // remove the bundle from the chain
            BundleAssetsGeneratingVCU::<T>::remove(bundle_id);
            // Generate event
            Self::deposit_event(Event::DestroyedBundleAssetsGeneratingVCU(bundle_id));
            // Return a successful DispatchResult
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        // function to validate a json string for no/std. It does not allocate of memory
        fn json_check_validity(j: Vec<u8>) -> bool {
            // minimum lenght of 2
            if j.len() < 2 {
                return false;
            }
            // checks star/end with {}
            if *j.get(0).unwrap() == b'{' && *j.last().unwrap() != b'}' {
                return false;
            }
            // checks start/end with []
            if *j.get(0).unwrap() == b'[' && *j.last().unwrap() != b']' {
                return false;
            }
            // check that the start is { or [
            if *j.get(0).unwrap() != b'{' && *j.get(0).unwrap() != b'[' {
                return false;
            }
            //checks that end is } or ]
            if *j.last().unwrap() != b'}' && *j.last().unwrap() != b']' {
                return false;
            }
            //checks " opening/closing and : as separator between name and values
            let mut s: bool = true;
            let mut d: bool = true;
            let mut pg: bool = true;
            let mut ps: bool = true;
            let mut bp = b' ';
            for b in j {
                if b == b'[' && s {
                    ps = false;
                }
                if b == b']' && s && !ps {
                    ps = true;
                }

                if b == b'{' && s {
                    pg = false;
                }
                if b == b'}' && s && !pg {
                    pg = true;
                }

                if b == b'"' && s && bp != b'\\' {
                    s = false;
                    bp = b;
                    d = false;
                    continue;
                }
                if b == b':' && s {
                    d = true;
                    bp = b;
                    continue;
                }
                if b == b'"' && !s && bp != b'\\' {
                    s = true;
                    bp = b;
                    d = true;
                    continue;
                }
                bp = b;
            }

            //fields are not closed properly
            if !s {
                return false;
            }
            //fields are not closed properly
            if !d {
                return false;
            }
            //fields are not closed properly
            if !ps {
                return false;
            }
            //fields are not closed properly
            if !pg {
                return false;
            }
            // every ok returns true
            true
        }

        // function to get value of a field for Substrate runtime (no std library and no variable allocation)
        fn json_get_value(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
            let mut result = Vec::new();
            let mut k = Vec::new();
            let keyl = key.len();
            let jl = j.len();
            k.push(b'"');
            for xk in 0..keyl {
                k.push(*key.get(xk).unwrap());
            }
            k.push(b'"');
            k.push(b':');
            let kl = k.len();
            for x in 0..jl {
                let mut m = 0;
                if x + kl > jl {
                    break;
                }
                for (xx, i) in (x..x + kl).enumerate() {
                    if *j.get(i).unwrap() == *k.get(xx).unwrap() {
                        m += 1;
                    }
                }
                if m == kl {
                    let mut lb = b' ';
                    let mut op = true;
                    let mut os = true;
                    for i in x + kl..jl - 1 {
                        if *j.get(i).unwrap() == b'[' && op && os {
                            os = false;
                        }
                        if *j.get(i).unwrap() == b'}' && op && !os {
                            os = true;
                        }
                        if *j.get(i).unwrap() == b':' && op {
                            continue;
                        }
                        if *j.get(i).unwrap() == b'"' && op && lb != b'\\' {
                            op = false;
                            continue;
                        }
                        if *j.get(i).unwrap() == b'"' && !op && lb != b'\\' {
                            break;
                        }
                        if *j.get(i).unwrap() == b'}' && op {
                            break;
                        }
                        if *j.get(i).unwrap() == b']' && op {
                            break;
                        }
                        if *j.get(i).unwrap() == b',' && op && os {
                            break;
                        }
                        result.push(*j.get(i).unwrap());
                        lb = *j.get(i).unwrap();
                    }
                    break;
                }
            }
            result
        }

        fn create_json_string(inputs: Vec<(&str, &mut Vec<u8>)>) -> Vec<u8> {
            let mut v: Vec<u8> = vec![b'{'];
            let mut flag = false;

            for (arg, val) in inputs {
                if flag {
                    v.push(b',');
                }
                v.push(b'"');
                for i in arg.as_bytes().to_vec().iter() {
                    v.push(*i);
                }
                v.push(b'"');
                v.push(b':');
                v.append(val);
                flag = true;
            }
            v.push(b'}');
            v
        }

        // function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
        fn json_get_recordvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
            let mut result = Vec::new();
            let mut op = true;
            let mut cn = 0;
            let mut lb = b' ';
            for b in ar {
                if b == b',' && op {
                    cn += 1;
                    continue;
                }
                if b == b'[' && op && lb != b'\\' {
                    continue;
                }
                if b == b']' && op && lb != b'\\' {
                    continue;
                }
                if b == b'{' && op && lb != b'\\' {
                    op = false;
                }
                if b == b'}' && !op && lb != b'\\' {
                    op = true;
                }
                // field found
                if cn == p {
                    result.push(b);
                }
                lb = b;
            }
            result
        }

        fn json_get_complexarray(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
            let mut result = Vec::new();
            let mut k = Vec::new();
            let keyl = key.len();
            let jl = j.len();
            k.push(b'"');
            for xk in 0..keyl {
                k.push(*key.get(xk).unwrap());
            }
            k.push(b'"');
            k.push(b':');
            let kl = k.len();
            for x in 0..jl {
                let mut m = 0;
                if x + kl > jl {
                    break;
                }
                for (xx, i) in (x..x + kl).enumerate() {
                    if *j.get(i).unwrap() == *k.get(xx).unwrap() {
                        m += 1;
                    }
                }
                if m == kl {
                    let mut os = true;
                    for i in x + kl..jl - 1 {
                        if *j.get(i).unwrap() == b'[' && os {
                            os = false;
                        }
                        result.push(*j.get(i).unwrap());
                        if *j.get(i).unwrap() == b']' && !os {
                            break;
                        }
                    }
                    break;
                }
            }
            result
        }
    }
}
