// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, GenesisBuild, OnInitialize},
};
use pallet_balances::Error as BalancesError;
use sp_runtime::traits::BadOrigin;

use crate as collator_selection;
use crate::{
	mock::*,
	types::{CandidateInfoOf, DelegationInfoOf},
	Error, Invulnerables,
};

#[test]
fn basic_setup_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(CollatorSelection::desired_candidates(), 2);
		assert_eq!(CollatorSelection::candidacy_bond(), 10);

		assert!(CollatorSelection::candidates().is_empty());
		assert_eq!(
			CollatorSelection::invulnerables(),
			vec![1, 2]
				.iter()
				.cloned()
				.map(|account| CandidateInfoOf::<Test> {
					who: account,
					deposit: Default::default(),
					delegators: Default::default(),
					total_stake: Default::default(),
				})
				.collect::<Vec<CandidateInfoOf<Test>>>()
		);
	});
}

#[test]
fn it_should_set_invulnerables() {
	new_test_ext().execute_with(|| {
		let new_set = vec![1, 2, 3, 4];
		let new_set_formatted = new_set
			.iter()
			.cloned()
			.map(|account| CandidateInfoOf::<Test> {
				who: account,
				deposit: Default::default(),
				delegators: Default::default(),
				total_stake: Default::default(),
			})
			.collect::<Vec<CandidateInfoOf<Test>>>();
		assert_ok!(CollatorSelection::set_invulnerables(
			RuntimeOrigin::root(),
			new_set_formatted.clone()
		));
		assert_eq!(CollatorSelection::invulnerables(), new_set_formatted);

		// cannot set with non-root.
		assert_noop!(
			CollatorSelection::set_invulnerables(RuntimeOrigin::signed(1), new_set_formatted),
			BadOrigin
		);

		// cannot set invulnerables without associated validator keys
		let invulnerables = vec![7];
		let invulnerables_formatted = invulnerables
			.iter()
			.cloned()
			.map(|account| CandidateInfoOf::<Test> {
				who: account,
				deposit: Default::default(),
				delegators: Default::default(),
				total_stake: Default::default(),
			})
			.collect::<Vec<CandidateInfoOf<Test>>>();
		assert_noop!(
			CollatorSelection::set_invulnerables(RuntimeOrigin::root(), invulnerables_formatted),
			Error::<Test>::ValidatorNotRegistered
		);
	});
}

#[test]
fn set_desired_candidates_works() {
	new_test_ext().execute_with(|| {
		// given
		assert_eq!(CollatorSelection::desired_candidates(), 2);

		// can set
		assert_ok!(CollatorSelection::set_desired_candidates(RuntimeOrigin::root(), 7));
		assert_eq!(CollatorSelection::desired_candidates(), 7);

		// rejects bad origin
		assert_noop!(
			CollatorSelection::set_desired_candidates(RuntimeOrigin::signed(1), 8),
			BadOrigin
		);
	});
}

#[test]
fn set_candidacy_bond() {
	new_test_ext().execute_with(|| {
		// given
		assert_eq!(CollatorSelection::candidacy_bond(), 10);

		// can set
		assert_ok!(CollatorSelection::set_candidacy_bond(RuntimeOrigin::root(), 7));
		assert_eq!(CollatorSelection::candidacy_bond(), 7);

		// rejects bad origin.
		assert_noop!(CollatorSelection::set_candidacy_bond(RuntimeOrigin::signed(1), 8), BadOrigin);
	});
}

#[test]
fn cannot_register_candidate_if_too_many() {
	new_test_ext().execute_with(|| {
		// reset desired candidates:
		<crate::DesiredCandidates<Test>>::put(0);

		// can't accept anyone anymore.
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)),
			Error::<Test>::TooManyCandidates,
		);

		// reset desired candidates:
		<crate::DesiredCandidates<Test>>::put(1);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));

		// but no more
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(5)),
			Error::<Test>::TooManyCandidates,
		);
	})
}

