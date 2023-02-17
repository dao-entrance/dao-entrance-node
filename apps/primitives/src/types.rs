use core::ops::{Add, Mul};

use codec::MaxEncodedLen;
pub use frame_support::codec::{Decode, Encode};
use frame_support::RuntimeDebug;
pub use scale_info::TypeInfo;
use sp_runtime::traits::{CheckedAdd, One};

/// Simple index type for proposal counting.
pub type ProposalIndex = u32;

/// 用户容量
pub type MemberCount = u32;

/// 真实请求ID
pub type RealCallId = u32;

/// DAO函数index
pub type CallId = u32;

/// 资源ID
pub type DaoAssetId = u64;

/// ProjectId
/// 项目ID
pub type ProjectId = u64;

/// BoardId
/// 看板ID
pub type BoardId = u64;

/// TaskId
/// 任务ID
pub type TaskId = u64;

/// 组织ID
#[derive(Decode, Encode, Copy, Clone, Default, Debug, TypeInfo, MaxEncodedLen, Eq, PartialEq)]
pub struct DaoId(pub u64);

impl From<DaoId> for u64 {
    fn from(x: DaoId) -> Self {
        x.0
    }
}

impl From<u64> for DaoId {
    fn from(x: u64) -> Self {
        DaoId(x)
    }
}

impl Mul<Self> for DaoId {
    type Output = DaoId;

    fn mul(self, d: Self) -> Self::Output {
        DaoId(self.0 * d.0)
    }
}

impl One for DaoId {
    fn one() -> Self {
        DaoId(1u64)
    }
}

impl Add<Self> for DaoId {
    type Output = DaoId;

    fn add(self, d: Self) -> Self::Output {
        DaoId(self.0.add(d.0))
    }
}

impl CheckedAdd for DaoId {
    fn checked_add(&self, d: &Self) -> Option<Self> {
        self.0.checked_add(d.0).map(DaoId)
    }
}

/// 定义可以被转换为 用户id 的泛型
pub trait AccountIdType<AccountId>: Sized {
    /// 转换为用户id
    fn into_account(&self) -> AccountId;

    /// 从用户id转换为本结构
    fn try_from_account(a: &AccountId) -> Option<Self>;
}

pub struct TrailingZeroInput<'a>(pub &'a [u8]);
impl<'a> codec::Input for TrailingZeroInput<'a> {
    fn remaining_len(&mut self) -> Result<Option<usize>, codec::Error> {
        Ok(None)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<(), codec::Error> {
        let len = into.len().min(self.0.len());
        into[..len].copy_from_slice(&self.0[..len]);
        for i in &mut into[len..] {
            *i = 0;
        }
        self.0 = &self.0[len..];
        Ok(())
    }
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, MaxEncodedLen)]
pub enum Proportion<MemberCount> {
    MoreThan(MemberCount, MemberCount),
    AtLeast(MemberCount, MemberCount),
}

impl Default for Proportion<MemberCount> {
    fn default() -> Self {
        Self::MoreThan(1, 1)
    }
}

#[cfg_attr(any(feature = "std", test), derive(Debug))]
#[derive(PartialEq, Encode, Decode, Clone, TypeInfo, Copy, MaxEncodedLen)]
pub enum DoAsEnsureOrigin<Pro, C> {
    Proportion(Pro),
    Member,
    Members(C),
    Root,
}

impl<Pro: Default, C: Default> Default for DoAsEnsureOrigin<Pro, C> {
    fn default() -> Self {
        Self::Root
    }
}
