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

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::Encode;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use frame_support::{
    codec::Decode,
    dispatch::DispatchResult,
    ensure,
    traits::{tokens::fungibles::Mutate, Get},
};
use frame_system::ensure_root;
use frame_system::ensure_signed;
use frame_system::RawOrigin;
use primitives::Balance;
use sp_runtime::traits::StaticLookup;
use sp_std::cmp::Ordering;
use sp_std::str::FromStr;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_settings)]
    pub type Settings<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_transaction_mint_tracker)]
    pub type TransactionMintTracker<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_mint_request)]
    pub type MintRequest<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, T::Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_mint_count)]
    pub type MintCounter<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_mint_confirmation)]
    pub type MintConfirmation<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, bool>;

    #[pallet::storage]
    #[pallet::getter(fn get_transaction_burn_tracker)]
    pub type TransactionBurnTracker<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_burn_request)]
    pub type BurnRequest<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, T::Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_burn_count)]
    pub type BurnCounter<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_burn_confirmation)]
    pub type BurnConfirmation<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, bool>;

    #[pallet::storage]
    #[pallet::getter(fn lockdown)]
    pub type Lockdown<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub lockdown_status: bool,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                lockdown_status: false,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            <Lockdown<T>>::put(&self.lockdown_status);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New setting has been created.
        SettingsCreated(Vec<u8>, Vec<u8>),
        /// setting has been destroyed.
        SettingsDestroyed(Vec<u8>),
        /// Minted
        Minted(T::AccountId, u32, T::AccountId, Balance, Vec<u8>, Vec<u8>),
        /// Minting Request added to the queue
        MintQueued(T::AccountId, u32, T::AccountId, Balance, Vec<u8>, Vec<u8>),
        /// Already minted the same transaction
        AlreadyMinted(T::AccountId, u32, T::AccountId, Balance),
        /// Burned
        Burned(T::AccountId, u32, T::AccountId, Balance, Vec<u8>),
        /// Burning Request added to the queue
        BurnQueued(T::AccountId, u32, T::AccountId, Balance, Vec<u8>, Vec<u8>),
        /// Already burned the same transaction
        AlreadyBurned(T::AccountId, u32, T::AccountId, Balance),
        /// User Request
        Request(T::AccountId, Vec<u8>, Vec<u8>, Balance),
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
        /// Invalid ChainId
        InvalidChainId,
        /// Invalid Description
        InvalidDescription,
        /// Address is Empty
        EmptyAddress,
        /// Asset does not exist,
        AssetDoesNotExist,
        /// Internal Keepers NotConfigured
        InternalKeepersNotConfigured,
        /// External Keepers NotConfigured
        ExternalKeepersNotConfigured,
        /// Internal Watchdogs NotConfigured
        InternalWatchdogsNotConfigured,
        /// External Watchdogs NotConfigured
        ExternalWatchdogsNotConfigured,
        /// Internal Watchcats NotConfigured
        InternalWatchcatsNotConfigured,
        /// External Watchcats NotConfigured
        ExternalWatchcatsNotConfigured,
        /// Internal Threshold NotFound
        InternalThresholdNotFound,
        /// External Threshold NotFound
        ExternalThresholdNotFound,
        /// The key cannot be shorter of 3 bytes
        SettingsKeyTooShort,
        /// The key cannot be longer of 8 bytes
        SettingsKeyTooLong,
        /// The internal Threshold should be > 0 and < 99
        InternalThresholdInvalid,
        /// The external Threshold should be > 0 and < 99
        ExternalThresholdInvalid,
        /// The internal keeper account is wrong
        InternalKeepersAccountIsWrong,
        /// The external keeper account is wrong
        ExternalKeepersAccountIsWrong,
        /// The number of internal keepers is not matching the threshold
        InternalKeepersNotMatchingThreshold,
        /// The number of external keepers is not matching the threshold
        ExternalKeepersNotMatchingThreshold,
        /// The internal Whatchdog account is wrong
        InternalWhatchDogsAccountIsWrong,
        /// The external Whatchdog account is wrong
        ExternalWatchddogsAccountIsWrong,
        /// The internal Watchcat account is wrong
        InternalWhatchCatsAccountIsWrong,
        /// The external Watchcat account is wrong
        ExternalWhatchCatsAccountIsWrong,
        /// SignerAlreadyConfirmed
        SignerAlreadyConfirmed,
        /// Signer is not a keeper account for the assetid
        SignerIsNotKeeper,
        /// Amount minting is not matching the first minting request. It can be a serious situation.
        AmountMintingIsNotMatching,
        /// Minting has been already confirmed and processed
        MintingAlreadyConfirmed,
        /// Burning has been already confirmed and processed
        BurningAlreadyConfirmed,
        /// Amount burning is not matching the first burning request. It can be a serious situation.
        AmountBurningIsNotMatching,
        /// Signer is not Authorized
        SignerIsNotAuthorized,
        /// Not allowed due to lockdown mode
        NotAllowedSystemLockdown,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_settings(
            origin: OriginFor<T>,
            key: Vec<u8>,
            data: Vec<u8>,
        ) -> DispatchResult {
            // check access for Sudo
            ensure_root(origin)?;

            // check if lockdownmode is off
            ensure!(!Lockdown::<T>::get(), Error::<T>::NotAllowedSystemLockdown);

            //check data json length
            ensure!(data.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(data.len() < 8192, Error::<T>::SettingsJsonTooLong);

            // check json validity
            let js = data.clone();
            ensure!(Self::json_check_validity(js), Error::<T>::InvalidJson);

            // check whether setting key for minx/max length
            ensure!(key.len() >= 3, Error::<T>::SettingsKeyTooShort);
            ensure!(key.len() <= 8, Error::<T>::SettingsKeyTooLong);

            // check whether setting key already exists
            ensure!(
                !Settings::<T>::contains_key(&key),
                Error::<T>::SettingsKeyExists
            );

            let chain_id = Self::json_get_value(data.clone(), "chainid".as_bytes().to_vec());
            ensure!(!chain_id.is_empty(), Error::<T>::InvalidChainId);
            ensure!(
                chain_id=="1".as_bytes().to_vec() ||     // Ethereum
                    chain_id=="2".as_bytes().to_vec() ||     // Binance
                    chain_id=="3".as_bytes().to_vec(), // Solana
                Error::<T>::InvalidChainId
            );
            // check for description not empty and <64 bytes
            let description = Self::json_get_value(data.clone(), "description".as_bytes().to_vec());
            ensure!(
                !description.is_empty() && description.len() <= 64,
                Error::<T>::InvalidDescription
            );
            // check for address not empty
            let address = Self::json_get_value(data.clone(), "address".as_bytes().to_vec());
            ensure!(!address.is_empty(), Error::<T>::EmptyAddress);
            // check for asset id validity
            let asset_id = Self::json_get_value(data.clone(), "assetid".as_bytes().to_vec());
            let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();
            // check whether asset exists or not
            ensure!(
                pallet_assets::Pallet::<T>::maybe_total_supply(asset_id).is_some(),
                Error::<T>::AssetDoesNotExist
            );
            //check internal threshold
            let internal_threshold =
                Self::json_get_value(data.clone(), "internalthreshold".as_bytes().to_vec());
            ensure!(
                !internal_threshold.is_empty(),
                Error::<T>::InternalThresholdNotFound
            );
            let itn = Self::vecu8_to_u32(internal_threshold);
            ensure!(!itn > 0 && itn <= 99, Error::<T>::InternalThresholdInvalid);
            //check external threshold
            let external_threshold =
                Self::json_get_value(data.clone(), "externathreshold".as_bytes().to_vec());
            ensure!(
                !external_threshold.is_empty(),
                Error::<T>::ExternalThresholdNotFound
            );
            let etn = Self::vecu8_to_u32(external_threshold);
            ensure!(!etn > 0 && etn <= 99, Error::<T>::ExternalThresholdInvalid);
            //check internal keepers accounts
            let internalkeepers =
                Self::json_get_complexarray(data.clone(), "internalkeepers".as_bytes().to_vec());
            if internalkeepers.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(internalkeepers.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() == 48, Error::<T>::InternalKeepersAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::InternalKeepersNotConfigured);
                ensure!(
                    x as u32 == itn,
                    Error::<T>::InternalKeepersNotMatchingThreshold
                );
            }
            //check external keepers accounts
            let externalkeepers =
                Self::json_get_complexarray(data.clone(), "externalkeepers".as_bytes().to_vec());
            if externalkeepers.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(externalkeepers.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() >= 32, Error::<T>::ExternalKeepersAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::ExternalKeepersNotConfigured);
                ensure!(
                    x as u32 == etn,
                    Error::<T>::ExternalKeepersNotMatchingThreshold
                );
            }
            //check internal watchdogs accounts
            let internalwatchdogs =
                Self::json_get_complexarray(data.clone(), "internalwatchdogs".as_bytes().to_vec());
            if internalwatchdogs.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(internalwatchdogs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() == 48, Error::<T>::InternalWhatchDogsAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::InternalWatchdogsNotConfigured);
            }
            //check external watchdogs accounts
            let externalwatchdogs =
                Self::json_get_complexarray(data.clone(), "externalwatchdogs".as_bytes().to_vec());
            if externalwatchdogs.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(externalwatchdogs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() >= 32, Error::<T>::ExternalWatchddogsAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::ExternalWatchdogsNotConfigured);
            }
            //check internal watchcats accounts
            let internalwatchcats =
                Self::json_get_complexarray(data.clone(), "internalwatchcats".as_bytes().to_vec());
            if internalwatchcats.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(internalwatchcats.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() == 48, Error::<T>::InternalWhatchCatsAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::InternalWatchcatsNotConfigured);
            }
            //check external watchcats accounts
            let externalwatchcats =
                Self::json_get_complexarray(data.clone(), "externalwatchcats".as_bytes().to_vec());
            if externalwatchcats.len() >= 2 {
                let mut x = 0;
                loop {
                    let w = Self::json_get_recordvalue(externalwatchcats.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() >= 32, Error::<T>::ExternalWhatchCatsAccountIsWrong);
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::ExternalWatchcatsNotConfigured);
            }

            Settings::<T>::insert(key.clone(), data.clone());
            // Generate event
            Self::deposit_event(Event::SettingsCreated(key, data));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy setting with the given key
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_settings(origin: OriginFor<T>, key: Vec<u8>) -> DispatchResult {
            // allow access only to SUDO
            ensure_root(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::<T>::get(), Error::<T>::NotAllowedSystemLockdown);
            // check whether setting key exists or not
            ensure!(
                Settings::<T>::contains_key(&key),
                Error::<T>::SettingsKeyNotFound
            );
            Settings::<T>::remove(key.clone());
            // Generate event
            Self::deposit_event(Event::SettingsDestroyed(key));
            // Return a successful DispatchResult
            Ok(())
        }

        // function to Mint an assetid
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn mint(
            origin: OriginFor<T>,
            token: Vec<u8>,
            recipient: T::AccountId,
            transaction_id: Vec<u8>,
            amount: Balance,
        ) -> DispatchResultWithPostInfo {
            // check for a signed transactions
            let signer = ensure_signed(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::<T>::get(), Error::<T>::NotAllowedSystemLockdown);
            // check for the token configuration in settings
            ensure!(
                Settings::<T>::contains_key(&token),
                Error::<T>::SettingsKeyNotFound
            );
            let content: Vec<u8> = Settings::<T>::get(&token).unwrap();
            let asset_idv = Self::json_get_value(content.clone(), "assetid".as_bytes().to_vec());
            let asset_id = Self::vecu8_to_u32(asset_idv);
            let internalthresholdv =
                Self::json_get_value(content.clone(), "internalthreshold".as_bytes().to_vec());
            let internalthreshold = Self::vecu8_to_u32(internalthresholdv);

            // check for authorized signer
            let mut flag = 0;
            let internal_keepers =
                Self::json_get_complexarray(content, "internalkeepers".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let internal_keeper = Self::json_get_arrayvalue(internal_keepers.clone(), x);
                if internal_keeper.is_empty() {
                    break;
                }
                let internal_keepervec = bs58::decode(internal_keeper).into_vec().unwrap();
                let accountid_internal_keepers =
                    T::AccountId::decode(&mut &internal_keepervec[1..33])
                        .map_err(|_| Error::<T>::InvalidJson)?;
                if accountid_internal_keepers == signer {
                    flag = 1;
                }
                x += 1;
            }

            ensure!(flag == 1, Error::<T>::SignerIsNotKeeper);

            // check for duplicated minting for the same transaction/signer
            ensure!(
                !TransactionMintTracker::<T>::contains_key(transaction_id.clone(), &signer),
                Error::<T>::SignerAlreadyConfirmed
            );
            // store minting tracker
            TransactionMintTracker::<T>::insert(transaction_id.clone(), signer.clone(), asset_id);

            // storing the minting request if it's not already present
            let key = &mut token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            if !MintRequest::<T>::contains_key(key.clone()) {
                MintRequest::<T>::insert(key.clone(), amount);
            } else {
                // when already present
                // checking that the amount to mint is the same of the previous one, if does not, we have an Oracle hacked or not updated
                let am = MintRequest::<T>::get(key.clone());
                ensure!(am == amount, Error::<T>::AmountMintingIsNotMatching);
            }

            // update the counter for the minting requests of the transaction
            let mut key = token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            MintCounter::<T>::try_mutate(&key, |count| -> DispatchResult {
                *count += 1;
                Ok(())
            })?;
            // get the number of minting requests
            let nmr = MintCounter::<T>::get(&key);
            // thresold not reached
            match nmr.cmp(&internalthreshold) {
                Ordering::Less => Self::deposit_event(Event::MintQueued(
                    signer,
                    asset_id,
                    recipient,
                    amount,
                    transaction_id.clone(),
                    token.clone(),
                )),
                Ordering::Greater => {
                    Self::deposit_event(Event::AlreadyMinted(signer, asset_id, recipient, amount))
                }
                Ordering::Equal => {
                    // check it's not already confirmed
                    ensure!(
                        !MintConfirmation::<T>::contains_key(key.clone()),
                        Error::<T>::MintingAlreadyConfirmed
                    );
                    // store the minting confirmation
                    MintConfirmation::<T>::insert(key, true);
                    //minting of the asset_id matching the token configured
                    <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
                        asset_id,
                        &recipient.clone(),
                        amount,
                    )?;
                    // generate an event
                    Self::deposit_event(Event::Minted(
                        signer,
                        asset_id,
                        recipient,
                        amount,
                        transaction_id.clone(),
                        token.clone(),
                    ))
                }
            };

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn request(
            origin: OriginFor<T>,
            token: Vec<u8>,
            destination: Vec<u8>,
            amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let signer = ensure_signed(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::<T>::get(), Error::<T>::NotAllowedSystemLockdown);
            ensure!(
                Settings::<T>::contains_key(&token),
                Error::<T>::SettingsKeyNotFound
            );
            // let content: Vec<u8> = Settings::<T>::get(&token).unwrap();
            // let asset_idv = Self::json_get_value(content.clone(),"assetid".as_bytes().to_vec());
            // let asset_id = Self::vecu8_to_u32(asset_idv);
            // generate an event
            Self::deposit_event(Event::Request(signer, token, destination.clone(), amount));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn burn(
            origin: OriginFor<T>,
            token: Vec<u8>,
            recipient: T::AccountId,
            transaction_id: Vec<u8>,
            amount: Balance,
        ) -> DispatchResultWithPostInfo {
            let signer = ensure_signed(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::<T>::get(), Error::<T>::NotAllowedSystemLockdown);
            ensure!(
                Settings::<T>::contains_key(&token),
                Error::<T>::SettingsKeyNotFound
            );
            let content: Vec<u8> = Settings::<T>::get(&token).unwrap();
            let asset_idv = Self::json_get_value(content.clone(), "assetid".as_bytes().to_vec());
            let asset_id = Self::vecu8_to_u32(asset_idv);
            let internalthresholdv =
                Self::json_get_value(content.clone(), "internalthreshold".as_bytes().to_vec());
            let internalthreshold = Self::vecu8_to_u32(internalthresholdv);

            // check for authorized signer
            let mut flag = 0;
            let internal_keepers =
                Self::json_get_complexarray(content, "internalkeepers".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let internal_keeper = Self::json_get_arrayvalue(internal_keepers.clone(), x);
                if internal_keeper.is_empty() {
                    break;
                }
                let internal_keepervec = bs58::decode(internal_keeper).into_vec().unwrap();
                let accountid_internal_keepers =
                    T::AccountId::decode(&mut &internal_keepervec[1..33])
                        .map_err(|_| Error::<T>::InvalidJson)?;
                if accountid_internal_keepers == signer {
                    flag = 1;
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotKeeper);

            // check for duplicated burning for the same transaction/signer
            ensure!(
                !TransactionBurnTracker::<T>::contains_key(transaction_id.clone(), &signer),
                Error::<T>::SignerAlreadyConfirmed
            );
            // store burning tracker
            TransactionBurnTracker::<T>::insert(transaction_id.clone(), signer.clone(), asset_id);

            // storing the burning request if it's not already present
            let key = &mut token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            if !MintRequest::<T>::contains_key(key.clone()) {
                MintRequest::<T>::insert(key.clone(), amount);
            } else {
                // when already present
                // checking that the amount to burn is the same of the previous one, if does not, we have an Oracle hacked or not updated
                let am = BurnRequest::<T>::get(key.clone());
                ensure!(am == amount, Error::<T>::AmountBurningIsNotMatching);
            }

            // update the counter for the minting requests of the transaction
            let mut key = token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            BurnCounter::<T>::try_mutate(&key, |count| -> DispatchResult {
                *count += 1;
                Ok(())
            })?;

            // get the number of burning requests
            let nmr = BurnCounter::<T>::get(&key);
            match nmr.cmp(&internalthreshold) {
                Ordering::Less => Self::deposit_event(Event::BurnQueued(
                    signer,
                    asset_id,
                    recipient,
                    amount,
                    transaction_id.clone(),
                    token.clone(),
                )),
                Ordering::Greater => {
                    Self::deposit_event(Event::AlreadyBurned(signer, asset_id, recipient, amount))
                }
                Ordering::Equal => {
                    // check it's not already confirmed
                    ensure!(
                        !BurnConfirmation::<T>::contains_key(key.clone()),
                        Error::<T>::BurningAlreadyConfirmed
                    );
                    // store the BurnConfirmation
                    BurnConfirmation::<T>::insert(key, true);
                    //burning of the asset_id matching the token configured
                    <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
                        asset_id,
                        &recipient.clone(),
                        amount,
                    )?;
                    // generate an event
                    Self::deposit_event(Event::Burned(
                        signer,
                        asset_id,
                        recipient,
                        amount,
                        transaction_id.clone(),
                    ));
                }
            };

            Ok(().into())
        }
        // function to set a system lockdown, watchdogs and watchcats account can set it
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn set_lockdown(origin: OriginFor<T>, token: Vec<u8>) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            ensure!(
                Settings::<T>::contains_key(&token),
                Error::<T>::SettingsKeyNotFound
            );
            let content: Vec<u8> = Settings::<T>::get(&token).unwrap();
            let mut flag = 0;
            let internal_watch_dogs = Self::json_get_complexarray(
                content.clone(),
                "internalwatchdogs".as_bytes().to_vec(),
            );
            let mut x = 0;
            loop {
                let internal_watch_dogs = Self::json_get_arrayvalue(internal_watch_dogs.clone(), x);
                if internal_watch_dogs.is_empty() {
                    break;
                }
                let internal_watch_dogsvec = bs58::decode(internal_watch_dogs).into_vec().unwrap();
                let accountid_internal_watch_dogs =
                    T::AccountId::decode(&mut &internal_watch_dogsvec[1..33])
                        .map_err(|_| Error::<T>::InvalidJson)?;
                if accountid_internal_watch_dogs == signer {
                    flag = 1;
                }
                x += 1;
            }
            let internal_watch_cats =
                Self::json_get_complexarray(content, "internalwatchcats".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let internal_watch_cats = Self::json_get_arrayvalue(internal_watch_cats.clone(), x);
                if internal_watch_cats.is_empty() {
                    break;
                }
                let internal_watch_catsvec = bs58::decode(internal_watch_cats).into_vec().unwrap();
                let accountid_internal_watch_cats =
                    T::AccountId::decode(&mut &internal_watch_catsvec[1..33])
                        .map_err(|_| Error::<T>::InvalidJson)?;
                if accountid_internal_watch_cats == signer {
                    flag = 1;
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorized);
            Lockdown::<T>::put(true);
            Ok(())
        }
        // function to remove the lockdown, it can be executed only from the superuser
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn set_unlockdown(origin: OriginFor<T>) -> DispatchResult {
            // check access for Sudo
            ensure_root(origin)?;
            Lockdown::<T>::put(false);
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
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
            if cn == p && b != b'"' {
                result.push(b);
            }
            lb = b;
        }
        result
    }

    // function to get value of a field with a complex array like [{....},{.....}] for Substrate runtime (no std library and no variable allocation)
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

    // function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the records is not present
    fn json_get_arrayvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
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
            if b == b'"' && op && lb != b'\\' {
                continue;
            }
            if b == b'"' && op && lb != b'\\' {
                op = false;
            }
            if b == b'"' && !op && lb != b'\\' {
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

    // function to convert vec<u8> to u32
    fn vecu8_to_u32(v: Vec<u8>) -> u32 {
        let vslice = v.as_slice();
        let vstr = sp_std::str::from_utf8(vslice).unwrap_or("0");
        let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
        vvalue
    }
}
