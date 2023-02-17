# DAO-entrance phase 1 - Milestone 1
This repository is for the submission of milestone 1 of the Web 3 Foundation Grant

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run Node
If you want to see the final running effect, you can just click [run node](./docs/run-node.md) and see it. You can also check out the code integration documentation below


### introduction to Substrate pallets
We provide pallets to make it easier for create a DAO based on substrate.
- As a user, you can create any number of daos for yourself based on the chain.
- As a developer, you can quickly integrate DAOs into current projects

We provide the following pallets: 
 - daoent-dao pallet is a basic DAO module through which you can create a DAO
 - daoent-assets pallet is a TOKEN management module specially designed for DAO, through which it can manage the native tokens of DAO on the chain and issue the organization tokens
 - daoent-sudo pallet is for the early DAO, which requires the core team to quickly adjust various parameters. After the organization is stable, this module will be disabled
 - daoent-gov pallet is a governance module specially designed for DAO. Through this module, DAO members can conduct global voting, intra-guild voting and intra-project voting to achieve the purpose of intra-organization governance
 - daoent-guild pallet is designed to organize the internal talent pool, and each guild will gather different groups of people
 - daoent-project pallet allows the DAO to run multiple projects at the same time, and each project team has multiple members from various guilds

### Integration step
#### 1. Add [dependencies] to Cargo.toml
Add the following to Cargo.toml dependencies of node-runtime, note that the subsrate version is polkadot-v0.9.36
```toml
# Local Dependencies
orml-traits = { default-features = false, git = "https://github.com/open-web3-stack/open-runtime-module-library.git", branch = "polkadot-v0.9.36"}
orml-tokens = { default-features = false, git = "https://github.com/open-web3-stack/open-runtime-module-library.git", branch = "polkadot-v0.9.36"}


daoent-primitives = { path = "../../primitives", package = "daoent-primitives", default-features = false}
daoent-dao = { path = "../../pallets/daoent-dao", package = "daoent-dao", default-features = false}
daoent-gov = { path = "../../pallets/daoent-gov", package = "daoent-gov", default-features = false}
daoent-sudo = { path = "../../pallets/daoent-sudo", package = "daoent-sudo", default-features = false}
daoent-assets = { path = "../../pallets/daoent-assets", package = "daoent-assets", default-features = false}
daoent-guild = { path = "../../pallets/daoent-guild", package = "daoent-guild", default-features = false}
daoent-project = { path = "../../pallets/daoent-project", package = "daoent-project", default-features = false}

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

#### 3. 
