#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use codec::{Decode, Encode};
use frame_support::inherent::Vec;
use frame_support::{
    dispatch::{DispatchResult as DResult, UnfilteredDispatchable},
    RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{BlockNumberProvider, CheckedAdd, CheckedMul, Hash, Saturating},
    DispatchError,
};
use sp_std::boxed::Box;
use sp_std::result;
use traits::*;

use orml_traits::MultiCurrency;

use daoent_assets;
use daoent_dao::{self};
use daoent_primitives::traits::BaseCallFilter;
use daoent_primitives::types::DaoAssetId;

use weights::WeightInfo;

pub use pallet::*;

pub type PropIndex = u32;
pub type ReferendumIndex = u32;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod traits;
pub mod weights;

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum MemmberData<ID> {
    /// 全局.
    GLOBAL,
    /// 公会.
    GUILD(ID),
    /// 项目.
    PROJECT(ID),
}

/// Voting Statistics.
/// 投票数据统计
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Tally<Balance> {
    /// The number of yes votes
    /// 同意的数量
    pub yes: Balance,
    /// The number of no votes
    /// 不同意的数量
    pub no: Balance,
}

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Opinion {
    /// Agree.
    YES,
    /// Reject.
    NO,
}

/// Information about votes.
/// 投票信息
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct VoteInfo<DaoId, AssetId, Pledge, BlockNumber, VoteWeight, Opinion, ReferendumIndex> {
    /// The id of the Dao where the vote is located.
    /// 投票所在组织
    dao_id: DaoId,
    /// The specific group id mapped by Dao.
    /// 资产管理组
    asset_id: AssetId,
    /// The specific thing that the vote pledged.
    /// 抵押
    pledge: Pledge,
    /// Object or agree.
    /// 是否同意
    opinion: Opinion,
    /// voting weight.
    /// 投票权重
    vote_weight: VoteWeight,
    /// Block height that can be unlocked.
    /// 投票解锁阶段
    unlock_block: BlockNumber,
    /// The referendum id corresponding to the vote.
    /// 投票的全民公投
    referendum_index: ReferendumIndex,
}

/// Info regarding an ongoing referendum.
/// 全民公投的状态
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ReferendumStatus<BlockNumber, Call, Balance> {
    /// When voting on this referendum will end.
    /// 投票结束事件
    pub end: BlockNumber,
    /// The hash of the proposal being voted on.
    /// 投票后执行内容
    pub proposal: Call,
    /// The delay (in blocks) to wait after a successful referendum before deploying.
    /// 投票完成后多久被允许执行
    pub delay: BlockNumber,
    /// The current tally of votes in this referendum.
    /// 投票统计
    pub tally: Tally<Balance>,
}

