use super::*;

use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Weight, traits::OnIdle};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};

#[cfg(test)]
use crate::Pallet as Reward;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn reward() {
        let rewarder: T::AccountId = whitelisted_caller();
        Pallet::<T>::add_contribution(&rewarder);
        let beneficiary: T::AccountId = account("sub", 0, 0);
        Pallet::<T>::add_contribution(&beneficiary);

        #[extrinsic_call]
        reward(RawOrigin::Signed(rewarder), beneficiary.clone(), 10);

        assert!(Reputations::<T>::get(beneficiary).unwrap().score > 0);
    }

    #[benchmark]
    fn punish() {
        let rewarder: T::AccountId = whitelisted_caller();
        Pallet::<T>::add_contribution(&rewarder);
        let beneficiary: T::AccountId = account("sub", 0, 0);
        Pallet::<T>::add_contribution(&beneficiary);

        #[extrinsic_call]
        punish(RawOrigin::Signed(rewarder), beneficiary.clone(), 10);

        assert_eq!(Reputations::<T>::get(beneficiary).unwrap().score, 0);
    }

    #[benchmark]
    fn slash() {
        let beneficiary: T::AccountId = account("sub", 0, 0);
        Pallet::<T>::add_contribution(&beneficiary);

        #[extrinsic_call]
        slash(RawOrigin::Root, beneficiary.clone(), 10);

        assert_eq!(Reputations::<T>::get(beneficiary).unwrap().score, 0);
    }

    #[benchmark]
    fn do_process_evaluation_queue() {
        assert!(EvaluationQueue::<T>::get().is_empty());

        #[block]
        {
            Pallet::<T>::process_evaluation_queue(Weight::MAX);
        }

        assert!(EvaluationQueue::<T>::get().is_empty());
    }

    #[benchmark]
    fn process_evaluation_queue(i: Linear<1, { 1024 }>) {
        for j in 0..i {
            let account: T::AccountId = account("sub", j, j);
            EvaluationQueue::<T>::mutate(|queue| {
                let _ = queue.try_push(account);
            });
        }
        assert_eq!(EvaluationQueue::<T>::get().len(), i as usize);

        #[block]
        {
            Pallet::<T>::process_evaluation_queue(Weight::MAX);
        }

        assert_eq!(EvaluationQueue::<T>::get().len(), 0);
    }

    #[benchmark]
    fn on_idle_noop() {
        assert_eq!(EvaluationQueue::<T>::get().len(), 0);

        #[block]
        {
            Pallet::<T>::on_idle(BlockNumberFor::<T>::zero(), Weight::MAX);
        }

        assert_eq!(EvaluationQueue::<T>::get().len(), 0);
    }

    impl_benchmark_test_suite!(Reward, crate::mock::new_test_ext(), crate::mock::Test);
}
