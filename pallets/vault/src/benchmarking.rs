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
	fn add_element() {
		let caller: T::AccountId = whitelisted_caller();
		let element_hash = T::ElementHash::default();

		#[extrinsic_call]
		add_element(RawOrigin::Signed(caller), element_hash);

		assert!(Vault::<T>::get(element_hash).is_some());
	}

	#[benchmark]
	fn set_element_source() {
		let caller: T::AccountId = whitelisted_caller();
		let element_hash = T::ElementHash::default();
		let _ = Pallet::<T>::add_element(RawOrigin::Signed(caller.clone()).into(), element_hash);
		assert!(Vault::<T>::get(element_hash).is_some());
		let element_source: BoundedVec<u8, ConstU32<100>> = vec![0u8; 100].try_into().unwrap();

		#[extrinsic_call]
		set_element_source(RawOrigin::Signed(caller), element_hash, element_source.clone());

		let sources: BoundedVec<BoundedVec<u8, ConstU32<100>>, ConstU32<100>> =
			vec![element_source].try_into().unwrap();
		assert_eq!(Vault::<T>::get(element_hash).unwrap().sources, sources);
	}

	#[benchmark]
	fn delete_element() {
		let caller: T::AccountId = whitelisted_caller();
		let element_hash = T::ElementHash::default();
		let _ = Pallet::<T>::add_element(RawOrigin::Signed(caller.clone()).into(), element_hash);
		assert!(Vault::<T>::get(element_hash).is_some());

		#[extrinsic_call]
		delete_element(RawOrigin::Root, element_hash);

		assert!(Vault::<T>::get(element_hash).is_none());
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
