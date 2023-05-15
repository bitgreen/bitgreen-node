// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! Bitgreen DEX Pallet
//! The DEX pallet allows permissionless listing and buying of carbon credits. The pallet currently
//! only supports fixed price purchase of carbon credits from a listing. A user can create a listing
//! with the amount of Carbon credits for sale and the price expected for each unit, this sale order
//! remains onchain until cancelled by the user or completely filled. While the listing is active,
//! any user can call buy_order specifying the number of Carbon credits to purchase, the amount from
//! the buyer is transferred to the seller and any fees applicable to the pallet account.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create_sell_order`: Creates a new sell order onchain
//! * `cancel_sell_order`: Cancel an existing sell order
//! * `buy_order`: Purchase units from exising sell order
//!
//! ### Permissioned Functions
//!
//! * `force_set_purchase_fee` : Set the purchase fee percentage for the dex
//! * `force_set_payment_fee` : Set the payment fee percentage for the dex
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use codec::{Decode, Encode, MaxEncodedLen};

use frame_support::RuntimeDebug;

pub use pallet::*;
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::{types::*, WeightInfo};
	use frame_support::{
		pallet_prelude::*,
		traits::{fungibles::Transfer, Contains},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::MultiCurrency;
	use primitives::CarbonCreditsValidator;
	use sp_runtime::{
		traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One, Zero},
		Percent,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The units in which we record currency balance.
		type CurrencyBalance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u128>;

		/// The units in which we record assets
		type AssetBalance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u128>;

		// Asset manager config
		type Asset: Transfer<Self::AccountId, Balance = Self::AssetBalance>;

		// Token handler config - this is what the pallet accepts as payment
		type Currency: MultiCurrency<Self::AccountId, Balance = Self::CurrencyBalance>;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Verify if the asset can be listed on the dex
		type AssetValidator: CarbonCreditsValidator<AssetId = AssetIdOf<Self>>;

		/// The minimum units of asset to create a sell order
		#[pallet::constant]
		type MinUnitsToCreateSellOrder: Get<AssetBalanceOf<Self>>;

		/// The minimum price per unit of asset to create a sell order
		#[pallet::constant]
		type MinPricePerUnit: Get<CurrencyBalanceOf<Self>>;

		/// The maximum payment fee that can be set
		#[pallet::constant]
		type MaxPaymentFee: Get<Percent>;

		/// The maximum purchase fee that can be set
		#[pallet::constant]
		type MaxPurchaseFee: Get<CurrencyBalanceOf<Self>>;

		/// The DEX pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The maximum validators for a payment
		type MaxValidators: Get<u32> + TypeInfo + Clone;

		/// The maximum length of tx hash that can be stored on chain
		type MaxTxHashLen: Get<u32> + TypeInfo + Clone;

		/// KYC provider config
		type KYCProvider: Contains<Self::AccountId>;

		/// The expiry time for buy order
		type BuyOrderExpiryTime: Get<Self::BlockNumber>;
	}

	// orders information
	#[pallet::storage]
	#[pallet::getter(fn order_count)]
	pub type OrderCount<T: Config> = StorageValue<_, OrderId, ValueQuery>;

	// orders information
	#[pallet::storage]
	#[pallet::getter(fn buy_order_count)]
	pub type BuyOrderCount<T: Config> = StorageValue<_, BuyOrderId, ValueQuery>;

	// Payment fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn payment_fees)]
	pub type PaymentFees<T: Config> = StorageValue<_, Percent, ValueQuery>;

	// purchase fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn purchase_fees)]
	pub type PurchaseFees<T: Config> = StorageValue<_, CurrencyBalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn order_info)]
	pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, OrderInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn buy_order_info)]
	pub type BuyOrders<T: Config> = StorageMap<_, Blake2_128Concat, BuyOrderId, BuyOrderInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn validator_accounts)]
	// List of ValidatorAccounts for the pallet
	pub type ValidatorAccounts<T: Config> = StorageValue<_, ValidatorAccountsListOf<T>, ValueQuery>;

	#[pallet::type_value]
	pub fn DefaultMinPaymentValidators<T: Config>() -> u32 {
		2u32
	}

	// Min validations required before a payment is accepted
	#[pallet::storage]
	#[pallet::getter(fn min_payment_validators)]
	pub type MinPaymentValidations<T: Config> =
		StorageValue<_, u32, ValueQuery, DefaultMinPaymentValidators<T>>;

	// Seller receivables from sales
	#[pallet::storage]
	#[pallet::getter(fn seller_receivables)]
	pub type SellerReceivables<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, CurrencyBalanceOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new sell order has been created
		SellOrderCreated {
			order_id: OrderId,
			asset_id: AssetIdOf<T>,
			project_id: ProjectIdOf<T>,
			group_id: GroupIdOf<T>,
			units: AssetBalanceOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
			owner: T::AccountId,
		},
		/// A sell order was cancelled
		SellOrderCancelled { order_id: OrderId, seller: T::AccountId },
		/// A buy order was processed successfully
		BuyOrderCreated {
			order_id: OrderId,
			sell_order_id: OrderId,
			units: AssetBalanceOf<T>,
			project_id: ProjectIdOf<T>,
			group_id: GroupIdOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
			fees_paid: CurrencyBalanceOf<T>,
			total_amount: CurrencyBalanceOf<T>,
			seller: T::AccountId,
			buyer: T::AccountId,
		},
		/// A new ValidatorAccount has been added
		ValidatorAccountAdded { account_id: T::AccountId },
		/// An ValidatorAccount has been removed
		ValidatorAccountRemoved { account_id: T::AccountId },
		/// A buy order payment was validated
		BuyOrderPaymentValidated { order_id: BuyOrderId, chain_id: u32, validator: T::AccountId },
		/// A buy order was completed successfully
		BuyOrderFilled {
			order_id: BuyOrderId,
			sell_order_id: OrderId,
			units: AssetBalanceOf<T>,
			project_id: ProjectIdOf<T>,
			group_id: GroupIdOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
			fees_paid: CurrencyBalanceOf<T>,
			seller: T::AccountId,
			buyer: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error when calculating orderId
		OrderIdOverflow,
		/// The orderId does not exist
		InvalidOrderId,
		/// Only the order owner can perform this call
		InvalidOrderOwner,
		/// The expected asset_id does not match the order
		InvalidAssetId,
		/// Error when calculating order units
		OrderUnitsOverflow,
		/// The amount does not cover fees + transaction
		InsufficientCurrency,
		/// Below minimum price
		BelowMinimumPrice,
		/// Below minimum units
		BelowMinimumUnits,
		/// Arithmetic overflow
		ArithmeticError,
		/// Asset not permitted to be listed
		AssetNotPermitted,
		/// Seller and buyer cannot be same
		SellerAndBuyerCannotBeSame,
		/// Cannot set more than the maximum payment fee
		CannotSetMoreThanMaxPaymentFee,
		/// The fee amount exceeds the limit set by user
		FeeExceedsUserLimit,
		/// The purchasea fee amount exceeds the limit
		CannotSetMoreThanMaxPurchaseFee,
		/// not authorized to perform action
		NotAuthorised,
		ValidatorAccountAlreadyExists,
		TooManyValidatorAccounts,
		ChainIdMismatch,
		TxProofMismatch,
		KYCAuthorisationFailed,
		DuplicateValidation,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// Look for expired buy orders and remove from storage
		fn on_idle(block: T::BlockNumber, remaining_weight: Weight) -> Weight {
			let mut remaining_weight = remaining_weight;
			for (key, buy_order) in BuyOrders::<T>::iter() {
				remaining_weight = remaining_weight.saturating_sub(T::DbWeight::get().reads(1));
				if buy_order.expiry_time < block {
					// log the start of removal
					log::info!(
						target: "runtime::dex",
						"INFO: Found expired buy order, going to remove buy_order_id: {}",
						key
					);
					BuyOrders::<T>::take(key);
					remaining_weight =
						remaining_weight.saturating_sub(T::DbWeight::get().writes(1));
					// add the credits to the sell order
					let sell_order_updated = Orders::<T>::try_mutate(
						buy_order.order_id,
						|maybe_order| -> DispatchResult {
							let order = maybe_order.as_mut().ok_or(Error::<T>::InvalidOrderId)?;
							order.units = order
								.units
								.checked_add(&buy_order.units)
								.ok_or(Error::<T>::OrderUnitsOverflow)?;
							Ok(())
						},
					);

					if sell_order_updated.is_err() {
						log::warn!(
							target: "runtime::dex",
							"WARNING: Sell order units not credited back for buy_order_id: {}",
							key
						);
					}

					log::info!(
						target: "runtime::dex",
						"INFO: Removed Expired buy order with buy_order_id: {}",
						key
					);

					// exit since we altered the map
					break
				}

				if remaining_weight.all_lte(T::DbWeight::get().reads(1)) {
					break
				}
			}
			remaining_weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new sell order for given `asset_id`
		#[transactional]
		#[pallet::weight(T::WeightInfo::create_sell_order())]
		pub fn create_sell_order(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;
			Self::check_kyc_approval(&seller)?;
			// ensure the asset_id can be listed
			let (project_id, group_id) = T::AssetValidator::get_project_details(&asset_id)
				.ok_or(Error::<T>::AssetNotPermitted)?;

			// ensure minimums are satisfied
			ensure!(units >= T::MinUnitsToCreateSellOrder::get(), Error::<T>::BelowMinimumUnits);
			ensure!(price_per_unit >= T::MinPricePerUnit::get(), Error::<T>::BelowMinimumPrice);

			// transfer assets from seller to pallet
			T::Asset::transfer(asset_id, &seller, &Self::account_id(), units, false)?;

			let order_id = Self::order_count();
			let next_order_id =
				order_id.checked_add(One::one()).ok_or(Error::<T>::OrderIdOverflow)?;
			OrderCount::<T>::put(next_order_id);

			// order values
			Orders::<T>::insert(
				order_id,
				OrderInfo { owner: seller.clone(), units, price_per_unit, asset_id },
			);

			Self::deposit_event(Event::SellOrderCreated {
				order_id,
				asset_id,
				project_id,
				group_id,
				units,
				price_per_unit,
				owner: seller,
			});

			Ok(())
		}

		/// Cancel an existing sell order with `order_id`
		#[transactional]
		#[pallet::weight(T::WeightInfo::cancel_sell_order())]
		pub fn cancel_sell_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;

			// check validity
			let order = Orders::<T>::take(order_id).ok_or(Error::<T>::InvalidOrderId)?;

			ensure!(seller == order.owner, Error::<T>::InvalidOrderOwner);

			// transfer assets from pallet to seller
			T::Asset::transfer(
				order.asset_id,
				&Self::account_id(),
				&order.owner,
				order.units,
				false,
			)?;

			Self::deposit_event(Event::SellOrderCancelled { order_id, seller });
			Ok(())
		}

		/// Buy `units` of `asset_id` from the given `order_id`
		/// This will be called by one of the approved validators when an order is created
		#[transactional]
		#[pallet::weight(T::WeightInfo::buy_order())]
		pub fn create_buy_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			max_fee: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;
			Self::check_kyc_approval(&buyer)?;

			if units.is_zero() {
				return Ok(())
			}

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let mut order = maybe_order.take().ok_or(Error::<T>::InvalidOrderId)?;

				// ensure the expected asset matches the order
				ensure!(asset_id == order.asset_id, Error::<T>::InvalidAssetId);

				// ensure the seller and buyer are not the same
				ensure!(buyer != order.owner, Error::<T>::SellerAndBuyerCannotBeSame);

				// ensure volume remaining can cover the buy order
				ensure!(units <= order.units, Error::<T>::OrderUnitsOverflow);

				// get the projectId and groupId for events
				let (project_id, group_id) = T::AssetValidator::get_project_details(&asset_id)
					.ok_or(Error::<T>::AssetNotPermitted)?;

				// reduce the buy_order units from total volume
				order.units =
					order.units.checked_sub(&units).ok_or(Error::<T>::OrderUnitsOverflow)?;

				// calculate fees
				let units_as_u128: u128 =
					units.try_into().map_err(|_| Error::<T>::ArithmeticError)?;
				let price_per_unit_as_u128: u128 =
					order.price_per_unit.try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				let required_currency = price_per_unit_as_u128
					.checked_mul(units_as_u128)
					.ok_or(Error::<T>::ArithmeticError)?;

				let payment_fee = PaymentFees::<T>::get().mul_ceil(required_currency);
				let purchase_fee: u128 =
					PurchaseFees::<T>::get().try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				let total_fee =
					payment_fee.checked_add(purchase_fee).ok_or(Error::<T>::OrderUnitsOverflow)?;

				let total_amount = total_fee
					.checked_add(required_currency)
					.ok_or(Error::<T>::OrderUnitsOverflow)?;

				ensure!(max_fee >= total_fee.into(), Error::<T>::FeeExceedsUserLimit);

				// Create buy order
				let buy_order_id = Self::buy_order_count();
				let next_buy_order_id =
					buy_order_id.checked_add(One::one()).ok_or(Error::<T>::OrderIdOverflow)?;
				BuyOrderCount::<T>::put(next_buy_order_id);

				let current_block_number = <frame_system::Pallet<T>>::block_number();
				let expiry_time = current_block_number
					.checked_add(&T::BuyOrderExpiryTime::get())
					.ok_or(Error::<T>::OrderIdOverflow)?;

				BuyOrders::<T>::insert(
					buy_order_id,
					BuyOrderInfo {
						order_id,
						buyer: buyer.clone(),
						units,
						price_per_unit: order.price_per_unit,
						asset_id,
						total_fee: total_fee.into(),
						total_amount: total_amount.into(),
						expiry_time,
						payment_info: None,
					},
				);

				Self::deposit_event(Event::BuyOrderCreated {
					order_id: buy_order_id,
					sell_order_id: order_id,
					units,
					project_id,
					group_id,
					price_per_unit: order.price_per_unit,
					fees_paid: total_fee.into(),
					total_amount: total_amount.into(),
					seller: order.owner.clone(),
					buyer,
				});

				*maybe_order = Some(order);

				Ok(())
			})
		}

		/// Force set PaymentFees value
		/// Can only be called by ForceOrigin
		#[transactional]
		#[pallet::weight(T::WeightInfo::force_set_payment_fee())]
		pub fn force_set_payment_fee(origin: OriginFor<T>, payment_fee: Percent) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			ensure!(
				payment_fee <= T::MaxPaymentFee::get(),
				Error::<T>::CannotSetMoreThanMaxPaymentFee
			);
			PaymentFees::<T>::set(payment_fee);
			Ok(())
		}

		/// Force set PurchaseFee value
		/// Can only be called by ForceOrigin
		#[transactional]
		#[pallet::weight(T::WeightInfo::force_set_purchase_fee())]
		pub fn force_set_purchase_fee(
			origin: OriginFor<T>,
			purchase_fee: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			ensure!(
				purchase_fee <= T::MaxPurchaseFee::get(),
				Error::<T>::CannotSetMoreThanMaxPurchaseFee
			);
			PurchaseFees::<T>::set(purchase_fee);
			Ok(())
		}

		/// Buy `units` of `asset_id` from the given `order_id`
		/// This will be called by one of the approved validators when an order is created
		#[transactional]
		#[pallet::weight(T::WeightInfo::buy_order())]
		pub fn validate_buy_order(
			origin: OriginFor<T>,
			order_id: BuyOrderId,
			chain_id: u32,
			tx_proof: BoundedVec<u8, T::MaxTxHashLen>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::check_validator_account(&sender)?;

			// fetch the buy order
			BuyOrders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let mut order = maybe_order.take().ok_or(Error::<T>::InvalidOrderId)?;

				let mut payment_info = order.payment_info.clone();

				// if paymentInfo exists, validate against existing payment
				if let Some(mut payment_info) = payment_info {
					ensure!(payment_info.chain_id == chain_id, Error::<T>::ChainIdMismatch);
					ensure!(payment_info.tx_proof == tx_proof, Error::<T>::TxProofMismatch);
					ensure!(
						!payment_info.validators.contains(&sender),
						Error::<T>::DuplicateValidation
					);

					payment_info
						.validators
						.try_push(sender.clone())
						.map_err(|_| Error::<T>::TooManyValidatorAccounts)?;

					order.payment_info = Some(payment_info.clone());

					Self::deposit_event(Event::BuyOrderPaymentValidated {
						order_id,
						chain_id,
						validator: sender.clone(),
					});

					// process payment if we have reached threshold
					if payment_info.validators.len() as u32 >= Self::min_payment_validators() {
						// fetch the sell order details
						let sell_order =
							Orders::<T>::get(order.order_id).ok_or(Error::<T>::InvalidOrderId)?;

						// transfer the asset to the buyer
						T::Asset::transfer(
							order.asset_id,
							&Self::account_id(),
							&order.buyer,
							order.units,
							false,
						)?;

						// add amount record to the seller
						SellerReceivables::<T>::try_mutate(
							sell_order.owner.clone(),
							|receivable| -> DispatchResult {
								let current_receivables =
									receivable.get_or_insert_with(Default::default);
								let amount_to_seller = order
									.total_amount
									.checked_sub(&order.total_fee)
									.ok_or(Error::<T>::OrderUnitsOverflow)?;
								let new_receivables = current_receivables
									.checked_add(&amount_to_seller)
									.ok_or(Error::<T>::OrderUnitsOverflow)?;
								*receivable = Some(new_receivables);
								Ok(())
							},
						)?;

						// get the projectId and groupId for events
						let (project_id, group_id) =
							T::AssetValidator::get_project_details(&order.asset_id)
								.ok_or(Error::<T>::AssetNotPermitted)?;

						Self::deposit_event(Event::BuyOrderFilled {
							order_id,
							sell_order_id: order.order_id,
							units: order.units,
							project_id,
							group_id,
							price_per_unit: order.price_per_unit,
							fees_paid: order.total_fee,
							seller: sell_order.owner,
							buyer: order.buyer,
						});

						// remove from storage if we reached the threshold and payment executed
						return Ok(())
					}

					*maybe_order = Some(order);

					Ok(())
				}
				// else if paymentInfo is empty create it
				else {
					let mut validators: BoundedVec<T::AccountId, T::MaxValidators> =
						Default::default();
					validators
						.try_push(sender.clone())
						.map_err(|_| Error::<T>::TooManyValidatorAccounts)?;
					payment_info = Some(PaymentInfo { chain_id, tx_proof, validators });

					order.payment_info = payment_info;

					Self::deposit_event(Event::BuyOrderPaymentValidated {
						order_id,
						chain_id,
						validator: sender.clone(),
					});

					*maybe_order = Some(order);

					Ok(())
				}
			})
		}

		/// Add a new account to the list of authorised Accounts
		/// The caller must be from a permitted origin
		#[transactional]
		#[pallet::weight(T::WeightInfo::force_set_purchase_fee())]
		pub fn force_add_validator_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			// add the account_id to the list of authorized accounts
			ValidatorAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
				ensure!(
					!account_list.contains(&account_id),
					Error::<T>::ValidatorAccountAlreadyExists
				);

				account_list
					.try_push(account_id.clone())
					.map_err(|_| Error::<T>::TooManyValidatorAccounts)?;
				Ok(())
			})?;

			Self::deposit_event(Event::ValidatorAccountAdded { account_id });
			Ok(())
		}

		/// Remove an account from the list of authorised accounts
		#[transactional]
		#[pallet::weight(T::WeightInfo::force_set_purchase_fee())]
		pub fn force_remove_validator_account(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			// remove the account_id from the list of authorized accounts if already exists
			ValidatorAccounts::<T>::try_mutate(|account_list| -> DispatchResult {
				if let Ok(index) = account_list.binary_search(&account_id) {
					account_list.swap_remove(index);
					Self::deposit_event(Event::ValidatorAccountRemoved { account_id });
				}

				Ok(())
			})
		}

		/// Set the minimum validators required to validator a payment
		#[transactional]
		#[pallet::weight(T::WeightInfo::force_set_purchase_fee())]
		pub fn force_set_min_validations(
			origin: OriginFor<T>,
			min_validators: u32,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			MinPaymentValidations::<T>::set(min_validators);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID of the CarbonCredits pallet
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Checks if the given account_id is part of authorized account list
		pub fn check_validator_account(account_id: &T::AccountId) -> DispatchResult {
			let validator_accounts = ValidatorAccounts::<T>::get();
			if !validator_accounts.contains(account_id) {
				Err(Error::<T>::NotAuthorised.into())
			} else {
				Ok(())
			}
		}

		/// Checks if given account is kyc approved
		pub fn check_kyc_approval(account_id: &T::AccountId) -> DispatchResult {
			if !T::KYCProvider::contains(account_id) {
				Err(Error::<T>::KYCAuthorisationFailed.into())
			} else {
				Ok(())
			}
		}
	}
}
