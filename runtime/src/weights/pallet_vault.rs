
//! Autogenerated weights for `pallet_vault`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/science-vault
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_vault`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vault::WeightInfo for WeightInfo<T> {
	/// Storage: `Vault::Vault` (r:1 w:1)
	/// Proof: `Vault::Vault` (`max_values`: None, `max_size`: Some(10298), added: 12773, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Vault::NextVaultElementId` (r:1 w:1)
	/// Proof: `Vault::NextVaultElementId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vault::AccountElements` (r:1 w:1)
	/// Proof: `Vault::AccountElements` (`max_values`: None, `max_size`: Some(3200052), added: 3202527, mode: `MaxEncodedLen`)
	/// Storage: `Reward::Reputations` (r:1 w:1)
	/// Proof: `Reward::Reputations` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Reward::AccountQueue` (r:1 w:1)
	/// Proof: `Reward::AccountQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	fn add_element() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3200245`
		//  Estimated: `3203517`
		// Minimum execution time: 10_046_628_000 picoseconds.
		Weight::from_parts(10_317_658_000, 0)
			.saturating_add(Weight::from_parts(0, 3203517))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Vault::Vault` (r:1 w:1)
	/// Proof: `Vault::Vault` (`max_values`: None, `max_size`: Some(10298), added: 12773, mode: `MaxEncodedLen`)
	fn set_element_source() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10328`
		//  Estimated: `13763`
		// Minimum execution time: 28_871_000 picoseconds.
		Weight::from_parts(29_665_000, 0)
			.saturating_add(Weight::from_parts(0, 13763))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vault::Vault` (r:1 w:1)
	/// Proof: `Vault::Vault` (`max_values`: None, `max_size`: Some(10298), added: 12773, mode: `MaxEncodedLen`)
	fn delete_element() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `228`
		//  Estimated: `13763`
		// Minimum execution time: 9_991_000 picoseconds.
		Weight::from_parts(10_527_000, 0)
			.saturating_add(Weight::from_parts(0, 13763))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