#[test]
fn cannot_unregister_candidate_if_too_few() {
	new_test_ext().execute_with(|| {
		// reset desired candidates:
		<crate::DesiredCandidates<Test>>::put(1);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));

		// can not remove too few
		assert_noop!(
			CollatorSelection::leave_intent(RuntimeOrigin::signed(4)),
			Error::<Test>::TooFewCandidates,
		);
	})
}

#[test]
fn cannot_register_as_candidate_if_invulnerable() {
	new_test_ext().execute_with(|| {
		// can't 1 because it is invulnerable.
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(1)),
			Error::<Test>::AlreadyCandidate,
		);
	})
}

#[test]
fn cannot_register_as_candidate_if_keys_not_registered() {
	new_test_ext().execute_with(|| {
		// can't 7 because keys not registered.
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(7)),
			Error::<Test>::ValidatorNotRegistered
		);
	})
}

#[test]
fn cannot_register_dupe_candidate() {
	new_test_ext().execute_with(|| {
		// can add 3 as candidate
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		let addition = CandidateInfoOf::<Test> {
			who: 3u64,
			deposit: 10u64,
			delegators: Default::default(),
			total_stake: 10u64,
		};
		assert_eq!(CollatorSelection::candidates().pop().unwrap(), addition);
		assert_eq!(CollatorSelection::last_authored_block(3), 10);
		assert_eq!(Balances::free_balance(3), 90);

		// but no more
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)),
			Error::<Test>::AlreadyCandidate,
		);
	})
}

#[test]
fn cannot_register_as_candidate_if_poor() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(&3), 100);
		assert_eq!(Balances::free_balance(&33), 0);

		// works
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));

		// poor
		assert_noop!(
			CollatorSelection::register_as_candidate(RuntimeOrigin::signed(33)),
			BalancesError::<Test>::InsufficientBalance,
		);
	});
}

#[test]
fn register_as_candidate_works() {
	new_test_ext().execute_with(|| {
		// given
		assert_eq!(CollatorSelection::desired_candidates(), 2);
		assert_eq!(CollatorSelection::candidacy_bond(), 10);
		assert_eq!(CollatorSelection::candidates(), Vec::new());

		// take two endowed, non-invulnerables accounts.
		assert_eq!(Balances::free_balance(&3), 100);
		assert_eq!(Balances::free_balance(&4), 100);

		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));

		assert_eq!(Balances::free_balance(&3), 90);
		assert_eq!(Balances::free_balance(&4), 90);

		assert_eq!(CollatorSelection::candidates().len(), 2);
	});
}

#[test]
fn leave_intent() {
	new_test_ext().execute_with(|| {
		// register a candidate.
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_eq!(Balances::free_balance(3), 90);

		// register too so can leave above min candidates
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(5)));
		assert_eq!(Balances::free_balance(5), 90);

		// cannot leave if not candidate.
		assert_noop!(
			CollatorSelection::leave_intent(RuntimeOrigin::signed(4)),
			Error::<Test>::NotCandidate
		);

		assert_ok!(CollatorSelection::leave_intent(RuntimeOrigin::signed(3)));

		// bond is not returned immediately
		assert_eq!(Balances::free_balance(3), 90);
		assert_eq!(Balances::reserved_balance(3), 10);
	});
}

#[test]
fn candidate_withdraw_unbonded() {
	new_test_ext().execute_with(|| {
		// register a candidate.
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_eq!(Balances::free_balance(3), 90);

		// register too so can leave above min candidates
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(5)));

		assert_ok!(CollatorSelection::leave_intent(RuntimeOrigin::signed(3)));

		// bond is not returned immediately
		assert_eq!(Balances::free_balance(3), 90);
		assert_eq!(Balances::reserved_balance(3), 10);

		// calling withdraw before expiry fails
		assert_noop!(
			CollatorSelection::candidate_withdraw_unbonded(RuntimeOrigin::signed(3), 3),
			Error::<Test>::UnbondingDelayNotPassed
		);
		initialize_to_block(10);
		assert_ok!(CollatorSelection::candidate_withdraw_unbonded(RuntimeOrigin::signed(3), 3));

		// bond is correctly returned
		assert_eq!(Balances::free_balance(3), 100);
		assert_eq!(Balances::reserved_balance(3), 0);
		assert_eq!(CollatorSelection::last_authored_block(3), 0);
	});
}

