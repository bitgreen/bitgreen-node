// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

use crate::NegativeImbalance;

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_treasury::Config + pallet_parachain_staking::Config,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
	<R as frame_system::Config>::AccountId: From<primitives::AccountId>,
	<R as frame_system::Config>::AccountId: Into<primitives::AccountId>,
	<R as pallet_balances::Config>::Balance: From<
		<<R as pallet_parachain_staking::Config>::Currency as Currency<
			<R as frame_system::Config>::AccountId,
		>>::Balance,
	>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 50% to treasury, 50% to author
			let mut split = fees.ration(50, 50);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut split.1);
			}
			use pallet_treasury::Pallet as Treasury;
			// transfer treasury portion to treasury
			<Treasury<R> as OnUnbalanced<_>>::on_unbalanced(split.0);

			// transfer author reward + inflation rewards to parachain staking
			let parachain_staking_pot = pallet_parachain_staking::Pallet::<R>::account_id();
			// add inflation rewards to the parachain_staking_pot
			let inflation_reward_per_block: pallet_balances::NegativeImbalance<R> =
				NegativeImbalance::new(
					<pallet_parachain_staking::Pallet<R>>::inflation_reward_per_block().into(),
				);

			inflation_reward_per_block.merge_into(&mut split.1);
			<pallet_balances::Pallet<R>>::resolve_creating(&parachain_staking_pot, split.1);
		}
	}
}
