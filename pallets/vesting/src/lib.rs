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
    decl_error, decl_event, decl_module, decl_storage,
    weights::Weight,
};
use primitives::Balance;
use codec::{Codec, Encode, Decode};
use sp_runtime::RuntimeDebug;
use frame_system::{self as system, ensure_signed, ensure_root};

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// The overarching event type.
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

decl_storage! {

    trait Store for Module<T: Config> as VestingModule {
        VestingAccount get(fn vesting_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32  => Vesting<T::AccountId, Balance>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Vesting Account Created
        VestingAccountCreated(AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Vesting Account Already Exist
        VestingAccountAlreadyExists,
  }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn create_vesting_account(origin, recipient: T:AccountId, uid: u32, deposit: Balance, expire_time:u32) -> DispatchResult {
            let vesting_creator = ensure_signed(origin)?;
            // Generate event
            Self::deposit_event(RawEvent::VestingAccountCreated(vesting_creator));
            // Return a successful DispatchResult
            Ok(())
        }
    }
}

