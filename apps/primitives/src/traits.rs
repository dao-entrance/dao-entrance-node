use core::result;

use crate::types::DaoAssetId;
use sp_runtime::DispatchError;

pub struct BadOrigin;

impl From<BadOrigin> for &'static str {
    fn from(_: BadOrigin) -> &'static str {
        "无效的用户"
    }
}

pub trait SetCollectiveMembers<AccountId: Clone + Ord, DispathErr> {
    fn set_members_sorted(
        dao_id: DaoAssetId,
        members: &[AccountId],
        prime: Option<AccountId>,
    ) -> result::Result<(), DispathErr>;
}

pub trait AfterCreate<AccountId> {
    fn run_hook(a: AccountId, b: DaoAssetId);
}

impl<AccountId: Clone> AfterCreate<AccountId> for () {
    fn run_hook(_a: AccountId, _b: DaoAssetId) {}
}

impl<AccountId: Clone + Ord> SetCollectiveMembers<AccountId, DispatchError> for () {
    fn set_members_sorted(
        _dao_id: DaoAssetId,
        _members: &[AccountId],
        _prime: Option<AccountId>,
    ) -> result::Result<(), DispatchError> {
        Ok(())
    }
}

/// Some sort of check on the origin is performed by this object.
pub trait EnsureOriginWithArg<OuterOrigin, Argument> {
    /// A return type.
    type Success;

    /// Perform the origin check.
    fn ensure_origin(o: OuterOrigin, a: &Argument) -> Result<Self::Success, BadOrigin> {
        Self::try_origin(o, a).map_err(|_| BadOrigin)
    }

    /// Perform the origin check, returning the origin value if unsuccessful. This allows chaining.
    fn try_origin(o: OuterOrigin, a: &Argument) -> Result<Self::Success, OuterOrigin>;

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin(a: &Argument) -> OuterOrigin;
}

impl<OuterOrigin: Clone, Argument: Clone> EnsureOriginWithArg<OuterOrigin, Argument> for () {
    type Success = u64;
    fn try_origin(_o: OuterOrigin, _a: &Argument) -> Result<Self::Success, OuterOrigin> {
        Ok(Default::default())
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin(_a: &Argument) -> OuterOrigin {
        todo!()
    }
}

pub trait BaseCallFilter<Call> {
    fn contains(&self, call: Call) -> bool;
}
