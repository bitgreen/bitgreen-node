//! Benchmarking setup for pallet-generalstorage

use super::*;

#[allow(unused)]
use crate::Pallet as Generalstorage;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Generalstorage, crate::mock::new_test_ext(), crate::mock::Test);
}
