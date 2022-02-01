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
use sp_std::prelude::*;
use core::str;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult, ensure
};
use primitives::Balance;
use codec::Encode;
use frame_system::ensure_signed;
use sp_std::vec;
use alloc::string::ToString;

/// Module configuration
pub trait Config: frame_system::Config  {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as VestingModule {
		VestingAccount get(fn vesting_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32  => Vec<u8>;
	}
}
// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Vesting Account Created
        VestingAccountCreated(AccountId),
		/// Vesting Account Destroyed
        VestingAccountDestroyed(AccountId),
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
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
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;

		// function to create a vesting account
        #[weight = 10_000]
		pub fn create_vesting_account(
			origin,
			recipient_account:T::AccountId,
			vesting_account:T::AccountId,
			uid: u32,
			initial_deposit: Balance,
			expire_time:u32,
			current_deposit: Balance,
			staking: Balance,
		) -> DispatchResult {
			// check for Super User access
			let vesting_creator = ensure_root(origin)?;
			// check that the same account is not already present
			ensure!(!VestingAccount::<T>::contains_key(&vesting_creator, &uid), Error::<T>::VestingAccountAlreadyExists);
			// others validity checks
			ensure!(uid > 0, Error::<T>::InvalidUID);
			ensure!(initial_deposit > 0, Error::<T>::InvalidIntialDeposit);
			ensure!(expire_time > 0, Error::<T>::InvalidExpireTime);
			ensure!(current_deposit > 0, Error::<T>::InvalidCurrentDeposit);
			ensure!(staking > 0, Error::<T>::InvalidStaking);

			// create json string
    		let json = Self::create_json_string(vec![
				("recipient_account",&mut recipient_account.encode()),
				("vesting_account",&mut  vesting_account.encode()),
				("uid",&mut  uid.to_string().as_bytes().to_vec()),
				("initial_deposit",&mut  initial_deposit.to_string().as_bytes().to_vec()),
				("expire_time",&mut  expire_time.to_string().as_bytes().to_vec()),
				("current_deposit",&mut  current_deposit.to_string().as_bytes().to_vec()),
				("staking",&mut  staking.to_string().as_bytes().to_vec()),
			]);
			// store the vesting account
			VestingAccount::<T>::insert(vesting_creator.clone(), &uid, json);
            // Generate event
            Self::deposit_event(RawEvent::VestingAccountCreated(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
		}
		// function to remove a vesting account
        #[weight = 10_000]
        pub fn destroy_vesting_account(origin, uid: u32) -> DispatchResult {
			// check for Super User access
			let vesting_creator = ensure_root(origin)?;
			// check the account is present in the storage
			ensure!(VestingAccount::<T>::contains_key(&vesting_creator, &uid), Error::<T>::VestingAccountDoesNotExist);
			// decode data
			let content: Vec<u8> = VestingAccount::<T>::get(vesting_creator.clone(), &uid);
	    	let initial_deposit = Self::json_get_value(content.clone(),"initial_deposit".as_bytes().to_vec());
			let initial_deposit = str::parse::<Balance>(sp_std::str::from_utf8(&initial_deposit).unwrap()).unwrap();
			let current_deposit = Self::json_get_value(content.clone(),"current_deposit".as_bytes().to_vec());
			let current_deposit = str::parse::<Balance>(sp_std::str::from_utf8(&current_deposit).unwrap()).unwrap();
			let expire_time = Self::json_get_value(content.clone(),"expire_time".as_bytes().to_vec());
			let expire_time = str::parse::<T::BlockNumber>(sp_std::str::from_utf8(&expire_time).unwrap()).ok().unwrap();
			/*
			let current_time: T::BlockNumber = frame_system::Module::<T>::block_number();
			ensure!(initial_deposit == current_deposit, Error::<T>::InvalidDeposit);
			ensure!(expire_time == current_time, Error::<T>::InvalidEpochTime);
			*/
			// delete the vestin account
			VestingAccount::<T>::remove(vesting_creator.clone(), &uid);
            // Generate event
            Self::deposit_event(RawEvent::VestingAccountDestroyed(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
        }

	}
}

impl<T: Config> Module<T> {

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

	fn create_json_string(inputs: Vec<(&str, &mut Vec<u8>)>) -> Vec<u8> {
		let mut v:Vec<u8>= vec![b'{'];
		let mut flag = false;

		for (arg, val) in  inputs{
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