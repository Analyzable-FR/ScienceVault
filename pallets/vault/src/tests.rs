use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, bounded_vec, pallet_prelude::ConstU32, BoundedVec};
use sp_runtime::traits::BadOrigin;

#[test]
fn insert_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		let elements: BoundedVec<u8, ConstU32<100_000>> = bounded_vec![42];
		assert_eq!(VaultModule::account_elements(1).unwrap(), elements);
		System::assert_last_event(Event::AddedToVault { element_id: 0, who: 1 }.into());

		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 44));
		assert_eq!(VaultModule::vault_element(44).unwrap().element_id, 1);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		System::assert_last_event(Event::AddedToVault { element_id: 1, who: 1 }.into());
	});
}

#[test]
fn noop_insert_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		let elements: BoundedVec<u8, ConstU32<100_000>> = bounded_vec![42];
		assert_eq!(VaultModule::account_elements(1).unwrap(), elements);
		System::assert_last_event(Event::AddedToVault { element_id: 0, who: 1 }.into());

		assert_noop!(
			VaultModule::add_element(RuntimeOrigin::signed(1), 42),
			Error::<Test>::AlreadyInVault
		);
		assert_eq!(VaultModule::account_elements(1).unwrap(), elements);
	});
}

#[test]
fn insert_source() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		let mut sources: BoundedVec<BoundedVec<u8, ConstU32<100>>, ConstU32<100>> =
			BoundedVec::default();
		assert_eq!(VaultModule::vault_element(42).unwrap().sources, sources.clone());
		let source: BoundedVec<u8, ConstU32<100>> = bounded_vec![0u8; 32];
		assert_ok!(VaultModule::set_element_source(RuntimeOrigin::signed(1), 42, source.clone()));
		let _ = sources.try_push(source.clone());
		assert_eq!(VaultModule::vault_element(42).unwrap().sources, sources);
		System::assert_last_event(Event::SourceAdded { element_id: 0, source }.into());
	});
}

#[test]
fn noop_insert_source() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let source: BoundedVec<u8, ConstU32<100>> = bounded_vec![0u8; 32];
		assert_noop!(
			VaultModule::set_element_source(RuntimeOrigin::signed(1), 42, source.clone()),
			Error::<Test>::NotFound
		);

		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		let sources: BoundedVec<BoundedVec<u8, ConstU32<100>>, ConstU32<100>> =
			BoundedVec::default();
		assert_eq!(VaultModule::vault_element(42).unwrap().sources, sources.clone());
		let source: BoundedVec<u8, ConstU32<100>> = bounded_vec![0u8; 32];
		assert_noop!(
			VaultModule::set_element_source(RuntimeOrigin::signed(0), 42, source.clone()),
			Error::<Test>::NotOwned
		);

		for _ in 0..100 {
			assert_ok!(VaultModule::set_element_source(
				RuntimeOrigin::signed(1),
				42,
				source.clone()
			));
		}
		assert_noop!(
			VaultModule::set_element_source(RuntimeOrigin::signed(1), 42, source.clone()),
			Error::<Test>::SourcesFull
		);
	});
}

#[test]
fn delete_element() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		assert_ok!(VaultModule::delete_element(RuntimeOrigin::root(), 42));
		System::assert_last_event(Event::DeletedFromVault { element_id: 0 }.into());
	});
}

#[test]
fn noop_delete_element() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		assert_noop!(
			VaultModule::delete_element(RuntimeOrigin::root(), 42),
			Error::<Test>::NotFound
		);
		assert_ok!(VaultModule::add_element(RuntimeOrigin::signed(1), 42));
		assert_eq!(VaultModule::vault_element(42).unwrap().element_id, 0);
		assert_eq!(VaultModule::vault_element(42).unwrap().owner, 1);
		assert_noop!(VaultModule::delete_element(RuntimeOrigin::signed(1), 42), BadOrigin);
	});
}
