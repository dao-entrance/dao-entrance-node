#![allow(dead_code)]
#![allow(unused_variables)]

use crate as daoent_gov;
use crate::Pledge;
use codec::{Decode, Encode, MaxEncodedLen};
use daoent_primitives::{traits::BaseCallFilter, types::Nft};
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64},
    PalletId, RuntimeDebug,
};
use frame_system;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchError,
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
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        DAO: daoent_dao::{ Pallet, Call, Event<T>, Storage },
        DAOSudo: daoent_sudo::{ Pallet, Call, Event<T>, Storage },
        DAOGov: daoent_gov::{ Pallet, Call, Event<T>, Storage },
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
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
        match call {
            _ => Ok(0u64),
        }
    }
}
impl BaseCallFilter<RuntimeCall> for Nft<u64> {
    fn contains(&self, call: RuntimeCall) -> bool {
        true
    }
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl daoent_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = u64;
    type AssetId = Nft<u64>;
    type AfterCreate = ();
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
}

#[derive(
    PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen, Default,
)]
pub struct Vote(pub u64);

impl Pledge<u64, u64, u64, (), u64, DispatchError> for Vote {
    fn try_vote(
        &self,
        _who: &u64,
        _dao_id: &u64,
        _conviction: &(),
    ) -> Result<(u64, u64), DispatchError> {
        Ok((100u64, 100u64))
    }

    fn vote_end_do(&self, _who: &u64, _dao_id: &u64) -> Result<(), DispatchError> {
        Ok(())
    }
}

impl daoent_gov::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Vote;
    type Conviction = ();
    type Asset = Balances;
    type WeightInfo = ();
}

impl daoent_sudo::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub fn new_test_run() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10), (2, 10), (3, 10), (10, 100), (20, 100), (30, 100)],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
