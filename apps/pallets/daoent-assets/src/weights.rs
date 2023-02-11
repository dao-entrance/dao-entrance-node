
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_currencies.
pub trait WeightInfo {
	fn create_asset() -> Weight;
	fn set_metadata() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn transfer_native_currency() -> Weight;
	fn update_balance() -> Weight;
}

/// Weights for pallet_currencies using the Substrate node and recommended hardware.
pub struct DicoWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for DicoWeight<T> {
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn create_asset() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn update_balance() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn create_asset() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn update_balance() -> Weight {
		Weight::from_ref_time(20_0000_0000)
	}
}