//! I'm Online pallet benchmarking.
#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as VCU;
use crate::*;
use frame_benchmarking::{account, benchmarks};
use frame_support::storage::child::get;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::convert::TryInto;

benchmarks! {

    where_clause { where T: crate::Config + pallet_balances::Config + pallet_assets::Config + pallet_timestamp::Config }

    add_authorized_account {
        let account_id : T::AccountId = account("account_id", 0, 0);
        let description : DescriptionOf<T> = b"Verra".to_vec().try_into().unwrap();
    }: _(RawOrigin::Root, account_id.clone().into(), description.clone())
    verify {
        assert_eq!(
            VCU::<T>::get_authorized_accounts(account_id),
            Some(description)
        );
    }

    destroy_authorized_account {
        let account_id : T::AccountId = account("account_id", 0, 0);
        let description : DescriptionOf<T> = b"Verra".to_vec().try_into().unwrap();
        VCU::<T>::add_authorized_account(RawOrigin::Root.into(), account_id.clone().into(), description.clone());
    }: _(RawOrigin::Root, account_id.clone().into())
    verify {
        assert_eq!(
            VCU::<T>::get_authorized_accounts(account_id),
            None
        );
    }

    create_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
    }: _(RawOrigin::Root, agv_account_id.clone(), agv_id, input.clone())
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu(agv_account_id, agv_id),
            Some(input)
        );
    }

    destroy_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
    }: _(RawOrigin::Root, agv_account_id.clone(), agv_id)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu(agv_account_id, agv_id),
            None
        );
    }

    mint_shares_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
    }: _(RawOrigin::Root, recipient, agv_account_id.clone(), agv_id, 100)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu_shares_minted(agv_account_id, agv_id),
            100
        );
    }

    burn_shares_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        VCU::<T>::mint_shares_asset_generating_vcu(RawOrigin::Root.into(), recipient.clone(), agv_account_id.clone(), agv_id, 100);
    }: _(RawOrigin::Root, recipient, agv_account_id.clone(), agv_id, 99)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu_shares_minted(agv_account_id, agv_id),
            1
        );
    }

    transfer_shares_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let transfer_receipient : T::AccountId = account("transfer_receipient", 0, 2);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        VCU::<T>::mint_shares_asset_generating_vcu(RawOrigin::Root.into(), recipient.clone(), agv_account_id.clone(), agv_id, 100);
    }: _(RawOrigin::Signed(recipient), transfer_receipient, agv_account_id.clone(), agv_id, 100)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu_shares_minted(agv_account_id, agv_id),
            100
        );
    }

    forcetransfer_shares_asset_generating_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let transfer_receipient : T::AccountId = account("transfer_receipient", 0, 2);
        let agv_id : u32 = 1;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        VCU::<T>::mint_shares_asset_generating_vcu(RawOrigin::Root.into(), recipient.clone(), agv_account_id.clone(), agv_id, 100);
    }: _(RawOrigin::Root, recipient, transfer_receipient, agv_account_id.clone(), agv_id, 100)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu_shares_minted(agv_account_id, agv_id),
            100
        );
    }

    create_asset_generating_vcu_schedule {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let period_days : u64 = 1;
        let amount_vcu = 1;
        let token_id = 10_000;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        pallet_balances::Pallet::<T>::make_free_balance_be(&agv_account_id, 1000_u32.into());
        pallet_assets::Pallet::<T>::create(RawOrigin::Signed(agv_account_id.clone()).into(), 10000, T::Lookup::unlookup(agv_account_id.clone()), 1_u32.into()).unwrap();

    }: _(RawOrigin::Root, agv_account_id.clone(), agv_id, period_days, amount_vcu, token_id)
    verify {
        let expected_schedule = AssetsGeneratingVCUScheduleContent {
            period_days: 1_u64,
            amount_vcu: 1_u128,
            token_id: 10000_u32,
        };
        assert_eq!(
            VCU::<T>::asset_generating_vcu_schedule(agv_account_id, agv_id),
            Some(expected_schedule)
        );
    }

    destroy_asset_generating_vcu_schedule {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let period_days : u64 = 1;
        let amount_vcu = 1;
        let token_id = 10_000;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        pallet_balances::Pallet::<T>::make_free_balance_be(&agv_account_id, 1000_u32.into());
        pallet_assets::Pallet::<T>::create(RawOrigin::Signed(agv_account_id.clone()).into(), 10000, T::Lookup::unlookup(agv_account_id.clone()), 1_u32.into()).unwrap();
        VCU::<T>::create_asset_generating_vcu_schedule(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, period_days, amount_vcu, token_id);
    }: _(RawOrigin::Root, agv_account_id.clone(), agv_id)
    verify {
        assert_eq!(
            VCU::<T>::asset_generating_vcu_schedule(agv_account_id, agv_id),
            None
        );
    }

    mint_scheduled_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let period_days : u64 = 1;
        let amount_vcu = 1;
        let token_id = 10_000;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        pallet_balances::Pallet::<T>::make_free_balance_be(&agv_account_id, 1000_u32.into());
        pallet_balances::Pallet::<T>::make_free_balance_be(&recipient, 1000_u32.into());
        pallet_assets::Pallet::<T>::create(RawOrigin::Signed(agv_account_id.clone()).into(), 10000, T::Lookup::unlookup(agv_account_id.clone()), 1_u32.into()).unwrap();
        VCU::<T>::mint_shares_asset_generating_vcu(RawOrigin::Root.into(), recipient.clone(), agv_account_id.clone(), agv_id, 100);
        VCU::<T>::create_asset_generating_vcu_schedule(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, period_days, amount_vcu, token_id);
        // advance time so we can mint
        let now = pallet_timestamp::Pallet::<T>::get();
        let future = now + (24_u32*60_u32).into();
        pallet_timestamp::Pallet::<T>::set(RawOrigin::Root.into(), future);
    }: _(RawOrigin::Root, agv_account_id.clone(), agv_id)
    verify {
        // TODO : add propoer verification
    }

    retire_vcu {
        let agv_account_id : T::AccountId = account("agv_account_id", 0, 0);
        let recipient : T::AccountId = account("recipient", 0, 1);
        let agv_id : u32 = 1;
        let period_days : u64 = 1;
        let amount_vcu = 1;
        let token_id = 10_000;
        let input: AssetGeneratingVCUContentOf<T> = AssetGeneratingVCUContent {
            description: b"Description".to_vec().try_into().unwrap(),
            proof_of_ownership: b"proof".to_vec().try_into().unwrap(),
            number_of_shares: 10000,
            other_documents: None,
            expiry: None,
        };
        VCU::<T>::create_asset_generating_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, input.clone());
        pallet_balances::Pallet::<T>::make_free_balance_be(&agv_account_id, 1000_u32.into());
        pallet_balances::Pallet::<T>::make_free_balance_be(&recipient, 1000_u32.into());
        pallet_assets::Pallet::<T>::create(RawOrigin::Signed(agv_account_id.clone()).into(), 10000, T::Lookup::unlookup(agv_account_id.clone()), 1_u32.into()).unwrap();
        VCU::<T>::mint_shares_asset_generating_vcu(RawOrigin::Root.into(), recipient.clone(), agv_account_id.clone(), agv_id, 100);
        VCU::<T>::create_asset_generating_vcu_schedule(RawOrigin::Root.into(), agv_account_id.clone(), agv_id, period_days, amount_vcu, token_id);
        // advance time so we can mint
        let now = pallet_timestamp::Pallet::<T>::get();
        let future = now + (24_u32*60_u32).into();
        pallet_timestamp::Pallet::<T>::set(RawOrigin::Root.into(), future);
        VCU::<T>::mint_scheduled_vcu(RawOrigin::Root.into(), agv_account_id.clone(), agv_id);
    }: _(RawOrigin::Signed(recipient), agv_account_id.clone(), agv_id, 1)
    verify {
        // TODO : add propoer verification
    }

    impl_benchmark_test_suite!(VCU, crate::mock::new_test_ext(), crate::mock::Test);
}
