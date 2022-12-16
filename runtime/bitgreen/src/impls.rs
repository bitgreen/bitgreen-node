// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::{Call, Contains, NegativeImbalance, Runtime};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

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

			// transfer author rewards to parachain staking
			let parachain_staking_pot = pallet_parachain_staking::Pallet::<R>::account_id();
			<pallet_balances::Pallet<R>>::resolve_creating(&parachain_staking_pot, split.1);
		}
	}
}

// Don't allow permission-less asset creation.
pub struct BaseFilter;
impl Contains<Call> for BaseFilter {
	fn contains(call: &Call) -> bool {
		if matches!(call, Call::Timestamp(_) | Call::ParachainSystem(_) | Call::System(_)) {
			// always allow core call
			// pallet-timestamp and parachainSystem could not be filtered because
			// they are used in communication between relaychain and parachain.
			return true
		}

		if pallet_transaction_pause::PausedTransactionFilter::<Runtime>::contains(call) {
			// no paused call
			return false
		}

		#[allow(clippy::match_like_matches_macro)]
		// keep CallFilter with explicit true/false for documentation
		match call {
			// Explicitly DISALLOWED calls
            | Call::Assets(_) // Filter Assets. Assets should only be accessed by CarbonCreditsPallet.
			| Call::Uniques(_) // Filter Uniques, which should only be accessed by CarbonCreditsPallet.
			| Call::Tokens(_) // Filter Tokens, we dont use them now
			| Call::VCU(_) // VCU, we dont use them now
			| Call::VCUPools(_) // VCUPools, we dont use them now
			| Call::Dex(_) // Dex, we dont use them now
			| Call::Contracts(_) // Contracts, we dont use them now
            // Filter callables from XCM pallets, we dont use them now
            | Call::XcmpQueue(_) | Call::PolkadotXcm(_) | Call::DmpQueue(_) => false,
            // ALLOW anything else
            | _ => true
        }
	}
}
