//! VCU pallet benchmarking
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
    }: _(RawOrigin::Root, account_id.clone().into())
    verify {
        assert_eq!(
            VCU::<T>::get_authorized_accounts().len(),
            1
        );
    }

    destroy_authorized_account {
        let account_id : T::AccountId = account("account_id", 0, 0);
        VCU::<T>::add_authorized_account(RawOrigin::Root.into(), account_id.clone().into(), description.clone());
    }: _(RawOrigin::Root, account_id.clone().into())
    verify {
        assert_eq!(
            VCU::<T>::get_authorized_accounts().len(),
            0
        );
    }

    impl_benchmark_test_suite!(VCU, crate::mock::new_test_ext(), crate::mock::Test);
}
