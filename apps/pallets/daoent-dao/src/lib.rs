#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
use codec::MaxEncodedLen;
use daoent_primitives::{
    traits::AfterCreate,
    types::{DaoAssetId, ProjectId},
};
use frame_support::{
    codec::{Decode, Encode},
    traits::IsSubType,
};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{traits::BlockNumberProvider, RuntimeDebug};
use sp_std::{prelude::*, result};

mod weights;
use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// DAO's status.
/// 组织状态
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Status {
    /// In use.
    /// 激活
    Active,
    /// Does not work properly.
    /// 未激活
    InActive,
}

/// DAO specific information
/// 组织信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct DaoInfo<AccountId, BlockNumber, Status> {
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the DAO
    /// DAO创建的区块
    pub start_block: BlockNumber,
    /// DAO account id.
    /// DAO 链上账户ID
    pub dao_account_id: AccountId,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub purpose: Vec<u8>,
    //// meta data
    /// DAO 元数据 图片等内容
    pub meta_data: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    status: Status,
}

/// Guild information
/// 组织内公会信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct GuildInfo<AccountId, BlockNumber, Status> {
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the DAO
    /// DAO创建的区块
    pub start_block: BlockNumber,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub name: Vec<u8>,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub desc: Vec<u8>,
    //// meta data
    /// DAO 元数据 图片等内容
    pub meta_data: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    pub status: Status,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoAssetAccount {
    pub dao_id: DaoAssetId,
    pub t: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoProjectAccount {
    pub dao_id: DaoAssetId,
    pub project_id: ProjectId,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::{DispatchResultWithPostInfo, GetDispatchInfo},
        pallet_prelude::*,
        traits::UnfilteredDispatchable,
        PalletId,
    };
    use frame_system::pallet_prelude::*;

    /// pallet config
    /// 组件配置文件
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// All calls supported by DAO
        /// 组件所有的函数
        type RuntimeCall: Parameter
            + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>
            + From<Call<Self>>
            + IsSubType<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>;

        /// Each Call has its own id
        /// 函数的调用id
        type CallId: Parameter
            + Copy
            + MaybeSerializeDeserialize
            + TypeInfo
            + MaxEncodedLen
            + Default
            + TryFrom<<Self as pallet::Config>::RuntimeCall>;

        /// pallet id
        /// 模块id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Do some things after creating dao, such as setting up a sudo account.
        /// 创建DAO之后的回调
        type AfterCreate: AfterCreate<Self::AccountId>;

        /// max member number
        /// 组织最大的人数
        type MaxMembers: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// All DAOs that have been created.
    /// 所有被创建组织
    #[pallet::storage]
    #[pallet::getter(fn daos)]
    pub type Daos<T: Config> =
        StorageMap<_, Identity, DaoAssetId, DaoInfo<T::AccountId, T::BlockNumber, Status>>;

    #[pallet::type_value]
    pub fn DefaultForm1() -> DaoAssetId {
        1
    }

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_dao_id)]
    pub type NextDaoId<T: Config> = StorageValue<_, DaoAssetId, ValueQuery, DefaultForm1>;

    /// the info of grutypes
    /// 组织内公会信息
    #[pallet::storage]
    #[pallet::getter(fn guilds)]
    pub type Guilds<T: Config> = StorageMap<
        _,
        Twox64Concat,
        DaoAssetId,
        BoundedVec<GuildInfo<T::AccountId, T::BlockNumber, Status>, ConstU32<100>>,
        ValueQuery,
    >;

    /// team members
    /// 团队的成员
    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config> = StorageMap<
        _,
        Twox64Concat,
        DaoAssetId,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// guild members
    /// 公会成员
    #[pallet::storage]
    #[pallet::getter(fn guild_members)]
    pub type GuildMembers<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        u64,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// project members
    /// 项目成员
    #[pallet::storage]
    #[pallet::getter(fn project_members)]
    pub type ProjectMembers<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        ProjectId,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// point
    /// 成员贡献点
    #[pallet::storage]
    #[pallet::getter(fn member_point)]
    pub type MemberPoint<T: Config> =
        StorageDoubleMap<_, Twox64Concat, DaoAssetId, Twox64Concat, T::AccountId, u32, ValueQuery>;

    /// success event
    /// 成功事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// DAO create event
        /// DAO创建成功事件
        CreatedDao(T::AccountId, DaoAssetId),
        /// nomal success
        /// 成功的事件
        Success,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Do not have permission to create.
        /// 没有创建的权限
        HaveNoCreatePermission,
        /// DAO already exists
        /// 组织已存在
        DaoExists,
        /// DAO does not exist.
        /// 组织不存在
        DaoNotExists,
        /// guild create error
        /// 公会创建失败
        GuildCreateError,
        /// guild does not exist.
        /// 公会不存在
        GuildNotExists,
        /// DAO unsupported call
        /// 无效的调用
        InVailCall,
        /// Wrong origin.
        /// 错误的组织
        BadOrigin,
        /// Not the id of this dao.
        /// 组织 id 不正确
        DaoIdNotMatch,
        /// The description of the DAO is too long.
        /// 组织目标太长
        PurposeTooLong,
        /// The description of the DAO is too long.
        /// 组织目标太长
        MetaDataTooLong,
        /// Numerical calculation overflow error.
        /// 溢出错误
        Overflow,
        /// member number is too long
        /// 成员数量太大
        TooManyMembers,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a DAO
        /// 从一个通证池,创建一个组织
        #[pallet::call_index(001)]
        #[pallet::weight(T::WeightInfo::create_dao())]
        pub fn create_dao(
            origin: OriginFor<T>,
            purpose: Vec<u8>,
            meta_data: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure!(purpose.len() <= 50, Error::<T>::PurposeTooLong);
            ensure!(meta_data.len() <= 1024, Error::<T>::MetaDataTooLong);

            let creator = ensure_signed(origin)?;

            // 创建 DAO
            let dao_id = NextDaoId::<T>::get();
            let now = frame_system::Pallet::<T>::current_block_number();
            Daos::<T>::insert(
                dao_id,
                DaoInfo {
                    creator: creator.clone(),
                    start_block: now,
                    purpose,
                    status: Status::Active,
                    dao_account_id: Self::dao_account(dao_id),
                    meta_data,
                },
            );

            // 初始化会员
            Self::try_add_member(dao_id, creator.clone())?;

            // 创建核心团队-coreTeam
            let mut guilds = <Guilds<T>>::get(dao_id);
            guilds
                .try_insert(
                    0,
                    GuildInfo {
                        creator: creator.clone(),
                        start_block: now,
                        name: "core team".as_bytes().to_vec(),
                        desc: "".as_bytes().to_vec(),
                        status: Status::Active,
                        meta_data: "{}".as_bytes().to_vec(),
                    },
                )
                .map_err(|_| Error::<T>::GuildCreateError)?;

            <Guilds<T>>::insert(dao_id, &guilds);

            // 获取
            Self::try_add_guild_member(dao_id, 0, creator.clone())?;

            // 记录下一个 DAO id
            let next_id = dao_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextDaoId::<T>::put(next_id);

            // 执行 DAO 创建后回调
            T::AfterCreate::run_hook(creator.clone(), dao_id);

            Self::deposit_event(Event::CreatedDao(creator, dao_id));
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取DAO账户
        pub fn dao_account(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(dao_id)
        }

        /// 获取DAO账户
        pub fn dao_asset(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoAssetAccount { dao_id, t: 1 })
        }

        /// 获取DAO账户
        pub fn dao_asset_pending(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoAssetAccount { dao_id, t: 2 })
        }

        /// 获取DAO项目账户
        pub fn dao_project(dao_id: DaoAssetId, p_id: ProjectId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoProjectAccount {
                dao_id,
                project_id: p_id,
            })
        }

        /// 获取创建者
        pub fn try_get_creator(dao_id: DaoAssetId) -> result::Result<T::AccountId, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao.creator)
        }

        /// 获取组织信息
        pub fn try_get_dao(
            dao_id: DaoAssetId,
        ) -> Result<DaoInfo<T::AccountId, T::BlockNumber, Status>, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao)
        }

        /// 获取公会信息
        pub fn try_get_guild(
            dao_id: DaoAssetId,
            guild_index: u32,
        ) -> Result<GuildInfo<T::AccountId, T::BlockNumber, Status>, DispatchError> {
            let guilds = <Guilds<T>>::get(dao_id);
            let guild = guilds
                .get(guild_index as usize)
                .ok_or(Error::<T>::DaoNotExists)?;
            Ok(guild.clone())
        }

        /// 获取 DAO 账户ID
        pub fn try_get_dao_account_id(
            dao_id: DaoAssetId,
        ) -> result::Result<T::AccountId, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao.dao_account_id)
        }

        /// 确认为 DAO 创建者
        pub fn ensrue_dao_root(
            who: T::AccountId,
            dao_id: DaoAssetId,
        ) -> result::Result<T::AccountId, DispatchError> {
            let dao_account_id = Self::try_get_dao_account_id(dao_id)?;
            ensure!(who == dao_account_id, Error::<T>::BadOrigin);
            Ok(who)
        }

        /// 添加成员
        pub fn try_add_guild_member(
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let guild = <Guilds<T>>::get(dao_id);
            ensure!(!guild.is_empty(), Error::<T>::BadOrigin);

            let gindex: u64 = guild_id.into();
            let mut members = <GuildMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;

            members
                .try_insert(index, who.clone())
                .map_err(|_| Error::<T>::TooManyMembers)?;

            <GuildMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_guild_member(
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let guild = <Guilds<T>>::get(dao_id);
            ensure!(!guild.is_empty(), Error::<T>::BadOrigin);

            let gindex: u64 = guild_id.into();
            let mut members = <GuildMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <GuildMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        pub fn try_add_member(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            // 初始化成员
            let mut members = <Members<T>>::get(dao_id);
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;
            members
                .try_insert(index, who.clone())
                .map_err(|_| Error::<T>::GuildCreateError)?;

            <Members<T>>::insert(dao_id, &members);
            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_member(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let mut members = <Members<T>>::get(dao_id);
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <Members<T>>::insert(dao_id, &members);
            Ok(index)
        }

        /// 添加项目成员
        pub fn try_add_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id.into();
            let mut members = <ProjectMembers<T>>::try_get(dao_id, gindex).unwrap_or_default();
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;

            members
                .try_insert(index, who.clone())
                .map_err(|_| Error::<T>::TooManyMembers)?;

            <ProjectMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id.into();
            let mut members = <ProjectMembers<T>>::try_get(dao_id, gindex).unwrap_or_default();
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <ProjectMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        pub fn try_add_member_point(
            dao_id: DaoAssetId,
            who: T::AccountId,
            point: u32,
        ) -> result::Result<u32, DispatchError> {
            let mut p = <MemberPoint<T>>::get(dao_id, who.clone());
            p = p + point;
            <MemberPoint<T>>::insert(dao_id, who, p);
            Ok(p)
        }
    }
}
