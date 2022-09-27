#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

use sp_core::U256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::weights::WeightInfo;
    use frame_support::dispatch::DispatchResult;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Name of the token
        #[pallet::constant]
        type Name: Get<Vec<u8>>;

        /// Symbol of the token
        #[pallet::constant]
        type Symbol: Get<Vec<u8>>;

        /// Decimals
        #[pallet::constant]
        type Decimals: Get<u8>;

        /// Before transfer callback
        type BeforeTransfer: BeforeTransfer<Self::AccountId>;

        /// After transfer callback
        type AfterTransfer: AfterTransfer<Self::AccountId>;

        /// Pallet weights
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// The total number of tokens that will ever be issued
    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, U256, ValueQuery>;

    /// Balance of token by accountId
    #[pallet::storage]
    #[pallet::getter(fn balance)]
    pub type Balance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, U256, ValueQuery>;

    /// Allowance to make a transfer from another account
    #[pallet::storage]
    #[pallet::getter(fn allowance)]
    pub type Allowance<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        U256,
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", T::BlockNumber = "BlockNumber")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// An event triggered when a transfer is successful  [from, to, amount]
        Transfer(T::AccountId, T::AccountId, U256),
        /// Changed allowance [owner, sender, allowance]
        Approval(T::AccountId, T::AccountId, U256),
        /// Mint [owner, amount, total_supply]
        Mint(T::AccountId, U256, U256),
        /// Burn [owner, amount, total_supply]
        Burn(T::AccountId, U256, U256),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Account balance not enough to make a transfer
        BalanceNotEnough,
        /// Account doesn't have enough allowance to make a transfer
        InsufficientAllowance,
        /// Arithmetic error
        ArithmeticError,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn do_transfer(from: T::AccountId, to: T::AccountId, amount: U256) -> DispatchResult {
            T::BeforeTransfer::before_transfer(Some(&from), &to, amount);

            Balance::<T>::try_mutate(from.clone(), |source_balance| -> Result<(), Error<T>> {
                ensure!(*source_balance >= amount, Error::<T>::BalanceNotEnough);
                *source_balance = *source_balance - amount;

                Balance::<T>::mutate(to.clone(), |balance| {
                    *balance = *balance + amount;
                });

                Ok(())
            })?;

            T::AfterTransfer::after_transfer(Some(&from), &to, amount);

            Self::deposit_event(Event::Transfer(from, to, amount));
            Ok(())
        }

        fn spend_allowance(
            owner: T::AccountId,
            spender: T::AccountId,
            amount: U256,
        ) -> DispatchResult {
            let new_allowance = Allowance::<T>::try_mutate(
                owner.clone(),
                spender.clone(),
                |allowance| -> Result<U256, Error<T>> {
                    ensure!(*allowance >= amount, Error::<T>::InsufficientAllowance);
                    *allowance = *allowance - amount;
                    Ok(*allowance)
                },
            )?;
            Self::deposit_event(Event::Approval(owner, spender, new_allowance));
            Ok(())
        }
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfer amount from origin account balance
        #[pallet::weight(<T as pallet::Config>::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: U256) -> DispatchResult {
            let from = ensure_signed(origin)?;
            Self::do_transfer(from, to, amount)?;

            Ok(())
        }

        /// Allow spender to spend amount from origin account balance
        #[pallet::weight(<T as pallet::Config>::WeightInfo::approve())]
        pub fn approve(
            origin: OriginFor<T>,
            spender: T::AccountId,
            amount: U256,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            <Allowance<T>>::insert(from.clone(), spender.clone(), amount);

            Self::deposit_event(Event::Approval(from, spender, amount));
            Ok(())
        }

        /// Increase spender allowance
        #[pallet::weight(<T as pallet::Config>::WeightInfo::increase_allowance())]
        pub fn increase_allowance(
            origin: OriginFor<T>,
            spender: T::AccountId,
            added_value: U256,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let allowance = Allowance::<T>::try_mutate(
                owner.clone(),
                spender.clone(),
                |allowance| -> Result<U256, Error<T>> {
                    *allowance = allowance
                        .checked_add(added_value)
                        .ok_or_else(|| Error::<T>::ArithmeticError)?;
                    Ok(*allowance)
                },
            )?;

            Self::deposit_event(Event::Approval(owner, spender, allowance));

            Ok(())
        }

        /// Decrease spender allowance
        #[pallet::weight(<T as pallet::Config>::WeightInfo::decrease_allowance())]
        pub fn decrease_allowance(
            origin: OriginFor<T>,
            spender: T::AccountId,
            subtracted_value: U256,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let allowance = Allowance::<T>::try_mutate(
                owner.clone(),
                spender.clone(),
                |allowance| -> Result<U256, Error<T>> {
                    *allowance = allowance
                        .checked_sub(subtracted_value)
                        .ok_or_else(|| Error::<T>::ArithmeticError)?;
                    Ok(*allowance)
                },
            )?;

            Self::deposit_event(Event::Approval(owner, spender, allowance));

            Ok(())
        }

        /// Transfer amount from `from` account to `to` account if origin account allow to spend amount
        #[pallet::weight(<T as pallet::Config>::WeightInfo::transfer_from())]
        pub fn transfer_from(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            amount: U256,
        ) -> DispatchResult {
            let spender = ensure_signed(origin)?;
            Self::spend_allowance(from.clone(), spender.clone(), amount)?;
            Self::do_transfer(from, to, amount)?;
            Ok(())
        }

        /// Root required! Creates `amount` tokens and assigns them to `account`, increasing the total supply
        #[pallet::weight(<T as pallet::Config>::WeightInfo::mint())]
        pub fn mint(origin: OriginFor<T>, owner: T::AccountId, amount: U256) -> DispatchResult {
            ensure_root(origin)?;

            T::BeforeTransfer::before_transfer(None, &owner, amount);

            let total_supply =
                TotalSupply::<T>::try_mutate(|total_supply| -> Result<U256, Error<T>> {
                    *total_supply = total_supply
                        .checked_add(amount)
                        .ok_or_else(|| Error::<T>::ArithmeticError)?;
                    Ok(*total_supply)
                })?;

            Balance::<T>::mutate(owner.clone(), |balance| {
                *balance = *balance + amount;
            });

            T::AfterTransfer::after_transfer(None, &owner, amount);

            Self::deposit_event(Event::Mint(owner, amount, total_supply));
            Ok(())
        }

        /// Root required! Destroys `amount` tokens from `account`, reducing the total supply
        #[pallet::weight(<T as pallet::Config>::WeightInfo::burn())]
        pub fn burn(origin: OriginFor<T>, owner: T::AccountId, amount: U256) -> DispatchResult {
            ensure_root(origin)?;

            T::BeforeTransfer::before_transfer(None, &owner, amount);

            Balance::<T>::try_mutate(owner.clone(), |balance| -> Result<(), Error<T>> {
                ensure!(*balance >= amount, Error::<T>::BalanceNotEnough);
                *balance = *balance - amount;
                Ok(())
            })?;

            let total_supply = TotalSupply::<T>::mutate(|total_supply| {
                *total_supply = *total_supply - amount;
                *total_supply
            });

            T::AfterTransfer::after_transfer(None, &owner, amount);

            Self::deposit_event(Event::Burn(owner, amount, total_supply));
            Ok(())
        }
    }
}

pub trait BeforeTransfer<AccountId> {
    fn before_transfer(from: Option<&AccountId>, to: &AccountId, amount: U256);
}

pub trait AfterTransfer<AccountId> {
    fn after_transfer(from: Option<&AccountId>, to: &AccountId, amount: U256);
}
