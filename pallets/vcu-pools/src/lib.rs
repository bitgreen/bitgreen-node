#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::HasCompact;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::tokens::fungibles::{metadata::Mutate as MetadataMutate, Create, Mutate},
        transactional, PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
    use sp_std::convert::TryInto;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The PoolId type for the pallet
        type PoolId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// The units in which we record balances.
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        // Asset manager config
        type AssetHandler: Create<Self::AccountId, AssetId = Self::PoolId, Balance = Self::Balance>
            + Mutate<Self::AccountId>
            + MetadataMutate<Self::AccountId>;

        /// Maximum registrys allowed in the pool config
        type MaxRegistryListCount: Get<u32>;
        /// Maximum issuance years allowed in the pool config
        type MaxIssuanceYearCount: Get<u32>;
        /// Maximum projectIds allowed in the pool config
        type MaxProjectIdList: Get<u32>;
        /// Max length of pool asset symbol
        type MaxAssetSymbolLength: Get<u32>;
        /// The vcu-pools pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn pools)]
    pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new pool was created
        PoolCreated {
            admin: T::AccountId,
            id: T::PoolId,
            config: PoolConfigOf<T>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// PoolId is already being used
        PoolIdInUse,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new vcu pool with given params
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            id: T::PoolId,
            config: PoolConfigOf<T>,
            max_limit: Option<u32>,
            asset_symbol: SymbolStringOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // TODO : Check if the user is authorised to create pools

            ensure!(!Pools::<T>::contains_key(id), Error::<T>::PoolIdInUse);

            // insert to storage
            <Pools<T>>::insert(
                id,
                Pool {
                    admin: who.clone(),
                    config: config.clone(),
                    max_limit,
                    asset_symbol,
                    credits: Default::default(),
                },
            );

            // create an asset collection to reserve asset-id
            T::AssetHandler::create(id, Self::account_id(), true, 1_u32.into())?;

            // Emit an event.
            Self::deposit_event(Event::PoolCreated {
                admin: who,
                id,
                config,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// The account ID of the vcu pallet
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account()
        }
    }
}
