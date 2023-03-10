#![allow(dead_code)]
#![allow(unused_variables)]

use crate as sudo;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64},
    PalletId,
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use sp_std::result::Result;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"ent--dao");
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        DAO: daoent_dao::{ Pallet, Call, Event<T>, Storage },
        DAOSudo: sudo::{ Pallet, Call, Event<T>, Storage },
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl TryFrom<RuntimeCall> for u64 {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        // match call {
        //     _ => Ok(0u64),
        // }
        Ok(0u64)
    }
}

impl daoent_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = u64;
    type PalletId = DaoPalletId;
    type AfterCreate = ();
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
}

impl sudo::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub fn new_test_run() -> sp_io::TestExternalities {
    let t = GenesisConfig {
        system: Default::default(),
    }
    .build_storage()
    .unwrap();
    t.into()
}
