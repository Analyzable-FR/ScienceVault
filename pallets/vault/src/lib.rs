#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use pallet_timestamp::{self as timestamp};

use codec::{Codec, Decode, Encode, MaxEncodedLen};
use core::fmt::Debug;
use frame_support::{pallet_prelude::TypeInfo, BoundedVec};
#[cfg(feature = "std")]
use sp_runtime::serde::{Deserialize, Serialize};
use sp_runtime::traits::{AtLeast32BitUnsigned, ConstU32, One};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[cfg_attr(feature = "std", derive(Debug, Deserialize, Serialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Data<ElementId, AccountId, Moment> {
	element_id: ElementId,
	owner: AccountId,
	timestamp: Moment,
	sources: BoundedVec<BoundedVec<u8, ConstU32<100>>, ConstU32<100>>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		type ElementId: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ Copy
			+ Debug
			+ MaxEncodedLen;

		type ElementHash: Parameter + Member + Default + Copy + MaxEncodedLen;
	}

	// Map vault element id to element hash.
	#[pallet::storage]
	#[pallet::getter(fn vault_element)]
	pub type Vault<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::ElementHash,
		Data<T::ElementId, T::AccountId, T::Moment>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type NextVaultElementId<T: Config> =
		StorageValue<Value = T::ElementId, QueryKind = ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedToVault { element_id: T::ElementId, who: T::AccountId },
		DeletedFromVault { element_id: T::ElementId },
		SourceAdded { element_id: T::ElementId, source: BoundedVec<u8, ConstU32<100>> },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyInVault,
		CannotBeAddedToVault,
		InsufficientFund,
		NotOwned,
		NotFound,
		SourcesFull,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_element())]
		pub fn add_element(origin: OriginFor<T>, element: T::ElementHash) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if !Vault::<T>::contains_key(element) {
				let element_id = NextVaultElementId::<T>::get();
				let timestamp = timestamp::Pallet::<T>::get();
				<Vault<T>>::insert(
					element,
					Data { element_id, timestamp, sources: Default::default(), owner: who.clone() },
				);
				NextVaultElementId::<T>::put(element_id + T::ElementId::one());
				Self::deposit_event(Event::AddedToVault { element_id, who });
				Ok(())
			} else {
				return Err(Error::<T>::AlreadyInVault.into());
			}
		}
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_element())]
		pub fn set_element_source(
			origin: OriginFor<T>,
			element: T::ElementHash,
			source: BoundedVec<u8, ConstU32<100>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Vault::<T>::try_mutate_exists(element, |data| {
				if let Some(ref mut data) = data {
					if data.owner == who {
						let _ = data
							.sources
							.try_push(source.clone())
							.map_err(|_| Error::<T>::SourcesFull)?;
						Self::deposit_event(Event::SourceAdded {
							element_id: data.element_id,
							source,
						});
						Ok(())
					} else {
						return Err(Error::<T>::NotOwned.into());
					}
				} else {
					return Err(Error::<T>::NotFound.into());
				}
			})
		}
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_element())]
		pub fn delete_element(origin: OriginFor<T>, element: T::ElementHash) -> DispatchResult {
			let _ = ensure_root(origin)?;
			if let Some(data) = Vault::<T>::take(element) {
				Self::deposit_event(Event::DeletedFromVault { element_id: data.element_id });
				Ok(())
			} else {
				return Err(Error::<T>::NotFound.into());
			}
		}
	}
}
