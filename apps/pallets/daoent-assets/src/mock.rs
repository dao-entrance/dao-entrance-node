#![allow(dead_code)]
use super::*;
use crate::{self as daoent_assets, asset_adaper_in_pallet::BasicCurrencyAdapter};
use frame_support::{construct_runtime, parameter_types, traits::Contains, PalletId};
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

use daoent_primitives::{
    traits::AfterCreate,
    types::{CallId, DaoAssetId},
};

type Amount = i128;
type Balance = u64;
pub type AccountId = u128;
pub type BlockNumber = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: AccountId = 101;
pub const BOB: AccountId = 102;

construct_runtime!(
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
    }
);

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

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = frame_support::traits::Everything;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"ent--dao");
}

pub struct CreatedHook;
impl AfterCreate<AccountId> for CreatedHook {
    fn run_hook(acount_id: AccountId, dao_id: DaoAssetId) {
        // 以 DAO 创建者设置为DAO初始的 root 账户
        daoent_sudo::Account::<Test>::insert(dao_id, acount_id);
    }
}

impl daoent_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type PalletId = DaoPalletId;
    type AfterCreate = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
}

impl TryFrom<RuntimeCall> for u32 {
    type Error = ();
    fn try_from(_call: RuntimeCall) -> Result<Self, Self::Error> {
        // match call {
        //     _ => Ok(0u32),
        // }
        Ok(0u32)
    }
}

parameter_types! {
    pub const MaxClassMetadata: u32 = 1;
    pub const MaxTokenMetadata: u32 = 1;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: u64| -> Balance {
        Zero::zero()
    };
}

pub struct MockDustRemovalWhitelist;
impl Contains<AccountId> for MockDustRemovalWhitelist {
    fn contains(_a: &AccountId) -> bool {
        false
    }
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

impl daoent_sudo::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub(crate) fn new_test_run() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 100000), (BOB, 10000), (103, 10)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        // Timestamp::set_timestamp(12345);
    });
    ext
}
