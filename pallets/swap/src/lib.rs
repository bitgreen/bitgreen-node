#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};

use frame_support::RuntimeDebug;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
enum OrderStatus {
    #[default]
    OrderCreated,
    OrderFulfilled,
    OrderCancelled,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct OrderInfo<AccountId, AssetId, Balance> {
    owner: AccountId,
    volume: Balance,
    price: Balance,
    token_id: AssetId,
    status: OrderStatus,
}

#[frame_support::pallet]
pub mod pallet {
    use crate::{OrderInfo, OrderStatus};
    use frame_support::{dispatch::PostDispatchInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::{OriginFor, *};
    use pallet_assets::Pallet as Asset;
    use sp_runtime::traits::StaticLookup;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type CurrencyTokenId: Get<<Self as pallet_assets::Config>::AssetId>;

        #[pallet::constant]
        type Owner: Get<<Self as frame_system::Config>::AccountId>;
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // owner of swap pool
    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

    // orders information
    #[pallet::storage]
    #[pallet::getter(fn order_count)]
    pub type OrderCount<T: Config> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn order_info)]
    pub type Orders<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, OrderInfo<T::AccountId, T::AssetId, T::Balance>>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),

        // parameters: [token_id, amount, price, owner]
        SellOrderCreated(T::AssetId, T::Balance, T::Balance, T::AccountId),
        // parameters: [order_id, status]
        SellOrderCancelled(u64, bool),
        // parameters: [order_id, amount of token sold, amount paid, seller, buyer]
        BuyOrderFilled(u64, T::Balance, T::Balance, T::AccountId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,

        OrderIdOverflow,
        InvalidOrderId,
        InvalidOrderOwner,
        OrderCancelledOrFulfilled,
        InvalidAssetId,
        OrderVolumeOverflow,
        InsufficientCurrency,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            // Update storage.
            <Something<T>>::put(something);

            const ACCOUNT_ALICE: u64 = 1;
            const ACCOUNT_BOB: u64 = 2;
            const COUNT_AIRDROP_RECIPIENTS: u64 = 2;
            const TOKENS_FIXED_SUPPLY: u64 = 100;

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
        pub fn create_sell_order(
            origin: OriginFor<T>,
            token_id: T::AssetId,
            volume: T::Balance,
            price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let owner = T::Owner::get();
            let seller = ensure_signed(origin.clone())?;

            // // transfer token to owner
            Asset::<T>::transfer(origin.clone(), token_id, T::Lookup::unlookup(owner), volume)?;

            let order_id = Self::order_count().ok_or(Error::<T>::OrderIdOverflow)?;

            OrderCount::<T>::put(order_id + 1);

            // order values
            Orders::<T>::insert(
                order_id,
                OrderInfo {
                    owner: seller.clone(),
                    volume: volume,
                    price: price,
                    token_id: token_id,
                    status: OrderStatus::OrderCreated,
                },
            );

            Self::deposit_event(Event::SellOrderCreated(token_id, volume, price, seller));

            Ok(PostDispatchInfo::from(Some(0)))
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
        pub fn cancel_sell_order(origin: OriginFor<T>, order_id: u64) -> DispatchResult {
            let seller = ensure_signed(origin.clone())?;

            // check validity
            let mut order = Self::order_info(order_id).ok_or(Error::<T>::InvalidOrderId)?;
            // ensure!(order_id <= Self::order_count(), Error::<T>::InvalidOrderId);

            ensure!(seller == order.owner, Error::<T>::InvalidOrderOwner);
            // ensure!(seller == Self::order_owner(order_id), Error::<T>::NotOwner);
            ensure!(
                OrderStatus::OrderCreated == order.status,
                Error::<T>::OrderCancelledOrFulfilled
            );
            // ensure!(Self::order_status(order_id), Error::<T>::AlreadyCancelled);

            order.status = OrderStatus::OrderCancelled;

            Orders::<T>::insert(order_id, order.clone());
            // OrderStatus::<T>::insert(order_id, false);

            let owner = T::Owner::get();
            // // transfer token to owner
            Asset::<T>::transfer(
                origin,
                order.token_id,
                T::Lookup::unlookup(owner),
                order.volume,
            )?;
            // ERC20::<T>::transfer_from(origin, seller, Self::owner(), Self::order_volume(order_id))?;

            Self::deposit_event(Event::SellOrderCancelled(order_id, true));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn buy_order(
            origin: OriginFor<T>,
            order_id: u64,
            token_id: T::AssetId,
            volume: T::Balance,
            currency_amount: T::Balance,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin.clone())?;

            // ERC20::<T>::get_project_details(origin.clone(), token_id)?;

            // check validity of order
            let mut order = Self::order_info(order_id).ok_or(Error::<T>::InvalidOrderId)?;
            // ensure!(order_id <= Self::order_count(), Error::<T>::InvalidOrderId);

            ensure!(
                OrderStatus::OrderCreated == order.status,
                Error::<T>::OrderCancelledOrFulfilled
            );
            // ensure!(
            //     Self::order_status(order_id),
            //     Error::<T>::CancelledOrFulfilled
            // );

            ensure!(token_id == order.token_id, Error::<T>::InvalidAssetId);
            ensure!(volume <= order.volume, Error::<T>::OrderVolumeOverflow);
            // ensure!(
            //     Self::order_volume(order_id) >= volume,
            //     Error::<T>::VolumeTooMuch
            // );

            let required_currency = order.price * volume;
            // // calculate necessary currency values
            // let price = Self::order_price(order_id);
            // let required_currency = BalanceOf::<T>::try_from((price * volume).as_u128())
            //     .ok()
            //     .ok_or(Error::<T>::BalanceError)?;

            let payment_fee = required_currency * T::Balance::from(2u32) / T::Balance::from(100u32);
            // // calculate fee
            // let payment_fee =
            //     required_currency * BalanceOf::<T>::from(2u32) / BalanceOf::<T>::from(100u32);
            let purchase_fee = volume * T::Balance::from(2u32) / T::Balance::from(10u32);
            // let purchase_fee = BalanceOf::<T>::try_from(volume.as_u128())
            //     .ok()
            //     .ok_or(Error::<T>::BalanceError)?
            //     * BalanceOf::<T>::from(2u32)
            //     / BalanceOf::<T>::from(10u32);

            let total_fee = payment_fee + purchase_fee;
            // let total_fee = payment_fee + purchase_fee;

            ensure!(
                currency_amount >= required_currency + total_fee,
                Error::<T>::InsufficientCurrency
            );
            // // ensure payment amount
            // ensure!(
            //     currency_amount >= required_currency + total_fee,
            //     Error::<T>::InsufficientCurrency
            // );

            let seller = order.owner.clone();
            // let seller = Self::order_owner(order_id);

            order.status = OrderStatus::OrderFulfilled;
            Orders::<T>::insert(order_id, order.clone());
            // // mark order as fulfilled
            // OrderStatus::<T>::insert(order_id, false);

            // // move funds

            let currency_id = T::CurrencyTokenId::get();
            let owner = T::Owner::get();

            // // move currency to seller
            Asset::<T>::transfer(
                origin.clone(),
                currency_id,
                T::Lookup::unlookup(seller.clone()),
                required_currency,
            )?;
            // T::Currency::transfer(
            //     &buyer,
            //     &seller,
            //     required_currency,
            //     ExistenceRequirement::KeepAlive,
            // )?;

            // // move fee to owner
            Asset::<T>::transfer(
                origin.clone(),
                currency_id,
                T::Lookup::unlookup(owner.clone()),
                required_currency,
            )?;
            // T::Currency::transfer(
            //     &buyer,
            //     &Self::owner(),
            //     total_fee,
            //     ExistenceRequirement::KeepAlive,
            // )?;

            // // move token to buyer
            Asset::<T>::transfer(
                origin.clone(),
                order.token_id,
                T::Lookup::unlookup(buyer.clone()),
                volume,
            )?;
            Asset::<T>::transfer(
                origin.clone(),
                order.token_id,
                T::Lookup::unlookup(seller.clone()),
                order.volume - volume,
            )?;
            // ERC20::<T>::transfer(origin.clone(), buyer.clone(), volume)?;
            // ERC20::<T>::transfer(
            //     origin.clone(),
            //     seller.clone(),
            //     Self::order_volume(order_id) - volume,
            // )?;

            Self::deposit_event(Event::BuyOrderFilled(
                order_id,
                volume,
                order.price,
                seller,
                buyer,
            ));

            Ok(())
        }
    }
}
