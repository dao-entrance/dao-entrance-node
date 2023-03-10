use super::*;

impl<T: Config>
    CurrenciesHandler<
        DaoAssetId,
        DaoAssetMeta,
        DispatchError,
        T::AccountId,
        BalanceOf<T>,
        DispatchResult,
    > for Pallet<T>
{
    fn get_metadata(asset_id: DaoAssetId) -> result::Result<DaoAssetMeta, DispatchError> {
        let asset_info_opt = DaoAssetsInfo::<T>::get(asset_id);
        let asset_info = match asset_info_opt {
            Some(x) => x,
            _ => {
                if cfg!(any(feature = "std", feature = "runtime-benchmarks", test)) {
                    return Ok(DaoAssetMeta {
                        name: [].into(),
                        symbol: [].into(),
                        decimals: 12,
                    });
                } else {
                    return Err(Error::<T>::AssetNotExists)?;
                }
            }
        };
        Ok(asset_info.metadata)
    }

    fn do_create(
        user: T::AccountId,
        asset_id: DaoAssetId,
        metadata: DaoAssetMeta,
        amount: BalanceOf<T>,
        _is_swap_deposit: bool,
    ) -> DispatchResult {
        ensure!(
            !Self::is_exists_metadata(asset_id)
                && <T as pallet::Config>::MultiAsset::total_issuance(asset_id)
                    == BalanceOf::<T>::from(0u32),
            Error::<T>::AssetAlreadyExists
        );

        ensure!(
            !Self::is_asset_id_too_large(asset_id),
            Error::<T>::CurrencyIdTooLarge
        );

        #[cfg(test)]
        println!(
            "\n初始化 TOKEN 池 =>> Asset_id:{:?} ||| Free_amount: {:?}",
            asset_id, amount
        );

        <T as pallet::Config>::MultiAsset::deposit(asset_id, &user, amount)?;

        DaoAssetsInfo::<T>::insert(
            asset_id,
            DaoAssetInfo {
                owner: user.clone(),
                metadata,
            },
        );
        Self::deposit_event(Event::CreateAsset(user, asset_id, amount));

        Ok(())
    }
}
