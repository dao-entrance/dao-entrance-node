#![allow(dead_code)]
#![allow(unused_variables)]

use crate as daoent_gov;
use crate::PledgeTrait;
use codec::{Decode, Encode, MaxEncodedLen};
use daoent_assets::asset_adaper_in_pallet::BasicCurrencyAdapter;
use daoent_primitives::types::{CallId, DaoAssetId};
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, Contains},
    PalletId, RuntimeDebug,
};
use frame_system;
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup, Zero},
    DispatchError,
};
use sp_std::result::Result;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
type Amount = i128;
type Balance = u64;
pub type BlockNumber = u64;
pub type AccountId = u64;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAO_ID: u64 = 1;
pub const P_ID: u32 = 0;

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
        Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},

        DAO: daoent_dao::{ Pallet, Call, Event<T>, Storage },
        DAOAsset: daoent_assets::{ Pallet, Call, Event<T>, Storage },
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
    type AccountId = AccountId;
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

parameter_types! {
    pub const TokensMaxReserves: u32 = 50;
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
    fn contains(a: &AccountId) -> bool {
        get_all_module_accounts().contains(a)
    }
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
    vec![]
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: u64| -> Balance {
        Zero::zero()
    };
}

impl orml_tokens::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CurrencyHooks = ();
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = DaoAssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type MaxLocks = MaxLocks;
    type MaxReserves = TokensMaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemovalWhitelist = DustRemovalWhitelist;
}

impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            RuntimeCall::DAOGov(func) => match func {
                daoent_gov::Call::create_propose { .. } => Ok(401 as CallId),
                daoent_gov::Call::recreate { .. } => Ok(402 as CallId),
                daoent_gov::Call::start_referendum { .. } => Ok(403 as CallId),
                daoent_gov::Call::vote_for_referendum { .. } => Ok(404 as CallId),
                daoent_gov::Call::cancel_vote { .. } => Ok(405 as CallId),
                daoent_gov::Call::run_proposal { .. } => Ok(406 as CallId),
                daoent_gov::Call::unlock { .. } => Ok(407 as CallId),
                daoent_gov::Call::set_min_vote_weight_for_every_call { .. } => Ok(408 as CallId),
                daoent_gov::Call::set_max_public_props { .. } => Ok(409 as CallId),
                daoent_gov::Call::set_launch_period { .. } => Ok(410 as CallId),
                daoent_gov::Call::set_minimum_deposit { .. } => Ok(411 as CallId),
                daoent_gov::Call::set_voting_period { .. } => Ok(412 as CallId),
                daoent_gov::Call::set_rerserve_period { .. } => Ok(413 as CallId),
                daoent_gov::Call::set_runment_period { .. } => Ok(414 as CallId),
                daoent_gov::Call::update_vote_model { .. } => Ok(415 as CallId),
                _ => Err(()),
            },
            _ => Err(()),
        }
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
    type PalletId = DaoPalletId;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type AfterCreate = ();
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxCreatableId: DaoAssetId = 100;
}

impl daoent_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiAsset = Tokens;
    type NativeAsset = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
}

#[derive(
    PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen, Default,
)]
pub struct Vote(pub AccountId);

impl PledgeTrait<u64, AccountId, u64, u64, DispatchError> for Vote {
    fn try_vote(
        &self,
        _who: &AccountId,
        _dao_id: &u64,
        vote_model: u8,
    ) -> Result<(u64, u64), DispatchError> {
        Ok((100u64, 100u64))
    }

    fn vote_end_do(&self, _who: &AccountId, _dao_id: &u64) -> Result<(), DispatchError> {
        Ok(())
    }
}

impl daoent_gov::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Vote;
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
        balances: vec![(ALICE, 100000), (BOB, 10000), (103, 10)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