#[test]
fn authorship_event_handler() {
	new_test_ext().execute_with(|| {
		// put 100 in the pot + 5 for ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 105);
		assert_ok!(CollatorSelection::set_block_inflation_reward(RuntimeOrigin::root(), 10));

		// 4 is the default author.
		assert_eq!(Balances::free_balance(4), 100);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		// triggers `note_author`
		Authorship::on_initialize(1);

		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 120, // deposit of 10 + block_reward of 100 + inflation reward of 10
			delegators: Default::default(),
			total_stake: 120, // deposit of 10 + block_reward of 100 + inflation reward of 10
		};

		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 0);

		// balance should not be updated, it should be 100 - candidate bond
		assert_eq!(Balances::free_balance(4), 90);
		// no change in the pot balance
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);
	});
}

#[test]
fn fees_edgecases() {
	new_test_ext().execute_with(|| {
		// Nothing panics, no reward when no ED in balance
		Authorship::on_initialize(1);
		// put some money into the pot at ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 5);
		// 4 is the default author.
		assert_eq!(Balances::free_balance(4), 100);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		// triggers `note_author`
		Authorship::on_initialize(1);

		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 10,
			delegators: Default::default(),
			total_stake: 10,
		};

		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 0);
		// Nothing received
		assert_eq!(Balances::free_balance(4), 90);
		// all fee stays
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 5);
	});
}

#[test]
fn session_management_works() {
	new_test_ext().execute_with(|| {
		initialize_to_block(1);

		assert_eq!(SessionChangeBlock::get(), 0);
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2]);

		initialize_to_block(4);

		assert_eq!(SessionChangeBlock::get(), 0);
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2]);

		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));

		// session won't see this.
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2]);
		// but we have a new candidate.
		assert_eq!(CollatorSelection::candidates().len(), 1);

		initialize_to_block(10);
		assert_eq!(SessionChangeBlock::get(), 10);
		// pallet-session has 1 session delay; current validators are the same.
		assert_eq!(Session::validators(), vec![1, 2]);
		// queued ones are changed, and now we have 3.
		assert_eq!(Session::queued_keys().len(), 3);
		// session handlers (aura, et. al.) cannot see this yet.
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2]);

		initialize_to_block(20);
		assert_eq!(SessionChangeBlock::get(), 20);
		// changed are now reflected to session handlers.
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2, 3]);
	});
}

#[test]
fn kick_mechanism() {
	new_test_ext().execute_with(|| {
		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		initialize_to_block(10);
		assert_eq!(CollatorSelection::candidates().len(), 2);
		initialize_to_block(20);
		assert_eq!(SessionChangeBlock::get(), 20);
		// 4 authored this block, gets to stay 3 was kicked
		assert_eq!(CollatorSelection::candidates().len(), 1);
		// 3 will be kicked after 1 session delay
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2, 3, 4]);
		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 10,
			delegators: Default::default(),
			total_stake: 10,
		};
		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 20);
		initialize_to_block(30);
		// 3 gets kicked after 1 session delay
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2, 4]);
	});
}

#[test]
fn should_not_kick_mechanism_too_few() {
	new_test_ext().execute_with(|| {
		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(5)));
		initialize_to_block(10);
		assert_eq!(CollatorSelection::candidates().len(), 2);
		initialize_to_block(20);
		assert_eq!(SessionChangeBlock::get(), 20);
		// 4 authored this block, 5 gets to stay too few 3 was kicked
		assert_eq!(CollatorSelection::candidates().len(), 1);
		// 3 will be kicked after 1 session delay
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2, 3, 5]);
		let collator = CandidateInfoOf::<Test> {
			who: 5,
			deposit: 10,
			delegators: Default::default(),
			total_stake: 10,
		};
		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 20);
		initialize_to_block(30);
		// 3 gets kicked after 1 session delay
		assert_eq!(SessionHandlerCollators::get(), vec![1, 2, 5]);
	});
}

