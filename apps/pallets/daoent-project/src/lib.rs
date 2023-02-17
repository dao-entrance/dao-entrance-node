#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{self, Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::result;

use orml_traits::MultiCurrency;

use daoent_dao::{self};
use daoent_primitives::types::{DaoAssetId, ProjectId, TaskId};

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

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    InReview,
    Done,
}

/// Project specific information
/// 看板信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct ProjectInfo<AccountId, Status> {
    /// boardID
    /// 看板ID
    pub id: ProjectId,
    /// 项目名
    pub name: Vec<u8>,
    /// 项目介绍
    pub description: Vec<u8>,
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
pub struct TaskInfo<AccountId, Balance, TaskStatus> {
    pub id: TaskId,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    /// task point
    /// 任务价值点
    pub point: u16,
    /// priority
    /// 优先程度
    pub priority: u8,
    /// projectID
    /// 看板ID
    pub project_id: ProjectId,
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// rewards
    /// 奖金
    pub rewards: Vec<(DaoAssetId, Balance)>,
    // 最大协作数量
    pub max_assignee: u8,
    /// assignes info
    /// 受托人
    pub assignees: Vec<AccountId>,
    /// reviewer
    /// 审查人
    pub reviewers: Vec<AccountId>,
    /// skill info
    /// 技能
    pub skills: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    pub status: TaskStatus,
}

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum ReviewOpinion {
    /// Agree.
    YES,
    /// Reject.
    NO,
}

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ReviewRecord<AccountId> {
    pub who: AccountId,
    pub meta: Vec<u8>,
    pub option: ReviewOpinion,
}

/// Info regarding an Review.
/// 审核的状态
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ReviewStatus<AccountId> {
    /// 审核历史
    pub records: Vec<ReviewRecord<AccountId>>,
    /// The current tally of Review.
    /// 审核统计
    pub tally: Tally,
}

