#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for daos_square.
pub trait WeightInfo {
    fn propose() -> Weight;
    fn second() -> Weight;
    fn open_referendum() -> Weight;
    fn vote_for_referendum() -> Weight;
    fn cancel_vote() -> Weight;
    fn run_proposal() -> Weight;
    fn unlock() -> Weight;
    fn set_min_vote_weight_for_every_call() -> Weight;
    fn set_max_public_props() -> Weight;
    fn set_launch_period() -> Weight;
    fn set_minimum_deposit() -> Weight;
    fn set_voting_period() -> Weight;
    fn set_rerserve_period() -> Weight;
    fn set_runment_period() -> Weight;
}

/// Weights for daos_square using the Substrate node and recommended hardware.
pub struct DaosWeight<T>(PhantomData<T>);
        impl<T: frame_system::Config> WeightInfo for DaosWeight<T> {
            // Storage: System Account (r:1 w:1)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinimumDeposit (r:1 w:0)
            // Storage: DaoSquare PublicPropCount (r:1 w:1)
            // Storage: DaoSquare PublicProps (r:1 w:1)
            // Storage: DaoSquare MaxPublicProps (r:1 w:0)
            // Storage: DaoSquare DepositOf (r:0 w:1)
        fn propose() -> Weight {
            Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare DepositOf (r:1 w:1)
            // Storage: System Account (r:1 w:1)
            // Storage: DaoSquare ReservePeriod (r:1 w:0)
            // Storage: DaoSquare ReserveOf (r:1 w:1)
        fn second() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare LaunchTag (r:1 w:0)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare LaunchPeriod (r:1 w:0)
            // Storage: DaoSquare PublicProps (r:1 w:1)
            // Storage: DaoSquare DepositOf (r:1 w:1)
            // Storage: DaoSquare VotingPeriod (r:1 w:0)
            // Storage: DaoSquare EnactmentPeriod (r:1 w:0)
            // Storage: DaoSquare ReferendumCount (r:1 w:1)
            // Storage: DaoSquare ReferendumInfoOf (r:0 w:1)
        fn open_referendum() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn vote_for_referendum() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn cancel_vote() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: DaoSquare MinVoteWeightOf (r:1 w:0)
            // Storage: CreateDao Daos (r:1 w:0)
        fn run_proposal() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReserveOf (r:1 w:1)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn unlock() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinVoteWeightOf (r:0 w:1)
        fn set_min_vote_weight_for_every_call() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MaxPublicProps (r:0 w:1)
        fn set_max_public_props() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare LaunchPeriod (r:0 w:1)
        fn set_launch_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinimumDeposit (r:0 w:1)
        fn set_minimum_deposit() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare VotingPeriod (r:0 w:1)
        fn set_voting_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare ReservePeriod (r:0 w:1)
        fn set_rerserve_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare EnactmentPeriod (r:0 w:1)
        fn set_runment_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
    }

    // For backwards compatibility and tests
    impl WeightInfo for () {
            // Storage: System Account (r:1 w:1)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinimumDeposit (r:1 w:0)
            // Storage: DaoSquare PublicPropCount (r:1 w:1)
            // Storage: DaoSquare PublicProps (r:1 w:1)
            // Storage: DaoSquare MaxPublicProps (r:1 w:0)
            // Storage: DaoSquare DepositOf (r:0 w:1)
        fn propose() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare DepositOf (r:1 w:1)
            // Storage: System Account (r:1 w:1)
            // Storage: DaoSquare ReservePeriod (r:1 w:0)
            // Storage: DaoSquare ReserveOf (r:1 w:1)
        fn second() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare LaunchTag (r:1 w:0)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare LaunchPeriod (r:1 w:0)
            // Storage: DaoSquare PublicProps (r:1 w:1)
            // Storage: DaoSquare DepositOf (r:1 w:1)
            // Storage: DaoSquare VotingPeriod (r:1 w:0)
            // Storage: DaoSquare EnactmentPeriod (r:1 w:0)
            // Storage: DaoSquare ReferendumCount (r:1 w:1)
            // Storage: DaoSquare ReferendumInfoOf (r:0 w:1)
        fn open_referendum() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn vote_for_referendum() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn cancel_vote() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReferendumInfoOf (r:1 w:1)
            // Storage: DaoSquare MinVoteWeightOf (r:1 w:0)
            // Storage: CreateDao Daos (r:1 w:0)
        fn run_proposal() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: DaoSquare ReserveOf (r:1 w:1)
            // Storage: DaoSquare VotesOf (r:1 w:1)
        fn unlock() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinVoteWeightOf (r:0 w:1)
        fn set_min_vote_weight_for_every_call() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MaxPublicProps (r:0 w:1)
        fn set_max_public_props() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare LaunchPeriod (r:0 w:1)
        fn set_launch_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare MinimumDeposit (r:0 w:1)
        fn set_minimum_deposit() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare VotingPeriod (r:0 w:1)
        fn set_voting_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare ReservePeriod (r:0 w:1)
        fn set_rerserve_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
            // Storage: CreateDao Daos (r:1 w:0)
            // Storage: DaoSquare EnactmentPeriod (r:0 w:1)
        fn set_runment_period() -> Weight {
                Weight::from_ref_time(20_0000_0000)
        }
    }