#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use core::fmt::Debug;
use frame_support::{pallet_prelude::TypeInfo, traits::BuildGenesisConfig};
#[cfg(feature = "std")]
use sp_runtime::serde::{Deserialize, Serialize};

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
pub struct Reputation {
	contribution: u128, // number of elements added in the vault
	score: u128,        /* score computed at each reward as score +=
	                     * rewarded_amount*rewarder_reputation */
	reputation: u128, // reputation computed as (contribution*score*100)/best_reputation
}
impl Default for Reputation {
	fn default() -> Self {
		Reputation { contribution: 0, score: 0, reputation: 50 }
	}
}

pub trait Reward<AccountId> {
	fn add_contribution(account: &AccountId);
}

impl<AccountId> Reward<AccountId> for () {
	fn add_contribution(_account: &AccountId) {}
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
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T> {
		pub initial_reputation: u128,
		phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { initial_reputation: 1, phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			MaxRawReputation::<T>::put(&self.initial_reputation);
		}
	}

	// Map vault element id to element hash.
	#[pallet::storage]
	#[pallet::getter(fn reputation)]
	pub type Reputations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Reputation, OptionQuery>;

	#[pallet::storage]
	pub(super) type MaxRawReputation<T: Config> =
		StorageValue<Value = u128, QueryKind = ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountRewarded { account: T::AccountId, who: T::AccountId, amount: u128 },
		AccountPunished { account: T::AccountId, who: T::AccountId, amount: u128 },
	}

	#[pallet::error]
	pub enum Error<T> {
		AccountNotFound,
		CannotRewardItself,
		AccountNeedToParticipateBeforeVoting,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reward())]
		pub fn reward(origin: OriginFor<T>, account: T::AccountId, amount: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if who == account {
				return Err(Error::<T>::CannotRewardItself.into());
			}
			Reputations::<T>::try_mutate_exists(account.clone(), |reputation| {
				if let Some(rewarder_reputation) = Reputations::<T>::get(&who) {
					if let Some(ref mut reputation) = reputation {
						reputation.score += rewarder_reputation.reputation * amount;
						let raw_reputation = reputation.score * reputation.contribution;
						MaxRawReputation::<T>::put(core::cmp::max(
							raw_reputation,
							MaxRawReputation::<T>::get(),
						));
						reputation.reputation = raw_reputation / (MaxRawReputation::<T>::get() + 1);
						Self::deposit_event(Event::AccountRewarded { account, who, amount });
						Ok(())
					} else {
						return Err(Error::<T>::AccountNotFound.into());
					}
				} else {
					return Err(Error::<T>::AccountNeedToParticipateBeforeVoting.into());
				}
			})
		}
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reward())]
		pub fn punish(origin: OriginFor<T>, account: T::AccountId, amount: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if who == account {
				return Err(Error::<T>::CannotRewardItself.into());
			}
			Reputations::<T>::try_mutate_exists(account.clone(), |reputation| {
				if let Some(rewarder_reputation) = Reputations::<T>::get(&who) {
					if let Some(ref mut reputation) = reputation {
						reputation.score -= rewarder_reputation.reputation * amount;
						let raw_reputation = reputation.score * reputation.contribution;
						MaxRawReputation::<T>::put(core::cmp::max(
							raw_reputation,
							MaxRawReputation::<T>::get(),
						));
						reputation.reputation = raw_reputation / (MaxRawReputation::<T>::get() + 1);
						Self::deposit_event(Event::AccountPunished { account, who, amount });
						Ok(())
					} else {
						return Err(Error::<T>::AccountNotFound.into());
					}
				} else {
					return Err(Error::<T>::AccountNeedToParticipateBeforeVoting.into());
				}
			})
		}
	}
	impl<T: Config> Pallet<T> {
		pub fn add_contribution(account: &T::AccountId) {
			Reputations::<T>::mutate(account, |reputation| {
				if let Some(ref mut reputation) = reputation {
					reputation.contribution += 1;
				} else {
					*reputation = Some(Reputation { contribution: 1, score: 0, reputation: 0 });
				}
			});
		}
	}
}
