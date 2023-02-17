#![allow(unused_imports)]
#![cfg(test)]

use crate as daoent_project;
use crate::mock::*;
use daoent_assets;
use daoent_gov::MemmberData;
use daoent_primitives::types::AccountIdType;
use daoent_primitives::types::DaoAssetId;
use daoent_primitives::types::ProjectId;
use frame_support::{assert_noop, assert_ok, debug, log::debug};

pub const PROJECT_INDEX: ProjectId = 1;
pub fn create_asset() -> DaoAssetId {
    let dao_id = daoent_dao::Pallet::<Test>::next_dao_id();

    daoent_dao::Pallet::<Test>::create_dao(RuntimeOrigin::signed(ALICE), vec![1; 4], vec![1; 4])
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

    println!("创建项目");
    let proposal2 = RuntimeCall::DAOProject(daoent_project::Call::create_project {
        dao_id,
        name: "TestP".as_bytes().to_vec(),
        description: "TestPd".as_bytes().to_vec(),
        creator: ALICE,
    });

    assert_ok!(daoent_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(daoent_dao::Daos::<Test>::get(dao_id).unwrap().creator),
        dao_id,
        Box::new(proposal2)
    ));

    return dao_id;
}

pub fn project_join_reques() -> DaoAssetId {
    let dao_id = create_asset();
    assert!(daoent_project::Pallet::<Test>::project_join_request(
        RuntimeOrigin::signed(BOB),
        dao_id,
        PROJECT_INDEX,
        BOB,
    )
    .is_err());

    // 创建提案
    let proposal = RuntimeCall::DAOProject(daoent_project::Call::project_join_request {
        dao_id: dao_id,
        project_id: PROJECT_INDEX,
        who: BOB,
    });

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
    assert!(
        daoent_gov::Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), dao_id, 0u32)
            .is_err()
    );

    frame_system::Pallet::<Test>::set_block_number(20000);

    // 运行代码
    assert!(
        daoent_gov::Pallet::<Test>::run_proposal(RuntimeOrigin::signed(ALICE), dao_id, 0u32)
            .is_ok()
    );

    let ms = daoent_dao::ProjectMembers::<Test>::get(dao_id, PROJECT_INDEX);
    println!("项目成员 => {:?}", ms);
    assert!(ms.len() == 2);

    return dao_id;
}

#[test]
pub fn test_project_join_request() {
    new_test_run().execute_with(|| {
        project_join_reques();
    });
}

#[test]
pub fn test_task() {
    new_test_run().execute_with(|| {
        let dao_id = project_join_reques();

        // 项目没有资金应该报错
        assert!(daoent_project::Pallet::<Test>::create_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            "TestA".as_bytes().to_vec(),
            "TestA".as_bytes().to_vec(),
            10,
            1,
            Some(1),
            Some(vec![1]),
            Some(vec![]),
            10,
        )
        .is_err());

        // 为项目申请资金
        let proposal = RuntimeCall::DAOProject(daoent_project::Call::apply_project_funds {
            dao_id,
            project_id: 0,
            amount: 19,
        });

        assert_ok!(daoent_sudo::Pallet::<Test>::sudo(
            RuntimeOrigin::signed(daoent_dao::Daos::<Test>::get(dao_id).unwrap().creator),
            dao_id,
            Box::new(proposal)
        ));

        print_account(dao_id);

        // 创建任务
        daoent_project::Pallet::<Test>::create_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            "TestA".as_bytes().to_vec(),
            "TestA".as_bytes().to_vec(),
            10,
            1,
            Some(1),
            Some(vec![1]),
            Some(vec![]),
            10,
        )
        .unwrap();

        // 加入任务
        daoent_project::Pallet::<Test>::join_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 成为任务审核员
        daoent_project::Pallet::<Test>::be_task_review(
            RuntimeOrigin::signed(BOB),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 开始任务
        daoent_project::Pallet::<Test>::start_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 请求审查
        daoent_project::Pallet::<Test>::requset_review(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 请求审查
        daoent_project::Pallet::<Test>::make_review(
            RuntimeOrigin::signed(BOB),
            dao_id,
            PROJECT_INDEX,
            1,
            daoent_project::ReviewOpinion::YES,
            "通过".as_bytes().to_vec(),
        )
        .unwrap();

        // 请求完成
        daoent_project::Pallet::<Test>::task_done(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        print_account(dao_id);
    });
}

pub fn print_account(dao_id: DaoAssetId) {
    let project_dao = daoent_assets::Pallet::<Test>::get_balance(
        dao_id,
        daoent_dao::Pallet::<Test>::dao_project(dao_id, PROJECT_INDEX),
    )
    .unwrap();

    let dao = daoent_assets::Pallet::<Test>::get_balance(
        dao_id,
        daoent_dao::Pallet::<Test>::dao_asset(dao_id),
    )
    .unwrap();

    // 判断现在项目的
    print!(
        "dao_id => {:?} project_dao => {:?} dao => {:?}\n",
        dao_id, project_dao, dao
    );

    let alice_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
    let bob_dao = daoent_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
    print!(
        "alice_dao => {:?} ||| bob_dao => {:?} \n",
        alice_dao, bob_dao
    );
}
