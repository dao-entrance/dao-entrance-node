#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
/// Sudo Module
use daoent_dao::{self};
use frame_support::RuntimeDebug;
use frame_support::{dispatch::DispatchResultWithPostInfo, inherent::*, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::BlockNumberProvider;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

/// App specific information
/// 程序信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct TeeApp<AccountId, BlockNumber> {
    /// creator of app
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the App
    /// App创建的区块
    pub start_block: BlockNumber,
    /// name of the app.
    /// 程序名字
    pub name: Vec<u8>,
    /// img of the App.
    /// image 目标宗旨
    pub image: Vec<u8>,
    /// port of service
    /// 服务端口号
    pub port: Vec<u32>,
    /// State of the App
    /// App状态
    status: u8,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

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

    /// apps
    /// 加密程序
    #[pallet::storage]
    #[pallet::getter(fn tee_apps)]
    pub type TeeApps<T: Config> =
        StorageMap<_, Identity, u64, TeeApp<T::AccountId, T::BlockNumber>>;

    /// apps
    /// 加密程序
    #[pallet::storage]
    #[pallet::getter(fn tee_app_minters)]
    pub type TeeAppMiners<T: Config> = StorageMap<_, Identity, u64, T::AccountId>;

    #[pallet::type_value]
    pub fn DefaultForm1() -> u64 {
        1
    }

    /// The id of the next dao to be created.
    /// 获取下一个程序id
    #[pallet::storage]
    #[pallet::getter(fn next_app_id)]
    pub type NextAppId<T: Config> = StorageValue<_, u64, ValueQuery, DefaultForm1>;

    /// The id of the next dao to be created.
    /// 获取下一个程序id
    #[pallet::storage]
    #[pallet::getter(fn wait_pool)]
    pub type WaitPool<T: Config> = StorageValue<_, BoundedVec<u64, ConstU32<5000>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CreatedApp { creator: T::AccountId, id: u64 },
        AppRuning { minter: T::AccountId, id: u64 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        AppStatusMismatch,
        RootNotExists,
        TooManyApp,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// created app
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn create_app(
            origin: OriginFor<T>,
            name: Vec<u8>,
            image: Vec<u8>,
            port: Vec<u32>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;

            // 创建 App
            let app_id = NextAppId::<T>::get();
            let now = frame_system::Pallet::<T>::current_block_number();
            TeeApps::<T>::insert(
                app_id,
                TeeApp {
                    name,
                    image,
                    creator: creator.clone(),
                    start_block: now,
                    port,
                    status: 0,
                },
            );

            let mut pool = <WaitPool<T>>::get();
            pool.try_insert(pool.len(), app_id)
                .map_err(|_| Error::<T>::TooManyApp)?;
            <WaitPool<T>>::put(&pool);

            NextAppId::<T>::put(app_id + 1);

            Self::deposit_event(Event::CreatedApp {
                creator,
                id: app_id,
            });
            Ok(().into())
        }

        ///  mint
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn run_app(origin: OriginFor<T>, app_id: u64) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let app = TeeAppMiners::<T>::contains_key(app_id);
            ensure!(!app, Error::<T>::AppStatusMismatch);

            let mut pool = <WaitPool<T>>::get();
            let location = pool
                .binary_search(&app_id)
                .ok()
                .ok_or(Error::<T>::AppStatusMismatch)?;
            pool.remove(location);
            <WaitPool<T>>::put(&pool);

            TeeAppMiners::<T>::insert(app_id, who.clone());
            Self::deposit_event(Event::AppRuning {
                minter: who,
                id: app_id,
            });
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn try_get_apps(app_id: Vec<u64>) -> Vec<TeeApp<T::AccountId, T::BlockNumber>> {
            let mut apps = Vec::new();
            for (_, v) in app_id.to_vec().into_iter().enumerate() {
                let app = TeeApps::<T>::get(v).unwrap();
                apps.push(app);
            }
            apps
        }
    }
}