#[test]
#[should_panic = "duplicate invulnerables in genesis."]
fn cannot_set_genesis_value_twice() {
	sp_tracing::try_init_simple();
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let invulnerables = vec![1, 1]
		.iter()
		.cloned()
		.map(|account| CandidateInfoOf::<Test> {
			who: account,
			deposit: Default::default(),
			delegators: Default::default(),
			total_stake: Default::default(),
		})
		.collect::<Vec<CandidateInfoOf<Test>>>();

	let collator_selection = collator_selection::GenesisConfig::<Test> {
		desired_candidates: 2,
		candidacy_bond: 10,
		invulnerables,
	};
	// collator selection must be initialized before session.
	collator_selection.assimilate_storage(&mut t).unwrap();
}

#[test]
fn delegate_works() {
	new_test_ext().execute_with(|| {
		// delegate to non existing candidate should fail
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 19),
			Error::<Test>::NotCandidate
		);

		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		initialize_to_block(10);

		// should fail if candidate and delegators is same
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(3), 3, 10),
			Error::<Test>::DelegatorAccountSameAsCandidateAccount
		);

		// delegate less than min delegation should fail
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 9),
			Error::<Test>::LessThanMinimumDelegation
		);

		assert_eq!(CollatorSelection::candidates().len(), 1);

		// should fail if the amount is not available to reserve
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(6), 3, 10),
			pallet_balances::Error::<Test>::InsufficientBalance
		);

		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 10));

		// duplicate delegation should fail with different amount
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 20),
			Error::<Test>::AlreadyDelegated
		);
		// duplicate delegation should fail with same amount
		assert_noop!(
			CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 10),
			Error::<Test>::AlreadyDelegated
		);

		// storage should be updated correctly
		let expected_delegator_info = DelegationInfoOf::<Test> { who: 5, deposit: 10 };
		assert_eq!(CollatorSelection::candidates()[0].delegators, vec![expected_delegator_info]);
		assert_eq!(CollatorSelection::candidates()[0].total_stake, 10 + 10);
		// the balane should be reserved correctly
		assert_eq!(Balances::reserved_balance(5), 10);
	});
}

#[test]
fn delegate_works_for_invulnerable() {
	new_test_ext().execute_with(|| {
		// we know that 1 is an invulnerable
		let invulnerable_collator = 1;
		let delegation_amount = 10;

		// delegate less than min delegation should fail
		assert_noop!(
			CollatorSelection::delegate(
				RuntimeOrigin::signed(5),
				invulnerable_collator,
				delegation_amount - 1
			),
			Error::<Test>::LessThanMinimumDelegation
		);

		assert_eq!(CollatorSelection::candidates().len(), 0);

		// should fail if the amount is not available to reserve
		assert_noop!(
			CollatorSelection::delegate(
				RuntimeOrigin::signed(6),
				invulnerable_collator,
				delegation_amount
			),
			pallet_balances::Error::<Test>::InsufficientBalance
		);

		println!("{:?}", Invulnerables::<Test>::get());

		assert_ok!(CollatorSelection::delegate(
			RuntimeOrigin::signed(5),
			invulnerable_collator,
			delegation_amount
		));
		// storage should be updated correctly
		let expected_delegator_info =
			DelegationInfoOf::<Test> { who: 5, deposit: delegation_amount };
		assert_eq!(CollatorSelection::invulnerables()[0].delegators, vec![expected_delegator_info]);
		assert_eq!(CollatorSelection::invulnerables()[0].total_stake, delegation_amount);
		// the balance should be reserved correctly
		assert_eq!(Balances::reserved_balance(5), delegation_amount);
	});
}

