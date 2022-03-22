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
use alloc::string::ToString;
use codec::Decode;
use codec::Encode;
use core::str;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	pallet_prelude::DispatchResultWithPostInfo, traits::Get,
};
use frame_system::ensure_signed;
use frame_system::RawOrigin;
use orml_traits::{BasicCurrency, MultiCurrency};
use primitives::Balance;
use sp_runtime::traits::{StaticLookup, Zero};
use sp_std::prelude::*;
use sp_std::vec;

type BalanceOf<T> = <<T as module_currencies::Config>::MultiCurrency as MultiCurrency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

/// Module configuration
pub trait Config:
	frame_system::Config
	+ pallet_assets::Config<AssetId = u32, Balance = u128>
	+ module_currencies::Config
{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type NativeTokenId: Get<u32>;
}

// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Vesting Account Created
		VestingAccountCreated(AccountId),
		/// Vesting Account Destroyed
		VestingAccountDestroyed(AccountId),
		/// WithdrewVestingAccount
		WithdrewVestingAccount(AccountId),
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		InsufficientFundsToCreateVestingAccount,
		RecipientAccountAlreadyExists,
		/// Vesting Account Already Exist
		VestingAccountAlreadyExists,
		VestingUidAlreadyExists,
		ExpiryBlockAlreadyExists,
		InitialDepositAlreadyExists,
		CurrentDepositAlreadyExists,
		StakedDepositAlreadyExists,
		/// Invalid UID
		InvalidUID,
		/// Invalid Intial Deposit
		InvalidIntialDeposit,
		/// Invalid Expiry Time
		InvalidExpiryBlock,
		/// Invalid Current Deposit
		InvalidCurrentDeposit,
		/// Invalid Staking
		InvalidStaking,
		/// VestingAccount Does Not Exist
		VestingAccountDoesNotExist,
		/// Invalid Deposit
		InvalidDeposit,
		/// Invalid Epoch Time
		VestingExpired,
		/// InvalidRecipent
		InvalidRecipent,
		VestingUidDoesNotExist,
		ExpiryBlockDoesNotExist,
		InitialDepositDoesNotExist,
		CurrentDepositDoesNotExist,
	}
}

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as VestingModule {
		VestingUid get(fn vesting_uid): map hasher(blake2_128_concat) u32 => ();
		RecipientAccount get(fn recipient_account): map hasher(blake2_128_concat) u32 => T::AccountId;
		VestingAccount get(fn vesting_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => T::AccountId;
		ExpiryBlock get(fn ExpiryBlock): map hasher(blake2_128_concat) u32 => T::BlockNumber;
		InitialDeposit get(fn existing_deposit): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
		CurrentDeposit get(fn current_deposit): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
		StakedDeposit get(fn staked_deposit): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn create_vesting_account(
			origin,
			vesting_uid: u32,
			expiry_block: T::BlockNumber,
			vesting_account: T::AccountId,
			recipient_account: T::AccountId,
			initial_deposit: BalanceOf<T>,
			current_deposit: BalanceOf<T>,
			staked_deposit: BalanceOf<T>,
		) -> DispatchResult {

			let vesting_creator = ensure_signed(origin)?;

			ensure!(vesting_uid > 0, Error::<T>::InvalidUID);
			ensure!(expiry_block > T::BlockNumber::zero(), Error::<T>::InvalidExpiryBlock);

			ensure!(T::NativeCurrency::free_balance(&vesting_creator) >= initial_deposit, Error::<T>::InsufficientFundsToCreateVestingAccount);
			ensure!(!VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidAlreadyExists);
			ensure!(!ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockAlreadyExists);
			ensure!(!RecipientAccount::<T>::contains_key(&vesting_uid), Error::<T>::RecipientAccountAlreadyExists);
			ensure!(!VestingAccount::<T>::contains_key(&vesting_creator, &vesting_uid), Error::<T>::VestingAccountAlreadyExists);
			ensure!(!InitialDeposit::<T>::contains_key(&vesting_uid), Error::<T>::InitialDepositAlreadyExists);
			ensure!(!CurrentDeposit::<T>::contains_key(&vesting_uid), Error::<T>::CurrentDepositAlreadyExists);
			ensure!(!StakedDeposit::<T>::contains_key(&vesting_uid), Error::<T>::StakedDepositAlreadyExists);

			VestingUid::insert(&vesting_uid, ());
			ExpiryBlock::<T>::insert(&vesting_uid, expiry_block);
			VestingAccount::<T>::insert(&vesting_creator.clone(), &vesting_uid, vesting_account.clone());
			RecipientAccount::<T>::insert(&vesting_uid, recipient_account);
			InitialDeposit::<T>::insert(&vesting_uid, initial_deposit);
			CurrentDeposit::<T>::insert(&vesting_uid, current_deposit);
			StakedDeposit::<T>::insert(&vesting_uid, staked_deposit);

			T::NativeCurrency::transfer(&vesting_creator, &vesting_account, initial_deposit)?;

			Self::deposit_event(RawEvent::VestingAccountCreated(vesting_creator));

			Ok(())
		}

		// function to remove a vesting account
		#[weight = 10_000]
		pub fn destroy_vesting_account(origin, vesting_uid: u32) -> DispatchResult {

			let vesting_creator = ensure_signed(origin)?;

			ensure!(VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidDoesNotExist);
			ensure!(ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockDoesNotExist);
			ensure!(VestingAccount::<T>::contains_key(&vesting_creator, &vesting_uid), Error::<T>::VestingAccountDoesNotExist);
			ensure!(InitialDeposit::<T>::contains_key(&vesting_uid), Error::<T>::InitialDepositDoesNotExist);
			ensure!(CurrentDeposit::<T>::contains_key(&vesting_uid), Error::<T>::CurrentDepositDoesNotExist);

			let expiry_block = ExpiryBlock::<T>::get(&vesting_uid);
			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
			ensure!(expiry_block > current_block, Error::<T>::VestingExpired);

			let initial_deposit = InitialDeposit::<T>::get(&vesting_uid);
			let current_deposit = CurrentDeposit::<T>::get(&vesting_uid);
			ensure!(initial_deposit == current_deposit, Error::<T>::InvalidDeposit);

			VestingAccount::<T>::remove(vesting_creator.clone(), &vesting_uid);

			T::NativeCurrency::transfer(&vesting_account, &vesting_creator, initial_deposit)?;

			Self::deposit_event(RawEvent::VestingAccountDestroyed(vesting_creator));

			Ok(())
		}

		 #[weight = 10_000]
		pub fn withdraw_vesting_account(origin, vesting_creator: T::AccountId, vesting_uid: u32) -> DispatchResultWithPostInfo {
			let recipient = ensure_signed(origin)?;

			ensure!(VestingAccount::<T>::contains_key(&vesting_creator, &vesting_uid), Error::<T>::VestingAccountAlreadyExists);
			// decode data
//			let content: Vec<u8> = VestingAccount::<T>::get(vesting_creator.clone(), &uid);
//			let initial_deposit = Self::json_get_value(content.clone(),"initial_deposit".as_bytes().to_vec());
//			let initial_deposit = str::parse::<Balance>(sp_std::str::from_utf8(&initial_deposit).unwrap()).unwrap();
//			let current_deposit = Self::json_get_value(content.clone(),"current_deposit".as_bytes().to_vec());
//			let current_deposit = str::parse::<Balance>(sp_std::str::from_utf8(&current_deposit).unwrap()).unwrap();
//			let expire_time = Self::json_get_value(content.clone(),"expire_time".as_bytes().to_vec());
//			let expire_time = str::parse::<T::BlockNumber>(sp_std::str::from_utf8(&expire_time).unwrap()).ok().unwrap();
//
//			let recipient_account = Self::json_get_value(content.clone(),"recipient_account".as_bytes().to_vec());
//			let recipient_account = T::AccountId::decode(&mut &recipient_account[1..33]).unwrap_or_default();
//			let staking = Self::json_get_value(content.clone(),"staking".as_bytes().to_vec());
//			let staking = str::parse::<Balance>(sp_std::str::from_utf8(&staking).unwrap()).unwrap();
//
//			ensure!(recipient == recipient_account, Error::<T>::InvalidRecipent);
//			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
//			ensure!(initial_deposit == current_deposit , Error::<T>::InvalidDeposit);
//			ensure!(expire_time > current_block, Error::<T>::VestingExpired);

//			pallet_assets::Module::<T>::transfer(RawOrigin::Signed(vesting_creator.clone()).into(), T::NativeTokenId::get(), T::Lookup::unlookup(recipient.clone()), staking)?;
			// Generate event
			Self::deposit_event(RawEvent::WithdrewVestingAccount(vesting_creator));
			// Return a successful DispatchResult
			   Ok(().into())
		}
	}
}

impl<T: Config> Module<T> {
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
