#![allow(unused_imports)]
#![cfg(test)]

use crate as daoent_sudo;
use crate::mock::{RuntimeCall, *};
use daoent_primitives::types::Nft;
use frame_support::{assert_noop, assert_ok, debug};

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAO_ID: u64 = 1;

pub fn create_dao() {
    daoent_dao::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        Nft(0u64),
        vec![1; 4],
        vec![1; 4],
    )
    .unwrap();
}
