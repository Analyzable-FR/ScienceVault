use crate::weights::extrinsic_weights::ExtrinsicBaseWeight;
use frame_support::{
    pallet_prelude::Weight,
    traits::Imbalance,
    weights::{
        WeightToFee, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
    },
};
use smallvec::smallvec;
use sp_arithmetic::{
    traits::{BaseArithmetic, Unsigned},
    MultiplyRational,
};
use sp_runtime::{Perbill, SaturatedConversion};

use super::AccountId;

pub struct LengthToFeeImpl<T>(sp_std::marker::PhantomData<T>);
impl<T> WeightToFee for LengthToFeeImpl<T>
where
    T: BaseArithmetic + From<u32> + Copy + Unsigned,
{
    type Balance = T;

    fn weight_to_fee(length_in_bytes: &Weight) -> Self::Balance {
        Self::Balance::saturated_from(length_in_bytes.ref_time() / 100u64)
    }
}

pub struct WeightToFeeImpl<T>(sp_std::marker::PhantomData<T>);
impl<T> WeightToFeePolynomial for WeightToFeeImpl<T>
where
    T: BaseArithmetic + From<u64> + Copy + Unsigned + From<u32> + MultiplyRational,
{
    type Balance = T;

    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        let p: Self::Balance = 1_000_000_000_000_000u64.into();
        let q: Self::Balance = Self::Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

/// Implementation of the RewardHandler trait to give reward at each new vault submission
pub struct HandleReward<T>(frame_support::pallet_prelude::PhantomData<T>);
impl<T> pallet_reward::Reward<T::AccountId> for HandleReward<T>
where
    T: pallet_reward::Config,
{
    fn add_contribution(account: &<T as frame_system::Config>::AccountId) {
        pallet_reward::Pallet::<T>::add_contribution(account);
    }
}

pub struct HandleDust<TreasuryAccount, Balances>(
    frame_support::pallet_prelude::PhantomData<TreasuryAccount>,
    frame_support::pallet_prelude::PhantomData<Balances>,
);
type CreditOf<Balances> = frame_support::traits::tokens::fungible::Credit<AccountId, Balances>;
impl<TreasuryAccount, Balances> frame_support::traits::OnUnbalanced<CreditOf<Balances>>
    for HandleDust<TreasuryAccount, Balances>
where
    TreasuryAccount: sp_core::Get<AccountId>,
    Balances: frame_support::traits::fungible::Balanced<AccountId>,
{
    fn on_nonzero_unbalanced(amount: CreditOf<Balances>) {
        let _ = Balances::deposit(
            &TreasuryAccount::get(),
            amount.peek(),
            frame_support::traits::tokens::Precision::Exact,
        );
    }
}

pub struct AccountIdOf<T>(frame_support::pallet_prelude::PhantomData<T>);
impl<T> sp_runtime::traits::Convert<T::ElementHash, Option<T::AccountId>> for AccountIdOf<T>
where
    T: pallet_vault::Config,
{
    fn convert(element: T::ElementHash) -> Option<T::AccountId> {
        pallet_vault::Pallet::<T>::account_id_of(&element)
    }
}
