#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use core::convert::TryFrom;

    use frame_support::dispatch::PostDispatchInfo;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_system::pallet_prelude::*;
    use pallet_erc20::Pallet as ERC20;
    use sp_core::U256;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_erc20::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;
    }

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // owner of swap pool
    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    // orders information
    #[pallet::storage]
    #[pallet::getter(fn order_count)]
    pub type OrderCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn order_owner)]
    pub type OrderOwner<T: Config> = StorageMap<_, Blake2_128Concat, u64, T::AccountId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn order_token_id)]
    pub type OrderTokenId<T: Config> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn order_volume)]
    pub type OrderVolume<T: Config> = StorageMap<_, Blake2_128Concat, u64, U256, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn order_price)]
    pub type OrderPrice<T: Config> = StorageMap<_, Blake2_128Concat, u64, U256, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn order_status)]
    pub type OrderStatus<T: Config> = StorageMap<_, Blake2_128Concat, u64, bool, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // parameters: [token_id, amount, price, owner]
        SellOrderCreated(u64, U256, U256, T::AccountId),
        // parameters: [order_id, status]
        SellOrderCancelled(u64, bool),
        // parameters: [order_id, amount of token sold, amount paid, seller, buyer]
        BuyOrderFilled(u64, U256, U256, T::AccountId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        Overflow,
        InvalidOrderId,
        AlreadyCancelled,
        NotOwner,
        CancelledOrFulfilled,
        VolumeTooMuch,
        BalanceError,
        InsufficientCurrency,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Create a sell order
        #[pallet::weight(10_000)]
        pub fn create_sell_order(
            origin: OriginFor<T>,
            token_id: u64,
            volume: U256,
            price: U256,
        ) -> DispatchResultWithPostInfo {
            let seller = ensure_signed(origin.clone())?;

            // transfer token to owner
            ERC20::<T>::transfer(origin, <Owner<T>>::get(), volume)?;

            let order_id = Self::order_count();

            OrderCount::<T>::put(order_id + 1);

            // order values
            OrderOwner::<T>::insert(order_id, seller.clone());
            OrderTokenId::<T>::insert(order_id, token_id);
            OrderVolume::<T>::insert(order_id, volume);
            OrderPrice::<T>::insert(order_id, price);
            OrderStatus::<T>::insert(order_id, true);

            Self::deposit_event(Event::SellOrderCreated(token_id, volume, price, seller));

            Ok(PostDispatchInfo::from(Some(order_id)))
        }

        // Cancel
        #[pallet::weight(10_000)]
        pub fn cancel_sell_order(origin: OriginFor<T>, order_id: u64) -> DispatchResult {
            let seller = ensure_signed(origin.clone())?;

            // check validity
            ensure!(order_id <= Self::order_count(), Error::<T>::InvalidOrderId);
            ensure!(seller == Self::order_owner(order_id), Error::<T>::NotOwner);
            ensure!(Self::order_status(order_id), Error::<T>::AlreadyCancelled);

            OrderStatus::<T>::insert(order_id, false);

            // transfer token to owner
            ERC20::<T>::transfer_from(origin, seller, Self::owner(), Self::order_volume(order_id))?;

            Self::deposit_event(Event::SellOrderCancelled(order_id, true));
            Ok(())
        }

        // Buy
        #[pallet::weight(10_000)]
        pub fn buy_order(
            origin: OriginFor<T>,
            order_id: u64,
            _project_id: u32,
            _bundle_id: u32,
            volume: U256,
            currency_amount: BalanceOf<T>,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin.clone())?;

            // check validity of order
            ensure!(order_id <= Self::order_count(), Error::<T>::InvalidOrderId);
            ensure!(
                Self::order_status(order_id),
                Error::<T>::CancelledOrFulfilled
            );
            ensure!(
                Self::order_volume(order_id) >= volume,
                Error::<T>::VolumeTooMuch
            );

            // calculate necessary currency values
            let price = Self::order_price(order_id);
            let required_currency = BalanceOf::<T>::try_from((price * volume).as_u128())
                .ok()
                .ok_or(Error::<T>::BalanceError)?;

            // calculate fee
            let payment_fee =
                required_currency * BalanceOf::<T>::from(2u32) / BalanceOf::<T>::from(100u32);
            let purchase_fee = BalanceOf::<T>::try_from(volume.as_u128())
                .ok()
                .ok_or(Error::<T>::BalanceError)?
                * BalanceOf::<T>::from(2u32)
                / BalanceOf::<T>::from(10u32);

            let total_fee = payment_fee + purchase_fee;

            // ensure payment amount
            ensure!(
                currency_amount >= required_currency + total_fee,
                Error::<T>::InsufficientCurrency
            );

            let seller = Self::order_owner(order_id);

            // mark order as fulfilled
            OrderStatus::<T>::insert(order_id, false);

            // move funds

            // move currency to seller
            T::Currency::transfer(
                &buyer,
                &seller,
                required_currency,
                ExistenceRequirement::KeepAlive,
            )?;

            // move fee to owner
            T::Currency::transfer(
                &buyer,
                &Self::owner(),
                total_fee,
                ExistenceRequirement::KeepAlive,
            )?;

            // move token to buyer
            ERC20::<T>::transfer(origin.clone(), buyer.clone(), volume)?;
            ERC20::<T>::transfer(
                origin.clone(),
                seller.clone(),
                Self::order_volume(order_id) - volume,
            )?;

            Self::deposit_event(Event::BuyOrderFilled(
                order_id, volume, price, seller, buyer,
            ));

            Ok(())
        }
    }
}
