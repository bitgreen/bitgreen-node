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
use codec::{Decode, Encode};
use core::str;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::DispatchResultWithPostInfo,
	traits::{Get, WithdrawReasons},
};
use frame_system::{ensure_root, ensure_signed, RawOrigin};
use orml_traits::{BasicCurrency, MultiCurrency};
use pallet_staking as Staking;
use primitives::Balance;
use sp_runtime::{
	traits::{StaticLookup, Zero},
	Permill,
};
use sp_std::{prelude::*, vec};
use Staking::{Call::bond, RewardDestination};

type BalanceOf<T> = <<T as module_currencies::Config>::MultiCurrency as MultiCurrency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

/// Module configuration
pub trait Config:
	frame_system::Config
	+ pallet_assets::Config<AssetId = u32, Balance = u128>
	+ module_currencies::Config
	+ pallet_staking::Config
{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type NativeTokenId: Get<u32>;
	type StakingLockPortion: Get<Permill>;
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
		UnbondedValue(AccountId),
		VestingCancelled(AccountId),
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
		LockedDepositAlreadyExists,
		/// Invalid UID
		InvalidUID,
		/// Invalid Intial Deposit
		InvalidIntialDeposit,
		/// Invalid Expiry Time
		InvalidExpiryBlock,
		/// Invalid Staking
		InvalidStaking,
		/// VestingAccount Does Not Exist
		VestingAccountDoesNotExist,
		/// Invalid Deposit
		InvalidDeposit,
		/// Invalid Epoch Time
		VestingExpired,
		/// InvalidRecipient
		InvalidRecipient,
		VestingUidDoesNotExist,
		ExpiryBlockDoesNotExist,
		LockedDepositDoesNotExist,
		InvalidVestingBalance,
		RecipientAccountDoesNotExist,
		VestingCreatorAlreadyExists,
		VestingCreatorDoesNotExist,
		VestingUidForRecipientAlreadyExists,
		VestingUidForRecipientDoesNotExist,
		VestingRateAlreadyExists,
		InitialBlockAlreadyExists,
		VestingTickDoesNotExist,
		VestingRateDoesNotExist,
		InitialBlockDoesNotExist,
		VestingTickAlreadyExists,
		NoBondedController,
		ValueExceedsBondedValue,
		CannotWithdraw,
	}
}

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as VestingModule {
		VestingUid get(fn vesting_uid): map hasher(blake2_128_concat) u32 => ();
		VestingUidForRecipient get(fn vesting_uid_for_recipient): map hasher(blake2_128_concat) T::AccountId => u32;
		RecipientAccount get(fn recipient_account): map hasher(blake2_128_concat) u32 => T::AccountId;
		VestingAccount get(fn vesting_account): map hasher(blake2_128_concat) u32 => T::AccountId;
		VestingCreator get(fn vesting_creator): map hasher(blake2_128_concat) u32 => T::AccountId;
		ExpiryBlock get(fn ExpiryBlock): map hasher(blake2_128_concat) u32 => T::BlockNumber;
		LockedDeposit get(fn existing_deposit): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
		InitialBlock get(fn initial_block): map hasher(blake2_128_concat) u32 => T::BlockNumber;
		VestingRate get(fn vesting_rate): map hasher(blake2_128_concat) u32 => Permill;
		VestingTick get(fn vesting_tick): map hasher(blake2_128_concat) u32 => T::BlockNumber;
		UnlockedAndFixed get(fn unlocked_deposit): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
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
			locked_deposit: BalanceOf<T>,
			vesting_rate: Permill,
			vesting_tick: T::BlockNumber,
		) -> DispatchResult {

			let _ = ensure_root(origin.clone())?;
			let vesting_creator = ensure_signed(origin)?;

			ensure!(vesting_uid > 0, Error::<T>::InvalidUID);
			ensure!(expiry_block > T::BlockNumber::zero(), Error::<T>::InvalidExpiryBlock);

			ensure!(T::NativeCurrency::free_balance(&vesting_creator) >= locked_deposit, Error::<T>::InsufficientFundsToCreateVestingAccount);
			ensure!(!VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidAlreadyExists);
			ensure!(!VestingUidForRecipient::<T>::contains_key(&recipient_account), Error::<T>::VestingUidForRecipientAlreadyExists);
			ensure!(!ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockAlreadyExists);
			ensure!(!RecipientAccount::<T>::contains_key(&vesting_uid), Error::<T>::RecipientAccountAlreadyExists);
			ensure!(!VestingAccount::<T>::contains_key(&vesting_uid), Error::<T>::VestingAccountAlreadyExists);
			ensure!(!VestingCreator::<T>::contains_key(&vesting_uid), Error::<T>::VestingCreatorAlreadyExists);
			ensure!(!LockedDeposit::<T>::contains_key(&vesting_uid), Error::<T>::LockedDepositAlreadyExists);
			ensure!(!InitialBlock::<T>::contains_key(&vesting_uid), Error::<T>::InitialBlockAlreadyExists);
			ensure!(!VestingRate::contains_key(&vesting_uid), Error::<T>::VestingRateAlreadyExists);
			ensure!(!VestingTick::<T>::contains_key(&vesting_uid), Error::<T>::VestingTickAlreadyExists);

			VestingUid::insert(&vesting_uid, ());
			VestingUidForRecipient::<T>::insert(&recipient_account, vesting_uid);
			ExpiryBlock::<T>::insert(&vesting_uid, expiry_block);
			VestingAccount::<T>::insert(&vesting_uid, vesting_account.clone());
			VestingCreator::<T>::insert(&vesting_uid, vesting_creator.clone());
			RecipientAccount::<T>::insert(&vesting_uid, recipient_account);
			LockedDeposit::<T>::insert(&vesting_uid, locked_deposit);

			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
			InitialBlock::<T>::insert(&vesting_uid, current_block);
			VestingRate::insert(&vesting_uid, vesting_rate);
			VestingTick::<T>::insert(&vesting_uid, vesting_tick);

			T::NativeCurrency::transfer(&vesting_creator, &vesting_account, locked_deposit)?;

			Self::deposit_event(RawEvent::VestingAccountCreated(vesting_creator));

			Ok(())
		}

		#[weight = 10_000]
		pub fn cancel_vesting(
			origin,
			vesting_uid: u32,
		) -> DispatchResult {

			let _ = ensure_root(origin)?;

			ensure!(VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidDoesNotExist);
			ensure!(ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockDoesNotExist);
			ensure!(RecipientAccount::<T>::contains_key(&vesting_uid), Error::<T>::RecipientAccountDoesNotExist);
			ensure!(VestingAccount::<T>::contains_key(&vesting_uid), Error::<T>::VestingAccountDoesNotExist);
			ensure!(VestingCreator::<T>::contains_key(&vesting_uid), Error::<T>::VestingCreatorDoesNotExist);
			ensure!(LockedDeposit::<T>::contains_key(&vesting_uid), Error::<T>::LockedDepositDoesNotExist);
			ensure!(InitialBlock::<T>::contains_key(&vesting_uid), Error::<T>::InitialBlockDoesNotExist);
			ensure!(VestingRate::contains_key(&vesting_uid), Error::<T>::VestingRateDoesNotExist);
			ensure!(VestingTick::<T>::contains_key(&vesting_uid), Error::<T>::VestingTickDoesNotExist);

			let expiry_block = ExpiryBlock::<T>::get(&vesting_uid);
			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
			ensure!(expiry_block > current_block, Error::<T>::VestingExpired);

			let locked_deposit = LockedDeposit::<T>::get(&vesting_uid);

			let recipient_account = RecipientAccount::<T>::get(&vesting_uid);
			let vesting_account = VestingAccount::<T>::get(&vesting_uid);
			let vesting_creator = VestingCreator::<T>::get(&vesting_uid);
			let initial_block = InitialBlock::<T>::get(&vesting_uid);
			let vesting_rate = VestingRate::get(&vesting_uid);
			let vesting_tick = VestingTick::<T>::get(&vesting_uid);

			let mut vesting_balance = T::NativeCurrency::free_balance(&vesting_account);
			ensure!(vesting_balance >= locked_deposit, Error::<T>::InvalidVestingBalance);

			let unlocked = Self::calculate_unlocked(
				initial_block,
				current_block,
				vesting_rate,
				vesting_tick,
				vesting_balance
				);

			if T::NativeCurrency::free_balance(&vesting_account) >= unlocked {
				T::NativeCurrency::transfer(&vesting_account, &recipient_account, unlocked)?;
				VestingUidForRecipient::<T>::remove(&recipient_account);
				RecipientAccount::<T>::remove(&vesting_uid);
			} else {
				UnlockedAndFixed::<T>::insert(&vesting_uid, unlocked);
			}

			VestingRate::insert(&vesting_uid, Permill::from_percent(0));

			Self::deposit_event(RawEvent::VestingCancelled(vesting_creator));

			Ok(())

		}

		// function to remove a vesting account
		#[weight = 10_000]
		pub fn destroy_vesting_account(
			origin,
			vesting_uid: u32
		) -> DispatchResult {

			let _ = ensure_root(origin)?;

			ensure!(VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidDoesNotExist);
			ensure!(ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockDoesNotExist);
			ensure!(VestingAccount::<T>::contains_key(&vesting_uid), Error::<T>::VestingAccountDoesNotExist);
			ensure!(VestingCreator::<T>::contains_key(&vesting_uid), Error::<T>::VestingCreatorDoesNotExist);
			ensure!(LockedDeposit::<T>::contains_key(&vesting_uid), Error::<T>::LockedDepositDoesNotExist);
			ensure!(InitialBlock::<T>::contains_key(&vesting_uid), Error::<T>::InitialBlockDoesNotExist);
			ensure!(VestingRate::contains_key(&vesting_uid), Error::<T>::VestingRateDoesNotExist);
			ensure!(VestingTick::<T>::contains_key(&vesting_uid), Error::<T>::VestingTickDoesNotExist);

			let expiry_block = ExpiryBlock::<T>::get(&vesting_uid);
			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
			ensure!(expiry_block > current_block, Error::<T>::VestingExpired);

			let locked_deposit = LockedDeposit::<T>::get(&vesting_uid);

			let vesting_account = VestingAccount::<T>::get(&vesting_uid);
			let vesting_creator = VestingCreator::<T>::get(&vesting_uid);
			let initial_block = InitialBlock::<T>::get(&vesting_uid);
			let vesting_rate = VestingRate::get(&vesting_uid);
			let vesting_tick = VestingTick::<T>::get(&vesting_uid);

			let mut vesting_balance = T::NativeCurrency::free_balance(&vesting_account);
			ensure!(vesting_balance >= locked_deposit, Error::<T>::InvalidVestingBalance);

			let unlocked_and_fixed = UnlockedAndFixed::<T>::get(&vesting_uid);
			let unlocked = if unlocked_and_fixed > BalanceOf::<T>::zero() { unlocked_and_fixed } else {
				Self::calculate_unlocked(
				initial_block,
				current_block,
				vesting_rate,
				vesting_tick,
				vesting_balance
				)
			};

			// check if funds are staked.
			// if they are then vesting can still be cancelled but funds cannot be withdrawn until
			// the vesting account is unbonded.
			ensure!(T::NativeCurrency::free_balance(&vesting_account) >= vesting_balance, Error::<T>::CannotWithdraw);

			// vesting has not been cancelled so transfer unlocked funds.
			// if cancel_vesting has been called this has already been done.
			if RecipientAccount::<T>::contains_key(&vesting_uid) {
				let recipient_account = RecipientAccount::<T>::get(&vesting_uid);
				T::NativeCurrency::transfer(&vesting_account, &recipient_account, unlocked)?;
			}

			// transfer locked funds back to the vesting creator
			T::NativeCurrency::transfer(&vesting_account, &vesting_creator, vesting_balance - unlocked)?;

			// cleanup
			if RecipientAccount::<T>::contains_key(&vesting_uid) {
				let recipient_account = RecipientAccount::<T>::get(&vesting_uid);
				VestingUidForRecipient::<T>::remove(&recipient_account);
			}

			VestingUid::remove(&vesting_uid);
			ExpiryBlock::<T>::remove(&vesting_uid);
			VestingAccount::<T>::remove(&vesting_uid);
			VestingCreator::<T>::remove(&vesting_uid);
			RecipientAccount::<T>::remove(&vesting_uid);
			LockedDeposit::<T>::remove(&vesting_uid);
			InitialBlock::<T>::remove(&vesting_uid);
			VestingRate::remove(&vesting_uid);
			VestingTick::<T>::remove(&vesting_uid);
			UnlockedAndFixed::<T>::remove(&vesting_uid);

			Self::deposit_event(RawEvent::VestingAccountDestroyed(vesting_creator));

			Ok(())
		}

		#[weight = 10_000]
		pub fn withdraw_vesting_account(
			origin,
		) -> DispatchResultWithPostInfo {

			let who = ensure_signed(origin)?;

			ensure!(VestingUidForRecipient::<T>::contains_key(&who), Error::<T>::VestingUidForRecipientDoesNotExist);

			let vesting_uid = VestingUidForRecipient::<T>::get(&who);

			ensure!(VestingUid::contains_key(&vesting_uid), Error::<T>::VestingUidDoesNotExist);
			ensure!(ExpiryBlock::<T>::contains_key(&vesting_uid), Error::<T>::ExpiryBlockDoesNotExist);
			ensure!(VestingAccount::<T>::contains_key(&vesting_uid), Error::<T>::VestingAccountDoesNotExist);
			ensure!(VestingCreator::<T>::contains_key(&vesting_uid), Error::<T>::VestingCreatorDoesNotExist);
			ensure!(RecipientAccount::<T>::contains_key(&vesting_uid), Error::<T>::RecipientAccountDoesNotExist);
			ensure!(LockedDeposit::<T>::contains_key(&vesting_uid), Error::<T>::LockedDepositDoesNotExist);
			ensure!(InitialBlock::<T>::contains_key(&vesting_uid), Error::<T>::InitialBlockDoesNotExist);
			ensure!(VestingRate::contains_key(&vesting_uid), Error::<T>::VestingRateDoesNotExist);
			ensure!(VestingTick::<T>::contains_key(&vesting_uid), Error::<T>::VestingTickDoesNotExist);

			let recipient_account = RecipientAccount::<T>::get(&vesting_uid);
			ensure!(who == recipient_account, Error::<T>::InvalidRecipient);

			let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
			let expiry_block = ExpiryBlock::<T>::get(&vesting_uid);
			ensure!(expiry_block > current_block, Error::<T>::VestingExpired);

			let locked_deposit = LockedDeposit::<T>::get(&vesting_uid);

			let vesting_account = VestingAccount::<T>::get(&vesting_uid);
			let vesting_creator = VestingCreator::<T>::get(&vesting_uid);
			let initial_block = InitialBlock::<T>::get(&vesting_uid);
			let vesting_rate = VestingRate::get(&vesting_uid);
			let vesting_tick = VestingTick::<T>::get(&vesting_uid);

			let vesting_balance = T::NativeCurrency::free_balance(&vesting_account);
			ensure!(vesting_balance >= locked_deposit, Error::<T>::InvalidVestingBalance);

			let unlocked_and_fixed = UnlockedAndFixed::<T>::get(&vesting_uid);
			let unlocked = if unlocked_and_fixed > BalanceOf::<T>::zero() { unlocked_and_fixed } else {
				Self::calculate_unlocked(
				initial_block,
				current_block,
				vesting_rate,
				vesting_tick,
				vesting_balance
				)
			};

			// check if staked
			ensure!(T::NativeCurrency::free_balance(&vesting_account) >= unlocked, Error::<T>::CannotWithdraw);

			T::NativeCurrency::transfer(&vesting_account, &recipient_account, unlocked)?;

			// staking rewards are treated as if they have been locked and vesting since initial_block.
			LockedDeposit::<T>::insert(&vesting_uid, vesting_balance - unlocked);

			// update initial_block
			InitialBlock::<T>::insert(&vesting_uid, current_block);

			T::NativeCurrency::transfer(&vesting_account, &recipient_account, unlocked)?;

			UnlockedAndFixed::<T>::remove(&vesting_uid);

			Self::deposit_event(RawEvent::WithdrewVestingAccount(vesting_creator));

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

	fn calculate_unlocked(
		initial_block: T::BlockNumber,
		current_block: T::BlockNumber,
		vesting_rate: Permill,
		vesting_tick: T::BlockNumber,
		locked_deposit: BalanceOf<T>,
	) -> BalanceOf<T> {
		// todo: prove this is right
		// vesting rate is the fraction of locked to be unlocked per x blocks
		// x is vesting tick
		// a partially completed tick does not unlock anything
		let mut a = current_block - initial_block;
		let mut unlocked = BalanceOf::<T>::zero();
		while a >= vesting_tick {
			unlocked += vesting_rate * locked_deposit;
			a -= vesting_tick;
		}
		unlocked
	}
}
