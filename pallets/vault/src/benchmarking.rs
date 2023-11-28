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
		let elements: BoundedVec<T::ElementHash, ConstU32<MAX_ELEMENTS>> =
			vec![T::ElementHash::default(); MAX_ELEMENTS as usize - 1].try_into().unwrap();
		AccountElements::<T>::insert(&caller, elements);
		assert!(AccountElements::<T>::get(&caller).unwrap().len() as u32 == MAX_ELEMENTS - 1);

		#[extrinsic_call]
		add_element(RawOrigin::Signed(caller.clone()), element_hash);

		assert!(Vault::<T>::get(element_hash).is_some());
		assert!(AccountElements::<T>::get(caller).unwrap().len() as u32 == MAX_ELEMENTS);
	}

	#[benchmark]
	fn set_element_source() {
		let caller: T::AccountId = whitelisted_caller();
		let element_hash = T::ElementHash::default();
		let _ = Pallet::<T>::add_element(RawOrigin::Signed(caller.clone()).into(), element_hash);
		assert!(Vault::<T>::get(element_hash).is_some());
		let element_source: BoundedVec<u8, ConstU32<MAX_SOURCE_LEN>> =
			vec![0u8; MAX_SOURCE_LEN as usize].try_into().unwrap();

		Vault::<T>::mutate(element_hash, |data| {
			if let Some(ref mut data) = data {
				for _i in 0..MAX_SOURCE_LEN - 1 {
					let _ = data.sources.try_push(element_source.clone());
				}
			}
		});
		assert_eq!(Vault::<T>::get(element_hash).unwrap().sources.len() as u32, MAX_SOURCES - 1);

		#[extrinsic_call]
		set_element_source(RawOrigin::Signed(caller), element_hash, element_source.clone());

		let sources: BoundedVec<BoundedVec<u8, ConstU32<MAX_SOURCE_LEN>>, ConstU32<MAX_SOURCES>> =
			vec![element_source; MAX_SOURCES as usize].try_into().unwrap();
		assert_eq!(Vault::<T>::get(element_hash).unwrap().sources, sources);
		assert_eq!(Vault::<T>::get(element_hash).unwrap().sources.len() as u32, MAX_SOURCES);
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
