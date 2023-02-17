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
        Some(daoent_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        }),
        10000,
        99,
    )
    .unwrap();

    let proposal = RuntimeCall::DAOAsset(daoent_assets::Call::set_existenial_deposit {
        asset_id: dao_id,
        existenial_deposit: 1,
    });

    assert_ok!(daoent_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(daoent_dao::Daos::<Test>::get(dao_id).unwrap().creator),
        dao_id,
        Box::new(proposal)
    ));

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

#[test]
pub fn test_asset_join() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        daoent_assets::Pallet::<Test>::join_request(RuntimeOrigin::signed(BOB), dao_id, 100, 100)
            .unwrap();

        let bob_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
        let bob = daoent_assets::Pallet::<Test>::get_balance(0, BOB).unwrap();

        let dao = daoent_assets::Pallet::<Test>::dao_asset(dao_id);
        let dao_b = daoent_assets::Pallet::<Test>::get_balance(0, dao).unwrap();

        println!(
            "join_request >>>>>> bob_dao token {:?} ||| bob token {:?} ||| dao_b {:?}",
            bob_dao, bob, dao_b
        );
        assert_eq!(bob_dao, 100);
        assert_eq!(bob, 9900);
        assert_eq!(dao_b, 10100);
    })
}
