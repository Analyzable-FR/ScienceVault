#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::TypeInfo,
	traits::{BuildGenesisConfig, Currency},
};
#[cfg(feature = "std")]
use sp_runtime::serde::{Deserialize, Serialize};
use sp_runtime::{
	traits::{BlockNumberProvider, One, Zero},
	Perbill, Saturating,
};
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
pub struct Reputation<BlockNumber> {
	contribution: u128, // number of elements added in the vault
	score: u128,        // score computed at each reward
	reputation: Perbill,
	last_evaluation: BlockNumber,
}
impl<BlockNumberFor: Zero> Default for Reputation<BlockNumberFor> {
	fn default() -> Self {
		Reputation {
			contribution: 0,
			score: 0,
			reputation: Perbill::zero(),
			last_evaluation: BlockNumberFor::zero(),
		}
	}
}

pub trait Reward<AccountId> {
	fn add_contribution(account: &AccountId);
}

impl<AccountId> Reward<AccountId> for () {
	fn add_contribution(_account: &AccountId) {}
}

pub const MAX_USERS: u32 = 100_000;

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
		type Currency: Currency<Self::AccountId>;
		#[pallet::constant]
		type ReevaluationPeriod: Get<u32>;
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
			MaxRawReputation::<T>::put(self.initial_reputation);
		}
	}

	// Map vault element id to element hash.
	#[pallet::storage]
	#[pallet::getter(fn reputation)]
	pub type Reputations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Reputation<BlockNumberFor<T>>, OptionQuery>;

	#[pallet::storage]
	pub(super) type MaxRawReputation<T: Config> =
		StorageValue<Value = u128, QueryKind = ValueQuery>; //TODO reeavaluate

	#[pallet::storage]
	pub(super) type AccountQueue<T: Config> =
		StorageValue<Value = BoundedVec<T::AccountId, ConstU32<MAX_USERS>>, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type EvaluationQueue<T: Config> =
		StorageValue<Value = BoundedVec<T::AccountId, ConstU32<MAX_USERS>>, QueryKind = ValueQuery>;

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
			let amount = core::cmp::min(amount, 100);
			Reputations::<T>::try_mutate_exists(account.clone(), |reputation| {
				if let Some(rewarder_reputation) = Reputations::<T>::get(&who) {
					if let Some(ref mut reputation) = reputation {
						let rewarder_reputation = Self::compute_reputation(rewarder_reputation);
						reputation.score =
							reputation.score.saturating_add(rewarder_reputation * amount);
						let raw_reputation = reputation.score * reputation.contribution;
						MaxRawReputation::<T>::put(core::cmp::max(
							raw_reputation,
							MaxRawReputation::<T>::get(),
						));
						reputation.reputation = Self::compute_reputation(reputation.clone());
						reputation.last_evaluation =
							frame_system::Pallet::<T>::current_block_number();
						Self::deposit_event(Event::AccountRewarded { account, who, amount });
						Ok(())
					} else {
						Err(Error::<T>::AccountNotFound.into())
					}
				} else {
					Err(Error::<T>::AccountNeedToParticipateBeforeVoting.into())
				}
			})
		}
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::punish())]
		pub fn punish(origin: OriginFor<T>, account: T::AccountId, amount: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if who == account {
				return Err(Error::<T>::CannotRewardItself.into());
			}
			let amount = core::cmp::min(amount, 100);
			Reputations::<T>::try_mutate_exists(account.clone(), |reputation| {
				if let Some(rewarder_reputation) = Reputations::<T>::get(&who) {
					if let Some(ref mut reputation) = reputation {
						let rewarder_reputation = Self::compute_reputation(rewarder_reputation);
						reputation.score =
							reputation.score.saturating_sub(rewarder_reputation * amount);
						reputation.reputation = Self::compute_reputation(reputation.clone());
						reputation.last_evaluation =
							frame_system::Pallet::<T>::current_block_number();
						Self::deposit_event(Event::AccountPunished { account, who, amount });
						Ok(())
					} else {
						Err(Error::<T>::AccountNotFound.into())
					}
				} else {
					Err(Error::<T>::AccountNeedToParticipateBeforeVoting.into())
				}
			})
		}
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::slash())]
		pub fn slash(origin: OriginFor<T>, account: T::AccountId, amount: u128) -> DispatchResult {
			ensure_root(origin)?;
			Reputations::<T>::try_mutate_exists(&account, |reputation| {
				if let Some(ref mut reputation) = reputation {
					reputation.score = reputation.score.saturating_sub(100 * amount);
					let raw_reputation = reputation.score * reputation.contribution;
					MaxRawReputation::<T>::put(core::cmp::max(
						raw_reputation,
						MaxRawReputation::<T>::get(),
					));
					reputation.reputation = Self::compute_reputation(reputation.clone());
					reputation.last_evaluation = frame_system::Pallet::<T>::current_block_number();
					Ok(())
				} else {
					Err(Error::<T>::AccountNotFound.into())
				}
			})
		}
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::slash())]
		pub fn evaluate_reputation(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Self::do_evaluate_reputation(&account)
		}
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		BlockNumberFor<T>: From<u32>,
	{
		fn on_idle(block: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
			if block %
				BlockNumberFor::<T>::one().saturating_mul(T::ReevaluationPeriod::get().into()) ==
				BlockNumberFor::<T>::zero()
			{
				return Self::process_evaluation_queue(remaining_weight);
			}
			remaining_weight.saturating_add(T::WeightInfo::on_idle_noop())
		}
	}
	impl<T: Config> Pallet<T> {
		pub fn add_contribution(account: &T::AccountId) {
			Reputations::<T>::mutate(account, |reputation| {
				if let Some(ref mut reputation) = reputation {
					reputation.contribution += 1;
				} else {
					*reputation = Some(Reputation {
						contribution: 1,
						score: 1,
						reputation: Perbill::zero(),
						last_evaluation: BlockNumberFor::<T>::zero(),
					});
					AccountQueue::<T>::mutate(|queue| {
						let _ = queue.try_push(account.clone());
					});
				}
			});
		}
		fn compute_reputation(reputation: Reputation<BlockNumberFor<T>>) -> Perbill {
			let raw_reputation = reputation.score * reputation.contribution;
			Perbill::from_rational(raw_reputation * 100, MaxRawReputation::<T>::get() + 1)
		}
		fn do_evaluate_reputation(account: &T::AccountId) -> DispatchResult {
			Reputations::<T>::try_mutate_exists(account, |reputation| {
				if let Some(ref mut reputation) = reputation {
					let raw_reputation = reputation.score * reputation.contribution;
					MaxRawReputation::<T>::put(core::cmp::max(
						raw_reputation,
						MaxRawReputation::<T>::get(),
					));
					reputation.reputation = Self::compute_reputation(reputation.clone());
					reputation.last_evaluation = frame_system::Pallet::<T>::current_block_number();
					Ok(())
				} else {
					Err(Error::<T>::AccountNotFound.into())
				}
			})
		}
		pub fn process_evaluation_queue(remaining_weight: Weight) -> Weight {
			EvaluationQueue::<T>::put(AccountQueue::<T>::get());
			MaxRawReputation::<T>::put(0);
			EvaluationQueue::<T>::mutate(|ref mut queue| -> Weight {
				let mut total_weight = T::WeightInfo::do_process_evaluation_queue();
				let overhead = T::WeightInfo::process_evaluation_queue(2)
					.saturating_sub(T::WeightInfo::process_evaluation_queue(1));

				if queue.is_empty() {
					return total_weight;
				}
				while total_weight.any_lt(remaining_weight.saturating_sub(overhead)) {
					if let Some(account) = queue.pop() {
						let _ = Self::do_evaluate_reputation(&account);
						let _ = T::Currency::deposit_into_existing(
							&account,
							Reputations::<T>::get(&account)
								.map_or_else(Perbill::zero, |account| account.reputation) *
								T::Currency::minimum_balance(),
						);
						total_weight += overhead;
					} else {
						break;
					}
				}
				total_weight
			})
		}
	}
}
