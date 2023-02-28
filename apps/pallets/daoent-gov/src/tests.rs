#![allow(unused_imports)]
#![cfg(test)]

use super::*;
use frame_support::assert_ok;
use mock::{RuntimeCall, RuntimeOrigin, *};
use sp_runtime::traits::BlakeTwo256;

pub fn create_dao() {
    daoent_dao::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        vec![1; 4],
        vec![1; 4],
        vec![],
    )
    .unwrap();

    daoent_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        daoent_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        10000,
        99,
    )
    .unwrap();
}

pub fn propose() {
    create_dao();
    assert!(Pallet::<Test>::start_referendum(RuntimeOrigin::signed(ALICE), DAO_ID, P_ID).is_err());
    frame_system::Pallet::<Test>::set_block_number(0);
    let proposal = RuntimeCall::DAOGov(Call::set_min_vote_weight_for_every_call {
        dao_id: DAO_ID,
        call_id: 0,
        min_vote_weight: 100u64,
    });
    assert_ok!(Pallet::<Test>::create_propose(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        MemmberData::GLOBAL,
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
    assert!(Pallet::<Test>::start_referendum(RuntimeOrigin::signed(ALICE), DAO_ID, P_ID).is_err());
    frame_system::Pallet::<Test>::set_block_number(10000);
    assert_ok!(Pallet::<Test>::start_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        P_ID
    ));
}

pub fn vote() {
    start_referendum();
    assert_ok!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        Opinion::YES,
    ));
    assert!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        Opinion::YES,
    )
    .is_err());
    frame_system::Pallet::<Test>::set_block_number(20000);
    assert!(Pallet::<Test>::vote_for_referendum(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
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

    let ole_min_weight = MinVoteWeightOf::<Test>::get(0u64, 0);
    MinVoteWeightOf::<Test>::insert(0u64, 0, 10000000000);
    assert!(Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), 0u64, 0u32).is_err());
    MinVoteWeightOf::<Test>::insert(0u64, 0, ole_min_weight);

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
        assert_ok!(Pallet::<Test>::unlock(RuntimeOrigin::signed(ALICE), DAO_ID));
    });
}
