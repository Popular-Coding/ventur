//! # Payments Pallet
// 
//! The Payments pallet supports the instantiation of lump sum
//! or scheduled, iterative payments. 
//! Payments can come out of an individual's account, or out of
//! an escrow account set up by the payer of the payment agreement
// 
//! Payments must be claimed by individuals
//! In the case of scheduled, iterative payments, payments can only
//! be claimed if the claim comes after the scheduled payment date
//
//! Inspiration for the source code of this pallet comes from 
//! the Pure-Stake Crowdloan Rewards Pallet:
//! https://github.com/PureStake/crowdloan-rewards/blob/main/src/lib.rs
//! 
//! While the Crowdloan Rewards Pallet supports claiming rewards,
//! this Payments Pallet supports the instantiation of scheduled payments
//! as well as lump sum, one-time payments

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
pub(crate) mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::RuntimeDebugNoBound,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::AllowDeath, UnixTime},
		storage::bounded_vec::BoundedVec,
	};
	use frame_system::pallet_prelude::*;

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, scale_info::TypeInfo)]
	#[scale_info(skip_type_params(T))]
	// The struct that stores info about the payment agreement
	// between two parties
	pub struct PaymentDetails<T: Config>{

		// The paying party of the payment contract
		// Note: If, for example, the payment is coming from
		// an escrow account, this won't be the account from
		// which the payment is coming
		pub(super) payer: T::AccountId,

		// Which account the funds will be transferred to
		pub(super) payee: T::AccountId,

		// The UID for payments, used for identifying this 
		// payment agreement
		pub(super) payment_id: T::PaymentId,

		// The id of the RFP associated with this payment
		// agreement
		pub(super) rfp_reference_id: T::RFPReferenceId,

		// The total payment amount that will be paid
		// out to the payee
		pub(super) total_payment_amount: BalanceOf<T>,

		// This bounded vec allows payments to be paid 
		// out in installments
		pub(super) payment_schedule: 
			BoundedVec<
				ScheduledPayment<T>, T::MaxPaymentsScheduled
			>,

		// A struct describing where the payment will 
		// be coming from 
		pub(super) payment_method: PaymentMethod<T>,

		// The id of the admin of this payment agreement
		// Admins will have special privileges w.r.t.
		// modifying payments
		pub(super) administrator_id: T::AccountId,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, scale_info::TypeInfo)]
	#[scale_info(skip_type_params(T))]
	// An instance of a payment that is to be issued and claimed
	pub struct ScheduledPayment<T: Config> {
		// When the payment will be eligible for claiming
		pub(super) payment_date: u64,

		// How much of the total amount can be claimed with
		// this instance of payment
		pub(super) amount_per_claim: BalanceOf<T>,

		// If false, this instance is not eligible for claim
		pub(super) released: bool,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound,  PartialEq, scale_info::TypeInfo)]
	#[scale_info(skip_type_params(T))]
	// Whether the payment is coming from a personal or an
	// escrow account
	pub enum PaymentSource {
		#[default]
		PersonalAccount,
		EscrowAccount,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, scale_info::TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct PaymentMethod<T: Config> {
		pub(super) payment_source: PaymentSource,

		// The account from which the transfer is to be drawn
		pub(super) account_id: T::AccountId,
	}

	pub type BalanceOf<T> = <<T as Config>::PaymentCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type PaymentId: Member + Parameter + Clone + Eq;
		type RFPReferenceId: Member + Parameter + MaxEncodedLen + Copy + Clone + Eq + TypeInfo;
		type PaymentCurrency: Currency<Self::AccountId> + Clone + Eq;
		#[pallet::constant]
		type MaxPaymentsScheduled: Get<u32>;
		type TimeProvider: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn payment_agreements)]
	// Here we store all payment agreements
	// Key: (payer, payee, payment_id)
	// Value: Payment Details
	pub type PaymentAgreements<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // payer_account
			NMapKey<Blake2_128Concat, T::AccountId>, // payee_account
			NMapKey<Blake2_128Concat, T::PaymentId>, // paymentId
		),
		PaymentDetails<T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PaymentInitialized(T::AccountId, T::AccountId, BalanceOf<T>),
		PartOfPaymentClaimed(T::AccountId, BalanceOf<T>),
		NextPaymentReleaseStatusChanged(T::AccountId, T::PaymentId, bool),
	}

	#[pallet::error]
	pub enum Error<T> {
		// Payment doesn't exist in storage with the specified key
		PaymentDetailsNonExistent,

		// There is no scheduled payment in the payment agreements
		NoScheduledPaymentRecorded,

		// The payment has not been released, or has been blocked by
		// the payer
		PaymentNotReleased,

		// A payment agreement with the specified key already exists
		PaymentAlreadyInitialized,

		// The scheduled date for payment has not passed yet, 
		// meaning the payment cannot be claimed
		PaymentNotAvailable,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn claim (
			origin: OriginFor<T>, 
			payer_id: T::AccountId,
			payment_id: T::PaymentId,
		) -> DispatchResult {
			let payee = ensure_signed(origin)?;
			<PaymentAgreements<T>>::try_mutate(
				(&payer_id, &payee.clone(), &payment_id), 
				| maybe_payment_agreements | -> DispatchResult {
					let payment_details = 
						maybe_payment_agreements
						.as_mut()
						.ok_or(<Error<T>>::PaymentDetailsNonExistent)?;
					let payment_schedule = &mut payment_details.payment_schedule;
					ensure!(
						!payment_schedule.is_empty(), 
						<Error<T>>::NoScheduledPaymentRecorded
					);

					// Try to claim the next payment
					let next_payment = payment_schedule.first().ok_or(
						<Error<T>>::NoScheduledPaymentRecorded
					)?;

					// Deny the payment if it is before the due date
					let time: u64 = T::TimeProvider::now().as_secs();
					ensure!(
						time >= next_payment.payment_date, 
						<Error<T>>::PaymentNotAvailable
					);
					let payment_amount = next_payment.amount_per_claim;
					let payment_method = &payment_details.payment_method;
					let payment_account_id = &payment_method.account_id;
					ensure!(next_payment.released, <Error<T>>::PaymentNotReleased);
					T::PaymentCurrency::transfer(
						&payment_account_id,
						&payee,
						payment_amount,
						AllowDeath,
					)?;
					
					// If successfully claimed, get rid of the first payment
					payment_schedule.remove(0);
					Self::deposit_event(
						Event::PartOfPaymentClaimed(payee, payment_amount)
					);
					Ok(())
				}
			)?;
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn initialize_payment (
			origin: OriginFor<T>, 
			payee: T::AccountId,
			payment_id: T::PaymentId,
			rfp_reference_id: T::RFPReferenceId,
			total_payment_amount: BalanceOf<T>,
			payment_schedule: BoundedVec<ScheduledPayment<T>, T::MaxPaymentsScheduled>,
			payment_method: PaymentMethod<T>,
			administrator_id: T::AccountId,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			let payment_details = <PaymentAgreements<T>>::get(
				(&payer, &payee, &payment_id)
			);
			ensure!(
				payment_details.is_none(),
				Error::<T>::PaymentAlreadyInitialized
			);

			let payment_details = PaymentDetails {
				payer: payer.clone(),
				payee: payee.clone(),
				payment_id: payment_id.clone(),
				rfp_reference_id,
				total_payment_amount,
				payment_schedule,
				payment_method: payment_method.clone(),
				administrator_id,
			};
			<PaymentAgreements<T>>::insert(
				(&payer, &payee, payment_id), 
				payment_details
			);
			Self::deposit_event(
				Event::PaymentInitialized(
					payment_method.account_id, 
					payee, 
					total_payment_amount
				)
			);
			
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn block_next_payment (
			origin: OriginFor<T>, 
			payee_id: T::AccountId,
			payment_id: T::PaymentId,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			Pallet::<T>::change_next_payment_release_status(
				&payer,
				&payee_id,
				&payment_id,
				false
			)
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn release_next_payment (
			origin: OriginFor<T>, 
			payee_id: T::AccountId,
			payment_id: T::PaymentId,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			Pallet::<T>::change_next_payment_release_status(
				&payer,
				&payee_id,
				&payment_id,
				true
			)
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn change_next_payment_release_status(
			payer: &T::AccountId,
			payee_id: &T::AccountId,
			payment_id: &T::PaymentId,
			released: bool 
		) -> DispatchResult {
			<PaymentAgreements<T>>::try_mutate(
				(&payer.clone(), &payee_id, &payment_id), 
				| maybe_payment_agreements | -> DispatchResult {
					let payment_details = 
						&mut maybe_payment_agreements
						.as_mut()
						.ok_or(<Error<T>>::PaymentDetailsNonExistent)?;
					let payment_schedule = &mut payment_details.payment_schedule;
					ensure!(
						!payment_schedule.is_empty(), 
						<Error<T>>::NoScheduledPaymentRecorded
					);
					let next_payment = &mut payment_schedule.get_mut(0)
						.ok_or(
						<Error<T>>::NoScheduledPaymentRecorded
					)?;

					// Modify the release of the next payment
					next_payment.released = released;
					Self::deposit_event(
						Event::NextPaymentReleaseStatusChanged(
							payer.clone(), 
							payment_id.clone(), 
							released
						)
					);
					Ok(())
				}
			)?;
			Ok(())
		}
	}
}
