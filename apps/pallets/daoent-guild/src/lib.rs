#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_system::{ensure_signed, pallet_prelude::*};

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
        GuildSuccess(DaoAssetId, u64, T::AccountId),
        GuildJoined(DaoAssetId, u64, T::AccountId),
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(1_500_000_000)]
        pub fn guild_join_request(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            daoent_dao::Pallet::<T>::try_add_guild_member(dao_id, guild_id, who.clone())?;

            Self::deposit_event(Event::GuildJoined(dao_id, guild_id, who));

            Ok(().into())
        }

        /// 创建公会
        #[pallet::call_index(002)]
        #[pallet::weight(50_000_000)]
        pub fn create_guild(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            name: Vec<u8>,
            desc: Vec<u8>,
            meta_data: Vec<u8>,
        ) -> DispatchResult {
            let me = ensure_signed(origin.clone())?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            ensure!(desc.len() <= 50, dao::Error::<T>::PurposeTooLong);
            ensure!(meta_data.len() <= 1024, dao::Error::<T>::MetaDataTooLong);

            let creator = ensure_signed(origin)?;
            let now = <frame_system::Pallet:: <T>  as sp_runtime::traits::BlockNumberProvider>::current_block_number();

            // 创建核心团队-coreTeam
            let mut guilds = <dao::Guilds<T>>::get(dao_id);
            guilds
                .try_insert(
                    guilds.len(),
                    dao::GuildInfo {
                        creator: creator.clone(),
                        start_block: now,
                        name,
                        desc,
                        status: dao::Status::Active,
                        meta_data,
                    },
                )
                .map_err(|_| dao::Error::<T>::GuildCreateError)?;
            <dao::Guilds<T>>::insert(dao_id, &guilds);

            // 更新团队成员
            let mut members = <dao::GuildMembers<T>>::get(dao_id, 0);
            members
                .try_insert(0, creator.clone())
                .map_err(|_| dao::Error::<T>::GuildCreateError)?;

            // 更新组织
            <dao::GuildMembers<T>>::insert(dao_id, 0, members);

            Self::deposit_event(Event::GuildJoined(
                dao_id,
                guilds.len().try_into().unwrap(),
                creator,
            ));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {}
}
