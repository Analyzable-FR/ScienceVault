//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

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

		assert_eq!(Reputations::<T>::get(beneficiary).unwrap().reputation, 99);
	}

	#[benchmark]
	fn punish() {
		let rewarder: T::AccountId = whitelisted_caller();
		Pallet::<T>::add_contribution(&rewarder);
		let beneficiary: T::AccountId = account("sub", 0, 0);
		Pallet::<T>::add_contribution(&beneficiary);

		#[extrinsic_call]
		punish(RawOrigin::Signed(rewarder), beneficiary.clone(), 10);

		assert_eq!(Reputations::<T>::get(beneficiary).unwrap().reputation, 0);
	}

	#[benchmark]
	fn slash() {
		let beneficiary: T::AccountId = account("sub", 0, 0);
		Pallet::<T>::add_contribution(&beneficiary);

		#[extrinsic_call]
		slash(RawOrigin::Root, beneficiary.clone(), 10);

		assert_eq!(Reputations::<T>::get(beneficiary).unwrap().reputation, 0);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
