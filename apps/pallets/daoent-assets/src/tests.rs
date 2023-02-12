#![allow(unused_imports)]
#![cfg(test)]

use crate as daoent_assets;
use crate::mock::*;
use daoent_primitives::types::AccountIdType;
use daoent_primitives::types::DaoAssetId;
use frame_support::{assert_noop, assert_ok, debug, log::debug};

pub fn create_asset() -> DaoAssetId {
    let dao_id = daoent_dao::Pallet::<Test>::next_dao_id();
    let asset = UnionId::FungToken(dao_id);

    daoent_dao::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        asset,
        vec![1; 4],
        vec![1; 4],
    )
    .unwrap();

    daoent_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        dao_id,
        10000,
        Some(daoent_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        }),
    )
    .unwrap();

    return dao_id;
}

#[test]
pub fn test_create_asset() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();
        let asset = UnionId::FungToken(dao_id);

        let dao_account_id: AccountId = asset.into_account();
        let dao_account = pallet_balances::Account::<Test>::get(dao_account_id);

        let alice_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();

        println!(
            "资产账户 {:?} ||| alice_dao token {:?} \n",
            dao_account, alice_dao,
        );

        assert_eq!(alice_dao, 10000);
    })
}

#[test]
pub fn test_asset_trans() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        daoent_assets::Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), BOB, dao_id, 1)
            .unwrap();

        let alice_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
        let bob_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
        println!(
            "\nalice_dao token {:?} ||| bob_dao token {:?}\n",
            alice_dao, bob_dao
        );

        assert_eq!(alice_dao, 9999);
        assert_eq!(bob_dao, 1);
    })
}

#[test]
pub fn test_asset_burn() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        daoent_assets::Pallet::<Test>::burn(RuntimeOrigin::signed(ALICE), dao_id, 1).unwrap();

        let alice_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
        println!("\nalice_dao token {:?}", alice_dao,);

        assert_eq!(alice_dao, 9999);
    })
}