#[test]
fn undelegate_works() {
	new_test_ext().execute_with(|| {
		// undelegate to non existing candidate should fail
		assert_noop!(
			CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3),
			Error::<Test>::NotCandidate
		);

		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		initialize_to_block(10);

		// undelegate without any delegation should fail
		assert_noop!(
			CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3),
			Error::<Test>::NotDelegator
		);

		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 10));
		assert_ok!(CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3));
		assert_eq!(CollatorSelection::candidates().len(), 1);
		// storage should be updated correctly
		assert_eq!(CollatorSelection::candidates()[0].delegators, vec![]);
		assert_eq!(CollatorSelection::candidates()[0].total_stake, 10);
		assert_eq!(CollatorSelection::unbonded_delegates(5).unwrap().deposit, 10);
		assert_eq!(CollatorSelection::unbonded_delegates(5).unwrap().unbonded_at, 10);

		// the balance is not immediately updated
		assert_eq!(Balances::reserved_balance(5), 10);
		assert_eq!(Balances::free_balance(5), 90);
	});
}

#[test]
fn withdraw_unbonded_works() {
	new_test_ext().execute_with(|| {
		// undelegate to non existing candidate should fail
		assert_noop!(
			CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3),
			Error::<Test>::NotCandidate
		);

		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		initialize_to_block(10);

		// undelegate without any delegation should fail
		assert_noop!(
			CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3),
			Error::<Test>::NotDelegator
		);

		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 10));

		// should fail if no unbonded delegation exists
		assert_noop!(
			CollatorSelection::withdraw_unbonded(RuntimeOrigin::signed(5)),
			Error::<Test>::NoUnbondingDelegation
		);

		assert_ok!(CollatorSelection::undelegate(RuntimeOrigin::signed(5), 3));

		// the balance is not immediately updated
		assert_eq!(Balances::reserved_balance(5), 10);
		assert_eq!(Balances::free_balance(5), 90);

		// skip to block before unbonding period
		initialize_to_block(19);

		// should fail since the unbonding period has not passed
		assert_noop!(
			CollatorSelection::withdraw_unbonded(RuntimeOrigin::signed(5)),
			Error::<Test>::UnbondingDelayNotPassed
		);

		initialize_to_block(20);
		assert_ok!(CollatorSelection::withdraw_unbonded(RuntimeOrigin::signed(5)));

		// the balance should be updated correctly
		assert_eq!(Balances::reserved_balance(5), 0);
		assert_eq!(Balances::free_balance(5), 100);
	});
}

#[test]
fn candidate_leave_removes_delegates() {
	new_test_ext().execute_with(|| {
		<crate::DesiredCandidates<Test>>::put(2);
		// add a new collator
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(3)));
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		initialize_to_block(10);

		// an account has delegated to this collator
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 3, 10));
		println!("{:?}", CollatorSelection::candidates());
		assert_eq!(CollatorSelection::candidates().len(), 2);

		// candidate leaves and bond is returned
		assert_ok!(CollatorSelection::leave_intent(RuntimeOrigin::signed(3)));
		assert_eq!(CollatorSelection::candidates().len(), 1);

		// balance is not immediately returned
		assert_eq!(Balances::reserved_balance(3), 10);
		assert_eq!(Balances::free_balance(3), 90);

		// skip to after unbonding period
		initialize_to_block(20);

		// withdraw the unbonded balance, any account can make this call
		assert_ok!(CollatorSelection::candidate_withdraw_unbonded(RuntimeOrigin::signed(30), 3));
		assert_eq!(Balances::free_balance(3), 100);
		assert_eq!(CollatorSelection::last_authored_block(3), 0);

		// delegator bond is also returned
		assert_eq!(Balances::free_balance(5), 100);
	});
}

