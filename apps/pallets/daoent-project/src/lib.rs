#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use scale_info::{prelude::boxed::Box, TypeInfo};
use serde::{self, Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::result;

use daoent_dao::{self};
use daoent_primitives::{
    de_string_to_bytes,
    types::{BoardId, DaoAssetId, TaskId},
};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

/// DAO's status.
/// 状态
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Status {
    /// In use.
    /// 激活
    Active,
    /// Does not work properly.
    /// 未激活
    InActive,
}

/// Board specific information
/// 看板信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct BoardInfo<AccountId, Status> {
    /// boardID
    /// 看板ID
    pub id: BoardId,
    /// guildID
    /// 公会ID
    pub guild_id: AccountId,
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// State of the DAO
    /// DAO状态
    pub status: Status,
}

/// task specific information
/// 任务信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct TaskInfo<AccountId, Status> {
    pub id: TaskId,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub name: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub description: Vec<u8>,
    /// task point
    /// 任务价值点
    pub point: u16,
    /// priority
    /// 优先程度
    pub priority: u8,
    /// guildID
    /// 公会ID
    pub guild_id: AccountId,
    /// boardID
    /// 看板ID
    pub board_id: BoardId,
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// rewards
    /// 奖金
    pub rewards: Vec<AccountId>,
    /// reviewer
    /// 审查人
    pub reviewers: Vec<AccountId>,
    /// review info
    /// 审查信息
    pub reviews: Vec<AccountId>,
    /// assignes info
    /// 受托人
    pub assignees: Vec<AccountId>,
    /// skill info
    /// 技能
    pub skills: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    pub status: Status,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
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

    /// project board
    /// 项目看板
    #[pallet::storage]
    #[pallet::getter(fn dao_boards)]
    pub type DaoBoards<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        BoundedVec<BoardInfo<T::AccountId, Status>, ConstU32<100>>,
    >;

    /// project task
    /// 任务看板
    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<
        _,
        Identity,
        BoardId,
        BoundedVec<TaskInfo<T::AccountId, Status>, ConstU32<20000>>,
    >;

    /// taskDone
    /// 已完成项目
    #[pallet::storage]
    #[pallet::getter(fn tasks_done)]
    pub type TaskDones<T: Config> =
        StorageMap<_, Identity, BoardId, Vec<TaskInfo<T::AccountId, Status>>>;

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

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn check_enable(dao_id: DaoAssetId) -> result::Result<(), DispatchError> {
            Ok(())
        }
    }
}
