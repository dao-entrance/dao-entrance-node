#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for daos_sudo.
pub trait WeightInfo {
    fn sudo() -> Weight;
    fn set_sudo_account() -> Weight;
    fn close_sudo() -> Weight;
}

/// Weights for daos_sudo using the Substrate node and recommended hardware.
pub struct DaosWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for DaosWeight<T> {
    fn sudo() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
    fn set_sudo_account() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
    fn close_sudo() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
        // Storage: CreateDao Daos (r:1 w:0)
        // Storage: DaoSudo Account (r:1 w:0)
    fn sudo() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
        // Storage: CreateDao Daos (r:1 w:0)
        // Storage: DaoSudo Account (r:1 w:1)
    fn set_sudo_account() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
        // Storage: CreateDao Daos (r:1 w:0)
        // Storage: DaoSudo Account (r:1 w:1)
    fn close_sudo() -> Weight {
            Weight::from_ref_time(20_0000_0000)
    }
}