#[test]
fn undelegate_works_for_invulnerable() {
	new_test_ext().execute_with(|| {
		// we know that 1 is an invulnerable
		let invulnerable_collator = 1;
		let delegation_amount = 10;
		let delegator_account = 5;

		// undelegate without any delegation should fail
		assert_noop!(
			CollatorSelection::undelegate(
				RuntimeOrigin::signed(delegator_account),
				invulnerable_collator
			),
			Error::<Test>::NotDelegator
		);

		assert_ok!(CollatorSelection::delegate(
			RuntimeOrigin::signed(delegator_account),
			invulnerable_collator,
			delegation_amount
		));
		assert_ok!(CollatorSelection::undelegate(
			RuntimeOrigin::signed(delegator_account),
			invulnerable_collator
		));
		assert_eq!(CollatorSelection::candidates().len(), 0);
		// storage should be updated correctly
		assert_eq!(CollatorSelection::invulnerables()[0].delegators, vec![]);
		assert_eq!(CollatorSelection::invulnerables()[0].total_stake, 0);
		// the balane should not be immediately updated
		assert_eq!(Balances::reserved_balance(delegator_account), 10);
		assert_eq!(Balances::free_balance(delegator_account), 90);
	});
}

#[test]
fn delegator_payout_works() {
	new_test_ext().execute_with(|| {
		// put 100 in the pot + 5 for ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 105);
		// block inflation reward is 50
		assert_ok!(CollatorSelection::set_block_inflation_reward(RuntimeOrigin::root(), 50));

		// 4 is the default author.
		assert_eq!(Balances::free_balance(4), 100);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		// two delegators delegators to 4
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(3), 4, 10));
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 4, 10));
		// triggers `note_author`
		Authorship::on_initialize(4);

		// this is the expected result
		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 10 + 15, // initial bond of 10 + 10% of reward (150)
			delegators: vec![
				// initial bond of 10 + 90% of reward (150) divided equally to two delegators
				DelegationInfoOf::<Test> { who: 3u64, deposit: 10 + 67 }, /* initial bond of 10
				                                                           * + 45% of reward
				                                                           * (67) */
				DelegationInfoOf::<Test> { who: 5u64, deposit: 10 + 67 }, /* initial bond of 10
				                                                           * + 45% of reward
				                                                           * (67) */
			]
			.try_into()
			.unwrap(),
			total_stake: 180, // initial bond of 30 + 100% of reward (150)
		};

		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 0);

		// balances should not change
		assert_eq!(Balances::free_balance(4), 90);
		assert_eq!(Balances::free_balance(3), 90);
		assert_eq!(Balances::free_balance(5), 90);
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);
	});
}

#[test]
fn delegator_payout_works_for_invulnerables() {
	new_test_ext().execute_with(|| {
		let invulnerable_collator = 4;
		// put 100 in the pot + 5 for ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 105);
		// block inflation reward is 50
		assert_ok!(CollatorSelection::set_block_inflation_reward(RuntimeOrigin::root(), 50));

		// set the 4 account as invulnerable
		let new_set = vec![4];
		let new_set_formatted = new_set
			.iter()
			.cloned()
			.map(|account| CandidateInfoOf::<Test> {
				who: account,
				deposit: Default::default(),
				delegators: Default::default(),
				total_stake: Default::default(),
			})
			.collect::<Vec<CandidateInfoOf<Test>>>();
		assert_ok!(CollatorSelection::set_invulnerables(RuntimeOrigin::root(), new_set_formatted));

		// 4 is invulnerable and the default author.
		assert_eq!(Balances::free_balance(invulnerable_collator), 100);
		// two delegators delegators to 1
		assert_ok!(CollatorSelection::delegate(
			RuntimeOrigin::signed(3),
			invulnerable_collator,
			10
		));
		assert_ok!(CollatorSelection::delegate(
			RuntimeOrigin::signed(5),
			invulnerable_collator,
			10
		));
		// triggers `note_author`
		Authorship::on_initialize(invulnerable_collator);

		let collator = CandidateInfoOf::<Test> {
			who: invulnerable_collator,
			deposit: 15, // initial bond of 0 + 10% of reward (150)
			delegators: vec![
				// initial bond of 10 + 90% of reward (150) divided equally to two delegators
				DelegationInfoOf::<Test> { who: 3u64, deposit: 10 + 67 }, /* initial bond of 10
				                                                           * + 45% of reward
				                                                           * (67) */
				DelegationInfoOf::<Test> { who: 5u64, deposit: 10 + 67 }, /* initial bond of 10
				                                                           * + 45% of reward
				                                                           * (67) */
			]
			.try_into()
			.unwrap(),
			total_stake: 170, // initial bond of 20 + 100% of reward (150)
		};

		assert_eq!(CollatorSelection::invulnerables()[0], collator);
		assert_eq!(CollatorSelection::last_authored_block(invulnerable_collator), 0);

		// balances should not change
		assert_eq!(Balances::free_balance(invulnerable_collator), 100);
		assert_eq!(Balances::free_balance(3), 90);
		assert_eq!(Balances::free_balance(5), 90);
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);
	});
}

