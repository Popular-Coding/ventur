//! Benchmarking setup for pallet-payments

use super::*;

#[allow(unused)]
use crate::{
    self as pallet_payments,
    Pallet as Payments,
};
use frame_benchmarking::{
    benchmarks, 
    whitelisted_caller,
    impl_benchmark_test_suite
};
use sp_std::prelude::*;
use frame_system::RawOrigin;
use frame_support::{
    traits::{
        Currency,
    },
};
use frame_support::{
    BoundedVec
};

benchmarks! {
	initialize_payment {
        let caller: T::AccountId = whitelisted_caller();
        let payee: T::AccountId = whitelisted_caller();
        let time: u64 = 0u64;
        let total_payment_amount: 
            BalanceOf<T> = T::PaymentCurrency::minimum_balance();
        let scheduled_payment = pallet_payments::ScheduledPayment::<T> {
            payment_date: time,
            amount_per_claim: total_payment_amount,
            released: true,
        };
        let payment_schedule: BoundedVec<
            pallet_payments::ScheduledPayment::<T>, 
            T::MaxPaymentsScheduled,
        > = vec![scheduled_payment].try_into().unwrap();
        let payment_method = pallet_payments::PaymentMethod::<T>{
            payment_source: pallet_payments::PaymentSource::EscrowAccount,
            account_id: caller.clone(),
        };
	}: _(RawOrigin::Signed(caller.clone()), payee, 0u32.into(), 0.into(), total_payment_amount, payment_schedule.clone(), payment_method, caller.clone())

	impl_benchmark_test_suite!(Payments, crate::mock::test_externalities(), crate::mock::Test);
}
