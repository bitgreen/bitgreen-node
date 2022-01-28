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

use sp_std::prelude::*;
use core::str;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch, ensure
};
use primitives::Balance;
use codec::{Encode, Decode};
use sp_runtime::RuntimeDebug;
use frame_system::ensure_signed;


/// Module configuration
pub trait Config: frame_system::Config  {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Vesting<AccountId, Balance> {
	pub recipient_account: AccountId, // the account of the recipient
	pub vesting_account: AccountId, // the account id of this vesting account
	pub initial_deposit: Balance, // the initial deposit of the vesting
	pub expire_epoch_time: u32, // epoch time of the vesting expiring date
	pub current_deposit: Balance, // current deposit (the initial_deposit less staking or withdrawn)
	pub staking: Balance, // The amount locked in staking
}

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as VestingModule {
		VestingAccount get(fn vesting_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32  => Vesting<T::AccountId, Balance>;	}
}
// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Vesting Account Created
        VestingAccountCreated(AccountId),
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Vesting Account Already Exist
        VestingAccountAlreadyExists,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;
		/// Store a new deposit, used for the genesis of the blockchain (no gas fees are charged because it's part of the genesis)
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
		) -> dispatch::DispatchResult {
			let vesting_creator = ensure_signed(origin)?;

			ensure!(!VestingAccount::<T>::contains_key(&vesting_creator, &uid), Error::<T>::VestingAccountAlreadyExists);
			let vesting = Vesting {
				recipient_account,
				vesting_account,
				initial_deposit,
				expire_epoch_time: expire_time,
				current_deposit,
				staking
			};

			VestingAccount::<T>::insert(vesting_creator.clone(), uid, vesting);
            // Generate event
            Self::deposit_event(RawEvent::VestingAccountCreated(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
		}

	}
}

