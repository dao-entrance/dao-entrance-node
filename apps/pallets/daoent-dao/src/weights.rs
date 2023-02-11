#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for daos_create_dao.
pub trait WeightInfo {
  fn create_dao() -> Weight;
  fn dao_remark() -> Weight;
}

/// Weights for daos_create_dao using the Substrate node and recommended hardware.
pub struct DaosWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for DaosWeight<T> {
  // Storage: CreateDao NextDaoId (r:1 w:1)
  // Storage: DaoSudo Account (r:0 w:1)
  // Storage: CreateDao Daos (r:0 w:1)
  fn create_dao() -> Weight {
    Weight::from_ref_time(28_576_000)
      .saturating_add(T::DbWeight::get().reads(1))
      .saturating_add(T::DbWeight::get().writes(3))
  }
  // Storage: CreateDao Daos (r:1 w:0)
  fn dao_remark() -> Weight {
    Weight::from_ref_time(11_086_000)
      .saturating_add(T::DbWeight::get().reads(1))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: CreateDao NextDaoId (r:1 w:1)
  // Storage: DaoSudo Account (r:0 w:1)
  // Storage: CreateDao Daos (r:0 w:1)
  fn create_dao() -> Weight {
    Weight::from_ref_time(28_576_000)
      .saturating_add(RocksDbWeight::get().reads(1))
      .saturating_add(RocksDbWeight::get().writes(3))
  }
      // Storage: CreateDao Daos (r:1 w:0)
  fn dao_remark() -> Weight {
    Weight::from_ref_time(11_086_000)
      .saturating_add(RocksDbWeight::get().reads(1))
  }
}