/// Info regarding a referendum, present or past.
/// 全民公投
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum ReferendumInfo<BlockNumber, Call, Balance> {
    /// Referendum is happening, the arg is the block number at which it will end.
    /// 进行中
    Ongoing(ReferendumStatus<BlockNumber, Call, Balance>),
    /// Referendum finished at `end`, and has been `approved` or rejected.
    /// 已结束
    Finished { approved: bool, end: BlockNumber },
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    pub(crate) type BalanceOf<T> = <<T as daoent_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + daoent_assets::Config + daoent_dao::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// What to stake when voting in a referendum.
        type Pledge: Clone
            + Default
            + Copy
            + Parameter
            + Member
            + PledgeTrait<
                BalanceOf<Self>,
                Self::AccountId,
                DaoAssetId,
                Self::Conviction,
                Self::BlockNumber,
                DispatchError,
            >;

        /// The number of times the vote is magnified.
        type Conviction: Clone
            + Default
            + Copy
            + Parameter
            + ConvertInto<Self::BlockNumber>
            + ConvertInto<BalanceOf<Self>>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Number of public proposals so for.
    #[pallet::storage]
    #[pallet::getter(fn public_prop_count)]
    pub type PublicPropCount<T: Config> =
        StorageMap<_, Identity, DaoAssetId, PropIndex, ValueQuery>;

    #[pallet::type_value]
    pub fn MaxPublicPropsOnEmpty() -> PropIndex {
        100u32
    }

    /// Maximum number of public proposals at one time.
    #[pallet::storage]
    #[pallet::getter(fn max_public_props)]
    pub type MaxPublicProps<T: Config> =
        StorageMap<_, Identity, DaoAssetId, u32, ValueQuery, MaxPublicPropsOnEmpty>;

    #[pallet::type_value]
    pub fn LaunchPeriodOnEmpty<T: Config>() -> T::BlockNumber {
        T::BlockNumber::from(900u32)
    }

    /// How soon can a referendum be called.
    #[pallet::storage]
    #[pallet::getter(fn launch_period)]
    pub type LaunchPeriod<T: Config> =
        StorageMap<_, Identity, DaoAssetId, T::BlockNumber, ValueQuery, LaunchPeriodOnEmpty<T>>;

    /// Minimum stake per person when making public proposals.
    #[pallet::storage]
    #[pallet::getter(fn minimum_deposit)]
    pub type MinimumDeposit<T: Config> =
        StorageMap<_, Identity, DaoAssetId, BalanceOf<T>, ValueQuery>;

    #[pallet::type_value]
    pub fn VotingPeriodOnEmpty<T: Config>() -> T::BlockNumber {
        T::BlockNumber::from(900u32)
    }

    /// How long each proposal can be voted on.
    #[pallet::storage]
    #[pallet::getter(fn voting_period)]
    pub type VotingPeriod<T: Config> =
        StorageMap<_, Identity, DaoAssetId, T::BlockNumber, ValueQuery, VotingPeriodOnEmpty<T>>;

    #[pallet::type_value]
    pub fn ReservePeriodOnEmpty<T: Config>() -> T::BlockNumber {
        T::BlockNumber::from(900u32)
    }

    /// How long does it take to release the mortgage.
    #[pallet::storage]
    #[pallet::getter(fn reserve_period)]
    pub type ReservePeriod<T: Config> =
        StorageMap<_, Identity, DaoAssetId, T::BlockNumber, ValueQuery, ReservePeriodOnEmpty<T>>;

    #[pallet::type_value]
    pub fn EnactmentPeriodOnEmpty<T: Config>() -> T::BlockNumber {
        T::BlockNumber::from(900u32)
    }

    /// How soon after voting closes the proposal can be implemented.
    #[pallet::storage]
    #[pallet::getter(fn runment_period)]
    pub type EnactmentPeriod<T: Config> =
        StorageMap<_, Identity, DaoAssetId, T::BlockNumber, ValueQuery, EnactmentPeriodOnEmpty<T>>;

    /// The public proposals. Unsorted. The second item is the proposal's hash.
    #[pallet::storage]
    #[pallet::getter(fn public_props)]
    pub type PublicProps<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        Vec<(
            PropIndex,
            T::Hash,
            <T as daoent_dao::Config>::RuntimeCall,
            MemmberData<u64>,
            T::AccountId,
        )>,
        ValueQuery,
    >;

    /// Those who have locked a deposit.
    ///
    /// TWOX-NOTE: Safe, as increasing integer keys are safe.
    #[pallet::storage]
    #[pallet::getter(fn deposit_of)]
    pub type DepositOf<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DaoAssetId,
        Identity,
        PropIndex,
        (Vec<T::AccountId>, BalanceOf<T>),
    >;

    /// Amount of proposal locked.
    #[pallet::storage]
    #[pallet::getter(fn reserve_of)]
    pub type ReserveOf<T: Config> =
        StorageMap<_, Identity, T::AccountId, Vec<(BalanceOf<T>, T::BlockNumber)>, ValueQuery>;

    /// Referendum specific information.
    #[pallet::storage]
    #[pallet::getter(fn referendum_info)]
    pub type ReferendumInfoOf<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DaoAssetId,
        Identity,
        ReferendumIndex,
        ReferendumInfo<T::BlockNumber, <T as daoent_dao::Config>::RuntimeCall, BalanceOf<T>>,
    >;

    /// Number of referendums so far.
    #[pallet::storage]
    #[pallet::getter(fn referendum_count)]
    pub type ReferendumCount<T: Config> =
        StorageMap<_, Identity, DaoAssetId, ReferendumIndex, ValueQuery>;

    /// Everyone's voting information.
    #[pallet::storage]
    #[pallet::getter(fn votes_of)]
    pub type VotesOf<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        Vec<
            VoteInfo<
                DaoAssetId,
                T::AssetId,
                T::Pledge,
                T::BlockNumber,
                BalanceOf<T>,
                Opinion,
                ReferendumIndex,
            >,
        >,
        ValueQuery,
    >;

    /// Minimum voting weight required for each external transaction.
    #[pallet::storage]
    #[pallet::getter(fn min_vote_weight_of)]
    pub type MinVoteWeightOf<T: Config> =
        StorageDoubleMap<_, Identity, DaoAssetId, Identity, T::CallId, BalanceOf<T>, ValueQuery>;

    /// When the referendum was last launched.
    #[pallet::storage]
    #[pallet::getter(fn launch_tag)]
    pub type LaunchTag<T: Config> = StorageMap<_, Identity, DaoAssetId, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// initiate a proposal.
        Proposed(DaoAssetId, T::Hash),
        /// Others support initiating proposals.
        Recreate(DaoAssetId, BalanceOf<T>),
        /// Open a referendum.
        StartTable(DaoAssetId, ReferendumIndex),
        /// Vote for the referendum.
        Vote(DaoAssetId, ReferendumIndex, T::Pledge),
        /// Cancel a vote on a referendum.
        CancelVote(DaoAssetId, ReferendumIndex),
        /// Vote and execute the transaction corresponding to the proposa.
        EnactProposal {
            dao_id: DaoAssetId,
            index: ReferendumIndex,
            result: DResult,
        },
        /// Unlock
        Unlock(T::AccountId, T::AssetId, T::Pledge),
        /// Unlock
        Unreserved(T::AccountId, BalanceOf<T>),
        /// Set Origin for each Call.
        SetMinVoteWeight(DaoAssetId, T::CallId, BalanceOf<T>),
        /// Set the maximum number of proposals at the same time.
        SetMaxPublicProps { dao_id: DaoAssetId, max: u32 },
        /// Set the referendum interval.
        SetLaunchPeriod {
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        },
        /// Set the minimum amount a proposal needs to stake.
        SetMinimumDeposit {
            dao_id: DaoAssetId,
            min: BalanceOf<T>,
        },
        /// Set the voting length of the referendum.
        SetVotingPeriod {
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        },
        /// Set the length of time that can be unreserved.
        SetReservePeriod {
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        },
        /// Set the time to delay the execution of the proposal.
        SetEnactmentPeriod {
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Integer computation overflow.
        Overflow,
        /// Insufficient amount of deposit.
        DepositTooLow,
        /// Maximum number of proposals reached.
        TooManyProposals,
        /// Proposal does not exist.
        ProposalMissing,
        /// There are no proposals in progress.
        NoneWaiting,
        /// Referendum does not exist.
        ReferendumNotExists,
        /// Referendum ends.
        ReferendumFinished,
        /// Referendum voting is underway.
        VoteNotEnd,
        /// Delayed execution time.
        InDelayTime,
        /// Referendum voting has ended.
        VoteEnd,
        /// Voting closed but proposal rejected.
        VoteEndButNotPass,
        /// It's not time to open a new referendum.
        NotTableTime,
        /// Bad origin.
        VoteWeightTooLow,
        ///
        PledgeNotEnough,
        Gov403,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create a proposal
        /// 创建一个提案
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::propose())]
        pub fn create_propose(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            member_data: MemmberData<u64>,
            proposal: Box<<T as daoent_dao::Config>::RuntimeCall>,
            #[pallet::compact] value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::check_auth_for_proposal(dao_id, who.clone())?;

            // 确认提案为当前资产支持的 调用
            ensure!(
                daoent_dao::Pallet::<T>::try_get_asset_id(dao_id)?.contains(*proposal.clone()),
                daoent_dao::Error::<T>::InVailCall
            );

            // 判断最小押金
            ensure!(
                value >= MinimumDeposit::<T>::get(dao_id),
                Error::<T>::DepositTooLow
            );

            let proposal_hash = T::Hashing::hash_of(&proposal);
            let proposal_index = Self::public_prop_count(dao_id);
            let real_prop_count = PublicProps::<T>::decode_len(dao_id).unwrap_or(0) as u32;
            let max_proposals = MaxPublicProps::<T>::get(dao_id);

            // 确定提案数是否超过了最大提案
            ensure!(
                real_prop_count < max_proposals,
                Error::<T>::TooManyProposals
            );

            daoent_assets::Pallet::<T>::reserve(dao_id, who.clone(), value)?;

            // 更新提案数量
            PublicPropCount::<T>::insert(dao_id, proposal_index + 1);

            // 添加提案抵押
            <DepositOf<T>>::insert(dao_id, proposal_index, (&[&who][..], value));

            // 添加提案
            <PublicProps<T>>::append(
                dao_id,
                (proposal_index, proposal_hash, *proposal, member_data, who),
            );

            Self::deposit_event(Event::<T>::Proposed(dao_id, proposal_hash));
            Ok(().into())
        }

        /// 重新提交提案
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::second())]
        pub fn recreate(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            #[pallet::compact] proposal_index: PropIndex,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let mut deposit =
                Self::deposit_of(dao_id, proposal_index).ok_or(Error::<T>::ProposalMissing)?;

            let deposit_amount = deposit.1;
            daoent_assets::Pallet::<T>::reserve(dao_id, who.clone(), deposit_amount)?;

            deposit.0.push(who.clone());
            <DepositOf<T>>::insert(dao_id, proposal_index, deposit);

            let unreserved_block = Self::now()
                .checked_add(&ReservePeriod::<T>::get(dao_id))
                .ok_or(Error::<T>::Overflow)?;
            ReserveOf::<T>::append(who, (deposit_amount, unreserved_block));

            Self::deposit_event(Event::<T>::Recreate(dao_id, deposit_amount));
            Ok(().into())
        }

        /// Open a referendum.
        /// 开始全民公投
        #[pallet::call_index(003)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::open_referendum())]
        pub fn start_referendum(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            propose_index: u32,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            let tag = LaunchTag::<T>::get(dao_id);
            let now = Self::now();
            let dao_start_time = daoent_dao::Pallet::<T>::try_get_dao(dao_id)?.start_block;
            // (now - dao_start_time) / LaunchPeriod > tag
            ensure!(
                tag.checked_mul(&LaunchPeriod::<T>::get(dao_id))
                    .ok_or(Error::<T>::Overflow)?
                    < (now - dao_start_time),
                Error::<T>::NotTableTime
            );

            // 获取提案
            let mut public_props = Self::public_props(dao_id);
            let (prop_index, _, proposal, _, _) =
                public_props.swap_remove(propose_index.try_into().unwrap());
            <PublicProps<T>>::insert(dao_id, public_props);

            // 获取抵押
            let mut referendum_index: Option<ReferendumIndex> = None;
            let now = Self::now();
            if <DepositOf<T>>::take(dao_id, prop_index).is_some() {
                referendum_index = Some(Self::inject_referendum(
                    dao_id,
                    proposal,
                    now.saturating_add(VotingPeriod::<T>::get(dao_id)),
                    EnactmentPeriod::<T>::get(dao_id),
                ));
            }

            if referendum_index.is_none() {
                Err(Error::<T>::NoneWaiting)?
            }

            Self::deposit_event(Event::<T>::StartTable(dao_id, referendum_index.unwrap()));

            Ok(().into())
        }

        /// Vote for the referendum
        /// 为全民公投投票
        #[pallet::call_index(004)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::vote_for_referendum())]
        pub fn vote_for_referendum(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            referendum_index: ReferendumIndex,
            pledge: T::Pledge,
            conviction: T::Conviction,
            opinion: Opinion,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = Self::now();
            let mut vote_weight = BalanceOf::<T>::from(0u32);

            ReferendumInfoOf::<T>::try_mutate_exists(
                dao_id,
                referendum_index,
                |h| -> result::Result<(), DispatchError> {
                    let mut info = h.take().ok_or(Error::<T>::ReferendumNotExists)?;
                    if let ReferendumInfo::Ongoing(ref mut x) = info {
                        if x.end > now {
                            let asset_id = daoent_dao::Pallet::<T>::try_get_asset_id(dao_id)?;
                            let vote_result = pledge.try_vote(&who, &dao_id, &conviction)?;
                            vote_weight = vote_result.0;

                            let duration = vote_result.1;
                            match opinion {
                                Opinion::NO => {
                                    x.tally.no += vote_weight;
                                }
                                Opinion::YES => {
                                    x.tally.yes += vote_weight;
                                }
                            };

                            VotesOf::<T>::append(
                                &who,
                                VoteInfo {
                                    dao_id,
                                    asset_id,
                                    pledge,
                                    opinion,
                                    vote_weight,
                                    unlock_block: now + duration,
                                    referendum_index: referendum_index,
                                },
                            );
                        } else {
                            return Err(Error::<T>::VoteEnd)?;
                        }
                    } else {
                        return Err(Error::<T>::ReferendumFinished)?;
                    }
                    *h = Some(info);
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::<T>::Vote(dao_id, referendum_index, pledge));
            Ok(().into())
        }

        /// Cancel a vote on a referendum
        /// 取消一个投票
        #[pallet::call_index(005)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_vote())]
        pub fn cancel_vote(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            index: ReferendumIndex,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ReferendumInfoOf::<T>::try_mutate_exists(
                dao_id,
                index,
                |h| -> result::Result<(), DispatchError> {
                    let mut info = h.take().ok_or(Error::<T>::ReferendumNotExists)?;
                    let now = Self::now();
                    if let ReferendumInfo::Ongoing(ref mut x) = info {
                        if x.end > now {
                            let mut votes = VotesOf::<T>::get(&who);
                            votes.retain(|h| {
                                if h.referendum_index == index
                                    && h.pledge.vote_end_do(&who, &dao_id).is_ok()
                                {
                                    match h.opinion {
                                        Opinion::NO => {
                                            x.tally.no = x.tally.no.saturating_sub(h.vote_weight);
                                        }
                                        _ => {
                                            x.tally.yes = x.tally.yes.saturating_sub(h.vote_weight);
                                        }
                                    };
                                    false
                                } else {
                                    true
                                }
                            });
                            VotesOf::<T>::insert(&who, votes);
                        } else {
                            return Err(Error::<T>::VoteEnd)?;
                        }
                    } else {
                        return Err(Error::<T>::ReferendumFinished)?;
                    }
                    *h = Some(info);
                    Ok(())
                },
            )?;
            Self::deposit_event(Event::<T>::CancelVote(dao_id, index));

            Ok(().into())
        }

        /// Vote and execute the transaction corresponding to the proposa
        /// 执行一个投票通过提案
        #[pallet::call_index(006)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::run_proposal())]
        pub fn run_proposal(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            index: ReferendumIndex,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            let now = Self::now();
            let mut approved = false;
            let info =
                ReferendumInfoOf::<T>::get(dao_id, index).ok_or(Error::<T>::ReferendumNotExists)?;
            match info {
                ReferendumInfo::Ongoing(state) => {
                    if state.end > now {
                        return Err(Error::<T>::VoteNotEnd)?;
                    } else if state.end.saturating_add(state.delay) > now {
                        return Err(Error::<T>::InDelayTime)?;
                    } else {
                        {
                            let call_id: T::CallId =
                                TryFrom::<<T as daoent_dao::Config>::RuntimeCall>::try_from(
                                    state.proposal.clone(),
                                )
                                .unwrap_or_default();

                            if state.tally.yes.saturating_add(state.tally.no)
                                >= MinVoteWeightOf::<T>::get(dao_id, call_id)
                            {
                                if state.tally.yes >= state.tally.no {
                                    approved = true;
                                    let res = state.proposal.dispatch_bypass_filter(
                                        frame_system::RawOrigin::Signed(
                                            daoent_dao::Pallet::<T>::try_get_dao_account_id(
                                                dao_id,
                                            )?,
                                        )
                                        .into(),
                                    );
                                    Self::deposit_event(Event::EnactProposal {
                                        dao_id,
                                        index,
                                        result: res.map(|_| ()).map_err(|e| e.error),
                                    });
                                } else {
                                    Self::deposit_event(Event::EnactProposal {
                                        dao_id,
                                        index,
                                        result: Err(Error::<T>::VoteEndButNotPass)?,
                                    });
                                }
                            } else {
                                return Err(Error::<T>::VoteWeightTooLow)?;
                            }
                        }
                    }
                }
                _ => return Err(Error::<T>::ReferendumFinished)?,
            }
            ReferendumInfoOf::<T>::insert(
                dao_id,
                index,
                ReferendumInfo::Finished { approved, end: now },
            );

            Ok(().into())
        }

        /// Unlock
        #[pallet::call_index(007)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::unlock())]
        pub fn unlock(origin: OriginFor<T>, dao_id: DaoAssetId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = Self::now();
            //
            {
                let mut total = BalanceOf::<T>::from(0u32);
                let mut reserve_info = ReserveOf::<T>::get(&who);
                reserve_info.retain(|h| {
                    if h.1 > now {
                        true
                    } else {
                        daoent_assets::Pallet::<T>::unreserve(dao_id, who.clone(), h.0).unwrap();
                        total += h.0;
                        false
                    }
                });
                ReserveOf::<T>::insert(&who, reserve_info);
                Self::deposit_event(Event::<T>::Unreserved(who.clone(), total));
            }

            //
            {
                let mut votes = VotesOf::<T>::get(&who);
                votes.retain(|h| {
                    if h.unlock_block > now || h.pledge.vote_end_do(&who, &h.dao_id).is_err() {
                        true
                    } else {
                        Self::deposit_event(Event::<T>::Unlock(who.clone(), h.asset_id, h.pledge));
                        false
                    }
                });
                VotesOf::<T>::insert(&who, votes);
            }

            Ok(().into())
        }

        /// Set Origin for each Call.
        #[pallet::call_index(008)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_min_vote_weight_for_every_call())]
        pub fn set_min_vote_weight_for_every_call(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            call_id: T::CallId,
            min_vote_weight: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;
            MinVoteWeightOf::<T>::insert(dao_id, call_id, min_vote_weight);
            Self::deposit_event(Event::<T>::SetMinVoteWeight(
                dao_id,
                call_id,
                min_vote_weight,
            ));

            Ok(().into())
        }

        /// Set the maximum number of proposals at the same time
        #[pallet::call_index(009)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_max_public_props())]
        pub fn set_max_public_props(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            max: u32,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            MaxPublicProps::<T>::insert(dao_id, max);
            Self::deposit_event(Event::<T>::SetMaxPublicProps { dao_id, max });

            Ok(().into())
        }

        /// Set the referendum interval
        #[pallet::call_index(010)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_launch_period())]
        pub fn set_launch_period(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            LaunchPeriod::<T>::insert(dao_id, period);
            Self::deposit_event(Event::<T>::SetLaunchPeriod { dao_id, period });

            Ok(().into())
        }

        /// Set the minimum amount a proposal needs to stake
        #[pallet::call_index(011)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_minimum_deposit())]
        pub fn set_minimum_deposit(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            min: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            MinimumDeposit::<T>::insert(dao_id, min);
            Self::deposit_event(Event::<T>::SetMinimumDeposit { dao_id, min });

            Ok(().into())
        }

        /// Set the voting length of the referendum
        #[pallet::call_index(012)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_voting_period())]
        pub fn set_voting_period(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            VotingPeriod::<T>::insert(dao_id, period);
            Self::deposit_event(Event::<T>::SetVotingPeriod { dao_id, period });

            Ok(().into())
        }

        /// Set the length of time that can be unreserved
        #[pallet::call_index(013)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_rerserve_period())]
        pub fn set_rerserve_period(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            ReservePeriod::<T>::insert(dao_id, period);
            Self::deposit_event(Event::<T>::SetReservePeriod { dao_id, period });

            Ok(().into())
        }

        /// Set the time to delay the execution of the proposal
        #[pallet::call_index(014)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_runment_period())]
        pub fn set_runment_period(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            period: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(me, dao_id)?;

            EnactmentPeriod::<T>::insert(dao_id, period);
            Self::deposit_event(Event::<T>::SetEnactmentPeriod { dao_id, period });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取当前投票的作用范围
        pub fn try_get_members(
            dao_id: DaoAssetId,
            member_data: MemmberData<u64>,
        ) -> result::Result<BoundedVec<T::AccountId, T::MaxMembers>, DispatchError> {
            let ms: BoundedVec<T::AccountId, T::MaxMembers>;
            match member_data {
                MemmberData::GLOBAL => ms = <daoent_dao::Members<T>>::get(dao_id),
                MemmberData::GUILD(v) => ms = <daoent_dao::GuildMembers<T>>::get(dao_id, v),
                MemmberData::PROJECT(v) => ms = <daoent_dao::ProjectMembers<T>>::get(dao_id, v),
            }
            Ok(ms)
        }

        /// 获取用户是否有 提案 的权利
        pub fn check_auth_for_proposal(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let ms = <daoent_dao::Members<T>>::get(dao_id);
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Gov403)?;

            Ok(index)
        }

        /// 获取用户是否有 提案//投票 的权利
        pub fn check_auth_for_vote(
            dao_id: DaoAssetId,
            member_data: MemmberData<u64>,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let ms: BoundedVec<T::AccountId, T::MaxMembers>;
            match member_data {
                MemmberData::GLOBAL => ms = <daoent_dao::Members<T>>::get(dao_id),
                MemmberData::GUILD(v) => ms = <daoent_dao::GuildMembers<T>>::get(dao_id, v),
                MemmberData::PROJECT(v) => ms = <daoent_dao::ProjectMembers<T>>::get(dao_id, v),
            }
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Gov403)?;

            Ok(index)
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn backing_for(dao_id: DaoAssetId, proposal: PropIndex) -> Option<BalanceOf<T>> {
        Self::deposit_of(dao_id, proposal).map(|(l, d)| d.saturating_mul((l.len() as u32).into()))
    }

    fn inject_referendum(
        dao_id: DaoAssetId,
        proposal: <T as daoent_dao::Config>::RuntimeCall,
        end: T::BlockNumber,
        delay: T::BlockNumber,
    ) -> ReferendumIndex {
        let ref_index = Self::referendum_count(dao_id);
        ReferendumCount::<T>::insert(dao_id, ref_index + 1);
        let status = ReferendumStatus {
            end,
            proposal,
            delay,
            tally: Default::default(),
        };

        let item = ReferendumInfo::Ongoing(status);
        <ReferendumInfoOf<T>>::insert(dao_id, ref_index, item);
        ref_index
    }

    fn now() -> T::BlockNumber {
        frame_system::Pallet::<T>::current_block_number()
    }
}
