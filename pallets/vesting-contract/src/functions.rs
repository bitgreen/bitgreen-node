use crate::*;

impl<T: Config> Pallet<T> {
	/// The account ID of the CarbonCredits pallet
	pub fn account_id() -> T::AccountId { T::PalletId::get().into_account_truncating() }

	/// Get the free balance of the pallet account
	pub fn pallet_free_balance() -> BalanceOf<T> { T::Currency::free_balance(&Self::account_id()) }

	/// Get the total balance of the pallet account
	pub fn pallet_total_balance() -> BalanceOf<T> {
		T::Currency::total_balance(&Self::account_id())
	}

	pub fn do_add_new_contract(
		recipient: T::AccountId,
		expiry: T::BlockNumber,
		amount: BalanceOf<T>,
	) -> DispatchResult {
		// ensure the expiry is in the future
		let now = frame_system::Pallet::<T>::block_number();
		ensure!(expiry > now, Error::<T>::ExpiryInThePast);

		// Update the total vesting balance
		let current_vesting_balance = VestingBalance::<T>::get();
		let new_vesting_balance = current_vesting_balance
			.checked_add(&amount)
			.ok_or(ArithmeticError::Overflow)?;

		// ensure the pallet balance is greater than the new vesting balance
		// ie. the pallet has funds to support this new contract
		ensure!(
			Self::pallet_free_balance() >= new_vesting_balance,
			Error::<T>::PalletOutOfFunds
		);

		// insert the updated balance to storage
		VestingBalance::<T>::put(new_vesting_balance);

		// Insert new recipient to storage
		VestingContracts::<T>::insert(recipient.clone(), ContractDetail { expiry, amount });

		// Emit an event.
		Self::deposit_event(Event::ContractAdded {
			recipient,
			expiry,
			amount,
		});

		Ok(())
	}

	pub fn do_remove_contract(recipient: T::AccountId) -> DispatchResult {
		// Remove recipient from storage
		let ContractDetail { expiry: _, amount } =
			VestingContracts::<T>::take(recipient.clone()).ok_or(Error::<T>::ContractNotFound)?;

		// Update the total vesting balance
		let current_vesting_balance = VestingBalance::<T>::get();
		let new_vesting_balance = current_vesting_balance
			.checked_sub(&amount)
			.ok_or(ArithmeticError::Overflow)?;
		VestingBalance::<T>::put(new_vesting_balance);

		// Emit an event.
		Self::deposit_event(Event::ContractRemoved { recipient });

		Ok(())
	}
}
