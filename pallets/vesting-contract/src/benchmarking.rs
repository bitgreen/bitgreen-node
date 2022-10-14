// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Vesting Contract pallet benchmarking
use frame_benchmarking::{account, benchmarks, vec};
use frame_support::PalletId;
use frame_system::RawOrigin;
use sp_std::convert::TryInto;

use super::*;
use crate::Pallet as VestingContract;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn load_initial_pallet_balance<T: Config + pallet_balances::Config>(amount: BalanceOf<T>)
where
	<T as pallet_balances::Config>::Balance: From<
		<<T as pallet::Config>::Currency as frame_support::traits::Currency<
			<T as frame_system::Config>::AccountId,
		>>::Balance,
	>,
{
	// send some initial balance to vesting pallet
	let vesting_contract_pallet_acccount: T::AccountId =
		PalletId(*b"bitg/vcp").into_account_truncating();
	<pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(
		&vesting_contract_pallet_acccount,
		amount.into(),
	);
}

benchmarks! {

	where_clause { where
		T: pallet_balances::Config,
		<T as pallet_balances::Config>::Balance: From<<<T as pallet::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance>
	}

	add_new_contract {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_u32.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>(vesting_amount);
	}: _(RawOrigin::Root, account_id.clone(), expiry_block, vesting_amount)
	verify {
		assert_last_event::<T>(Event::ContractAdded { recipient : account_id, expiry : expiry_block, amount : vesting_amount }.into());
	}

	remove_contract {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_u32.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>(vesting_amount);
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
	}: _(RawOrigin::Root, account_id.clone())
	verify {
		assert_last_event::<T>(Event::ContractRemoved { recipient : account_id }.into());
	}

	// bulk_add_new_contract {
	// 	let account_id : T::AccountId = account("account_id", 0, 0);
	// 	let vesting_amount : BalanceOf<T> = 1_u32.into();
	// 	let expiry_block : T::BlockNumber  = 10_u32.into();
	// 	load_initial_pallet_balance::<T>(vesting_amount);
	// }: _(RawOrigin::Root, account_id.clone(), expiry_block, vesting_amount)
	// verify {
	// 	assert_last_event::<T>(Event::ContractAdded { recipient : account_id, expiry : expiry_block, amount : vesting_amount }.into());
	// }

	// bulk_remove_contract {
	// 	let account_id : T::AccountId = account("account_id", 0, 0);
	// 	let vesting_amount : BalanceOf<T> = 1_u32.into();
	// 	let expiry_block : T::BlockNumber  = 10_u32.into();
	// 	load_initial_pallet_balance::<T>(vesting_amount);
	// 	VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
	// }: _(RawOrigin::Root, account_id.clone())
	// verify {
	// 	assert_last_event::<T>(Event::ContractRemoved { recipient : account_id }.into());
	// }

	withdraw_vested {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_u32.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>(vesting_amount + 1_u32.into());
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
		frame_system::Pallet::<T>::set_block_number(expiry_block + 1_u32.into());
	}: _(RawOrigin::Signed(account_id.clone()))
	verify {
		assert_last_event::<T>(Event::ContractWithdrawn { recipient : account_id, amount : vesting_amount, expiry : expiry_block }.into());
	}

	force_withdraw_vested {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_u32.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>(vesting_amount + 1_u32.into());
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
		frame_system::Pallet::<T>::set_block_number(expiry_block + 1_u32.into());
	}: _(RawOrigin::Root, account_id.clone())
	verify {
		assert_last_event::<T>(Event::ContractWithdrawn { recipient : account_id, amount : vesting_amount, expiry : expiry_block  }.into());
	}

	impl_benchmark_test_suite!(VestingContract, crate::mock::new_test_ext(), crate::mock::Test);
}
