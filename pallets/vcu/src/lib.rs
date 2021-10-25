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
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, traits::Get};
use primitives::Balance;
use codec::{Decode, Encode};
use frame_system::ensure_root;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// IPFS content valid length.
	type IpfsHashLength: Get<u32>;

	/// Veera project id minimum length
	type MinPIDLength: Get<u32>;
}

/// Verified Carbon Units (VCU) The VCU data (serial number, project, amount of CO2 in tons,
/// photos, videos, documentation) will  be stored off-chain on IPFS (www.ipfs.io).
/// IPFS uses a unique hash of 32 bytes to pull the data when necessary.
#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VCU {
	pub amount_co2: Balance,
	pub ipfs_hash: Vec<u8>,
}

decl_storage! {

	trait Store for Module<T: Config> as VCUModule {
		/// VCUs stored in system
		VCUs get(fn get_vcu): map hasher(blake2_128_concat) u32 => Vec<VCU>;
		/// Settings configuration, we define some administrator accounts for the pallet VCU without using the super user account.
		Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
		/// AuthorizedAccountsAGV, we define authorized accounts to store/change the Assets Generating VCU (Verified Carbon Credit).
		AuthorizedAccountsAGV get(fn get_authorized_accounts): map hasher(blake2_128_concat) T::AccountId => Vec<u8>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// A VCU was stored with a serial number.
		VCUStored(u32),
		/// A VCU was updated.
		VCUUpdated(u32, AccountId),
		/// New proxy setting has been created.
		SettingsCreated(Vec<u8>,Vec<u8>),
		/// Proxy setting has been destroyed.
        SettingsDestroyed(Vec<u8>),
		/// Added authorized account.
        AuthorizedAccountAdded(AccountId),
		/// Destroyed authorized account.
        AuthorizedAccountsAGVDestroyed(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Invalid IPFS Hash
		InvalidIPFSHash,
		/// Invalid Project id
		InvalidPidLength,
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
		/// Invalid Description
		InvalidDescription,
		/// AuthorizedAccountsAGV has not been found on the blockchain
		AuthorizedAccountsAGVNotFound,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Create new VCU on chain
		///
		/// `create_vcu` will accept `pid`, `amount_co2` and `ipfs_hash` as parameter
		/// and create new VCU in system
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_vcu(origin, pid: u32, amount_co2: Balance, ipfs_hash: Vec<u8>) -> DispatchResult {

			ensure_root(origin)?;

			ensure!(ipfs_hash.len() == T::IpfsHashLength::get() as usize, Error::<T>::InvalidIPFSHash);
			ensure!(pid > T::MinPIDLength::get(), Error::<T>::InvalidPidLength);
			VCUs::try_mutate(pid, |vcu_details| -> DispatchResult {
				let vcu = VCU {
					amount_co2,
					ipfs_hash
				};
				vcu_details.push(vcu);

				Ok(())
			})?;

			Self::deposit_event(RawEvent::VCUStored(pid));
			Ok(())
		}

		/// Create new proxy setting
		///
		/// key=="admin" {"accounts": ["accountid1", "accountid2"] }
		/// `create_proxy_settings` will accept `accounts` as parameter
		/// and create new proxy setting in system with key `admin`
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_proxy_settings(origin, accounts: Vec<u8>) -> DispatchResult {

			ensure_root(origin)?;

			//check accounts json length
			ensure!(accounts.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(accounts.len() < 8192, Error::<T>::SettingsJsonTooLong);

			// check json validity
			let js=accounts.clone();
			ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);

			let key = "admin".as_bytes().to_vec();

			// check whether setting key already exists
			ensure!(!Settings::contains_key(&key), Error::<T>::SettingsKeyExists);

			Settings::insert(key.clone(),accounts.clone());
			// Generate event
			Self::deposit_event(RawEvent::SettingsCreated(key,accounts));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Destroy proxy setting
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_proxy_settings(origin) -> DispatchResult {

			ensure_root(origin)?;

			let key = "admin".as_bytes().to_vec();

			// check whether setting key exists or not
			ensure!(Settings::contains_key(&key), Error::<T>::SettingsKeyNotFound);

			Settings::remove(key.clone());

			// Generate event
			Self::deposit_event(RawEvent::SettingsDestroyed(key));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Store/update an AuthorizedAccountsAGV
		///
		/// `add_authorized_accounts` will accept `account_id` and `description` as parameter
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_authorized_account(origin, account_id: T::AccountId, description: Vec<u8>) -> DispatchResult {

			ensure_root(origin)?;
			ensure!(description.len()!=0, Error::<T>::InvalidDescription);

			AuthorizedAccountsAGV::<T>::try_mutate_exists(account_id.clone(), |desc| {
				*desc = Some(description);

				// Generate event
				Self::deposit_event(RawEvent::AuthorizedAccountAdded(account_id));
				// Return a successful DispatchResult
				Ok(())
			})
		}

		/// Destroy an AuthorizedAccountsAGV
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_authorized_account(origin, account_id: T::AccountId) -> DispatchResult {

			ensure_root(origin)?;

			// check whether authorized account exists or not
			ensure!(AuthorizedAccountsAGV::<T>::contains_key(&account_id), Error::<T>::AuthorizedAccountsAGVNotFound);

			AuthorizedAccountsAGV::<T>::remove(account_id.clone());

			// Generate event
			Self::deposit_event(RawEvent::AuthorizedAccountsAGVDestroyed(account_id));
			// Return a successful DispatchResult
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {

	// function to validate a json string for no/std. It does not allocate of memory
	fn json_check_validity(j:Vec<u8>) -> bool{
		// minimum lenght of 2
		if j.len()<2 {
			return false;
		}
		// checks star/end with {}
		if *j.get(0).unwrap()==b'{' && *j.get(j.len()-1).unwrap()!=b'}' {
			return false;
		}
		// checks start/end with []
		if *j.get(0).unwrap()==b'[' && *j.get(j.len()-1).unwrap()!=b']' {
			return false;
		}
		// check that the start is { or [
		if *j.get(0).unwrap()!=b'{' && *j.get(0).unwrap()!=b'[' {
			return false;
		}
		//checks that end is } or ]
		if *j.get(j.len()-1).unwrap()!=b'}' && *j.get(j.len()-1).unwrap()!=b']' {
			return false;
		}
		//checks " opening/closing and : as separator between name and values
		let mut s:bool=true;
		let mut d:bool=true;
		let mut pg:bool=true;
		let mut ps:bool=true;
		let mut bp = b' ';
		for b in j {
			if b==b'[' && s {
				ps=false;
			}
			if b==b']' && s && ps==false {
				ps=true;
			}

			if b==b'{' && s {
				pg=false;
			}
			if b==b'}' && s && pg==false {
				pg=true;
			}

			if b == b'"' && s && bp != b'\\' {
				s=false;
				bp=b;
				d=false;
				continue;
			}
			if b == b':' && s {
				d=true;
				bp=b;
				continue;
			}
			if b == b'"' && !s && bp != b'\\' {
				s=true;
				bp=b;
				d=true;
				continue;
			}
			bp=b;
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
		return true;
	}
}