/// Review Statistics.
/// 审核数据统计
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Tally {
    /// The number of yes votes
    /// 同意的数量
    pub yes: u32,
    /// The number of no votes
    /// 不同意的数量
    pub no: u32,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as daoent_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

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
    pub type DaoProjects<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DaoAssetId,
        Identity,
        ProjectId,
        ProjectInfo<T::AccountId, Status>,
    >;

    /// project task
    /// 任务看板
    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<
        _,
        Identity,
        ProjectId,
        BoundedVec<TaskInfo<T::AccountId, BalanceOf<T>, TaskStatus>, ConstU32<20000>>,
        ValueQuery,
    >;

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_task_id)]
    pub type NextTaskId<T: Config> = StorageValue<_, ProjectId, ValueQuery, DefaultForm1>;

    /// TODO taskDone
    /// 已完成项目
    // #[pallet::storage]
    // #[pallet::getter(fn tasks_done)]
    // pub type TaskDones<T: Config> =
    //     StorageMap<_, Identity, ProjectId, Vec<TaskInfo<T::AccountId,BalanceOf<T>, TaskStatus>>>;

    /// task reviews
    /// 项目审核报告
    #[pallet::storage]
    #[pallet::getter(fn tasks_reviews)]
    pub type TaskReviews<T: Config> = StorageMap<_, Identity, TaskId, ReviewStatus<T::AccountId>>;

    #[pallet::error]
    pub enum Error<T> {
        InVailCall,
        TooManyMembers,
        ProjectCreateError,
        Project403,
        AlreadyAssignee,
        NotAssignee,
        NotReviewer,
        AlreadyReviewer,
        TooManyAssignee,
        TaskIsStared,
        NoReviewer,
        RepeatReview,
        ReviewPending,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        ProjectJoined(DaoAssetId, ProjectId, T::AccountId),
        ProjectCreated(DaoAssetId, ProjectId, T::AccountId),
        TaskCreated(DaoAssetId, ProjectId, u64, T::AccountId),
        TaskInProgress(DaoAssetId, ProjectId, u64, T::AccountId),
        TaskInReview(DaoAssetId, ProjectId, u64, T::AccountId),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 申请加入团队
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

            daoent_dao::Pallet::<T>::try_add_project_member(dao_id, project_id, who.clone())?;

            Self::deposit_event(Event::ProjectJoined(dao_id, project_id, who));

            Ok(().into())
        }

        /// 创建项目
        #[pallet::call_index(002)]
        #[pallet::weight(1500_000_000)]
        pub fn create_project(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            name: Vec<u8>,
            description: Vec<u8>,
            creator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me.clone(), dao_id)?;

            let project_id = Self::try_add_project(
                dao_id,
                ProjectInfo {
                    id: 0,
                    name,
                    creator,
                    description,
                    status: Status::Active,
                },
            )?;

            Self::deposit_event(Event::ProjectCreated(dao_id, project_id, me));

            Ok(().into())
        }

        /// 为项目申请资金
        #[pallet::call_index(012)]
        #[pallet::weight(1500_000_000)]
        pub fn apply_project_funds(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me.clone(), dao_id)?;

            daoent_assets::Pallet::<T>::try_transfer(
                dao_id,
                daoent_assets::Pallet::<T>::dao_asset(dao_id),
                daoent_assets::Pallet::<T>::dao_project(dao_id, project_id),
                amount,
            )?;

            Self::deposit_event(Event::ProjectCreated(dao_id, project_id, me));

            Ok(().into())
        }

        /// 创建任务
        #[pallet::call_index(003)]
        #[pallet::weight(1500_000_000)]
        pub fn create_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            name: Vec<u8>,
            description: Vec<u8>,
            point: u16,
            priority: u8,
            max_assignee: Option<u8>,
            skills: Option<Vec<u8>>,
            assignees: Option<Vec<T::AccountId>>,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            Self::check_auth_for_project(dao_id, project_id, me.clone())?;

            // 初始化任务 id
            let task_id = NextTaskId::<T>::get();
            let mut task = TaskInfo {
                id: task_id,
                name,
                project_id: project_id,
                description,
                point,
                priority,
                creator: me.clone(),
                rewards: vec![(dao_id, amount)],
                max_assignee: 1,
                assignees: vec![],
                reviewers: vec![],
                skills: vec![],
                status: TaskStatus::ToDo,
            };
            if max_assignee.is_some() {
                task.max_assignee = max_assignee.unwrap();
            }
            if skills.is_some() {
                task.skills = skills.unwrap();
            }
            if assignees.is_some() {
                task.assignees = assignees.unwrap();
            }

            // 预备资金
            daoent_assets::Pallet::<T>::reserve(
                dao_id,
                daoent_assets::Pallet::<T>::dao_project(dao_id, project_id),
                amount,
            )?;

            // 插入任务id
            let mut tasks = <Tasks<T>>::get(project_id);
            tasks
                .try_insert(tasks.len(), task)
                .map_err(|_| Error::<T>::TooManyMembers)?;
            <Tasks<T>>::insert(project_id, tasks);

            // taskid 自增
            NextTaskId::<T>::put(task_id + 1);

            Self::deposit_event(Event::TaskCreated(dao_id, project_id, task_id, me));

            Ok(().into())
        }

        /// 加入任务
        #[pallet::call_index(004)]
        #[pallet::weight(1500_000_000)]
        pub fn join_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();

            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::ToDo, Error::<T>::TaskIsStared);

            // 确保任务人数
            let max_assignee: usize = task.max_assignee.try_into().unwrap();
            ensure!(
                task.assignees.len() + 1 <= max_assignee,
                Error::<T>::TooManyAssignee
            );

            // 查询用户是否存在
            task.assignees
                .binary_search(&me)
                .err()
                .ok_or(Error::<T>::AlreadyAssignee)?;

            // 添加用户
            tasks[index]
                .assignees
                .insert(task.assignees.len(), me.clone());

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskCreated(dao_id, task.project_id, 1, me));

            Ok(().into())
        }

        /// 离开项目
        #[pallet::call_index(005)]
        #[pallet::weight(1500_000_000)]
        pub fn leave_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();

            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::ToDo, Error::<T>::InVailCall);

            // 查询用户是否存在
            let location = task
                .assignees
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotAssignee)?;

            // 删除用户
            tasks[index].assignees.remove(location);

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskCreated(dao_id, task.project_id, 1, me));

            Ok(().into())
        }

        /// 加入项目审核团队
        #[pallet::call_index(006)]
        #[pallet::weight(1500_000_000)]
        pub fn be_task_review(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::ToDo, Error::<T>::TaskIsStared);

            // 查询用户是否存在
            task.assignees
                .binary_search(&me)
                .err()
                .ok_or(Error::<T>::AlreadyAssignee)?;

            // 查询用户是否存在
            task.reviewers
                .binary_search(&me)
                .err()
                .ok_or(Error::<T>::AlreadyReviewer)?;

            // 添加用户
            tasks[index]
                .reviewers
                .insert(task.reviewers.len(), me.clone());

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskCreated(dao_id, task.project_id, 1, me));

            Ok(().into())
        }

        /// 离开任务审核
        #[pallet::call_index(007)]
        #[pallet::weight(1500_000_000)]
        pub fn leave_task_review(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::ToDo, Error::<T>::InVailCall);

            // 查询用户是否存在
            let location = task
                .reviewers
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotReviewer)?;

            // 删除用户
            tasks[index].reviewers.remove(location);

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskCreated(dao_id, task.project_id, 1, me));

            Ok(().into())
        }

        /// 开始任务
        #[pallet::call_index(008)]
        #[pallet::weight(1500_000_000)]
        pub fn start_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保审核人数大于1
            ensure!(task.reviewers.len() > 0, Error::<T>::NoReviewer);
            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::ToDo, Error::<T>::InVailCall);

            // 查询用户是否存在,只有用户才能开始项目
            let _ = task
                .assignees
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotAssignee)?;

            // 修改状态
            tasks[index].status = TaskStatus::InProgress;

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskInProgress(dao_id, task.project_id, 1, me));
            Ok(().into())
        }

        /// 申请审核
        #[pallet::call_index(009)]
        #[pallet::weight(1500_000_000)]
        pub fn requset_review(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(
                task.status == TaskStatus::InProgress,
                Error::<T>::InVailCall
            );

            // 查询用户是否存在,只有用户才能提交申请项目
            let _ = task
                .assignees
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotAssignee)?;

            // 修改状态
            tasks[index].status = TaskStatus::InReview;

            <Tasks<T>>::insert(task_id, tasks);
            <TaskReviews<T>>::insert(
                task_id,
                ReviewStatus {
                    records: vec![],
                    tally: Tally { yes: 0, no: 0 },
                },
            );

            Self::deposit_event(Event::TaskInReview(dao_id, task.project_id, 1, me));
            Ok(().into())
        }

        /// 完成任务
        #[pallet::call_index(010)]
        #[pallet::weight(1500_000_000)]
        pub fn task_done(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (mut tasks, task, index) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 确保任务只能是未开始加入
            ensure!(task.status == TaskStatus::InReview, Error::<T>::InVailCall);

            // 查询用户是否存在,只有用户才能结束
            let _ = task
                .assignees
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotAssignee)?;

            // 获取审查状态
            let review = <TaskReviews<T>>::get(task_id).unwrap();

            // 确保任务被1/2的人审核并且通过
            ensure!(
                review.tally.yes > (task.assignees.len() / 2).try_into().unwrap(),
                Error::<T>::ReviewPending
            );

            let project_account = daoent_assets::Pallet::<T>::dao_project(dao_id, task.project_id);
            let total = task.rewards[0].1;
            let total_u64: u64 = total.saturated_into::<u64>();
            let amount_u64 =
                total_u64 / <usize as TryInto<u64>>::try_into(task.assignees.len()).unwrap();
            let amount: BalanceOf<T> = amount_u64.saturated_into();

            // 解锁预备资金
            daoent_assets::Pallet::<T>::unreserve(dao_id, project_account.clone(), total)?;

            // 为所有的贡献者转帐
            for assignee in task.assignees.iter() {
                daoent_assets::Pallet::<T>::try_transfer(
                    dao_id,
                    project_account.clone(),
                    assignee.clone(),
                    amount,
                )?;
            }

            // 修改状态
            tasks[index].status = TaskStatus::Done;

            <Tasks<T>>::insert(task_id, tasks);

            Self::deposit_event(Event::TaskInReview(dao_id, task.project_id, 1, me));
            Ok(().into())
        }

        /// 发送审核报告
        #[pallet::call_index(011)]
        #[pallet::weight(1500_000_000)]
        pub fn make_review(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            project_id: ProjectId,
            task_id: ProjectId,
            opinion: ReviewOpinion,
            meta: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let (_, task, _) = Self::get_task(project_id, task_id).unwrap();
            Self::check_auth_for_project(dao_id, task.project_id, me.clone())?;

            // 查询用户是否存在
            task.reviewers
                .binary_search(&me)
                .ok()
                .ok_or(Error::<T>::NotReviewer)?;

            // 获取审查状态
            let mut review = <TaskReviews<T>>::get(task_id).unwrap();

            // 避免重复成功审核
            let index = review
                .records
                .iter()
                .position(|x| x.who == me && x.option == ReviewOpinion::YES);

            // 确保任务只能是未开始加入
            ensure!(index.is_none(), Error::<T>::RepeatReview);

            // 记录审核信息
            review.records.push(ReviewRecord {
                who: me,
                meta,
                option: opinion.clone(),
            });
            match opinion {
                ReviewOpinion::YES => review.tally.yes = review.tally.yes + 1,
                ReviewOpinion::NO => review.tally.no = review.tally.no + 1,
            }

            <TaskReviews<T>>::insert(task_id, review);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // 添加项目
        pub fn try_add_project(
            dao_id: DaoAssetId,
            mut project: ProjectInfo<T::AccountId, Status>,
        ) -> result::Result<ProjectId, DispatchError> {
            let project_id = NextProjectId::<T>::get();
            project.id = project_id;

            <DaoProjects<T>>::insert(dao_id, project_id, project.clone());
            daoent_dao::Pallet::<T>::try_add_project_member(
                dao_id,
                project_id.clone(),
                project.creator.clone(),
            )?;
            NextProjectId::<T>::put(project_id + 1);

            Ok(project_id)
        }

        /// 删除项目
        pub fn try_remove_project(
            dao_id: DaoAssetId,
            project_id: ProjectId,
        ) -> result::Result<ProjectId, DispatchError> {
            <DaoProjects<T>>::remove(dao_id, project_id);
            Ok(project_id)
        }

        pub fn get_project(
            dao_id: DaoAssetId,
            project_id: ProjectId,
        ) -> result::Result<ProjectInfo<T::AccountId, Status>, DispatchError> {
            let p = <DaoProjects<T>>::try_get(dao_id, project_id).unwrap();
            Ok(p)
        }

        /// 获取用户是否有 project 的权利
        pub fn check_auth_for_project(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let ms = <daoent_dao::ProjectMembers<T>>::get(dao_id, project_id);
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Project403)?;

            Ok(index)
        }

        /// 获取任务列表
        pub fn get_task(
            project_id: ProjectId,
            task_id: ProjectId,
        ) -> result::Result<
            (
                BoundedVec<TaskInfo<T::AccountId, BalanceOf<T>, TaskStatus>, ConstU32<20000>>,
                TaskInfo<T::AccountId, BalanceOf<T>, TaskStatus>,
                usize,
            ),
            DispatchError,
        > {
            // 获取人物列表
            let tasks = <Tasks<T>>::try_get(project_id).unwrap();
            let index = tasks
                .binary_search_by(|t| t.id.cmp(&task_id))
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            // 获取原始任务
            let task_brow = tasks.get(index).unwrap();
            let task = task_brow.clone();

            Ok((tasks, task, index))
        }
    }
}
