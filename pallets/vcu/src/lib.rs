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
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// A VCU was stored with a serial number.
		VCUStored(u32),
		/// A VCU was updated.
		VCUUpdated(u32, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Invalid IPFS Hash
		InvalidIPFSHash,
		/// Invalid Project id
		InvalidPidLength,
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
	}
}
