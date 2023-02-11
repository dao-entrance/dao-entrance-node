#![allow(unused_imports)]
#![cfg(test)]

use super::*;
use frame_support::assert_ok;
use mock::{RuntimeCall, RuntimeOrigin, *};
use sp_runtime::traits::BlakeTwo256;

use daoent_primitives::types::Nft;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAO_ID: u64 = 1;

pub fn create_dao() {
    daoent_dao::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        Nft(0u64),
        vec![1; 4],
        vec![],
    )
    .unwrap();
}

pub fn propose() {
    create_dao();
    frame_system::Pallet::<Test>::set_block_number(10000);
    assert!(Pallet::<Test>::start_referendum(RuntimeOrigin::signed(ALICE), 0u64).is_err());
    frame_system::Pallet::<Test>::set_block_number(0);
    let proposal = RuntimeCall::DAOGov(Call::set_min_vote_weight_for_every_call {
        dao_id: DAO_ID,
        call_id: 0u64,
        min_vote_weight: 100u64,
    });
    assert_ok!(Pallet::<Test>::create_propose(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        Box::new(proposal),
        0u64
    ));
}

pub fn second() {
    propose();
    assert_ok!(Pallet::<Test>::recreate(
        RuntimeOrigin::signed(BOB),
        DAO_ID,
        0u32
    ));
}

pub fn start_referendum() {
    second();
    assert!(Pallet::<Test>::start_referendum(RuntimeOrigin::signed(ALICE), DAO_ID).is_err());
    frame_system::Pallet::<Test>::set_block_number(10000);
    assert_ok!(Pallet::<Test>::start_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID
    ));
}

pub fn vote() {
    start_referendum();
    assert_ok!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        (),
        Opinion::YES,
    ));
    assert_ok!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        (),
        Opinion::YES,
    ));
    assert_ok!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        (),
        Opinion::NO,
    ));
    frame_system::Pallet::<Test>::set_block_number(20000);
    assert!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        (),
        Opinion::NO,
    )
    .is_err());
    frame_system::Pallet::<Test>::set_block_number(10000);
}

pub fn run() {
    vote();
    assert!(Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), DAO_ID, 0u32).is_err());
    frame_system::Pallet::<Test>::set_block_number(
        10000 + VotingPeriod::<Test>::get(0u64) + EnactmentPeriod::<Test>::get(0u64) - 2,
    );
    assert!(Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), DAO_ID, 0u32).is_err());
    frame_system::Pallet::<Test>::set_block_number(20000);

    let ole_min_weight = MinVoteWeightOf::<Test>::get(0u64, 0u64);
    MinVoteWeightOf::<Test>::insert(0u64, 0u64, 10000000000);
    assert!(Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), 0u64, 0u32).is_err());
    MinVoteWeightOf::<Test>::insert(0u64, 0u64, ole_min_weight);

    assert_ok!(Pallet::<Test>::run_proposal(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32
    ));
    assert!(Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), DAO_ID, 0u32).is_err());
    assert!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        (),
        Opinion::NO,
    )
    .is_err());
    assert!(Pallet::<Test>::cancel_vote(RuntimeOrigin::signed(ALICE), DAO_ID, 0u32).is_err());
}

#[test]
pub fn propose_should_work() {
    new_test_run().execute_with(|| {
        propose();
    });
}

#[test]
pub fn second_should_work() {
    new_test_run().execute_with(|| second());
}

#[test]
pub fn vote_should_work() {
    new_test_run().execute_with(|| {
        vote();
    });
}

#[test]
pub fn cancel_vote_should_work() {
    new_test_run().execute_with(|| {
        vote();
        frame_system::Pallet::<Test>::set_block_number(20000);
        assert!(Pallet::<Test>::cancel_vote(RuntimeOrigin::signed(ALICE), DAO_ID, 0u32).is_err());
        frame_system::Pallet::<Test>::set_block_number(10000);
        assert_ok!(Pallet::<Test>::cancel_vote(
            RuntimeOrigin::signed(ALICE),
            DAO_ID,
            0u32
        ));
    });
}

#[test]
pub fn run_proposal_should_work() {
    new_test_run().execute_with(|| {
        run();
    });
}

#[test]
pub fn unlock_should_work() {
    new_test_run().execute_with(|| {
        run();
        assert_ok!(Pallet::<Test>::unlock(RuntimeOrigin::signed(ALICE)));
    });
}
