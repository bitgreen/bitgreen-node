// SBP M2 review: compilation warnings.
// SBP M2 review: M1 comments not applied
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

// SBP M1 review: missing documentation, tests & benchmarks.
// Note: you could also look at the vesting pallet in Substrate FRAME,
// to see if it matches your specific vesting requirements.

extern crate alloc;
use crate::alloc::string::ToString;
use codec::Encode;
use core::str;
use frame_support::sp_runtime::traits::StaticLookup;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    pallet_prelude::DispatchResultWithPostInfo, traits::Get,
};
use frame_system::RawOrigin;
pub use pallet::*;
use primitives::Balance;
use sp_std::prelude::*;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;

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
        type NativeTokenId: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn vesting_account)]
    pub type VestingAccount<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        Vec<u8>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Vesting Account Created
        VestingAccountCreated(T::AccountId),
        /// Vesting Account Destroyed
        VestingAccountDestroyed(T::AccountId),
        /// WithdrewVestingAccount
        WithdrewVestingAccount(T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Vesting Account Already Exist
        VestingAccountAlreadyExists,
        /// Invalid UID
        InvalidUID,
        /// Invalid Intial Deposit
        InvalidIntialDeposit,
        /// Invalid  Expire Time
        InvalidExpireTime,
        /// Invalid Current Deposit
        InvalidCurrentDeposit,
        /// Invalid Staking
        InvalidStaking,
        /// VestingAccount Does Not Exist
        VestingAccountDoesNotExist,
        /// Invalid Deposit
        InvalidDeposit,
        /// Invalid Epoch Time
        InvalidEpochTime,
        /// InvalidRecipent
        InvalidRecipent,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // SBP M1 review: dispatchable calls should be benchmarked.
        // function to create a vesting account
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_vesting_account(
            origin: OriginFor<T>,
            recipient_account: T::AccountId,
            vesting_account: T::AccountId,
            uid: u32,
            initial_deposit: Balance,
            expire_time: u32,
            current_deposit: Balance,
            staking: Balance,
        ) -> DispatchResult {
            // check for Super User access
            let vesting_creator = ensure_signed(origin)?;
            // check that the same account is not already present
            ensure!(
                !VestingAccount::<T>::contains_key(&vesting_creator, &uid),
                Error::<T>::VestingAccountAlreadyExists
            );
            // others validity checks
            ensure!(uid > 0, Error::<T>::InvalidUID);
            ensure!(initial_deposit > 0, Error::<T>::InvalidIntialDeposit);
            ensure!(expire_time > 0, Error::<T>::InvalidExpireTime);
            ensure!(current_deposit > 0, Error::<T>::InvalidCurrentDeposit);
            ensure!(staking > 0, Error::<T>::InvalidStaking);

            // SBP M1 review: why use json ? you could create a struct instead.

            // create json string
            let json = Self::create_json_string(vec![
                ("recipient_account", &mut recipient_account.encode()),
                ("vesting_account", &mut vesting_account.encode()),
                ("uid", &mut uid.to_string().as_bytes().to_vec()),
                (
                    "initial_deposit",
                    &mut initial_deposit.to_string().as_bytes().to_vec(),
                ),
                (
                    "expire_time",
                    &mut expire_time.to_string().as_bytes().to_vec(),
                ),
                (
                    "current_deposit",
                    &mut current_deposit.to_string().as_bytes().to_vec(),
                ),
                ("staking", &mut staking.to_string().as_bytes().to_vec()),
            ]);
            // store the vesting account
            VestingAccount::<T>::insert(vesting_creator.clone(), &uid, json);
            // Generate event
            Self::deposit_event(Event::VestingAccountCreated(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
        }

        // function to remove a vesting account
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn destroy_vesting_account(origin: OriginFor<T>, uid: u32) -> DispatchResult {
            // check for Super User access
            let vesting_creator = ensure_signed(origin)?;
            // check the account is present in the storage
            ensure!(
                VestingAccount::<T>::contains_key(&vesting_creator, &uid),
                Error::<T>::VestingAccountDoesNotExist
            );

            // SBP M1 review: why use json ? you could create a struct instead, data is stored on-chain in a binary-efficient encoding (SCALE).

            // decode data
            let content: Vec<u8> = VestingAccount::<T>::get(vesting_creator.clone(), &uid);
            let initial_deposit =
                Self::json_get_value(content.clone(), "initial_deposit".as_bytes().to_vec());
            let initial_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&initial_deposit).unwrap()).unwrap();
            let current_deposit =
                Self::json_get_value(content.clone(), "current_deposit".as_bytes().to_vec());
            let current_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&current_deposit).unwrap()).unwrap();
            let expire_time =
                Self::json_get_value(content.clone(), "expire_time".as_bytes().to_vec());
            let expire_time =
                str::parse::<T::BlockNumber>(sp_std::str::from_utf8(&expire_time).unwrap())
                    .ok()
                    .unwrap();
            let current_time: T::BlockNumber = frame_system::Module::<T>::block_number();
            ensure!(
                initial_deposit == current_deposit,
                Error::<T>::InvalidDeposit
            );
            ensure!(expire_time > current_time, Error::<T>::InvalidEpochTime);
            // delete the vestin account
            VestingAccount::<T>::remove(vesting_creator.clone(), &uid);
            // Generate event
            Self::deposit_event(Event::VestingAccountDestroyed(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn withdraw_vesting_account(
            origin: OriginFor<T>,
            vesting_creator: T::AccountId,
            uid: u32,
        ) -> DispatchResultWithPostInfo {
            let recipient = ensure_signed(origin)?;

            ensure!(
                VestingAccount::<T>::contains_key(&vesting_creator, &uid),
                Error::<T>::VestingAccountDoesNotExist
            );
            // decode data
            let content: Vec<u8> = VestingAccount::<T>::get(vesting_creator.clone(), &uid);
            let initial_deposit =
                Self::json_get_value(content.clone(), "initial_deposit".as_bytes().to_vec());
            let initial_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&initial_deposit).unwrap()).unwrap();
            let current_deposit =
                Self::json_get_value(content.clone(), "current_deposit".as_bytes().to_vec());
            let current_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&current_deposit).unwrap()).unwrap();
            let expire_time =
                Self::json_get_value(content.clone(), "expire_time".as_bytes().to_vec());
            let expire_time =
                str::parse::<T::BlockNumber>(sp_std::str::from_utf8(&expire_time).unwrap())
                    .ok()
                    .unwrap();
            ensure!(
                VestingAccount::<T>::contains_key(&vesting_creator, &uid),
                Error::<T>::VestingAccountDoesNotExist
            );

            // SBP M1 review: same remark as above regarding the use of json format internally.

            // decode data
            let content: Vec<u8> = VestingAccount::<T>::get(vesting_creator.clone(), &uid);
            let initial_deposit =
                Self::json_get_value(content.clone(), "initial_deposit".as_bytes().to_vec());
            let initial_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&initial_deposit).unwrap()).unwrap();
            let current_deposit =
                Self::json_get_value(content.clone(), "current_deposit".as_bytes().to_vec());
            let current_deposit =
                str::parse::<Balance>(sp_std::str::from_utf8(&current_deposit).unwrap()).unwrap();
            let expire_time =
                Self::json_get_value(content.clone(), "expire_time".as_bytes().to_vec());
            let expire_time =
                str::parse::<T::BlockNumber>(sp_std::str::from_utf8(&expire_time).unwrap())
                    .ok()
                    .unwrap();

            let recipient_account =
                Self::json_get_value(content.clone(), "recipient_account".as_bytes().to_vec());
            let recipient_account = T::AccountId::decode(&mut &recipient_account[1..33])
                .map_err(|_| Error::<T>::InvalidRecipent)?;
            let staking = Self::json_get_value(content.clone(), "staking".as_bytes().to_vec());
            let staking = str::parse::<Balance>(sp_std::str::from_utf8(&staking).unwrap()).unwrap();

            ensure!(recipient == recipient_account, Error::<T>::InvalidRecipent);
            let current_time: T::BlockNumber = frame_system::Pallet::<T>::block_number();
            ensure!(
                initial_deposit == current_deposit,
                Error::<T>::InvalidDeposit
            );
            ensure!(expire_time > current_time, Error::<T>::InvalidEpochTime);

            // SBP M1 review: this introduces coupling between your vesting pallet and the assets pallet.
            // A more decoupled approach would be to inject an implementation of one of the Currency traits via
            // the pallet's Config trait. As an example: https://github.com/paritytech/substrate/blob/59a21506a26229a52ffcc2d11d878c49ceedb992/frame/assets/src/lib.rs#L204
            // Also, instead of usings transfers, you could look at using the `LockableCurrency` trait,
            // and draw inspiration from the FRAME vesting pallet implementation:
            // see https://github.com/paritytech/substrate/blob/59a21506a26229a52ffcc2d11d878c49ceedb992/frame/vesting/src/lib.rs#L156
            pallet_assets::Pallet::<T>::transfer(
                RawOrigin::Signed(vesting_creator.clone()).into(),
                T::NativeTokenId::get(),
                T::Lookup::unlookup(recipient.clone()),
                staking,
            )?;

            // Generate event
            Self::deposit_event(Event::WithdrewVestingAccount(vesting_creator));
            // Return a successful DispatchResult
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
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
    }
}
