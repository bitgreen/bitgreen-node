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
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use frame_support::traits::{Vec, UnixTime};
use sp_std::vec;
use alloc::string::ToString;
use sp_runtime::traits::StaticLookup;
use sp_runtime::traits::One;
use pallet_assets::Asset;
use frame_system::RawOrigin;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128> {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// Veera project id minimum length
	type MinPIDLength: Get<u32>;

	/// Unix time
	type UnixTime: UnixTime;
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
		/// AssetsGeneratingVCUGenerated Minting of Scheduled VCU
		AssetsGeneratingVCUGenerated get(fn vcu_generated): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => u64;
		/// VCUsBurnedAccounts: store the burned vcu for each account
		VCUsBurnedAccounts get(fn vcu_burned_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => u128;
		/// VCUsBurned: store the burned VCU for each type of VCU token
		VCUsBurned get(fn vcu_burned):map hasher(blake2_128_concat) u32 => u128;
		/// OraclesAccountMintingVCU: allow to store the account of the Oracle to mint the VCU for its AVG
		OraclesAccountMintingVCU get(fn oracle_account_generating_vcu): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => T::AccountId;
		/// OraclesTokenMintingVCU: allows to store the tokenid of the Oracle to mint the VCU for its AVG
		OraclesTokenMintingVCU get(fn oracle_tokenid_generating_vcu): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => u32;
		/// BundleAssetsGeneratingVCU: a "bundle" of AVG
		BundleAssetsGeneratingVCU get(fn bundle_asset_generating_vcu): map hasher(blake2_128_concat) u32 => Vec<u8>;
		/// A counter of burned tokens for the signer
		BurnedCounter get(fn get_burn_count): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => u32;
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
		/// Added AssetsGeneratingVCUGenerated.
        AssetsGeneratingVCUGenerated(AccountId, u32),
		/// Added VCUBurned.
        VCUsBurnedAdded(AccountId, u32, u32),
		/// Added OraclesAccountMintingVCU
        OraclesAccountMintingVCUAdded(AccountId, u32, AccountId),
		/// Destroyed OraclesAccountMintingVCUDestroyed
		OraclesAccountMintingVCUDestroyed(AccountId, u32),
		/// OracleAccountVCUMinted
		OracleAccountVCUMinted(AccountId, u32, AccountId),
		/// Added BundleAssetsGeneratingVCU
		AddedBundleAssetsGeneratingVCU(u32),
		/// Destroyed BundleAssetsGeneratingVCU
		DestroyedBundleAssetsGeneratingVCU(u32),
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
		/// ProofOwnership too long
		ProofOwnershipTooLong,
		/// NumberofShares not found
		NumberofSharesNotFound,
		/// Number of share cannot be zero
		NumberofSharesCannotBeZero,
		/// Too many NumberofShares
		TooManyShares,
		/// AssetGeneratedVCU has not been found on the blockchain
		AssetGeneratingVCUNotFound,
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
		AssetGeneratedVCUScheduleNotFound,
		/// Asset does not exist,
		AssetDoesNotExist,
		/// AssetGeneratingSchedule has been Expired
		AssetGeneratedScheduleExpired,
		/// AOraclesAccountMintingVCU Not Found
		OraclesAccountMintingVCUNotFound,
		/// BundleAssetsGeneratingVCU JSON is too short to be valid
        BundleAssetsGeneratingVCUJsonTooShort,
        /// BundleAssetsGeneratingVCU is too long to be valid
        BundleAssetsGeneratingVCUJsonTooLong,
		/// InvalidAVGs
		InvalidAVGs,
		/// Bundle does not exist,
		BundleDoesNotExist,
		/// BundleAssetIdNotSame
		BundleAssetIdNotSame,
		/// The recipient has not shares minted
		RecipientSharesNotFound,
		/// Recipient Shares are less of burning shares
		RecipientSharesLessOfBurningShares,
		/// Total shares are not enough to burn the amount requested
		TotalSharesNotEnough,
		/// Invalid period in days
		InvalidPeriodDays,
		/// The schedule is already present on chain
		ScheduleDuplicated,
		/// The minting time is not not yet arrived based on the schedule
		AssetGeneratedScheduleNotYetArrived,
		/// Token id not found in Assets Pallet
		TokenIdNotFound,
		/// The schedule is already present on chain
		AssetsGeneratingVCUScheduleAlreadyOnChain,
		/// The Oracle account is not matching the signer of the transaction
		OracleAccountNotMatchingSigner,
		/// Token for Oracle has not been found, inconsistency in stored data
		OraclesTokenMintingVCUNotFound,
		/// InsufficientVCUs
		InsufficientVCUs,
		/// Token id must have a value > 10000
		TokenIdMorethanTenThousand,
		/// Asset Already In Use
		AssetAlreadyInUse,
  }
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Create new proxy setting that allow to define some accounts with administrator rights on the pallet.
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

		/// Destroy proxy setting with key:"admin"
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_proxy_settings(origin) -> DispatchResult {
			// check for SUDO 
			ensure_root(origin)?;
			// search for the key of the proxy settings
			let key = "admin".as_bytes().to_vec();
			// check whether setting key exists
			ensure!(Settings::contains_key(&key), Error::<T>::SettingsKeyNotFound);
			// remove the proxy settings
			Settings::remove(key.clone());
			// Generate event
			Self::deposit_event(RawEvent::SettingsDestroyed(key));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Store/update an AuthorizedAccountsAGV
		/// This function allows to store the Accounts enabled to create Assets generating VCU (AGV).
		///
		/// `add_authorized_accounts` will accept `account_id` and `description` as parameter
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_authorized_account(origin, account_id: T::AccountId, description: Vec<u8>) -> DispatchResult {
			// check for SUDO 
			ensure_root(origin)?;
			// description is mandatory 
			ensure!(!description.is_empty(), Error::<T>::InvalidDescription);
			//minimu lenght of 4 chars
			ensure!(description.len()>4, Error::<T>::InvalidDescription);
			// add/replace the description for the account received
			AuthorizedAccountsAGV::<T>::try_mutate_exists(account_id.clone(), |desc| {
				*desc = Some(description);
				// Generate event
				Self::deposit_event(RawEvent::AuthorizedAccountAdded(account_id));
				// Return a successful DispatchResult
				Ok(())
			})
		}

		/// Destroys an authorized account revekin its authorization
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_authorized_account(origin, account_id: T::AccountId) -> DispatchResult {
			// check for SUDO 
			ensure_root(origin)?;
			// check whether authorized account exists or not
			ensure!(AuthorizedAccountsAGV::<T>::contains_key(&account_id), Error::<T>::AuthorizedAccountsAGVNotFound);
			// remove the authorized account from the state
			AuthorizedAccountsAGV::<T>::remove(account_id.clone());
			// Generate event
			Self::deposit_event(RawEvent::AuthorizedAccountsAGVDestroyed(account_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Create new Assets Generating VCU on chain
		///
		/// `create_asset_generating_vcu` will accept `avg_account_id`, `avg_id` and `json content` as parameter
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
		pub fn create_asset_generating_vcu(origin, avg_account_id: T::AccountId, avg_id: u32, content: Vec<u8>) -> DispatchResult {
			// check for SUDO user or owner account
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
			//check content json length
			ensure!(content.len() > 12, Error::<T>::AssetGeneratingJsonTooShort);
            ensure!(content.len() < 8192, Error::<T>::AssetGeneratingJsonTooLong);

			// check json validity
			let js = content.clone();
			ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);
			// checj for description validity
			let description = Self::json_get_value(content.clone(),"description".as_bytes().to_vec());
            ensure!(!description.is_empty() && description.len()<=64 , Error::<T>::InvalidDescription);
			// check for proof of ownership
			let proof_ownership = Self::json_get_value(content.clone(),"proofOwnership".as_bytes().to_vec());
            ensure!(!proof_ownership.is_empty(), Error::<T>::ProofOwnershipNotFound);
			ensure!(proof_ownership.len()<=128, Error::<T>::ProofOwnershipTooLong);
			// check for number of shares
			let number_of_shares = Self::json_get_value(content.clone(),"numberOfShares".as_bytes().to_vec());
            ensure!(!number_of_shares.is_empty() , Error::<T>::NumberofSharesNotFound);
			ensure!(str::parse::<i32>(sp_std::str::from_utf8(&number_of_shares).unwrap()).unwrap() <= 10000 , Error::<T>::TooManyShares);
			ensure!(str::parse::<i32>(sp_std::str::from_utf8(&number_of_shares).unwrap()).unwrap() >0 , Error::<T>::NumberofSharesCannotBeZero);
			// store the asset
			AssetsGeneratingVCU::<T>::try_mutate_exists(avg_account_id, avg_id, |desc| {
				*desc = Some(content);
				// Generate event
				Self::deposit_event(RawEvent::AssetsGeneratingVCUCreated(avg_id));
				// Return a successful DispatchResult
				Ok(())
			})
		}

		/// Destroy Assets Generating VCU from storage.
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_asset_generating_vcu(origin, avg_account_id: T::AccountId, avg_id: u32) -> DispatchResult {
			// check for SUDO or authorized account
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
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratingVCUNotFound);
			// TODO check for VCU already generated to avoid orphans
			// renove the assets generating VCU
			AssetsGeneratingVCU::<T>::remove(avg_account_id, avg_id);
			// Generate event
			Self::deposit_event(RawEvent::AssetGeneratingVCUDestroyed(avg_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// The AVG shares can be minted from the Authorized account up to the maximum number set in the AssetsGeneratingVCU.
		///
		/// ex: avgaccout: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_shares_asset_generating_vcu(origin, recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32) -> DispatchResult {
			// checking for SUDO or authorized account
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
			// split the field avg_account
			let avg_id_vec: Vec<&str> = sp_std::str::from_utf8(&avg_account).unwrap().split('-').collect();
			ensure!(avg_id_vec.len() == 2, Error::<T>::InvalidAVGId);
			let (str_account_id, avg_id): (&str, u32) = (avg_id_vec[0], str::parse::<u32>(avg_id_vec[1]).unwrap());
			let account_id = T::AccountId::decode(&mut &str_account_id.as_bytes().to_vec()[1..33]).unwrap_or_default();
			// check whether asset generating VCU (AGV) exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &avg_id), Error::<T>::AssetGeneratingVCUNotFound);
			
			// read info about the AVG
			let content: Vec<u8> = AssetsGeneratingVCU::<T>::get(&account_id, &avg_id);
			let total_shares = Self::json_get_value(content,"numberOfShares".as_bytes().to_vec());
			let int_shares = str::parse::<u32>(sp_std::str::from_utf8(&total_shares).unwrap()).unwrap();

			// increase the total shares minted
			AssetsGeneratingVCUSharesMinted::<T>::try_mutate(&account_id, &avg_id, |share| -> DispatchResult {
				let total_sh = share.checked_add(number_of_shares).ok_or(Error::<T>::Overflow)?;
				ensure!(total_sh <= int_shares, Error::<T>::TooManyShares);
				*share = total_sh;
				Ok(())
			})?;

			// increase the total shares minted for the recipient
			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &avg_account, |share| -> DispatchResult {
				let total_sha = share.checked_add(number_of_shares).ok_or(Error::<T>::Overflow)?;
				*share = total_sha;
				Ok(())
			})?;
			
			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesMinted(account_id, avg_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// The AVG shares can be burned from the Authorized account in the AssetsGeneratingVCU.
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn burn_shares_asset_generating_vcu(origin, recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32) -> DispatchResult {
			// checking for SUDO or authorized account
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
			// get account  and avg_id
			let avg_id_vec: Vec<&str> = sp_std::str::from_utf8(&avg_account).unwrap().split('-').collect();
			ensure!(avg_id_vec.len() == 2, Error::<T>::InvalidAVGId);
			let (str_account_id, avg_id): (&str, u32) = (avg_id_vec[0], str::parse::<u32>(avg_id_vec[1]).unwrap());
			let account_id = T::AccountId::decode(&mut &str_account_id.as_bytes().to_vec()[1..33]).unwrap_or_default();
			// check whether asset generated VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &avg_id), Error::<T>::AssetGeneratingVCUNotFound);
			// check for previously minted shares for the recipient
			ensure!(AssetsGeneratingVCUShares::<T>::contains_key(&recipient, &avg_account), Error::<T>::RecipientSharesNotFound);
			// check  the number of burnable shares for the recipient
			let currentshares=AssetsGeneratingVCUShares::<T>::get(recipient.clone(), avg_account.clone());
			ensure!(currentshares>=number_of_shares,Error::<T>::RecipientSharesLessOfBurningShares);
			// check the number of burnable shares in total
			ensure!(AssetsGeneratingVCUSharesMinted::<T>::contains_key(&account_id, &avg_id),Error::<T>::TotalSharesNotEnough);
			let totalcurrentshares=AssetsGeneratingVCUSharesMinted::<T>::get(&account_id, &avg_id);
			ensure!(totalcurrentshares>=number_of_shares,Error::<T>::TotalSharesNotEnough);
			// decrease total shares minted
			AssetsGeneratingVCUSharesMinted::<T>::try_mutate(&account_id, &avg_id, |share| -> DispatchResult {
				let total_sh = share.checked_sub(number_of_shares).ok_or(Error::<T>::InsufficientShares)?;
				ensure!(total_sh >0, Error::<T>::TooLessShares);
				*share = total_sh;
				Ok(())
			})?;
			// decrease shares minted for the recipient account
			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &avg_account, |share| -> DispatchResult {
				let total_sha = share.checked_sub(number_of_shares).ok_or(Error::<T>::Overflow)?;
				*share = total_sha;
				Ok(())
			})?;
			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesBurned(account_id, avg_id));
			// Return a successful DispatchResult
			Ok(())
		}
	/// The owner can transfer its own shares to a recipient
	   ///
	   /// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
	   /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
	   #[weight = 10_000 + T::DbWeight::get().writes(1)]
	   pub fn transfer_shares_asset_generating_vcu(origin, recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32) -> DispatchResult {
		    let sender = ensure_signed(origin)?;
		   // check that the shares are present
		   ensure!(AssetsGeneratingVCUShares::<T>::contains_key(&sender, &avg_account), Error::<T>::AssetGeneratedSharesNotFound);
		   // get the shares available
		   let sender_shares = AssetsGeneratingVCUShares::<T>::get(&sender, &avg_account);
		   // check whether shares are enough for the transfer
		   ensure!(number_of_shares <= sender_shares, Error::<T>::NumberofSharesNotFound);
		   // decrease the shares for the sender
		   AssetsGeneratingVCUShares::<T>::try_mutate(&sender, &avg_account, |share| -> DispatchResult {
			   let total_sh = share.checked_sub(number_of_shares).ok_or(Error::<T>::TooLessShares)?;
			   *share = total_sh;
			   Ok(())
		   })?;
		   // increase the shares for the recipient for the same amount
		   AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &avg_account, |share| -> DispatchResult {
			   let total_sh = share.checked_add(number_of_shares).ok_or(Error::<T>::Overflow)?;
			   *share = total_sh;
			   Ok(())
		   })?;
		   // Generate event
		   Self::deposit_event(RawEvent::AssetsGeneratingVCUSharesTransferred(recipient));
		   // Return a successful DispatchResult
		   Ok(())
	   }
       /// The administrator can force a transfer of shares from a sender to a recipient
	   ///
	   /// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
	   /// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn forcetransfer_shares_asset_generating_vcu(origin, sender: T::AccountId, recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32) -> DispatchResult {
			// chec for administrator access
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
			// check that the shares are present
			ensure!(AssetsGeneratingVCUShares::<T>::contains_key(&sender, &avg_account), Error::<T>::AssetGeneratedSharesNotFound);
			// get the shares available
			let sender_shares = AssetsGeneratingVCUShares::<T>::get(&sender, &avg_account);
			// check whether shares are enough for the transfer
			ensure!(number_of_shares <= sender_shares, Error::<T>::NumberofSharesNotFound);
			// decrease the shares for the sender
			AssetsGeneratingVCUShares::<T>::try_mutate(&sender, &avg_account, |share| -> DispatchResult {
				let total_sh = share.checked_sub(number_of_shares).ok_or(Error::<T>::TooLessShares)?;
				*share = total_sh;
				Ok(())
			})?;
			// increase the shares for the recipient for the same amount
			AssetsGeneratingVCUShares::<T>::try_mutate(&recipient, &avg_account, |share| -> DispatchResult {
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
		/// ex: avg_account: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_asset_generating_vcu_schedule(origin, avg_account_id: T::AccountId, avg_id: u32, period_days: u64, amount_vcu: Balance, token_id: u32) -> DispatchResult {
			// check for Sudo or other admnistrator account	
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

			// check whether asset generating VCU exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratingVCUNotFound);
			// check for VCU amount > 0
			ensure!(amount_vcu > 0, Error::<T>::InvalidVCUAmount);
			// check for days >0
			ensure!(period_days > 0, Error::<T>::InvalidPeriodDays);
			// check the schedule is not alreayd on chain
			ensure!(!AssetsGeneratingVCUSchedule::<T>::contains_key(&avg_account_id,&avg_id),Error::<T>::AssetsGeneratingVCUScheduleAlreadyOnChain);
			// check the token id is present on chain
			ensure!(Asset::<T>::contains_key(token_id),Error::<T>::TokenIdNotFound);
			// check the token id > 10000 (because under 10000 reserver for the bridge)
			ensure!(token_id>=10000,Error::<T>::TokenIdMorethanTenThousand);
			// TODO control the property of the tokenid, it should match the one of the AGV for security? Because otherwise even wrapped Eth could be minted
			// create json string
    		let json = Self::create_json_string(vec![("period_days",&mut period_days.to_string().as_bytes().to_vec()), ("amount_vcu",&mut  amount_vcu.to_string().as_bytes().to_vec()), ("token_id",&mut  token_id.to_string().as_bytes().to_vec())]);
			// store the schedule
			AssetsGeneratingVCUSchedule::<T>::insert(&avg_account_id, &avg_id, json);
			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUScheduleAdded(avg_account_id, avg_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// To destroy asset generating vcu schedule
		///
		/// ex: avg_id: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_asset_generating_vcu_schedule(origin, avg_account_id: T::AccountId, avg_id: u32) -> DispatchResult {
			// check for Sudo or other admnistrator account	
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
			ensure!(AssetsGeneratingVCUSchedule::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratedVCUScheduleNotFound);
			// remove the schedule
			AssetsGeneratingVCUSchedule::<T>::remove(&avg_account_id, &avg_id);
			// Generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUScheduleDestroyed(avg_account_id, avg_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// This function allows the minting of the VCU periodically. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.
		///
		/// This function checks if it’s time to mint new VCU based on the schedule and the previous generated VCU stored in AssetsGeneratingVCUGenerated or
		/// if it’s time to generate new VCU, it mints the scheduled “Assets” (see Assets pallets), and stores in AssetsGeneratingVCUGenerated  a json structure with the following fields:
		/// ```json
		/// {
		/// “timestamp”: u32  (epoch time in seconds)
		/// “amountvcu”: i32,
		/// }
		/// ```
		/// The function must deny further minting once is done till the new schedule is expired.
		/// For example with a schedule every year, the minting will be executed only one time every 365 days.
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		/// the first minting can be done anytime, the  following minting not before the scheduled time
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_scheduled_vcu(origin, avg_account_id: T::AccountId, avg_id: u32) -> DispatchResultWithPostInfo {
			// check for Sudo or other admnistrator account	
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
			ensure!(AssetsGeneratingVCUSchedule::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratedVCUScheduleNotFound);
			let content: Vec<u8> = AssetsGeneratingVCUSchedule::<T>::get(avg_account_id.clone(), &avg_id);
			let period_days = Self::json_get_value(content.clone(),"period_days".as_bytes().to_vec());
			let period_days = str::parse::<u64>(sp_std::str::from_utf8(&period_days).unwrap()).unwrap();
			let token_id = Self::json_get_value(content.clone(),"token_id".as_bytes().to_vec());
			let token_id = str::parse::<u32>(sp_std::str::from_utf8(&token_id).unwrap()).unwrap();
			let amount_vcu = Self::json_get_value(content,"amount_vcu".as_bytes().to_vec());
			let amount_vcu = str::parse::<Balance>(sp_std::str::from_utf8(&amount_vcu).unwrap()).unwrap();
			let mut timestamp:u64=0;
			let now:u64 = T::UnixTime::now().as_secs();
			// check for the last minting done
			if AssetsGeneratingVCUGenerated::<T>::contains_key(&avg_account_id, &avg_id) {
				timestamp = AssetsGeneratingVCUGenerated::<T>::get(&avg_account_id, &avg_id);
				
			}
			let elapse:u64=period_days*24*60;
			ensure!(now+elapse<=timestamp,Error::<T>::AssetGeneratedScheduleNotYetArrived);

			ensure!(!Asset::<T>::contains_key(token_id),Error::<T>::AssetAlreadyInUse);
			pallet_assets::Module::<T>::force_create(RawOrigin::Root.into(), token_id, T::Lookup::unlookup(avg_account_id.clone()), One::one(), One::one())?;

			// TODO - Minting must be in favor of the shares holders accounts in proportion to the number of shares
			// mint the assets
			pallet_assets::Module::<T>::mint(RawOrigin::Signed(avg_account_id.clone()).into(), token_id, T::Lookup::unlookup(avg_account_id.clone()), amount_vcu)?;
			// store the last minting time in AssetsGeneratingVCUGenerated
			if AssetsGeneratingVCUGenerated::<T>::contains_key(&avg_account_id, &avg_id){
				AssetsGeneratingVCUGenerated::<T>::take(&avg_account_id, &avg_id);		
			}
			AssetsGeneratingVCUGenerated::<T>::insert(&avg_account_id, &avg_id,now);		
			// generate event
			Self::deposit_event(RawEvent::AssetsGeneratingVCUGenerated(avg_account_id, avg_id));
			// return
			Ok(().into())
		}

		/// The owner of the “VCUs”  can decide anytime to “retire”, basically burning them.
		///
		/// The dispatch origin for this call must be `Signed` from the owner of the VCU
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn retire_vcu(origin, avg_account_id: T::AccountId, avg_id: u32, amount: u128) -> DispatchResultWithPostInfo {
			// check for a signed transaction
			let sender = ensure_signed(origin)?;
			// check for the schedule of the assetid
			ensure!(AssetsGeneratingVCUSchedule::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratedVCUScheduleNotFound);
			let content: Vec<u8> = AssetsGeneratingVCUSchedule::<T>::get(avg_account_id.clone(), &avg_id);
			let token_id = Self::json_get_value(content.clone(),"token_id".as_bytes().to_vec());
			let token_id = str::parse::<u32>(sp_std::str::from_utf8(&token_id).unwrap()).unwrap();
			// check for enough balance
			let amount_vcu = pallet_assets::Module::<T>::balance(token_id, sender.clone());
			ensure!(amount_vcu >= amount, Error::<T>::InsufficientVCUs);

			// burn the tokens on assets pallet for the requested amount
			pallet_assets::Module::<T>::burn(RawOrigin::Signed(sender.clone()).into(), token_id, T::Lookup::unlookup(avg_account_id.clone()), amount)?;
			// increase the counter of burned VCU for the signer of th transaction
			BurnedCounter::<T>::try_mutate(&sender, &token_id, |count| -> DispatchResult {
				*count += 1;
				Ok(())
			})?;
			//increase burned VCU for the AVG
			VCUsBurnedAccounts::<T>::try_mutate(&avg_account_id, &avg_id, |vcu| -> DispatchResult {
				let total_vcu = vcu.checked_add(amount).ok_or(Error::<T>::Overflow)?;
				*vcu = total_vcu;
				Ok(())
			})?;
			// increase global counter burned VCU
			VCUsBurned::try_mutate(&token_id, |vcu| -> DispatchResult {
				let total_vcu = vcu.checked_add(amount).ok_or(Error::<T>::Overflow)?;
				*vcu = total_vcu;
				Ok(())
			})?;
			// Generate event
			Self::deposit_event(RawEvent::VCUsBurnedAdded(avg_account_id, avg_id, token_id));
			// Return a successful DispatchResult
			Ok(().into())
		}

		/// The VCUs may be generated from Oracle collecting data from off-chain. For example a Solar Panel field may have an Oracle collecting the
		/// output power and generating the VCUs periodically on Chain. We have allowed the account of the Oracle to mint the VCU for his AVG.
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_oracle_account_minting_vcu(origin, avg_account_id: T::AccountId, avg_id: u32, oracle_account_id: T::AccountId,token_id: u32) -> DispatchResult {
			// check for SUDO or administrator accounts
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
			// check if the AGV exists or not
			ensure!(AssetsGeneratingVCU::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::AssetGeneratingVCUNotFound);
			// check token id >10000
			ensure!(token_id>=10000,Error::<T>::TokenIdMorethanTenThousand);
			// store the token if assigned for the Oracle
			if OraclesTokenMintingVCU::<T>::contains_key(avg_account_id.clone(), avg_id.clone()) {
				OraclesTokenMintingVCU::<T>::take(avg_account_id.clone(), avg_id.clone());
			}
			OraclesTokenMintingVCU::<T>::insert(avg_account_id.clone(), avg_id.clone(),token_id.clone());
			//store the oracle or replace if already present, we allow only one oracle for each AGV
			OraclesAccountMintingVCU::<T>::try_mutate_exists(avg_account_id.clone(), avg_id, |oracle| {
				*oracle = Some(oracle_account_id.clone());
				// Generate event
				Self::deposit_event(RawEvent::OraclesAccountMintingVCUAdded(avg_account_id, avg_id, oracle_account_id));
				// Return a successful DispatchResult
				Ok(())
			})
		}

		/// Removes Oracles Generating VCU from storage.
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_oracle_account_minting_vcu(origin, avg_account_id: T::AccountId, avg_id: u32) -> DispatchResult {
			//store the oracle or replace if already present, we allow only one oracle for each AGV
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
			// check for Oracle presence on chain
			ensure!(OraclesAccountMintingVCU::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::OraclesAccountMintingVCUNotFound);
			// remove the Oracle Account
			OraclesAccountMintingVCU::<T>::remove(avg_account_id.clone(), &avg_id);
			// remove the Oracle Token Id
			OraclesTokenMintingVCU::<T>::remove(avg_account_id.clone(), &avg_id);
			// Generate event
			Self::deposit_event(RawEvent::OraclesAccountMintingVCUDestroyed(avg_account_id, avg_id));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Mints Oracles Generating VCUs
		///
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_vcu_from_oracle(origin, avg_account_id: T::AccountId, avg_id: u32, amount_vcu: Balance) -> DispatchResultWithPostInfo {
			let sender=ensure_signed(origin)?;
			// check for Oracle presence on chain
			ensure!(OraclesAccountMintingVCU::<T>::contains_key(&avg_account_id, &avg_id), Error::<T>::OraclesAccountMintingVCUNotFound);
			// check for matching signer with Oracle Account
			let oracle_account: T::AccountId = OraclesAccountMintingVCU::<T>::get(&avg_account_id, &avg_id);
			ensure!(oracle_account==sender,Error::<T>::OracleAccountNotMatchingSigner);
			// check for Token id in Oracle configuration
			ensure!(OraclesTokenMintingVCU::<T>::contains_key(&avg_account_id, &avg_id),Error::<T>::OraclesTokenMintingVCUNotFound);
			// get the token id
			let token_id=OraclesTokenMintingVCU::<T>::get(&avg_account_id, &avg_id);
			if !Asset::<T>::contains_key(token_id) {
				pallet_assets::Module::<T>::force_create(RawOrigin::Root.into(), token_id, T::Lookup::unlookup(oracle_account.clone()), One::one(), One::one())?;
			}
			//TODO - The VCU have to be minted in favour of the shareholders of the AGV
			pallet_assets::Module::<T>::mint(RawOrigin::Signed(oracle_account.clone()).into(), token_id, T::Lookup::unlookup(avg_account_id.clone()), amount_vcu)?;
			// generate event
			Self::deposit_event(RawEvent::OracleAccountVCUMinted(avg_account_id, avg_id, oracle_account));
			Ok(().into())
		}

		/// To store a "bundle" of AGV that has the constraint of using the same "asset id"
		/// but potentially different schedules or Oracle for the generation of the VCU.
		///
		/// example: {"description":"xxxxxxx","agvs":[{"accountid","xxxxxxx","id":xx},{..}],assetid:xx}
		/// The dispatch origin for this call must be `Signed` either by the Root or authorized account.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_bundle_avg(origin, bundle_id: u32, info: Vec<u8>) -> DispatchResult {
			// check for SUDO or administrator user
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
			ensure!(info.len() > 12, Error::<T>::BundleAssetsGeneratingVCUJsonTooShort);
            ensure!(info.len() < 8192, Error::<T>::BundleAssetsGeneratingVCUJsonTooLong);

			// check json validity
			let js = info.clone();
			ensure!(Self::json_check_validity(js),Error::<T>::InvalidJson);
			// check for description validity
			let description = Self::json_get_value(info.clone(),"description".as_bytes().to_vec());
            ensure!(!description.is_empty() && description.len()<=64 , Error::<T>::InvalidDescription);
			// check for asset id
			let asset_id = Self::json_get_value(info.clone(),"assetid".as_bytes().to_vec());
			let asset_id = str::parse::<u32>(sp_std::str::from_utf8(&asset_id).unwrap()).unwrap();
			// check whether asset exists or not
			ensure!(Asset::<T>::contains_key(asset_id), Error::<T>::AssetDoesNotExist);
			// check the validity of the AGV in the array
			let agvs= Self::json_get_complexarray(info.clone(),"agvs".as_bytes().to_vec());
			let mut x=0;
			if agvs.len()>2 {
				loop {
					let w= Self::json_get_recordvalue(agvs.clone(),x);
					if w.is_empty() {
						break;
					}
					let account_id= Self::json_get_value(w.clone(),"accountid".as_bytes().to_vec());
					let id= Self::json_get_value(w.clone(),"id".as_bytes().to_vec());

					let account_id = T::AccountId::decode(&mut &account_id[1..33]).unwrap_or_default();
					let id = str::parse::<u32>(sp_std::str::from_utf8(&id).unwrap()).unwrap();
					// check whether asset generated VCU exists or not
					ensure!(AssetsGeneratingVCU::<T>::contains_key(&account_id, &id), Error::<T>::AssetGeneratingVCUNotFound);
					x += 1;
				}
			}
			ensure!(x>0,Error::<T>::InvalidAVGs);
			BundleAssetsGeneratingVCU::insert(&bundle_id, &info);
			Self::deposit_event(RawEvent::AddedBundleAssetsGeneratingVCU(bundle_id));

			Ok(())
		}

		/// Destroys an AVG bundle from storage.
		///
		/// The dispatch origin for this call must be `Signed` by the Root.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn destroy_bundle_avg(origin, bundle_id: u32) -> DispatchResult {
			// check for SUDO or administrator user
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
			// check if the bundle is on chain
			ensure!(BundleAssetsGeneratingVCU::contains_key(&bundle_id), Error::<T>::BundleDoesNotExist);
			// remove the bundle from the chain
			BundleAssetsGeneratingVCU::remove(bundle_id);
			// Generate event
			Self::deposit_event(RawEvent::DestroyedBundleAssetsGeneratingVCU(bundle_id));
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
		if *j.get(0).unwrap()==b'{' && *j.last().unwrap()!=b'}' {
			return false;
		}
		// checks start/end with []
		if *j.get(0).unwrap()==b'[' && *j.last().unwrap()!=b']' {
			return false;
		}
		// check that the start is { or [
		if *j.get(0).unwrap()!=b'{' && *j.get(0).unwrap()!=b'[' {
			return false;
		}
		//checks that end is } or ]
		if *j.last().unwrap()!=b'}' && *j.last().unwrap()!=b']' {
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
			if b==b']' && s && !ps {
				ps=true;
			}

			if b==b'{' && s {
				pg=false;
			}
			if b==b'}' && s && !pg {
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
		true
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

	// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
	fn json_get_recordvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
		let mut result=Vec::new();
		let mut op=true;
		let mut cn=0;
		let mut lb=b' ';
		for b in ar {
			if b==b',' && op {
				cn += 1;
				continue;
			}
			if b==b'[' && op && lb!=b'\\' {
				continue;
			}
			if b==b']' && op && lb!=b'\\' {
				continue;
			}
			if b==b'{' && op && lb!=b'\\' {
				op=false;
			}
			if b==b'}' && !op && lb!=b'\\' {
				op=true;
			}
			// field found
			if cn==p {
				result.push(b);
			}
			lb = b;
		}
		result
	}

	fn json_get_complexarray(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
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
				let mut os=true;
				for i in x+kl..jl-1 {
					if *j.get(i).unwrap()==b'[' && os{
						os=false;
					}
					result.push(*j.get(i).unwrap());
					if *j.get(i).unwrap()==b']' && !os {
						break;
					}
				}
				break;
			}
		}
		result
	}
}
