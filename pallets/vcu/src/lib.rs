#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, traits::Get};
use frame_system::ensure_signed;
use primitives::Balance;
use codec::{Decode, Encode};
use frame_support::ensure;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VCU {
	pub serial_number: Vec<u8>,
	pub project: Vec<u8>,
	pub amount_co2: Balance,
	pub photos: Vec<u8>,
	pub videos: Vec<u8>,
	pub documentation: Vec<u8>,
}

decl_storage! {

	trait Store for Module<T: Config> as VCUModule {
		VCUs get(fn get_vcu): map hasher(blake2_128_concat) T::AccountId => VCU;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		VCUStored(AccountId, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		VCUAlreadyExists
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_vcu(origin, serial_number: Vec<u8>, project: Vec<u8>, amount_co2: Balance, photos: Vec<u8>, videos: Vec<u8>, documentation: Vec<u8>) -> DispatchResult {

			let who = ensure_signed(origin)?;

			ensure!(!VCUs::<T>::contains_key(who.clone()), Error::<T>::VCUAlreadyExists);
			let vcu = VCU {
				serial_number: serial_number.clone(),
				project,
				amount_co2,
				photos,
				videos,
				documentation
			};

			VCUs::<T>::insert(who.clone(), vcu);

			Self::deposit_event(RawEvent::VCUStored(who, serial_number));
			Ok(())
		}
	}
}
