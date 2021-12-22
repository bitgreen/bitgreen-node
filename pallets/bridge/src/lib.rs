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
    traits::Get, codec::Decode
};
use frame_system::ensure_root;
use sp_std::vec::Vec;
use pallet_assets::Asset;
use frame_system::ensure_signed;
use primitives::Balance;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128>{
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
        /// SignerNotFound
        SignerNotFound
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
        pub fn create_settings(origin, key: Vec<u8>, data: Vec<u8>) -> DispatchResult {

            ensure_root(origin)?;

            //check accounts json length
            ensure!(data.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(data.len() < 8192, Error::<T>::SettingsJsonTooLong);

            // check json validity
            let js=data.clone();
            ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);

            // check whether setting key already exists
            ensure!(!Settings::contains_key(&key), Error::<T>::SettingsKeyExists);

            let chain_id = Self::json_get_value(data.clone(),"chainid".as_bytes().to_vec());
            ensure!(!chain_id.is_empty() , Error::<T>::InvalidChainId);

			let description = Self::json_get_value(data.clone(),"description".as_bytes().to_vec());
            ensure!(!description.is_empty() && description.len()<=64 , Error::<T>::InvalidDescription);

            let address = Self::json_get_value(data.clone(),"address".as_bytes().to_vec());
            ensure!(!address.is_empty() , Error::<T>::EmptyAddress);

            let asset_id = Self::json_get_value(data.clone(),"assetid".as_bytes().to_vec());

			let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();

            // check whether asset exists or not
			ensure!(Asset::<T>::contains_key(asset_id), Error::<T>::AssetDoesNotExist);

            let internal_threshold = Self::json_get_value(data.clone(),"internalthreshold".as_bytes().to_vec());
            ensure!(!internal_threshold.is_empty() , Error::<T>::InternalThresholdNotFound);

            let external_threshold = Self::json_get_value(data.clone(),"externathreshold".as_bytes().to_vec());
            ensure!(!external_threshold.is_empty() , Error::<T>::ExternalThresholdNotFound);

            let internalkeepers=Self::json_get_complexarray(data.clone(),"internalkeepers".as_bytes().to_vec());
                if internalkeepers.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalkeepers.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalKeepersNotConfigured);
                }
            let externalkeepers=Self::json_get_complexarray(data.clone(),"externalkeepers".as_bytes().to_vec());
                if externalkeepers.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalkeepers.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::ExternalKeepersNotConfigured);
                }

            let internalwatchdogs=Self::json_get_complexarray(data.clone(),"internalwatchdogs".as_bytes().to_vec());
                if internalwatchdogs.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalwatchdogs.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalWatchdogsNotConfigured);
                }
            let externalwatchdogs=Self::json_get_complexarray(data.clone(),"externalwatchdogs".as_bytes().to_vec());
                if externalwatchdogs.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalwatchdogs.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::ExternalWatchdogsNotConfigured);
                }

            let internalwatchcats=Self::json_get_complexarray(data.clone(),"internalwatchcats".as_bytes().to_vec());
                if internalwatchcats.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(internalwatchcats.clone(),x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    ensure!(x>0,Error::<T>::InternalWatchcatsNotConfigured);
                }
            let externalwatchcats=Self::json_get_complexarray(data.clone(),"externalwatchcats".as_bytes().to_vec());
                if externalwatchcats.len()>=2 {
                    let mut x=0;
                    loop {
                        let w=Self::json_get_recordvalue(externalwatchcats.clone(),x);
                        if w.is_empty() {
                            break;
                        }
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

            ensure_root(origin)?;

            // check whether setting key exists or not
            ensure!(Settings::contains_key(&key), Error::<T>::SettingsKeyNotFound);

            Settings::remove(key.clone());

            // Generate event
            Self::deposit_event(RawEvent::SettingsDestroyed(key));
            // Return a successful DispatchResult
            Ok(())
        }
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn mint(origin, token:Vec<u8>,recipient: T::AccountId, transaction_id:Vec<u8>, amount: Balance)-> DispatchResult {
            let signer = ensure_signed(origin)?;

            ensure!(Settings::contains_key(&token), Error::<T>::SettingsKeyNotFound);
            let content: Vec<u8> = Settings::get(&token).unwrap();
            let asset_id = Self::json_get_value(content.clone(),"assetid".as_bytes().to_vec());

			let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();

            let mut flag=0;
            let internal_keepers = Self::json_get_value(content.clone(),"internalkeepers".as_bytes().to_vec());
            if !internal_keepers.is_empty() {
                let internal_keepers_vec=bs58::decode(internal_keepers).into_vec().unwrap();
                let accountid_internal_keepers=T::AccountId::decode(&mut &internal_keepers_vec[1..33]).unwrap_or_default();
                if signer==accountid_internal_keepers {
                    flag=1;
                }
            }
            ensure!(flag==1, Error::<T>::SignerNotFound);


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
            if cn==p {
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

}
