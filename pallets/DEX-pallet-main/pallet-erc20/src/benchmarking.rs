use super::*;

use crate::Pallet;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_core::U256;

benchmarks! {
	transfer {
		let src_account: T::AccountId = account("acc_1", 0, 0);
		let dest_account: T::AccountId = account("acc_2", 1, 0);

		let _ = Pallet::<T>::mint(RawOrigin::Root.into(), src_account.clone(), U256::from(1000));

		let amount = U256::from(100);

	}: _(RawOrigin::Signed(src_account.clone()), dest_account.clone(),  amount)
	verify {
		assert_eq!(Pallet::<T>::balance(src_account), U256::from(900));
		assert_eq!(Pallet::<T>::balance(dest_account), U256::from(100));
		assert_eq!(Pallet::<T>::total_supply(), U256::from(1000));
	}

	approve {
		let owner: T::AccountId = account("acc_1", 0, 0);
		let spender: T::AccountId = account("acc_2", 1, 0);
		let allowance = U256::from(100);
	}: _(RawOrigin::Signed(owner.clone()), spender.clone(),  allowance)
	verify {
		assert_eq!(Pallet::<T>::allowance(owner, spender), U256::from(100));
	}

	increase_allowance {
		let owner: T::AccountId = account("acc_1", 0, 0);
		let spender: T::AccountId = account("acc_2", 1, 0);
		let allowance = U256::from(10);

		let _ = Pallet::<T>::approve(RawOrigin::Signed(owner.clone()).into(), spender.clone(), allowance);

		let added_allowance = U256::from(100);
	}: _(RawOrigin::Signed(owner.clone()), spender.clone(),  added_allowance)
	verify {
		assert_eq!(Pallet::<T>::allowance(owner, spender), allowance + added_allowance);
	}

	decrease_allowance {
		let owner: T::AccountId = account("acc_1", 0, 0);
		let spender: T::AccountId = account("acc_2", 1, 0);
		let allowance = U256::from(100);

		let _ = Pallet::<T>::approve(RawOrigin::Signed(owner.clone()).into(), spender.clone(), allowance);

		let subtracted_allowance = U256::from(20);
	}: _(RawOrigin::Signed(owner.clone()), spender.clone(),  subtracted_allowance)
	verify {
		assert_eq!(Pallet::<T>::allowance(owner, spender), allowance - subtracted_allowance);
	}

	transfer_from {
		let spender: T::AccountId = account("acc_1", 0, 0);
		let src_account: T::AccountId = account("acc_2", 1, 0);
		let dest_account: T::AccountId = account("acc_3", 2, 0);
		let allowance = U256::from(20);
		let amount = U256::from(100);

		let _ = Pallet::<T>::approve(RawOrigin::Signed(src_account.clone()).into(), spender.clone(), allowance);
		let _ = Pallet::<T>::mint(RawOrigin::Root.into(), src_account.clone(), amount);

		let transfer_amount = U256::from(5);
	}: _(RawOrigin::Signed(spender.clone()),src_account.clone(), dest_account.clone(),  transfer_amount)
	verify {
		assert_eq!(Pallet::<T>::allowance(src_account.clone(), spender), allowance - transfer_amount);
		assert_eq!(Pallet::<T>::balance(src_account), amount - transfer_amount);
		assert_eq!(Pallet::<T>::balance(dest_account), transfer_amount);
	}


	mint {
		let account: T::AccountId = account("acc_1", 1, 0);
		let amount = U256::from(100);
	}: _(RawOrigin::Root, account.clone(), amount)
	verify {
		assert_eq!(Pallet::<T>::balance(account), amount);
		assert_eq!(Pallet::<T>::total_supply(), amount);
	}


	burn {
		let account: T::AccountId = account("acc_1", 1, 0);
		let amount = U256::from(25);
		let _ = Pallet::<T>::mint(RawOrigin::Root.into(), account.clone(), amount);
		let burn_amount = U256::from(20);
	}: _(RawOrigin::Root, account.clone(), burn_amount)
	verify {
		assert_eq!(Pallet::<T>::balance(account), amount - burn_amount);
		assert_eq!(Pallet::<T>::total_supply(), amount - burn_amount);
	}
}
