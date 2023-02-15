#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{self, Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::result;

use daoent_dao::{self};
use daoent_primitives::{
    de_string_to_bytes,
    types::{DaoAssetId, ProjectId, TaskId},
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

/// Project specific information
/// 看板信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct ProjectInfo<AccountId, Status> {
    /// boardID
    /// 看板ID
    pub id: ProjectId,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub name: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub description: Vec<u8>,
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
    pub board_id: ProjectId,
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
    pub trait Config: frame_system::Config + daoent_dao::Config + daoent_assets::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::type_value]
    pub fn DefaultForm1() -> ProjectId {
        1
    }

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_project_id)]
    pub type NextProjectId<T: Config> = StorageValue<_, ProjectId, ValueQuery, DefaultForm1>;

    /// project board
    /// 项目看板
    #[pallet::storage]
    #[pallet::getter(fn dao_boards)]
    pub type DaoProjects<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        BoundedVec<ProjectInfo<T::AccountId, Status>, ConstU32<100>>,
    >;

    /// project task
    /// 任务看板
    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<
        _,
        Identity,
        ProjectId,
        BoundedVec<TaskInfo<T::AccountId, Status>, ConstU32<20000>>,
    >;

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_task_id)]
    pub type NextTaskId<T: Config> = StorageValue<_, ProjectId, ValueQuery, DefaultForm1>;

    /// taskDone
    /// 已完成项目
    #[pallet::storage]
    #[pallet::getter(fn tasks_done)]
    pub type TaskDones<T: Config> =
        StorageMap<_, Identity, ProjectId, Vec<TaskInfo<T::AccountId, Status>>>;

    #[pallet::error]
    pub enum Error<T> {
        InVailCall,
        TooManyMembers,
        ProjectCreateError,
        Project403,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        ProjectJoined(DaoAssetId, ProjectId, T::AccountId),
        ProjectCreated(DaoAssetId, ProjectId, T::AccountId),
        TaskCreated(DaoAssetId, ProjectId, u32, T::AccountId),
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    /// guild members
    /// 公会成员
    #[pallet::storage]
    #[pallet::getter(fn guild_members)]
    pub type ProjectdMembers<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        ProjectId,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(1500_000_000)]
        pub fn project_join_request(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me.clone(), dao_id)?;

            Self::try_add_project_member(dao_id, project_id, who.clone())?;

            Self::deposit_event(Event::ProjectJoined(dao_id, project_id, who));

            Ok(().into())
        }

        #[pallet::call_index(002)]
        #[pallet::weight(1500_000_000)]
        pub fn create_project(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project: ProjectInfo<T::AccountId, Status>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me.clone(), dao_id)?;

            let project_id = Self::try_add_project(dao_id, project)?;
            Self::try_add_project_member(dao_id, project_id, me.clone())?;

            Self::deposit_event(Event::ProjectCreated(dao_id, project_id, me));

            Ok(().into())
        }

        #[pallet::call_index(003)]
        #[pallet::weight(1500_000_000)]
        pub fn create_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task: TaskInfo<T::AccountId, Status>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            Self::check_auth_for_task(dao_id, project_id, me.clone())?;

            // 添加提案
            let task_id = NextTaskId::<T>::get();
            let mut tasks = <Tasks<T>>::get(task_id).unwrap();
            tasks
                .try_insert(tasks.len(), task)
                .map_err(|_| Error::<T>::TooManyMembers)?;
            <Tasks<T>>::insert(task_id, tasks);
            NextTaskId::<T>::put(project_id + 1);

            Self::deposit_event(Event::TaskCreated(dao_id, project_id, 1, me));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 添加项目成员
        pub fn try_add_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id.into();
            let mut members = <ProjectdMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;

            members
                .try_insert(index, who.clone())
                .map_err(|_| Error::<T>::TooManyMembers)?;

            <ProjectdMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id.into();
            let mut members = <ProjectdMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <ProjectdMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        pub fn try_add_project(
            dao_id: DaoAssetId,
            project: ProjectInfo<T::AccountId, Status>,
        ) -> result::Result<ProjectId, DispatchError> {
            let mut projects = <DaoProjects<T>>::get(dao_id).unwrap();
            let project_id = NextProjectId::<T>::get();
            projects
                .try_insert(project_id.try_into().unwrap(), project)
                .map_err(|_| Error::<T>::ProjectCreateError)?;

            <DaoProjects<T>>::insert(dao_id, &projects);
            NextProjectId::<T>::put(project_id + 1);

            Ok(project_id)
        }

        /// 删除项目
        pub fn try_remove_project(
            dao_id: DaoAssetId,
            project_index: ProjectId,
        ) -> result::Result<ProjectId, DispatchError> {
            let mut ps = <DaoProjects<T>>::get(dao_id).unwrap();

            ps.remove(project_index.try_into().unwrap());
            <DaoProjects<T>>::insert(dao_id, &ps);

            Ok(project_index)
        }

        /// 获取用户是否有 project 的权利
        pub fn check_auth_for_task(
            dao_id: DaoAssetId,
            project_index: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let pindex: u32 = project_index.try_into().unwrap();
            let ms = <daoent_dao::ProjectMembers<T>>::get(dao_id, pindex);
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Project403)?;

            Ok(index)
        }
    }
}
