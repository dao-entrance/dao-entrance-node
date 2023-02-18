# DAO-entrance phase 1 - Milestone 1
This repository is for the submission of milestone 1 of the Web 3 Foundation Grant

### Rust Setup

- [Linux development environment](https://docs.substrate.io/install/linux/).
- [MacOS development environment](https://docs.substrate.io/install/linux/).
- [Windows development environment](https://docs.substrate.io/install/linux/).

### Run Node
If you want to see the final running effect, you can just click [run node](./docs/run-node.md) and see it. You can also check out the code integration documentation below

### Run Docker
If you want to see the final running effect, you can just click [run docker](./docs/run-docker.md) and see it. You can also check out the code integration documentation below


### introduction to Substrate pallets
We provide pallets to make it easier for create a DAO based on substrate.
- As a user, you can create any number of daos for yourself based on the chain.
- As a developer, you can quickly integrate DAOs into current projects

We provide the following pallets: 
> Click on the pallet name to view the api 
 - [daoent-dao](./pallets/daoent-dao/README.md) pallet is a basic DAO module through which you can create a DAO.
 - [daoent-assets](./pallets/daoent-assets/README.md) pallet is a TOKEN management module specially designed for DAO, through which it can manage the native tokens of DAO on the chain and issue the organization tokens.
 - [daoent-sudo](./pallets/daoent-sudo/README.md) pallet is for the early DAO, which requires the core team to quickly adjust various parameters. After the organization is stable, this module will be disabled.
 - [daoent-gov](./pallets/daoent-gov/README.md) pallet is a governance module specially designed for DAO. Through this module, DAO members can conduct global voting, intra-guild voting and intra-project voting to achieve the purpose of intra-organization governance.
 - [daoent-guild](./pallets/daoent-guild/README.md) pallet is designed to organize the internal talent pool, and each guild will gather different groups of people.
 - [daoent-project](./pallets/daoent-project/README.md) pallet allows the DAO to run multiple projects at the same time, and each project team has multiple members from various guilds.

### Integration step
#### 1. Add [dependencies] to Cargo.toml
Add the following to Cargo.toml dependencies of node-runtime, note that the subsrate version is polkadot-v0.9.36
```toml
# Local Dependencies
orml-traits = { default-features = false, git = "https://github.com/open-web3-stack/open-runtime-module-library.git", branch = "polkadot-v0.9.36"}
orml-tokens = { default-features = false, git = "https://github.com/open-web3-stack/open-runtime-module-library.git", branch = "polkadot-v0.9.36"}


daoent-primitives = { path = "../../primitives", package = "daoent-primitives", default-features = false}
daoent-dao = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-dao", default-features = false}
daoent-gov = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-gov", default-features = false}
daoent-sudo = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-sudo", default-features = false}
daoent-assets = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-assets", default-features = false}
daoent-guild = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-guild", default-features = false}
daoent-project = { path = "https://github.com/dao-entrance/dao-entrance-node.git", package = "daoent-project", default-features = false}

```

#### 2. Add [features] to Cargo.toml
Add the following to Cargo.toml features of node-runtime, note that the subsrate version is polkadot-v0.9.36
```toml
std = [
    ...
	"orml-traits/std",
    "orml-tokens/std",
	"daoent-primitives/std",
	"daoent-dao/std",
	"daoent-sudo/std",
	"daoent-gov/std",
	"daoent-assets/std",
    "daoent-guild/std",
    "daoent-project/std",
]
```

#### 3. Add code to the node-runtime src/lib.rs file
Add the import code to the previous section of src/lib.rs
```rust
// Import the DAO pallet.

use codec::MaxEncodedLen;
use daoent_assets::{self as daoent_assets, asset_adaper_in_pallet::BasicCurrencyAdapter};
use daoent_gov::traits::ConvertInto;
use daoent_gov::traits::PledgeTrait;
use daoent_primitives::{
    traits::AfterCreate,
    types::{CallId, DaoAssetId},
};
use sp_runtime::{DispatchError, RuntimeDebug};

use frame_support::{
    codec::{Decode, Encode},
    traits::Contains,
    PalletId,
};
use orml_traits::parameter_type_with_key;
pub use scale_info::TypeInfo;
use sp_runtime::traits::Zero;

// end DAO pallet.
```

Add import code to src/lib.rs
```rust

/// DAO Start
type Amount = i128;

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen)]
pub enum Pledge<Balance> {
    FungToken(Balance),
}

impl Default for Pledge<Balance> {
    fn default() -> Self {
        Pledge::FungToken(0)
    }
}
impl PledgeTrait<Balance, AccountId, DaoAssetId, (), BlockNumber, DispatchError>
    for Pledge<Balance>
{
    fn try_vote(
        &self,
        who: &AccountId,
        dao_id: &DaoAssetId,
        conviction: &(),
    ) -> Result<(Balance, BlockNumber), DispatchError> {
        if cfg!(any(feature = "std", feature = "runtime-benchmarks", test)) {
            return Ok((Default::default(), Default::default()));
        }

        match self {
            Pledge::FungToken(x) => {
                DAOAsset::reserve(dao_id.clone(), who.clone(), *x)?;
                let amount = *x;
                return Ok((
                    amount
                        .checked_mul(conviction.convert_into())
                        .ok_or(daoent_gov::Error::<Runtime>::Overflow)?,
                    conviction.convert_into(),
                ));
            }
        }
        // Err(daoent_gov::Error::<Runtime>::PledgeNotEnough)?
    }

    fn vote_end_do(&self, who: &AccountId, dao_id: &DaoAssetId) -> Result<(), DispatchError> {
        if cfg!(any(feature = "std", feature = "runtime-benchmarks", test)) {
            return Ok(());
        }
        match self {
            Pledge::FungToken(x) => {
                DAOAsset::unreserve(dao_id.clone(), who.clone(), *x)?;
                return Ok(());
            }
        }
        // Err(daoent_gov::Error::<Runtime>::PledgeNotEnough)?
    }
}

impl daoent_gov::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Pledge<Balance>;
    type Conviction = ();
    type WeightInfo = ();
}

impl daoent_project::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
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

impl orml_tokens::Config for Runtime {
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
    pub const DaoPalletId: PalletId = PalletId(*b"ent--dao");
}

pub struct CreatedHook;
impl AfterCreate<AccountId> for CreatedHook {
    fn run_hook(acount_id: AccountId, dao_id: DaoAssetId) {
        // 以 DAO 创建者设置为DAO初始的 root 账户
        daoent_sudo::Account::<Runtime>::insert(dao_id, acount_id);
    }
}

impl daoent_dao::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type AfterCreate = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type PalletId = DaoPalletId;
}

/// 定义那些函数能被当作 sudo/gov 方式调用
impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            RuntimeCall::DAO(func) => match func {
                daoent_dao::Call::create_dao { .. } => Ok(101 as CallId),
                _ => Err(()),
            },
            RuntimeCall::DAOAsset(func) => match func {
                daoent_assets::Call::create_asset { .. } => Ok(101 as CallId),
                _ => Err(()),
            },
            RuntimeCall::DAOGuild(func) => match func {
                daoent_guild::Call::guild_join_request { .. } => Ok(301 as CallId),
                _ => Err(()),
            },
            // dao
            RuntimeCall::DAOProject(func) => match func {
                daoent_project::Call::project_join_request { .. } => Ok(501 as CallId),
                daoent_project::Call::create_project { .. } => Ok(502 as CallId),
                daoent_project::Call::apply_project_funds { .. } => Ok(503 as CallId),
                daoent_project::Call::create_task { .. } => Ok(504 as CallId),
                daoent_project::Call::join_task { .. } => Ok(505 as CallId),
                daoent_project::Call::leave_task { .. } => Ok(506 as CallId),
                daoent_project::Call::be_task_review { .. } => Ok(507 as CallId),
                daoent_project::Call::leave_task_review { .. } => Ok(508 as CallId),
                daoent_project::Call::start_task { .. } => Ok(509 as CallId),
                daoent_project::Call::requset_review { .. } => Ok(510 as CallId),
                daoent_project::Call::task_done { .. } => Ok(511 as CallId),
                daoent_project::Call::make_review { .. } => Ok(512 as CallId),
                _ => Err(()),
            },
            _ => Err(()),
        }
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

impl daoent_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiAsset = Tokens;
    type NativeAsset = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
}

impl daoent_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl daoent_guild::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// DAO END
```

Add code to construct_runtime! part
```rust
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
    {
		...
	    // token
        Tokens: orml_tokens,
        // DAO
        DAO: daoent_dao,
        DAOAsset: daoent_assets,
        DAOSudo: daoent_sudo,
        DAOGuild: daoent_guild,
        DAOProject: daoent_project,
        DAOGov: daoent_gov,
	}
);
```