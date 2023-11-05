use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, bounded_vec, pallet_prelude::ConstU32, BoundedVec};
use sp_runtime::traits::BadOrigin;

#[test]
fn reward() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		RewardModule::add_contribution(&1);
		assert_ok!(RewardModule::reward(RuntimeOrigin::signed(0), 1, 10));
		System::assert_last_event(Event::AccountRewarded { who: 0, account: 1, amount: 10 }.into());
	});
}

#[test]
fn noop_reward() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&1);
		assert_noop!(
			RewardModule::reward(RuntimeOrigin::signed(0), 1, 10),
			Error::<Test>::AccountNeedToParticipateBeforeVoting
		);
	});
}

#[test]
fn noop_reward_() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		assert_noop!(
			RewardModule::reward(RuntimeOrigin::signed(0), 1, 10),
			Error::<Test>::AccountNotFound
		);
	});
}

#[test]
fn noop_reward_itself() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		assert_noop!(
			RewardModule::reward(RuntimeOrigin::signed(0), 0, 10),
			Error::<Test>::CannotRewardItself
		);
	});
}

#[test]
fn punish() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		RewardModule::add_contribution(&1);
		assert_ok!(RewardModule::punish(RuntimeOrigin::signed(0), 1, 10));
		System::assert_last_event(Event::AccountPunished { who: 0, account: 1, amount: 10 }.into());
	});
}

#[test]
fn noop_punish() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&1);
		assert_noop!(
			RewardModule::punish(RuntimeOrigin::signed(0), 1, 10),
			Error::<Test>::AccountNeedToParticipateBeforeVoting
		);
	});
}

#[test]
fn noop_punish_() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		assert_noop!(
			RewardModule::punish(RuntimeOrigin::signed(0), 1, 10),
			Error::<Test>::AccountNotFound
		);
	});
}

#[test]
fn noop_punish_itself() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		RewardModule::add_contribution(&0);
		assert_noop!(
			RewardModule::punish(RuntimeOrigin::signed(0), 0, 10),
			Error::<Test>::CannotRewardItself
		);
	});
}
