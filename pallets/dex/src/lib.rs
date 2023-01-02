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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct OrderInfo<AccountId, AssetId, AssetBalance, TokenBalance> {
	owner: AccountId,
	units: AssetBalance,
	price_per_unit: TokenBalance,
	asset_id: AssetId,
}

pub type OrderId = u128;

#[frame_support::pallet]
pub mod pallet {
	use crate::{OrderId, OrderInfo, WeightInfo};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Transfer},
			Contains,
		},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::MultiCurrency;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedSub, One, Zero},
		Percent,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub type CurrencyBalanceOf<T> =
		<<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type CurrencyIdOf<T> = <<T as Config>::Currency as MultiCurrency<
		<T as frame_system::Config>::AccountId,
	>>::CurrencyId;

	pub type AssetBalanceOf<T> =
		<<T as Config>::Asset as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	pub type AssetIdOf<T> =
		<<T as Config>::Asset as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// Asset manager config
		type Asset: Transfer<Self::AccountId>;

		// Token handler config - this is what the pallet accepts as payment
		type Currency: MultiCurrency<Self::AccountId>;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// Verify if the asset can be listed on the dex
		type AssetValidator: Contains<AssetIdOf<Self>>;

		/// The CurrencyId of the stable currency we accept as payment
		#[pallet::constant]
		type StableCurrencyId: Get<CurrencyIdOf<Self>>;

		/// The minimum units of asset to create a sell order
		#[pallet::constant]
		type MinUnitsToCreateSellOrder: Get<AssetBalanceOf<Self>>;

		/// The minimum price per unit of asset to create a sell order
		#[pallet::constant]
		type MinPricePerUnit: Get<CurrencyBalanceOf<Self>>;

		/// The maximum payment fee that can be set
		#[pallet::constant]
		type MaxPaymentFee: Get<Percent>;

		/// The DEX pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	// owner of swap pool
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

	// orders information
	#[pallet::storage]
	#[pallet::getter(fn order_count)]
	pub type OrderCount<T: Config> = StorageValue<_, OrderId, ValueQuery>;

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
	pub type Orders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		OrderId,
		OrderInfo<T::AccountId, AssetIdOf<T>, AssetBalanceOf<T>, CurrencyBalanceOf<T>>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new sell order has been created
		SellOrderCreated {
			order_id: OrderId,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
			owner: T::AccountId,
		},
		/// A sell order was cancelled
		SellOrderCancelled { order_id: OrderId },
		/// A buy order was processed successfully
		BuyOrderFilled {
			order_id: OrderId,
			units: AssetBalanceOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
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

			// ensure the asset_id can be listed
			ensure!(T::AssetValidator::contains(&asset_id), Error::<T>::AssetNotPermitted);

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

			Self::deposit_event(Event::SellOrderCancelled { order_id });
			Ok(())
		}

		/// Buy `units` of `asset_id` from the given `order_id`
		#[transactional]
		#[pallet::weight(T::WeightInfo::buy_order())]
		pub fn buy_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			max_fee: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin.clone())?;

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

				// reduce the buy_order units from total volume
				order.units =
					order.units.checked_sub(&units).ok_or(Error::<T>::OrderUnitsOverflow)?;

				// calculate fees
				let units_as_u32: u32 =
					units.try_into().map_err(|_| Error::<T>::ArithmeticError)?;
				let price_per_unit_as_u32: u32 =
					order.price_per_unit.try_into().map_err(|_| Error::<T>::ArithmeticError)?;
				let required_currency = price_per_unit_as_u32
					.checked_mul(units_as_u32)
					.ok_or(Error::<T>::ArithmeticError)?;

				let payment_fee = PaymentFees::<T>::get().mul_ceil(required_currency);
				let purchase_fee: u32 =
					PurchaseFees::<T>::get().try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				let required_fees =
					payment_fee.checked_add(purchase_fee).ok_or(Error::<T>::OrderUnitsOverflow)?;

				ensure!(max_fee >= required_fees.into(), Error::<T>::FeeExceedsUserLimit);

				// send purchase price to seller
				T::Currency::transfer(
					T::StableCurrencyId::get(),
					&buyer,
					&order.owner,
					required_currency.into(),
				)?;

				// transfer fee to pallet
				T::Currency::transfer(
					T::StableCurrencyId::get(),
					&buyer,
					&Self::account_id(),
					required_fees.into(),
				)?;

				// transfer asset to buyer
				T::Asset::transfer(order.asset_id, &Self::account_id(), &buyer, units, false)?;

				Self::deposit_event(Event::BuyOrderFilled {
					order_id,
					units,
					price_per_unit: order.price_per_unit,
					seller: order.owner.clone(),
					buyer,
				});

				// remove the sell order if all units are filled
				if !order.units.is_zero() {
					*maybe_order = Some(order)
				}

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
			PurchaseFees::<T>::set(purchase_fee);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID of the CarbonCredits pallet
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
