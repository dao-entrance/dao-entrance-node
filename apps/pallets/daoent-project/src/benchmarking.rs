use super::*;
use crate::{Call as SudoCall, Config, Pallet as Sudo};
use dao::Call as DaoCall;
use frame_benchmarking::{
	account, benchmarks, benchmarks_instance, benchmarks_instance_pallet,
	impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin as SystemOrigin;
use primitives::AccountIdConversion;
use sp_std::vec;

fn get_alice<T: Config>() -> T::AccountId {
	account("alice", 1, 1)
}

fn get_dao_account<T: Config>(second_id: T::AssetId) -> T::AccountId {
	second_id.into_account()
}

fn create_dao<T: Config>() -> (T::DaoId, T::AssetId) {
	let second_id = T::AssetId::default();
	let dao_id = T::DaoId::default();
	assert!(dao::Pallet::<T>::create_dao(
		SystemOrigin::Signed(get_alice::<T>()).into(),
		second_id,
		vec![1; 4],
	)
	.is_ok());
	(dao_id, second_id)
}

fn get_proposal<T: Config>(dao_id: T::DaoId) -> <T as dao::Config>::Call {
	let proposal: <T as dao::Config>::Call =
		DaoCall::<T>::dao_remark { dao_id, remark: vec![1; 20] }.into();
	proposal
}

benchmarks! {
	sudo {
		let alice = get_alice::<T>();
		let (dao_id, second_id) = create_dao::<T>();
		let proposal = get_proposal::<T>(dao_id);
	}:_(SystemOrigin::Signed(alice), dao_id, Box::new(proposal))

	set_sudo_account {
		let alice = get_alice::<T>();
		let (dao_id, second_id) = create_dao::<T>();
		let dao_account = get_dao_account::<T>(second_id);
	}:_(SystemOrigin::Signed(alice), dao_id, dao_account)

	close_sudo {
		let alice = get_alice::<T>();
		let (dao_id, second_id) = create_dao::<T>();
	}:_(SystemOrigin::Signed(alice), dao_id)
}
