#![allow(unused_imports)]
#![cfg(test)]

use crate as daoent_guild;
use crate::mock::*;
use daoent_assets;
use daoent_gov::MemmberData;
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
pub fn test_guild_join_request() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();
        assert!(daoent_guild::Pallet::<Test>::guild_join_request(
            RuntimeOrigin::signed(BOB),
            dao_id,
            0,
            BOB,
        )
        .is_err());

        // 创建提案
        let proposal = RuntimeCall::DAOGuild(daoent_guild::Call::guild_join_request {
            dao_id: dao_id,
            guild_id: 0,
            who: BOB,
        });

        // 未加入社区尝试提案
        assert!(daoent_gov::Pallet::<Test>::create_propose(
            RuntimeOrigin::signed(BOB),
            dao_id,
            MemmberData::GLOBAL,
            Box::new(proposal.clone()),
            0u64
        )
        .is_err());

        // 加入社区
        daoent_assets::Pallet::<Test>::join_request(RuntimeOrigin::signed(BOB), dao_id, 100, 100)
            .unwrap();

        // 加入社区后尝试提案
        assert_ok!(daoent_gov::Pallet::<Test>::create_propose(
            RuntimeOrigin::signed(BOB),
            dao_id,
            MemmberData::GLOBAL,
            Box::new(proposal),
            0u64
        ));

        // 开始投票
        frame_system::Pallet::<Test>::set_block_number(10000);
        assert_ok!(daoent_gov::Pallet::<Test>::start_referendum(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32
        ));

        // 投票
        assert_ok!(daoent_gov::Pallet::<Test>::vote_for_referendum(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32,
            Vote(100000),
            (),
            daoent_gov::Opinion::YES,
        ));

        //
        assert!(daoent_gov::Pallet::<Test>::run_proposal(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32
        )
        .is_err());

        frame_system::Pallet::<Test>::set_block_number(20000);

        // 运行代码
        assert!(daoent_gov::Pallet::<Test>::run_proposal(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32
        )
        .is_ok());

        let ms = daoent_dao::GuildMembers::<Test>::get(dao_id, 0);
        assert!(ms.len() == 2);
    });
}