#[test]
fn delegator_payout_is_divided_in_correct_propotion() {
	new_test_ext().execute_with(|| {
		// put 100 in the pot + 5 for ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 105);
		Balances::make_free_balance_be(&6, 100);
		// block inflation reward is 50
		assert_ok!(CollatorSelection::set_block_inflation_reward(RuntimeOrigin::root(), 50));

		// 4 is the default author.
		assert_eq!(Balances::free_balance(4), 100);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		// three delegators delegators to 4
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(3), 4, 30));
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 4, 20));
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(6), 4, 10));
		// triggers `note_author`
		Authorship::on_initialize(4);

		// this is the expected result
		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 10 + 15, // initial bond of 10 + 10% of reward (150)
			delegators: vec![
				// initial bond of 10 + 90% of reward (135) divided in propotion of stake to 3
				// delegators
				DelegationInfoOf::<Test> { who: 3u64, deposit: 30 + 67 }, /* initial bond of 30
				                                                           * + 50% of reward
				                                                           * (75) */
				DelegationInfoOf::<Test> { who: 5u64, deposit: 20 + 44 }, /* initial bond of 10
				                                                           * + 33% of reward
				                                                           * (44) */
				DelegationInfoOf::<Test> { who: 6u64, deposit: 10 + 22 }, /* initial bond of 10
				                                                           * + 16% of reward
				                                                           * (22) */
			]
			.try_into()
			.unwrap(),
			total_stake: 220, // initial bond of 70 + 100% of reward
		};

		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 0);

		// balances should not change
		assert_eq!(Balances::free_balance(4), 90);
		assert_eq!(Balances::free_balance(3), 70);
		assert_eq!(Balances::free_balance(5), 80);
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);
	});
}

#[test]
fn delegator_payout_complete_flow_test() {
	new_test_ext().execute_with(|| {
		// put 100 in the pot + 5 for ED
		Balances::make_free_balance_be(&CollatorSelection::account_id(), 105);
		Balances::make_free_balance_be(&6, 100);
		// block inflation reward is 50
		assert_ok!(CollatorSelection::set_block_inflation_reward(RuntimeOrigin::root(), 50));

		// 4 is the default author.
		assert_eq!(Balances::free_balance(4), 100);
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(4)));
		// three delegators delegators to 4
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(3), 4, 30));
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(5), 4, 20));
		assert_ok!(CollatorSelection::delegate(RuntimeOrigin::signed(6), 4, 10));
		// triggers `note_author`
		Authorship::on_initialize(4);

		// this is the expected result
		let collator = CandidateInfoOf::<Test> {
			who: 4,
			deposit: 10 + 15, // initial bond of 10 + 10% of reward (150)
			delegators: vec![
				// initial bond of 10 + 90% of reward (135) divided in propotion of stake to 3
				// delegators
				DelegationInfoOf::<Test> { who: 3u64, deposit: 30 + 67 }, /* initial bond of 30
				                                                           * + 50% of reward
				                                                           * (75) */
				DelegationInfoOf::<Test> { who: 5u64, deposit: 20 + 44 }, /* initial bond of 10
				                                                           * + 33% of reward
				                                                           * (44) */
				DelegationInfoOf::<Test> { who: 6u64, deposit: 10 + 22 }, /* initial bond of 10
				                                                           * + 16% of reward
				                                                           * (22) */
			]
			.try_into()
			.unwrap(),
			total_stake: 220, // initial bond of 70 + 100% of reward
		};

		assert_eq!(CollatorSelection::candidates().pop().unwrap(), collator);
		assert_eq!(CollatorSelection::last_authored_block(4), 0);

		// balances should not change
		assert_eq!(Balances::free_balance(4), 90);
		assert_eq!(Balances::free_balance(3), 70);
		assert_eq!(Balances::free_balance(5), 80);
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);

		// bond another candidate so existing can unbond
		assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(100)));

		// let the candidate unbond
		assert_ok!(CollatorSelection::leave_intent(RuntimeOrigin::signed(4)));
		assert_eq!(CollatorSelection::candidates().len(), 1);

		// balance is not immediately returned
		assert_eq!(Balances::free_balance(4), 90);
		assert_eq!(Balances::free_balance(3), 70);
		assert_eq!(Balances::free_balance(5), 80);
		assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 105);

		// skip to after unbonding period
		initialize_to_block(20);

		// let the collator call withdraw unbond
		assert_ok!(CollatorSelection::candidate_withdraw_unbonded(RuntimeOrigin::signed(4), 4));
		assert_eq!(CollatorSelection::last_authored_block(3), 0);

		// collator bond is returned + rewards
		assert_eq!(Balances::free_balance(4), 90 + 10 + 15);

		// delegator bond is also returned + rewards
		assert_eq!(Balances::free_balance(3), 70 + 30 + 67);
		assert_eq!(Balances::free_balance(5), 80 + 20 + 44);
	});
}

