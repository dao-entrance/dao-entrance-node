#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_app;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

pub const ALICE: u64 = 1;
// pub const BOB: u64 = 2;
// pub const DAO_ID: u64 = 1;

#[test]
pub fn set_sudo() {
    new_test_run().execute_with(|| {
        assert_ok!(WeteeApp::create_app(
            RuntimeOrigin::signed(ALICE),
            vec![1; 4],
            vec![1; 4],
            vec![80]
        ));

        assert_ok!(WeteeApp::run_app(RuntimeOrigin::signed(ALICE), 1));

        // frame_system::Pallet::<Test>::set_block_number(10000);
        // assert_eq!(crate::Account::<Test>::get(DAO_ID), Some(BOB));
    });
}
