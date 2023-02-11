#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use codec::{Codec, Decode, Encode, MaxEncodedLen};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    pallet_prelude::*,
    traits::{
        Currency as PalletCurrency, ExistenceRequirement, Get,
        LockableCurrency as PalletLockableCurrency, ReservableCurrency as PalletReservableCurrency,
        WithdrawReasons,
    },
    PalletId,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use orml_traits::{
    arithmetic::{Signed, SimpleArithmetic},
    BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency,
    BasicReservableCurrency, LockIdentifier, MultiCurrency, MultiCurrencyExtended,
    MultiLockableCurrency, MultiReservableCurrency,
};
use scale_info::TypeInfo;

use daoent_dao::{self as dao};
use daoent_primitives::types::DaoAssetId;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{CheckedSub, MaybeSerializeDeserialize, StaticLookup, Zero},
    RuntimeDebug,
};
use sp_std::vec::Vec;
use sp_std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    marker, result,
};

pub mod asset_adaper_in_pallet;
pub mod asset_in_pallet;
pub mod impl_currency_handler;
pub mod impl_multi_currency;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

pub mod traits;
use traits::CurrenciesHandler;

pub const NATIVE_ASSET_ID: DaoAssetId = 0;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoAssetMeta {
    /// project name
    /// token 名
    pub name: Vec<u8>,
    /// The ticker symbol for this asset.
    /// 通证符号
    pub symbol: Vec<u8>,
    /// The number of decimals this asset uses to represent one unit.
    /// 资产小数点位数
    pub decimals: u8,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoAssetInfo<AccountId, DaoAssetMeta> {
    pub owner: AccountId,
    pub metadata: Option<DaoAssetMeta>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_runtime::traits::AccountIdConversion;

    pub(crate) type BalanceOf<T> = <<T as Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    pub(crate) type AmountOf<T> = <<T as Config>::MultiAsset as MultiCurrencyExtended<
        <T as frame_system::Config>::AccountId,
    >>::Amount;

    #[pallet::config]
    pub trait Config: frame_system::Config + dao::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// pallet id
        /// 模块id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// dao asset
        /// 组织内部资产
        type MultiAsset: MultiCurrency<Self::AccountId, CurrencyId = DaoAssetId>
            + MultiCurrencyExtended<Self::AccountId>
            + MultiLockableCurrency<Self::AccountId>
            + MultiReservableCurrency<Self::AccountId>;

        /// dao naive token
        /// 链上原生通证
        type NativeAsset: BasicCurrencyExtended<
                Self::AccountId,
                Balance = BalanceOf<Self>,
                Amount = AmountOf<Self>,
            > + BasicLockableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
            + BasicReservableCurrency<Self::AccountId, Balance = BalanceOf<Self>>;

        /// Weight information for extrinsics in this pallet.
        /// 链上 weight
        type WeightInfo: WeightInfo;

        /// Maximum assets that can be created
        /// 最多可创建组织数量
        type MaxCreatableId: Get<DaoAssetId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        AmountIntoBalanceFailed,
        BalanceTooLow,
        AssetAlreadyExists,
        AssetNotExists,
        MetadataNotChange,
        MetadataErr,
        NotOwner,
        ShouldNotChangeDecimals,
        MetadataNotExists,
        NativeCurrency,
        CurrencyIdTooLarge,
        CurrencyIdTooLow,
        DaoExists,
        CexTransferClosed,
        AssetIdExisted,
        BadLocation,
        MultiLocationExisted,
        CrossTransferNotOpen,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Currency transfer success. [asset_id, from, to, amount]
        Transferred(DaoAssetId, T::AccountId, T::AccountId, BalanceOf<T>),
        /// Update balance success. [asset_id, who, amount]
        BalanceUpdated(DaoAssetId, T::AccountId, AmountOf<T>),
        /// Deposit success. [asset_id, who, amount]
        Deposited(DaoAssetId, T::AccountId, BalanceOf<T>),
        /// Withdraw success. [asset_id, who, amount]
        Withdrawn(DaoAssetId, T::AccountId, BalanceOf<T>),
        CreateAsset(T::AccountId, DaoAssetId, BalanceOf<T>),
        SetMetadata(T::AccountId, DaoAssetId, DaoAssetMeta),
        Burn(T::AccountId, DaoAssetId, BalanceOf<T>),
        SetWeightRateMultiple {
            asset_id: DaoAssetId,
            multiple: u128,
        },
        SetExistenialDepposit {
            asset_id: DaoAssetId,
            existenial_deposit: BalanceOf<T>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn asset_info)]
    pub type DaoAssetsInfo<T: Config> =
        StorageMap<_, Blake2_128Concat, DaoAssetId, DaoAssetInfo<T::AccountId, DaoAssetMeta>>;

    #[pallet::storage]
    #[pallet::getter(fn users_number)]
    pub type UsersNumber<T: Config> = StorageMap<_, Identity, DaoAssetId, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn existenial_deposits)]
    pub type ExistentialDeposits<T: Config> =
        StorageMap<_, Identity, DaoAssetId, BalanceOf<T>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create dao asset.
        /// 创建 DAO 资产
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_asset())]
        pub fn create_asset(
            origin: OriginFor<T>,
            asset_id: DaoAssetId,
            amount: BalanceOf<T>,
            metadata: Option<DaoAssetMeta>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                daoent_dao::Daos::<T>::contains_key(asset_id),
                Error::<T>::AssetNotExists
            );
            let user = ensure_signed(origin)?;

            Self::do_create(user.clone(), asset_id, metadata, amount, false)?;

            Ok(().into())
        }

        #[pallet::call_index(003)]
        #[pallet::weight(1500_000_000)]
        pub fn set_existenial_deposit(
            origin: OriginFor<T>,
            asset_id: DaoAssetId,
            existenial_deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(who, asset_id)?;

            ExistentialDeposits::<T>::insert(asset_id, existenial_deposit);
            Self::deposit_event(Event::SetExistenialDepposit {
                asset_id,
                existenial_deposit,
            });

            Ok(().into())
        }

        /// You should have created the asset first.
        /// 设置资产元数据
        #[pallet::call_index(004)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_metadata())]
        pub fn set_metadata(
            origin: OriginFor<T>,
            asset_id: DaoAssetId,
            metadata: DaoAssetMeta,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(user.clone(), asset_id)?;

            ensure!(
                daoent_dao::Daos::<T>::contains_key(asset_id),
                Error::<T>::AssetNotExists
            );

            ensure!(
                metadata.name.len() > 2
                    && metadata.symbol.len() > 1
                    && metadata.decimals > 0u8
                    && metadata.decimals < 19,
                Error::<T>::MetadataErr
            );

            let mut asset_info =
                DaoAssetsInfo::<T>::get(asset_id).ok_or(Error::<T>::AssetNotExists)?;

            match asset_info.metadata.as_ref() {
                Some(x) => {
                    ensure!(x != &metadata, Error::<T>::MetadataNotChange);
                    ensure!(
                        x.decimals == metadata.decimals,
                        Error::<T>::ShouldNotChangeDecimals
                    );
                }
                None => {}
            }

            asset_info.metadata = Some(metadata.clone());

            DaoAssetsInfo::<T>::insert(asset_id, asset_info);
            Self::deposit_event(Event::SetMetadata(user, asset_id, metadata));

            Ok(().into())
        }

        /// Users destroy their own assets.
        /// 销毁资产
        #[pallet::call_index(005)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            asset_id: DaoAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                daoent_dao::Daos::<T>::contains_key(asset_id),
                Error::<T>::AssetNotExists
            );
            let user = ensure_signed(origin)?;
            // daoent_dao::Pallet::<T>::ensrue_dao_root(user.clone(), asset_id)?;

            ensure!(
                Self::is_exists_metadata(asset_id),
                Error::<T>::MetadataNotExists
            );

            <T as pallet::Config>::MultiAsset::withdraw(asset_id, &user, amount)?;
            Self::deposit_event(Event::Burn(user, asset_id, amount));
            Ok(().into())
        }

        /// Transfer some balance to another account under `asset_id`.
        /// 转移资产
        #[pallet::call_index(006)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            asset_id: DaoAssetId,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(dest)?;
            ensure!(
                daoent_dao::Daos::<T>::contains_key(asset_id),
                Error::<T>::AssetNotExists
            );

            match daoent_dao::Daos::<T>::get(asset_id) {
                Some(dao) => {
                    let _dao_account = dao.dao_account_id;
                    // let fee: Fee<VcBalanceOf<T>, Permill> = pallet_vc::Pallet::<T>::fees(dao_id);
                    // match fee {
                    // 			Fee::Permill(x) => {
                    // 				let real_fee = x * amount;
                    // 				<T as pallet::Config>::MultiCurrency::transfer(asset_id, &from, &dao_account, real_fee)?;
                    // 				<T as pallet::Config>::MultiCurrency::reserve(asset_id, &dao_account, real_fee);
                    // 			}
                    // 			Fee::Amount(x) => {
                    // 				<T as pallet_vc::Config>::MultiCurrency::transfer(
                    // 					T::USDCurrencyId::get(),
                    // 					&from,
                    // 					&dao_account,
                    // 					x,
                    // 				)?;
                    // 				<T as pallet_vc::Config>::MultiCurrency::reserve(T::USDCurrencyId::get(), &dao_account, x);
                    // 			}
                    // }
                }
                _ => {}
            };

            ensure!(
                Self::is_exists_metadata(asset_id),
                Error::<T>::MetadataNotExists
            );

            <Self as MultiCurrency<T::AccountId>>::transfer(asset_id, &from, &to, amount)?;
            Ok(().into())
        }

        #[pallet::call_index(007)]
        #[pallet::weight(1500_000_000)]
        pub fn join_request(
            origin: OriginFor<T>,
            asset_id: DaoAssetId,
            existenial_deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            daoent_dao::Pallet::<T>::ensrue_dao_root(who, asset_id)?;

            ExistentialDeposits::<T>::insert(asset_id, existenial_deposit);
            Self::deposit_event(Event::SetExistenialDepposit {
                asset_id,
                existenial_deposit,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取DAO账户
        pub fn account_id(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(dao_id)
        }

        /// 获取DAO账户
        pub fn get_balance(
            asset_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<BalanceOf<T>, DispatchError> {
            ensure!(
                Self::is_exists_metadata(asset_id),
                Error::<T>::MetadataNotExists
            );

            let balance = <Self as MultiCurrency<T::AccountId>>::total_balance(asset_id, &who);

            Ok(balance)
        }
    }
}

impl<T: Config> Pallet<T> {
    fn is_exists_metadata(asset_id: DaoAssetId) -> bool {
        if asset_id == NATIVE_ASSET_ID {
            return true;
        }
        match DaoAssetsInfo::<T>::get(asset_id).as_ref() {
            Some(x) => {
                if x.metadata.is_some() {
                    return true;
                }
            }
            None => {}
        }
        false
    }

    pub fn is_owner(asset_id: DaoAssetId, who: &T::AccountId) -> bool {
        if let Some(info) = DaoAssetsInfo::<T>::get(asset_id).as_ref() {
            return &info.owner == who;
        }
        false
    }

    fn is_asset_id_too_large(asset_id: DaoAssetId) -> bool {
        if asset_id >= T::MaxCreatableId::get() {
            return true;
        }
        false
    }
}
