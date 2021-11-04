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
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, traits::Get};
use primitives::Balance;
use codec::Decode;
use frame_system::{ensure_root, ensure_signed};
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Vec;
use sp_std::vec;
use alloc::string::ToString;
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

decl_storage! {

	trait Store for Module<T: Config> as VCUModule {
		/// Settings configuration, we define some administrator accounts for the pallet VCU without using the super user account.
		Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
		/// AuthorizedAccountsAGV, we define authorized accounts to store/change the Assets Generating VCU (Verified Carbon Credit).
		AuthorizedAccountsAGV get(fn get_authorized_accounts): map hasher(blake2_128_concat) T::AccountId => Vec<u8>;
		/// AssetsGeneratingVCU (Verified Carbon Credit) should be stored on chain from the authorized accounts.
		AssetsGeneratingVCU get(fn asset_generating_vcu): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Vec<u8>;
		/// AssetsGeneratingVCUShares The AVG shares can be minted/burned from the Authorized account up to the maximum number set in the AssetsGeneratingVCU.
		AssetsGeneratingVCUShares get(fn asset_generating_vcu_shares): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) Vec<u8>  => u32;
		/// AssetsGeneratingVCUSharesMinted
		AssetsGeneratingVCUSharesMinted get(fn asset_generating_vcu_shares_minted): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32  => u32;
		/// AssetsGeneratingVCUSchedule (Verified Carbon Credit) should be stored on chain from the authorized accounts.
		AssetsGeneratingVCUSchedule get(fn asset_generating_vcu_schedule): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Vec<u8>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// New proxy setting has been created.
		SettingsCreated(Vec<u8>,Vec<u8>),
		/// Proxy setting has been destroyed.
        SettingsDestroyed(Vec<u8>),
		/// Added authorized account.
        AuthorizedAccountAdded(AccountId),
		/// Destroyed authorized account.
        AuthorizedAccountsAGVDestroyed(AccountId),
		/// AssetsGeneratingVCU has been stored.
        AssetsGeneratingVCUCreated(u32),
		/// Destroyed AssetGeneratedVCU.
        AssetGeneratingVCUDestroyed(u32),
		/// Minted AssetGeneratedVCU.
        AssetsGeneratingVCUSharesMinted(AccountId, u32),
		/// Burned AssetGeneratedVCU.
        AssetsGeneratingVCUSharesBurned(AccountId, u32),
		/// Transferred AssetGeneratedVCU.
        AssetsGeneratingVCUSharesTransferred(AccountId),
		/// Added AssetsGeneratingVCUSchedule
		AssetsGeneratingVCUScheduleAdded(AccountId, u32),
		/// Destroyed AssetsGeneratingVCUSchedule
		AssetsGeneratingVCUScheduleDestroyed(AccountId, u32),
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
		/// Invalid Description
		InvalidDescription,
		/// AuthorizedAccountsAGV has not been found on the blockchain
		AuthorizedAccountsAGVNotFound,
		/// Settings data is too short to be valid
        AssetGeneratingJsonTooShort,
        /// Settings data is too long to be valid
        AssetGeneratingJsonTooLong,
		/// ProofOwnership Not found
		ProofOwnershipNotFound,
		/// NumberofShares not found
		NumberofSharesNotFound,
		/// Too many NumberofShares
		TooManyNumberofShares,
		/// AssetGeneratedVCU has not been found on the blockchain
		AssetGeneratedVCUNotFound,
		/// Invalid AVGId
		InvalidAVGId,
		/// Too less NumberofShares
		TooLessShares,
		/// InsufficientShares
		InsufficientShares,
		/// Got an overflow after adding
		Overflow,
		/// AssetGeneratedShares has not been found on the blockchain
		AssetGeneratedSharesNotFound,
		/// Invalid VCU Amount
		InvalidVCUAmount,
		/// AssetGeneratedVCUSchedule has not been found on the blockchain
		AssetGeneratedVCUSchedule,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

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

		/// Create new Assets Generating VCU on chain
		///
		/// `create_asset_generating_vcu` will accept `authorized_account`, `signer` and `json content` as parameter
		/// and create new Assets Generating VCU in system
		///
		/// a value: json structure as follows:
		/// {
		///     Description: Vec<u8> (max 64 bytes) (mandatory)
		///     ProofOwnership: ipfs link to a folder with the proof of ownership (mandatory)
		///     OtherDocuments: [{description:string,ipfs:hash},{....}], (Optional)
		///     ExpiringDateTime: DateTime, (YYYY-MM-DD hh:mm:ss) (optional)
		///     NumberofShares: Integer (maximum 10000 shares mandatory)
		/// }
		/// ex: {"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":10000}
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_asset_generating_vcu(origin, authorized_account: T::AccountId, signer: u32, content: Vec<u8>) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			//check accounts json length
			ensure!(content.len() > 12, Error::<T>::AssetGeneratingJsonTooShort);
            ensure!(content.len() < 8192, Error::<T>::AssetGeneratingJsonTooLong);

			// check json validity
			let js = content.clone();
			ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);

			let description = Self::json_get_value(content.clone(),"description".as_bytes().to_vec());
            ensure!(description.len()!=0 && description.len()<=64 , Error::<T>::InvalidDescription);

			let proof_ownership = Self::json_get_value(content.clone(),"proofOwnership".as_bytes().to_vec());
            ensure!(proof_ownership.len()!=0 , Error::<T>::ProofOwnershipNotFound);

			let number_of_shares = Self::json_get_value(content.clone(),"numberOfShares".as_bytes().to_vec());

            ensure!(number_of_shares.len()!=0 , Error::<T>::NumberofSharesNotFound);

			ensure!(str::parse::<i32>(sp_std::str::from_utf8(&number_of_shares).unwrap()).unwrap() <= 10000 , Error::<T>::TooManyNumberofShares);

			AssetsGeneratingVCU::<T>::try_mutate_exists(authorized_account, signer.clone(), |desc| {
				*desc = Some(content);

				// Generate event
				Self::deposit_event(RawEvent::AssetsGeneratingVCUCreated(signer));
				// Return a successful DispatchResult
				Ok(())
			})
		}

		/// Destroy an Asset Generated VCU
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_asset_generating_vcu(origin, account_id: T::AccountId, signer: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			// check whether asset generated VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &signer), Error::<T>::AssetGeneratedVCUNotFound);

			AssetsGeneratingVCU::<T>::remove(account_id, signer.clone());

			// Generate event
			Self::deposit_event(RawEvent::AssetGeneratingVCUDestroyed(signer));
			// Return a successful DispatchResult
			Ok(())

		}

		/// To mint the shares
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_shares_asset_generating_vcu(origin, recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			let avg_id_vec: Vec<&str> = sp_std::str::from_utf8(&agv_id).unwrap().split("-").collect();
			ensure!(avg_id_vec.len() == 2, Error::<T>::InvalidAVGId);


			let (str_account_id, signer): (&str, u32) = (avg_id_vec[0], str::parse::<u32>(avg_id_vec[1]).unwrap());

			let account_id = T::AccountId::decode(&mut &str_account_id.as_bytes().to_vec()[1..33]).unwrap_or_default();

			// check whether asset generated VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &signer), Error::<T>::AssetGeneratedVCUNotFound);

			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &agv_id, |share| -> DispatchResult {
				*share = number_of_shares;
				Ok(())
			})?;

			let content: Vec<u8> = AssetsGeneratingVCU::<T>::get(&account_id, &signer);
			let total_shares = Self::json_get_value(content.clone(),"numberOfShares".as_bytes().to_vec());
			let int_shares = str::parse::<u32>(sp_std::str::from_utf8(&total_shares).unwrap()).unwrap();


			AssetsGeneratingVCUSharesMinted::<T>::try_mutate(&account_id, &signer, |share| -> DispatchResult {
				let total_sh = share.checked_add(number_of_shares).ok_or(Error::<T>::Overflow)?;
				ensure!(total_sh <= int_shares, Error::<T>::TooManyNumberofShares);
				*share = total_sh;
				Ok(())
			})?;

			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesMinted(account_id, signer));
			// Return a successful DispatchResult
			Ok(())
		}

		/// To burn the shares
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn burn_shares_asset_generating_vcu(origin, recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			let avg_id_vec: Vec<&str> = sp_std::str::from_utf8(&agv_id).unwrap().split("-").collect();
			ensure!(avg_id_vec.len() == 2, Error::<T>::InvalidAVGId);


			let (str_account_id, signer): (&str, u32) = (avg_id_vec[0], str::parse::<u32>(avg_id_vec[1]).unwrap());

			let account_id = T::AccountId::decode(&mut &str_account_id.as_bytes().to_vec()[1..33]).unwrap_or_default();

			// check whether asset generated VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &signer), Error::<T>::AssetGeneratedVCUNotFound);

			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &agv_id, |share| -> DispatchResult {
				*share = number_of_shares;
				Ok(())
			})?;

			AssetsGeneratingVCUSharesMinted::<T>::try_mutate(&account_id, &signer, |share| -> DispatchResult {
				let total_sh = share.checked_sub(number_of_shares).ok_or(Error::<T>::InsufficientShares)?;
				ensure!(total_sh >0, Error::<T>::TooLessShares);
				*share = total_sh;
				Ok(())
			})?;

			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesBurned(account_id, signer));
			// Return a successful DispatchResult
			Ok(())
		}

       /// To transfer the shares
	   ///
	   /// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
	   /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn transfer_shares_asset_generating_vcu(origin, sender: T::AccountId, recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			ensure!(AssetsGeneratingVCUShares::<T>::contains_key(&sender, &agv_id), Error::<T>::AssetGeneratedSharesNotFound);

			let sender_shares = AssetsGeneratingVCUShares::<T>::get(&sender, &agv_id);

			// check whether asset generated shares exists or not
			ensure!(number_of_shares >= sender_shares, Error::<T>::NumberofSharesNotFound);

			AssetsGeneratingVCUShares::<T>::try_mutate(&sender, &agv_id, |share| -> DispatchResult {
				let total_sh = share.checked_sub(number_of_shares).ok_or(Error::<T>::TooLessShares)?;
				*share = total_sh;
				Ok(())
			})?;

			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &agv_id, |share| -> DispatchResult {
				let total_sh = share.checked_add(number_of_shares).ok_or(Error::<T>::Overflow)?;
				*share = total_sh;
				Ok(())
			})?;

			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesTransferred(recipient));
			// Return a successful DispatchResult
			Ok(())
		}

		/// To store asset generating vcu schedule
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_asset_generating_vcu_schedule(origin, account_id: T::AccountId, signer: u32, period_days: u32, amount_vcu: Balance, token_id: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			// check whether asset generated VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &signer), Error::<T>::AssetGeneratedVCUNotFound);
			ensure!(amount_vcu > 0, Error::<T>::InvalidVCUAmount);

    		let json = Self::create_json_string(vec![("period_days",&mut period_days.to_string().as_bytes().to_vec()), ("amount_vcu",&mut  amount_vcu.to_string().as_bytes().to_vec()), ("token_id",&mut  token_id.to_string().as_bytes().to_vec())]);

			AssetsGeneratingVCUSchedule::<T>::insert(&account_id, &signer, json);

			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUScheduleAdded(account_id, signer));
			// Return a successful DispatchResult
			Ok(())
		}

		/// To destroy asset generating vcu schedule
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_asset_generating_vcu_schedule(origin, account_id: T::AccountId, signer: u32) -> DispatchResult {

			match ensure_root(origin.clone()) {
				Ok(()) => Ok(()),
				Err(e) => {
					ensure_signed(origin).and_then(|o: T::AccountId| {
						if AuthorizedAccountsAGV::<T>::contains_key(&o) {
							Ok(())
						} else {
							Err(e)
						}
					})
				}
			}?;

			// check whether asset generated VCU exists or not

			ensure!(AssetsGeneratingVCUSchedule::<T>::contains_key(&account_id, &signer), Error::<T>::AssetGeneratedVCUSchedule);

			AssetsGeneratingVCUSchedule::<T>::remove(&account_id, &signer);
			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUScheduleDestroyed(account_id, signer));
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
			let mut xx=0;
			if x+kl>jl {
				break;
			}
			for i in x..x+kl {
				if *j.get(i).unwrap()== *k.get(xx).unwrap() {
					m=m+1;
				}
				xx=xx+1;
			}
			if m==kl{
				let mut lb=b' ';
				let mut op=true;
				let mut os=true;
				for i in x+kl..jl-1 {
					if *j.get(i).unwrap()==b'[' && op==true && os==true{
						os=false;
					}
					if *j.get(i).unwrap()==b'}' && op==true && os==false{
						os=true;
					}
					if *j.get(i).unwrap()==b':' && op==true{
						continue;
					}
					if *j.get(i).unwrap()==b'"' && op==true && lb!=b'\\' {
						op=false;
						continue
					}
					if *j.get(i).unwrap()==b'"' && op==false && lb!=b'\\' {
						break;
					}
					if *j.get(i).unwrap()==b'}' && op==true{
						break;
					}
					if *j.get(i).unwrap()==b']' && op==true{
						break;
					}
					if *j.get(i).unwrap()==b',' && op==true && os==true{
						break;
					}
					result.push(j.get(i).unwrap().clone());
					lb=j.get(i).unwrap().clone();
				}
				break;
			}
		}
		return result;
	}

	fn create_json_string(inputs: Vec<(&str, &mut Vec<u8>)>) -> Vec<u8> {
		let mut v:Vec<u8>= Vec::new();
		v.push(b'{');
		let mut flag = false;

		for (arg, val) in  inputs{
			if flag {
				v.push(b',');
			}
			v.push(b'"');
			for i in arg.as_bytes().to_vec().iter() {
				v.push(i.clone());
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
