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
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

extern crate alloc;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    traits::Get, codec::Decode
};
use frame_system::ensure_root;
use sp_std::vec::Vec;
use pallet_assets::Asset;
use frame_system::ensure_signed;
use primitives::Balance;
use codec::Encode;
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use core::str;
use core::str::FromStr;
use sp_std::cmp::Ordering;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128>{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {

    trait Store for Module<T: Config> as VCUModule {
        /// Settings configuration.
        Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
        TransactionMintTracker get(fn get_transaction_mint_tracker): double_map hasher(blake2_128_concat) Vec<u8>,hasher(blake2_128_concat) T::AccountId => u32;
        MintRequest get(fn get_mint_request): map hasher(blake2_128_concat) Vec<u8> => Balance;
        MintCounter get(fn get_mint_count): map hasher(blake2_128_concat) Vec<u8> => u32;
        MintConfirmation get(fn get_mint_confirmation): map hasher(blake2_128_concat) Vec<u8> => bool;
        TransactionBurnTracker get(fn get_transaction_burn_tracker): double_map hasher(blake2_128_concat) Vec<u8>,hasher(blake2_128_concat) T::AccountId => u32;
        BurnRequest get(fn get_burn_request): map hasher(blake2_128_concat) Vec<u8> => Balance;
        BurnCounter get(fn get_burn_count): map hasher(blake2_128_concat) Vec<u8> => u32;
        BurnConfirmation get(fn get_burn_confirmation): map hasher(blake2_128_concat) Vec<u8> => bool;
        Lockdown get(fn lockdown) build(|config: &GenesisConfig| config.lockdown_status): bool;
    }
    add_extra_genesis {
		config(lockdown_status): bool;
	}
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// New setting has been created.
        SettingsCreated(Vec<u8>, Vec<u8>),
        /// setting has been destroyed.
        SettingsDestroyed(Vec<u8>),
        /// Minted
        Minted(AccountId, u32, AccountId, Balance),
        /// Minting Request added to the queue
        MintQueued(AccountId, u32, AccountId, Balance, Vec<u8>),
        /// Already minted the same transaction
        AlreadyMinted(AccountId, u32, AccountId, Balance),
        /// Burned
        Burned(AccountId, u32, AccountId, Balance),
        /// Burning Request added to the queue
        BurnQueued(AccountId, u32, AccountId, Balance, Vec<u8>),
        /// Already burned the same transaction
        AlreadyBurned(AccountId, u32, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
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
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Create new setting
        ///
        /// key: XXXXX (the token symbol)
        /// data:{
        /// 	"chainid": xx, // the chain id to identify the blockchain
        /// 	"description": "xxxxxxxxxx", // description of the blockchain
        /// 	"address":"0x......", // address of the smart contract on the external blockchain
        /// 	"assetid": xx // assetid on Bitgreen Blockchain
        /// 	"internalkeepers":[".....",".....",".....",], // accounts of the "keepers" delegate to sign the transactions
        /// 	"internalthreshold",x, // minimum number of signer to confirm a transaction for Bitgreen blockchain
        /// 	"externalkeepers":["...",".....",".....",], // accounts of the "keepers" delegate to sign the transactions on the other blockchain
        /// 	"externathreshold",x, // minimum number of signer to confirm a transaction on the external blockchain
        /// 	"internalwatchdogs":[".....",".....",".....",], // accounts of the watchdogs account that are enable to fire a lockdown on Bitgreen blockchain
        /// 	"externalwatchdogs":[".....",".....",".....",], // accounts of the watchdogs account that are enable to fire a lockdown on the external blockchain
        /// 	"internalwatchcats":[".....",".....",".....",], // accounts of the watchcats account that are enable to fire a lockdown on Bitgreen blockchain before the confirmation
        /// 	"externalwatchcats":[".....",".....",".....",], // accounts of the watchcats account that are enable to fire a lockdown on the external blockchain before the confirmation
        /// }
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn create_settings(origin, key: Vec<u8>, data: Vec<u8>) -> DispatchResult {
            // check access for Sudo
            ensure_root(origin)?;

            // check if lockdownmode is off
            ensure!(!Lockdown::get(), Error::<T>::NotAllowedSystemLockdown);

            //check data json length
            ensure!(data.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(data.len() < 8192, Error::<T>::SettingsJsonTooLong);

            // check json validity
            let js=data.clone();
            ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);

            // check whether setting key for minx/max length
            ensure!(key.len() >= 3, Error::<T>::SettingsKeyTooShort);
            ensure!(key.len() <=8, Error::<T>::SettingsKeyTooLong);

            // check whether setting key already exists
            ensure!(!Settings::contains_key(&key), Error::<T>::SettingsKeyExists);

            let chain_id = Self::json_get_value(data.clone(),"chainid".as_bytes().to_vec());
            ensure!(!chain_id.is_empty() , Error::<T>::InvalidChainId);
            ensure!(chain_id=="1".as_bytes().to_vec() ||     // Ethereum
                    chain_id=="2".as_bytes().to_vec() ||     // Binance
                    chain_id=="3".as_bytes().to_vec()        // Solana
                    , Error::<T>::InvalidChainId);
            // check for description not empty and <64 bytes
			let description = Self::json_get_value(data.clone(),"description".as_bytes().to_vec());
            ensure!(!description.is_empty() && description.len()<=64 , Error::<T>::InvalidDescription);
            // check for address not empty
            let address = Self::json_get_value(data.clone(),"address".as_bytes().to_vec());
            ensure!(!address.is_empty() , Error::<T>::EmptyAddress);
            // check for asset id validity
            let asset_id = Self::json_get_value(data.clone(),"assetid".as_bytes().to_vec());
			let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();
            // check whether asset exists or not
			ensure!(Asset::<T>::contains_key(asset_id), Error::<T>::AssetDoesNotExist);
            //check internal threshold
            let internal_threshold = Self::json_get_value(data.clone(),"internalthreshold".as_bytes().to_vec());
            ensure!(!internal_threshold.is_empty() , Error::<T>::InternalThresholdNotFound);
            let itn=vecu8_to_u32(internal_threshold);
            ensure!(!itn>0 && itn<=99, Error::<T>::InternalThresholdInvalid);
            //check external threshold
            let external_threshold = Self::json_get_value(data.clone(),"externathreshold".as_bytes().to_vec());
            ensure!(!external_threshold.is_empty() , Error::<T>::ExternalThresholdNotFound);
            let etn=vecu8_to_u32(external_threshold);
            ensure!(!etn>0  && etn<=99, Error::<T>::ExternalThresholdInvalid);
            //check internal keepers accounts
            let internalkeepers=Self::json_get_complexarray(data.clone(),"internalkeepers".as_bytes().to_vec());
                if internalkeepers.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalkeepers.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()==48,Error::<T>::InternalKeepersAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalKeepersNotConfigured);
                    ensure!(x as u32 == itn,Error::<T>::InternalKeepersNotMatchingThreshold);

                }
            //check external keepers accounts
            let externalkeepers=Self::json_get_complexarray(data.clone(),"externalkeepers".as_bytes().to_vec());
                if externalkeepers.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalkeepers.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()>=32,Error::<T>::ExternalKeepersAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::ExternalKeepersNotConfigured);
                    ensure!(x as u32 == etn,Error::<T>::ExternalKeepersNotMatchingThreshold);

                }
            //check internal watchdogs accounts
            let internalwatchdogs = Self::json_get_complexarray(data.clone(),"internalwatchdogs".as_bytes().to_vec());
                if internalwatchdogs.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalwatchdogs.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()==48,Error::<T>::InternalWhatchDogsAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalWatchdogsNotConfigured);
                }
            //check external watchdogs accounts
            let externalwatchdogs=Self::json_get_complexarray(data.clone(),"externalwatchdogs".as_bytes().to_vec());
                if externalwatchdogs.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalwatchdogs.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()>=32,Error::<T>::ExternalWatchddogsAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::ExternalWatchdogsNotConfigured);
                }
            //check internal watchcats accounts
            let internalwatchcats=Self::json_get_complexarray(data.clone(),"internalwatchcats".as_bytes().to_vec());
                if internalwatchcats.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalwatchcats.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()==48,Error::<T>::InternalWhatchCatsAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalWatchcatsNotConfigured);
                }
            //check external watchcats accounts
            let externalwatchcats=Self::json_get_complexarray(data.clone(),"externalwatchcats".as_bytes().to_vec());
                if externalwatchcats.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalwatchcats.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        ensure!(w.len()>=32,Error::<T>::ExternalWhatchCatsAccountIsWrong);
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::ExternalWatchcatsNotConfigured);
                }

            Settings::insert(key.clone(),data.clone());
            // Generate event
            Self::deposit_event(RawEvent::SettingsCreated(key,data));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy setting with the given key
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn destroy_settings(origin, key: Vec<u8>) -> DispatchResult {
            // allow access only to SUDO
            ensure_root(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::get(), Error::<T>::NotAllowedSystemLockdown);
            // check whether setting key exists or not
            ensure!(Settings::contains_key(&key), Error::<T>::SettingsKeyNotFound);
            Settings::remove(key.clone());
            // Generate event
            Self::deposit_event(RawEvent::SettingsDestroyed(key));
            // Return a successful DispatchResult
            Ok(())
        }
        // function to Mint an assetid
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn mint(origin, token:Vec<u8>,recipient: T::AccountId, transaction_id:Vec<u8>, amount: Balance)-> DispatchResultWithPostInfo {
            // check for a signed transactions
            let signer = ensure_signed(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::get(), Error::<T>::NotAllowedSystemLockdown);
            // check for the token configuration in settings
            ensure!(Settings::contains_key(&token), Error::<T>::SettingsKeyNotFound);
            let content: Vec<u8> = Settings::get(&token).unwrap();
            let asset_idv = Self::json_get_value(content.clone(),"assetid".as_bytes().to_vec());
			let asset_id = vecu8_to_u32(asset_idv);
            let internalthresholdv = Self::json_get_value(content.clone(),"internalthreshold".as_bytes().to_vec());
            let internalthreshold = vecu8_to_u32(internalthresholdv);

            // check for authorized signer
            let mut flag=0;
            let internal_keepers = Self::json_get_complexarray(content,"internalkeepers".as_bytes().to_vec());
            let mut x=0;
            loop {
                let internal_keeper= Self::json_get_arrayvalue(internal_keepers.clone(),x);
                if internal_keeper.is_empty() {
                    break;
                }
                let internal_keepervec=bs58::decode(internal_keeper).into_vec().unwrap();
                let accountid_internal_keepers=T::AccountId::decode(&mut &internal_keepervec[1..33]).unwrap_or_default();
                if accountid_internal_keepers==signer {
                    flag=1;
                }
                x += 1;
            }

            ensure!(flag==1, Error::<T>::SignerIsNotKeeper);

            // check for duplicated minting for the same transaction/signer
            ensure!(!TransactionMintTracker::<T>::contains_key(transaction_id.clone(),&signer), Error::<T>::SignerAlreadyConfirmed);
            // store minting tracker
            TransactionMintTracker::<T>::insert(transaction_id.clone(),signer.clone(),asset_id);
            
            // storing the minting request if it's not already present
            let key = &mut token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            if !MintRequest::contains_key(key.clone()) {
                MintRequest::insert(key.clone(),amount);
            }else {
                // when already present 
                // checking that the amount to mint is the same of the previous one, if does not, we have an Oracle hacked or not updated
                let am=MintRequest::get(key.clone());
                ensure!(am==amount,Error::<T>::AmountMintingIsNotMatching);
            }
 
            // update the counter for the minting requests of the transaction
            let mut key = token;
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            MintCounter::try_mutate(&key, |count| -> DispatchResult {
				*count += 1;
				Ok(())
			})?;
            // get the number of minting requests
            let nmr=MintCounter::get(&key);
            // thresold not reached
            match nmr.cmp(&internalthreshold) {
                Ordering::Less => Self::deposit_event(RawEvent::MintQueued(signer, asset_id, recipient, amount, transaction_id.clone())),
                Ordering::Greater => Self::deposit_event(RawEvent::AlreadyMinted(signer, asset_id, recipient, amount)),
                Ordering::Equal => {
                    // check it's not already confirmed
                    ensure!(!MintConfirmation::contains_key(key.clone()),Error::<T>::MintingAlreadyConfirmed);
                    // store the minting confirmation
                    MintConfirmation::insert(key,true);
                    //minting of the asset_id matching the token configured
                    let asset_owner = Asset::<T>::get(asset_id).unwrap().owner;
                    pallet_assets::Module::<T>::mint(RawOrigin::Signed(asset_owner).into(), asset_id, T::Lookup::unlookup(recipient.clone()), amount)?;
                    // generate an event
                    Self::deposit_event(RawEvent::Minted(signer, asset_id, recipient, amount))
                },
            };

            Ok(().into())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn burn(origin, token:Vec<u8>, recipient: T::AccountId, transaction_id:Vec<u8>, amount: Balance)-> DispatchResultWithPostInfo {
            let signer = ensure_signed(origin)?;
            // check if lockdownmode is off
            ensure!(!Lockdown::get(), Error::<T>::NotAllowedSystemLockdown);
            ensure!(Settings::contains_key(&token), Error::<T>::SettingsKeyNotFound);
            let content: Vec<u8> = Settings::get(&token).unwrap();
            let asset_idv = Self::json_get_value(content.clone(),"assetid".as_bytes().to_vec());
			let asset_id = vecu8_to_u32(asset_idv);
            let internalthresholdv = Self::json_get_value(content.clone(),"internalthreshold".as_bytes().to_vec());
            let internalthreshold = vecu8_to_u32(internalthresholdv);

            // check for authorized signer
            let mut flag=0;
            let internal_keepers = Self::json_get_complexarray(content,"internalkeepers".as_bytes().to_vec());
            let mut x=0;
            loop {
                let internal_keeper= Self::json_get_arrayvalue(internal_keepers.clone(),x);
                if internal_keeper.is_empty() {
                    break;
                }
                let internal_keepervec=bs58::decode(internal_keeper).into_vec().unwrap();
                let accountid_internal_keepers=T::AccountId::decode(&mut &internal_keepervec[1..33]).unwrap_or_default();
                if accountid_internal_keepers==signer {
                    flag=1;
                }
                x += 1;
            }
            ensure!(flag==1, Error::<T>::SignerIsNotKeeper);

            // check for duplicated burning for the same transaction/signer
            ensure!(!TransactionBurnTracker::<T>::contains_key(transaction_id.clone(),&signer), Error::<T>::SignerAlreadyConfirmed);
            // store burning tracker
            TransactionBurnTracker::<T>::insert(transaction_id.clone(),signer.clone(),asset_id);

            // storing the burning request if it's not already present
            let key = &mut token.clone();
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            if !BurnRequest::contains_key(key.clone()) {
                BurnRequest::insert(key.clone(),amount);
            }else {
                // when already present
                // checking that the amount to burn is the same of the previous one, if does not, we have an Oracle hacked or not updated
                let am=BurnRequest::get(key.clone());
                ensure!(am==amount,Error::<T>::AmountBurningIsNotMatching);
            }

            // update the counter for the minting requests of the transaction
            let mut key = token;
            key.push(b'-');
            key.append(&mut recipient.encode());
            key.push(b'-');
            key.append(&mut transaction_id.clone());
            BurnCounter::try_mutate(&key, |count| -> DispatchResult {
				*count += 1;
				Ok(())
			})?;

            // get the number of burning requests
            let nmr=BurnCounter::get(&key);
            match nmr.cmp(&internalthreshold) {
                Ordering::Less => Self::deposit_event(RawEvent::BurnQueued(signer, asset_id, recipient, amount, transaction_id.clone())),
                Ordering::Greater => Self::deposit_event(RawEvent::AlreadyBurned(signer, asset_id, recipient, amount)),
                Ordering::Equal => {
                    // check it's not already confirmed
                    ensure!(!BurnConfirmation::contains_key(key.clone()),Error::<T>::BurningAlreadyConfirmed);
                    // store the BurnConfirmation
                    BurnConfirmation::insert(key,true);
                    //burning of the asset_id matching the token configured
                    let asset_owner = Asset::<T>::get(asset_id).unwrap().owner;
                    pallet_assets::Module::<T>::burn(RawOrigin::Signed(asset_owner.clone()).into(), asset_id, T::Lookup::unlookup(recipient.clone()), amount)?;
                    // generate an event
                    Self::deposit_event(RawEvent::Burned(signer, asset_id, recipient, amount));
                },
            };

            Ok(().into())
        }
        // function to set a system lockdown, watchdogs and watchcats account can set it
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn set_lockdown(origin, token: Vec<u8>) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            ensure!(Settings::contains_key(&token), Error::<T>::SettingsKeyNotFound);
            let content: Vec<u8> = Settings::get(&token).unwrap();
            let mut flag=0;
            let internal_watch_dogs = Self::json_get_complexarray(content.clone(),"internalwatchdogs".as_bytes().to_vec());
            let mut x=0;
            loop {
                let internal_watch_dogs= Self::json_get_arrayvalue(internal_watch_dogs.clone(),x);
                if internal_watch_dogs.is_empty() {
                    break;
                }
                let internal_watch_dogsvec=bs58::decode(internal_watch_dogs).into_vec().unwrap();
                let accountid_internal_watch_dogs=T::AccountId::decode(&mut &internal_watch_dogsvec[1..33]).unwrap_or_default();
                if accountid_internal_watch_dogs==signer {
                    flag=1;
                }
                x += 1;
            }
            let internal_watch_cats = Self::json_get_complexarray(content,"internalwatchcats".as_bytes().to_vec());
            let mut x=0;
            loop {
                let internal_watch_cats= Self::json_get_arrayvalue(internal_watch_cats.clone(),x);
                if internal_watch_cats.is_empty() {
                    break;
                }
                let internal_watch_catsvec=bs58::decode(internal_watch_cats).into_vec().unwrap();
                let accountid_internal_watch_cats=T::AccountId::decode(&mut &internal_watch_catsvec[1..33]).unwrap_or_default();
                if accountid_internal_watch_cats==signer {
                    flag=1;
                }
                x += 1;
            }
            ensure!(flag==1, Error::<T>::SignerIsNotAuthorized);
            Lockdown::put(true);
            Ok(())
        }
        // function to remove the lockdown, it can be executed only from the superuser
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn set_unlockdown(origin) -> DispatchResult {
            // check access for Sudo
            ensure_root(origin)?;
            Lockdown::put(false);
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {

    // function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
    fn json_get_recordvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
        let mut result=Vec::new();
        let mut op=true;
        let mut cn=0;
        let mut lb=b' ';
        for b in ar {
            if b==b',' && op {
                cn += 1;
                continue;
            }
            if b==b'[' && op && lb!=b'\\' {
                continue;
            }
            if b==b']' && op && lb!=b'\\' {
                continue;
            }
            if b==b'{' && op && lb!=b'\\' {
                op=false;
            }
            if b==b'}' && !op && lb!=b'\\' {
                op=true;
            }
            // field found
            if cn==p && b!=b'"' {
                result.push(b);
            }
            lb= b ;
        }
        result
    }

    // function to get value of a field with a complex array like [{....},{.....}] for Substrate runtime (no std library and no variable allocation)
    fn json_get_complexarray(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
        let mut result=Vec::new();
        let mut k=Vec::new();
        let keyl = key.len();
        let jl = j.len();
        k.push(b'"');
        for xk in 0..keyl{
            k.push(*key.get(xk).unwrap());
        }
        k.push(b'"');
        k.push(b':');
        let kl = k.len();
        for x in  0..jl {
            let mut m=0;
            if x+kl>jl {
                break;
            }
            for (xx, i) in (x..x+kl).enumerate() {
                if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                    m += 1;
                }
            }
            if m==kl{
                let mut os=true;
                for i in x+kl..jl-1 {
                    if *j.get(i).unwrap()==b'[' && os{
                        os=false;
                    }
                    result.push(*j.get(i).unwrap());
                    if *j.get(i).unwrap()==b']' && !os {
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
    fn json_get_value(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
        let mut result=Vec::new();
        let mut k=Vec::new();
        let keyl = key.len();
        let jl = j.len();
        k.push(b'"');
        for xk in 0..keyl{
            k.push(*key.get(xk).unwrap());
        }
        k.push(b'"');
        k.push(b':');
        let kl = k.len();
        for x in  0..jl {
            let mut m=0;
            if x+kl>jl {
                break;
            }
            for (xx, i) in (x..x+kl).enumerate() {
                if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                    m += 1;
                }
            }
            if m==kl{
                let mut lb=b' ';
                let mut op=true;
                let mut os=true;
                for i in x+kl..jl-1 {
                    if *j.get(i).unwrap()==b'[' && op && os{
                        os=false;
                    }
                    if *j.get(i).unwrap()==b'}' && op && !os{
                        os=true;
                    }
                    if *j.get(i).unwrap()==b':' && op{
                        continue;
                    }
                    if *j.get(i).unwrap()==b'"' && op && lb!=b'\\' {
                        op=false;
                        continue
                    }
                    if *j.get(i).unwrap()==b'"' && !op && lb!=b'\\' {
                        break;
                    }
                    if *j.get(i).unwrap()==b'}' && op{
                        break;
                    }
                    if *j.get(i).unwrap()==b']' && op{
                        break;
                    }
                    if *j.get(i).unwrap()==b',' && op && os{
                        break;
                    }
                    result.push(*j.get(i).unwrap());
                    lb= *j.get(i).unwrap();
                }
                break;
            }
        }
        result
    }

    // function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the records is not present
    fn json_get_arrayvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
        let mut result=Vec::new();
        let mut op=true;
        let mut cn=0;
        let mut lb=b' ';
        for b in ar {
            if b==b',' && op {
                cn += 1;
                continue;
            }
            if b==b'[' && op && lb!=b'\\' {
                continue;
            }
            if b==b']' && op && lb!=b'\\' {
                continue;
            }
            if b==b'"' && op && lb!=b'\\' {
                continue;
            }
            if b==b'"' && op && lb!=b'\\' {
                op=false;
            }
            if b==b'"' && !op && lb!=b'\\' {
                op=true;
            }
            // field found
            if cn==p {
                result.push(b);
            }
            lb= b;
        }
        result
    }

    pub fn mint_tracker_contains(transaction_id: Vec<u8>, signer: T::AccountId) -> bool {
        TransactionMintTracker::<T>::contains_key(transaction_id,&signer)
    }

    pub fn burn_tracker_contains(transaction_id: Vec<u8>, signer: T::AccountId) -> bool {
        TransactionBurnTracker::<T>::contains_key(transaction_id.clone(),&signer)
    }

    pub fn is_threshold_mint(token:Vec<u8>) -> bool {
        if let Some(content) = Settings::get(&token) {
            let internalthresholdv = Self::json_get_value(content.clone(),"internalthreshold".as_bytes().to_vec());
            let internalthreshold = vecu8_to_u32(internalthresholdv);
            let nmr= MintCounter::get(&token);
            nmr >= internalthreshold
        } else {
            false
        }
    }

    pub fn is_threshold_burn(token:Vec<u8>) -> bool {
        if let Some(content) = Settings::get(&token) {
            let internalthresholdv = Self::json_get_value(content.clone(),"internalthreshold".as_bytes().to_vec());
            let internalthreshold = vecu8_to_u32(internalthresholdv);
            let nmr= BurnCounter::get(&token);
            nmr >= internalthreshold
        } else {
            false
        }
    }

}
// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
    let vslice = v.as_slice();
    let vstr = str::from_utf8(vslice).unwrap_or("0");
    let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
    vvalue
}

