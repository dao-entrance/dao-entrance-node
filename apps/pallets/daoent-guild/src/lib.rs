#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::*;
use frame_system::{ensure_signed, pallet_prelude::*};

use daoent_assets;
use daoent_dao::{self as dao};
use daoent_primitives::types::DaoAssetId;
use sp_std::convert::TryInto;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod weights;
pub use weights::WeightInfo;

mod traits;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + dao::Config + daoent_assets::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        GuildJoined(DaoAssetId, u32, T::AccountId),
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(1500_000_000)]
        pub fn guild_join_request(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            guild_id: u32,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me.clone(), dao_id)?;

            daoent_dao::Pallet::<T>::try_add_guild_member(dao_id, guild_id, who.clone())?;

            Self::deposit_event(Event::GuildJoined(dao_id, guild_id, who));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {}
}
