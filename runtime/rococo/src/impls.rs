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
			// transfer author reward to parachain staking
			let parachain_staking_pot = pallet_parachain_staking::Pallet::<R>::account_id();
			<pallet_balances::Pallet<R>>::resolve_creating(&parachain_staking_pot, split.1);
		}
	}
}

#[cfg(test)]
mod tests {
	use frame_support::{
		dispatch::DispatchClass, parameter_types, traits::FindAuthor, weights::Weight, PalletId,
	};
	use frame_system::limits;
	use primitives::v2::AccountId;
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
		Perbill,
	};

	use super::*;

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;
	const TEST_ACCOUNT: AccountId = AccountId::new([1; 32]);

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
			.base_block(Weight::from_ref_time(10))
			.for_class(DispatchClass::all(), |weight| {
				weight.base_extrinsic = Weight::from_ref_time(100);
			})
			.for_class(DispatchClass::non_mandatory(), |weight| {
				weight.max_total = Some(Weight::from_ref_time(1024).set_proof_size(u64::MAX));
			})
			.build_or_panic();
		pub BlockLength: limits::BlockLength = limits::BlockLength::max(2 * 1024);
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}

	impl frame_system::Config for Test {
		type AccountData = pallet_balances::AccountData<u64>;
		type AccountId = AccountId;
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockHashCount = BlockHashCount;
		type BlockLength = BlockLength;
		type BlockNumber = u64;
		type BlockWeights = BlockWeights;
		type DbWeight = ();
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Header = Header;
		type Index = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type MaxConsumers = frame_support::traits::ConstU32<16>;
		type OnKilledAccount = ();
		type OnNewAccount = ();
		type OnSetCode = ();
		type PalletInfo = PalletInfo;
		type RuntimeCall = RuntimeCall;
		type RuntimeEvent = RuntimeEvent;
		type RuntimeOrigin = RuntimeOrigin;
		type SS58Prefix = ();
		type SystemWeightInfo = ();
		type Version = ();
	}

	impl pallet_balances::Config for Test {
		type AccountStore = System;
		type Balance = u64;
		type DustRemoval = ();
		type ExistentialDeposit = ();
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type RuntimeEvent = RuntimeEvent;
		type WeightInfo = ();
	}

	parameter_types! {
		pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
		pub const MaxApprovals: u32 = 100;
	}

	impl pallet_treasury::Config for Test {
		type ApproveOrigin = frame_system::EnsureRoot<AccountId>;
		type Burn = ();
		type BurnDestination = ();
		type Currency = pallet_balances::Pallet<Test>;
		type MaxApprovals = MaxApprovals;
		type OnSlash = ();
		type PalletId = TreasuryPalletId;
		type ProposalBond = ();
		type ProposalBondMaximum = ();
		type ProposalBondMinimum = ();
		type RejectOrigin = frame_system::EnsureRoot<AccountId>;
		type RuntimeEvent = RuntimeEvent;
		type SpendFunds = ();
		type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u64>;
		type SpendPeriod = ();
		type WeightInfo = ();
	}

	pub struct OneAuthor;
	impl FindAuthor<AccountId> for OneAuthor {
		fn find_author<'a, I>(_: I) -> Option<AccountId>
		where
			I: 'a,
		{
			Some(TEST_ACCOUNT)
		}
	}
	impl pallet_authorship::Config for Test {
		type EventHandler = ();
		type FilterUncle = ();
		type FindAuthor = OneAuthor;
		type UncleGenerations = ();
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		pallet_balances::GenesisConfig::<Test>::default()
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}

	#[test]
	fn test_fees_and_tip_split() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(10);
			let tip = Balances::issue(20);

			assert_eq!(Balances::free_balance(Treasury::account_id()), 0);
			assert_eq!(Balances::free_balance(TEST_ACCOUNT), 0);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 100% of tip and 20% of fee = 22
			assert_eq!(Balances::free_balance(TEST_ACCOUNT), 22);
			// Treasury gets 80% of fee
			assert_eq!(Balances::free_balance(Treasury::account_id()), 8);
		});
	}
}
