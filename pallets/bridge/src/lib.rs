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
extern crate alloc;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    traits::Get,
};
use frame_system::ensure_root;
use sp_std::vec::Vec;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {

    trait Store for Module<T: Config> as VCUModule {
        /// Settings configuration.
        Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
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
        /// BridgeAdded
        BridgeAdded(AccountId),
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
        /// 	"internalkeepers":[".....",".....",".....",], // account of the "keepers" delegate to sign the transactions
        /// 	"internalthreshold",x, // minimum number of signer to confirm a transaction for Bitgreen blockchain
        /// 	"externalkeepers":["...",".....",".....",], // account of the "keepers" delegate to sign the transactions
        /// 	"externathreshold",x, // minimum number of signer to confirm a transaction on the external blockchain
        /// 	"internalwatchdogs":[".....",".....",".....",], // accounts of the watchdogs account that are enable to fire a lockdown on Bitgreen blockchain
        /// 	"externalwatchdogs":[".....",".....",".....",], // accounts of the watchdogs account that are enable to fire a lockdown on the external blockchain
        /// 	"internalwatchcats":[".....",".....",".....",], // accounts of the watchcats account that are enable to fire a lockdown on Bitgreen blockchain before the confirmation
        /// 	"externalwatchcats":[".....",".....",".....",], // accounts of the watchcats account that are enable to fire a lockdown on the external blockchain before the confirmation
        /// }
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn create_settings(origin, key: Vec<u8>, accounts: Vec<u8>) -> DispatchResult {

            ensure_root(origin)?;

            //check accounts json length
            ensure!(accounts.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(accounts.len() < 8192, Error::<T>::SettingsJsonTooLong);

            // check json validity
            let js=accounts.clone();
            ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);

            // check whether setting key already exists
            ensure!(!Settings::contains_key(&key), Error::<T>::SettingsKeyExists);

            Settings::insert(key.clone(),accounts.clone());
            // Generate event
            Self::deposit_event(RawEvent::SettingsCreated(key,accounts));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy setting with the given key
        ///
        /// The dispatch origin for this call must be `Signed` by the Root.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn destroy_settings(origin, key: Vec<u8>) -> DispatchResult {

            ensure_root(origin)?;

            // check whether setting key exists or not
            ensure!(Settings::contains_key(&key), Error::<T>::SettingsKeyNotFound);

            Settings::remove(key.clone());

            // Generate event
            Self::deposit_event(RawEvent::SettingsDestroyed(key));
            // Return a successful DispatchResult
            Ok(())
        }

    }
}

impl<T: Config> Module<T> {
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
}
