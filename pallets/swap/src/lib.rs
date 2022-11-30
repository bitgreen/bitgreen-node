#![cfg_attr(not(feature = "std"), no_std)]

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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct OrderInfo<AccountId, AssetId, AssetBalance, TokenBalance> {
	owner: AccountId,
	units: AssetBalance,
	price_per_unit: TokenBalance,
	asset_id: AssetId,
}

#[frame_support::pallet]
pub mod pallet {
	use crate::OrderInfo;
	use codec::HasCompact;
	use frame_support::{
		dispatch::PostDispatchInfo,
		pallet_prelude::*,
		traits::{
			fungibles::{Create, Inspect, MutateHold, Transfer},
			ExistenceRequirement,
		},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::MultiCurrency;
	use pallet_assets::Pallet as Asset;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub,
			Saturating, StaticLookup,
		},
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
		type Asset: MutateHold<Self::AccountId> + Transfer<Self::AccountId>;

		// Token handler config - this is what the pallet accepts as payment
		type Currency: MultiCurrency<Self::AccountId>;

		/// The CurrencyId of the stable currency we accept as payment
		#[pallet::constant]
		type StableCurrencyId: Get<CurrencyIdOf<Self>>;

		/// The DEX pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::Origin>;
	}

	// owner of swap pool
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

	// orders information
	#[pallet::storage]
	#[pallet::getter(fn order_count)]
	pub type OrderCount<T: Config> = StorageValue<_, u64>;

	// Payment fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn payment_fees)]
	pub type PaymentFees<T: Config> = StorageValue<_, Percent, ValueQuery>;

	// purchase fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn purchase_fees)]
	pub type PurchaseFees<T: Config> = StorageValue<_, Percent, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn order_info)]
	pub type Orders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		OrderInfo<T::AccountId, AssetIdOf<T>, AssetBalanceOf<T>, CurrencyBalanceOf<T>>,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// parameters: [asset_id, amount, price, owner]
		SellOrderCreated(AssetIdOf<T>, AssetBalanceOf<T>, CurrencyBalanceOf<T>, T::AccountId),
		// parameters: [order_id]
		SellOrderCancelled(u64),
		// parameters: [order_id, amount of token sold, amount paid, seller, buyer]
		BuyOrderFilled(u64, AssetBalanceOf<T>, CurrencyBalanceOf<T>, T::AccountId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		OrderIdOverflow,
		InvalidOrderId,
		InvalidOrderOwner,
		OrderCancelledOrFulfilled,
		InvalidAssetId,
		OrderUnitsOverflow,
		InsufficientCurrency,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_sell_order(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let seller = ensure_signed(origin.clone())?;

			// hold asset from the seller
			T::Asset::hold(asset_id, &seller, units)?;

			let order_id = Self::order_count().ok_or(Error::<T>::OrderIdOverflow)?;

			OrderCount::<T>::put(order_id + 1);

			// order values
			Orders::<T>::insert(
				order_id,
				OrderInfo { owner: seller.clone(), units, price_per_unit, asset_id },
			);

			Self::deposit_event(Event::SellOrderCreated(asset_id, units, price_per_unit, seller));

			Ok(PostDispatchInfo::from(Some(0)))
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn cancel_sell_order(origin: OriginFor<T>, order_id: u64) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;

			// check validity
			let order = Orders::<T>::take(order_id).ok_or(Error::<T>::InvalidOrderId)?;

			ensure!(seller == order.owner, Error::<T>::InvalidOrderOwner);

			// remove hold on asset
			T::Asset::release(order.asset_id, &seller, order.units, true)?;

			Self::deposit_event(Event::SellOrderCancelled(order_id));
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000)]
		pub fn buy_order(
			origin: OriginFor<T>,
			order_id: u64,
			asset_id: AssetIdOf<T>,
			units: AssetBalanceOf<T>,
			currency_amount: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin.clone())?;

			Orders::<T>::try_mutate(order_id, |order| -> DispatchResult {
				let order = order.as_mut().ok_or(Error::<T>::InvalidOrderId)?;

				// ensure the expected token matches the order
				ensure!(asset_id == order.asset_id, Error::<T>::InvalidAssetId);

				// ensure volume remaining can cover the buy order
				ensure!(units <= order.units, Error::<T>::OrderUnitsOverflow);

				// reduce the buy_order units from total volume
				order.units =
					order.units.checked_sub(&units).ok_or(Error::<T>::OrderUnitsOverflow)?;

				// calculate fees
				let units_as_u32: u32 =
					units.try_into().map_err(|_| Error::<T>::OrderUnitsOverflow)?;
				let price_per_unit_as_u32: u32 =
					order.price_per_unit.try_into().map_err(|_| Error::<T>::OrderUnitsOverflow)?;
				let required_currency = price_per_unit_as_u32
					.checked_mul(units_as_u32)
					.ok_or(Error::<T>::OrderUnitsOverflow)?;

				let payment_fee = PaymentFees::<T>::get().mul_floor(required_currency);
				let purchase_fee = PurchaseFees::<T>::get().mul_floor(required_currency);

				let required_fees =
					payment_fee.checked_add(purchase_fee).ok_or(Error::<T>::OrderUnitsOverflow)?;
				let required_currency_with_fees = required_fees
					.checked_add(required_currency)
					.ok_or(Error::<T>::OrderUnitsOverflow)?;

				ensure!(
					currency_amount >= required_currency_with_fees.into(),
					Error::<T>::InsufficientCurrency
				);

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

				// release asset from seller and transfer to buyer
				T::Asset::release(order.asset_id, &order.owner, order.units, true)?;
				T::Asset::transfer(order.asset_id, &order.owner, &buyer, order.units, false)?;

				Self::deposit_event(Event::BuyOrderFilled(
					order_id,
					units,
					order.price_per_unit,
					order.owner.clone(),
					buyer,
				));

				Ok(())
			})
		}

		/// Force set PaymentFees value
		/// Can only be called by ForceOrigin
		#[transactional]
		#[pallet::weight(10_000)]
		pub fn force_set_payment_fee(origin: OriginFor<T>, payment_fee: Percent) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			PaymentFees::<T>::set(payment_fee);
			Ok(())
		}

		/// Force set PurchaseFee value
		/// Can only be called by ForceOrigin
		#[transactional]
		#[pallet::weight(10_000)]
		pub fn force_set_purchase_fee(
			origin: OriginFor<T>,
			purchase_fee: Percent,
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