#[test]
fn test_remove_duplicate_delegators() {
	use crate::{migration::v3::MigrateToV3, types::DelegationInfo};

	let delegate_1 = DelegationInfo { who: 1, deposit: 1 };

	let delegate_2 = DelegationInfo { who: 2, deposit: 1 };

	let delegate_3 = DelegationInfo { who: 3, deposit: 1 };

	let no_delegators = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![].try_into().unwrap(),
		total_stake: 1,
	}];

	// no change should happen
	assert_eq!(
		MigrateToV3::<Test>::remove_duplicate_delegators(no_delegators.clone()),
		no_delegators
	);

	let no_duplicates = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![delegate_1.clone(), delegate_2.clone(), delegate_3.clone()]
			.try_into()
			.unwrap(),
		total_stake: 1,
	}];

	assert_eq!(
		MigrateToV3::<Test>::remove_duplicate_delegators(no_duplicates.clone()),
		no_duplicates
	);

	let some_duplicates = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![
			delegate_1.clone(),
			delegate_1.clone(),
			delegate_2.clone(),
			delegate_3.clone(),
		]
		.try_into()
		.unwrap(),
		total_stake: 1,
	}];

	let expected_some_duplicates = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![
			DelegationInfo { who: 1, deposit: 2 },
			delegate_2.clone(),
			delegate_3.clone(),
		]
		.try_into()
		.unwrap(),
		total_stake: 1,
	}];

	assert_eq!(
		MigrateToV3::<Test>::remove_duplicate_delegators(some_duplicates),
		expected_some_duplicates
	);

	let many_duplicates = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![
			delegate_2.clone(),
			delegate_3.clone(),
			delegate_1.clone(),
			delegate_1.clone(),
			DelegationInfo { who: 1, deposit: 10 },
			delegate_3.clone(),
			delegate_3.clone(),
		]
		.try_into()
		.unwrap(),
		total_stake: 1,
	}];

	let expected_many_duplicates = vec![CandidateInfoOf::<Test> {
		who: 1,
		deposit: 1,
		delegators: vec![
			DelegationInfo { who: 1, deposit: 12 },
			delegate_2,
			DelegationInfo { who: 3, deposit: 3 },
		]
		.try_into()
		.unwrap(),
		total_stake: 1,
	}];

	assert_eq!(
		MigrateToV3::<Test>::remove_duplicate_delegators(many_duplicates),
		expected_many_duplicates
	);
}
