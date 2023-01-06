// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Vesting Contract pallet benchmarking
use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;

use super::*;
use crate::Pallet as VestingContract;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn load_initial_pallet_balance<T: Config + pallet_balances::Config<Balance = u128>>()
where
	<T as pallet_balances::Config>::Balance: From<
		<<T as pallet::Config>::Currency as frame_support::traits::Currency<
			<T as frame_system::Config>::AccountId,
		>>::Balance,
	>,
{
	// send some initial balance to vesting pallet
	let vesting_contract_pallet_acccount: T::AccountId = VestingContract::<T>::account_id();
	<pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(
		&vesting_contract_pallet_acccount,
		u128::MAX,
	);
}

benchmarks! {

	where_clause { where
		T: pallet_balances::Config<Balance = u128>,
		<T as pallet_balances::Config>::Balance: From<u128>,
		<T as pallet_balances::Config>::Balance: From<<<T as pallet::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance>,
		<<T as pallet::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance : From<u128>
	}

	add_new_contract {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>();
	}: _(RawOrigin::Root, account_id.clone(), expiry_block, vesting_amount)
	verify {
		assert_last_event::<T>(Event::ContractAdded { recipient : account_id, expiry : expiry_block, amount : vesting_amount }.into());
	}

	remove_contract {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>();
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
	}: _(RawOrigin::Root, account_id.clone())
	verify {
		assert_last_event::<T>(Event::ContractRemoved { recipient : account_id }.into());
	}

	bulk_add_new_contracts {
		let i in 0 .. T::MaxContractInputLength::get();
		let mut input_list : BulkContractInputs::<T> = Default::default();
		for index in 0..1 {
			let account_id : T::AccountId = account("account_id", 0, index);
			let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
			let expiry_block : T::BlockNumber  = 10_u32.into();
			input_list.try_push(BulkContractInputOf::<T> {
				recipient : account_id.clone(),
				amount : vesting_amount.clone(),
				expiry : expiry_block.clone(),
			}).unwrap();
		}
		load_initial_pallet_balance::<T>();
	}: _(RawOrigin::Root, input_list)
	verify {
		//assert_last_event::<T>(Event::ContractAdded { recipient : account_id, expiry : expiry_block, amount : vesting_amount }.into());
	}

	bulk_remove_contracts {
		let i in 0 .. T::MaxContractInputLength::get();

		let mut input_list : BulkContractInputs::<T> = Default::default();
		for index in 0..1 {
			let account_id : T::AccountId = account("account_id", 0, index);
			let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
			let expiry_block : T::BlockNumber  = 10_u32.into();
			input_list.try_push(BulkContractInputOf::<T> {
				recipient : account_id.clone(),
				amount : vesting_amount.clone(),
				expiry : expiry_block.clone(),
			}).unwrap();
		}
		load_initial_pallet_balance::<T>();
		VestingContract::<T>::bulk_add_new_contracts(RawOrigin::Root.into(), input_list).unwrap();

		let mut input_list : BulkContractRemove::<T> = Default::default();
		for index in 0..1 {
			let account_id : T::AccountId = account("account_id", 0, index);
			input_list.try_push(account_id).unwrap();
		}
	}: _(RawOrigin::Root, input_list)
	verify {
		//assert_last_event::<T>(Event::ContractAdded { recipient : account_id, expiry : expiry_block, amount : vesting_amount }.into());
	}

	withdraw_vested {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>();
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
		frame_system::Pallet::<T>::set_block_number(expiry_block + 1u32.into());
	}: _(RawOrigin::Signed(account_id.clone()))
	verify {
		assert_last_event::<T>(Event::ContractWithdrawn { recipient : account_id, amount : vesting_amount, expiry : expiry_block }.into());
	}

	force_withdraw_vested {
		let account_id : T::AccountId = account("account_id", 0, 0);
		let vesting_amount : BalanceOf<T> = 1_000_000_000_u128.into();
		let expiry_block : T::BlockNumber  = 10_u32.into();
		load_initial_pallet_balance::<T>();
		VestingContract::<T>::add_new_contract(RawOrigin::Root.into(), account_id.clone(), expiry_block, vesting_amount).unwrap();
		frame_system::Pallet::<T>::set_block_number(expiry_block + 1u32.into());
	}: _(RawOrigin::Root, account_id.clone())
	verify {
		assert_last_event::<T>(Event::ContractWithdrawn { recipient : account_id, amount : vesting_amount, expiry : expiry_block  }.into());
	}

	impl_benchmark_test_suite!(VestingContract, crate::mock::new_test_ext(), crate::mock::Test);
}
