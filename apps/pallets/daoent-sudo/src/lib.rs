#![cfg_attr(not(feature = "std"), no_std)]

/// Sudo Module
use daoent_dao::{self};
use frame_support::traits::UnfilteredDispatchable;
use scale_info::prelude::boxed::Box;
use sp_std::result;

use daoent_primitives::types::DaoAssetId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::Event::{CloseSudo, SetSudo, SudoDone};
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + daoent_dao::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// DAO Root account id.
    /// 组织最高权限 id
    #[pallet::storage]
    #[pallet::getter(fn sudo_account)]
    pub type Account<T: Config> = StorageMap<_, Identity, DaoAssetId, T::AccountId>;

    /// DAO Root account id.
    /// 组织最高权限 id
    #[pallet::storage]
    #[pallet::getter(fn close_dao)]
    pub type CloseDao<T: Config> = StorageMap<_, Identity, DaoAssetId, bool>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// root executes external transaction successfully.
        SudoDone {
            sudo: T::AccountId,
            sudo_result: DispatchResult,
        },
        /// Set root account or reopen sudo.
        SetSudo {
            dao_id: DaoAssetId,
            sudo_account: T::AccountId,
        },
        /// delete root account.
        CloseSudo { dao_id: DaoAssetId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        NotSudo,
        RootNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Execute external transactions as root
        /// 以 root 账户执行函数
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn sudo(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            call: Box<<T as daoent_dao::Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {
            Self::check_enable(dao_id)?;

            let sudo = Self::check_sudo(dao_id, origin)?;
            let asset_id = daoent_dao::Pallet::<T>::try_get_asset_id(dao_id)?;
            ensure!(
                daoent_primitives::traits::BaseCallFilter::contains(&asset_id, *call.clone()),
                daoent_dao::Error::<T>::InVailCall
            );

            let res = call.dispatch_bypass_filter(
                frame_system::RawOrigin::Signed(daoent_dao::Pallet::<T>::try_get_dao_account_id(
                    dao_id,
                )?)
                .into(),
            );
            Self::deposit_event(SudoDone {
                sudo,
                sudo_result: res.map(|_| ()).map_err(|e| e.error),
            });
            Ok(().into())
        }

        /// set sudo account
        /// 设置超级用户账户
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_sudo_account())]
        pub fn set_sudo_account(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            sudo_account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            Self::check_enable(dao_id)?;

            let _sudo = Self::check_sudo(dao_id, origin)?;
            Account::<T>::insert(dao_id, sudo_account.clone());
            Self::deposit_event(SetSudo {
                dao_id,
                sudo_account,
            });
            Ok(().into())
        }

        /// close sudo
        /// 关闭 sudo 功能
        #[pallet::call_index(003)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::close_sudo())]
        pub fn close_sudo(origin: OriginFor<T>, dao_id: DaoAssetId) -> DispatchResultWithPostInfo {
            let _sudo = Self::check_sudo(dao_id, origin)?;
            CloseDao::<T>::insert(dao_id, true);

            Self::deposit_event(CloseSudo { dao_id });
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 测试账户是否为 DAO root 账户
        fn check_sudo(
            dao_id: DaoAssetId,
            o: OriginFor<T>,
        ) -> result::Result<T::AccountId, DispatchError> {
            let who = ensure_signed(o)?;
            ensure!(
                who == Account::<T>::get(dao_id).ok_or(Error::<T>::RootNotExists)?,
                Error::<T>::NotSudo
            );
            Ok(who)
        }

        fn check_enable(dao_id: DaoAssetId) -> result::Result<(), DispatchError> {
            let is_close = CloseDao::<T>::get(dao_id);

            ensure!(
                is_close.is_none() || is_close.unwrap() == false,
                Error::<T>::RootNotExists
            );

            Ok(())
        }
    }
}
