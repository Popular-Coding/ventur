// This file is part of Ventur, it implements the instantiation
// and claiming of scheduled or one-time payments

// Copyright (C) 2022 Popular Coding LLC.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Payments pallet provides functions for:
//!
//! - Setting up payments
//! - Claiming payments
//! - Blocking/Releasing payments from being claimed
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `initialize_payment` - Creates the payment details and commits them to storage
//! - `claim` - Transfers the next available funds to the payee's account
//! - `block_next_payment` - Prevent the claiming of the next and all subsequent payments
//! - `release_next_payment` - Free up the next available and all subsequent payments for claiming


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
		traits::{
			Currency, 
			ExistenceRequirement::AllowDeath, 
			WithdrawReasons, 
			UnixTime,
			LockableCurrency,
		},
		storage::bounded_vec::BoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use pallet_escrow;

	pub const VEC_LIMIT: u32 = u32::MAX;

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	/// The struct that stores info about the payment agreement
	/// between two parties
	pub struct PaymentDetails<T: Config>{

		/// The paying party of the payment contract
		/// Note: If, for example, the payment is coming from
		/// an escrow account, this won't be the account from
		/// which the payment is coming
		pub payer: T::AccountId,

		/// Which account the funds will be transferred to
		pub payee: T::AccountId,

		/// The UID for payments, used for identifying this 
		/// payment agreement
		pub payment_id: T::PaymentId,

		/// The id of the RFP associated with this payment
		/// agreement
		pub rfp_reference_id: T::RFPReferenceId,

		/// The total payment amount that will be paid
		/// out to the payee
		pub total_payment_amount: BalanceOf<T>,

		/// This bounded vec allows payments to be paid 
		/// out in installments
		pub payment_schedule: 
			BoundedVec<
				ScheduledPayment<T>, ConstU32<{VEC_LIMIT}>
			>,

		/// A struct describing where the payment will 
		/// be coming from 
		pub payment_method: PaymentMethod<T>,

		/// The id of the admin of this payment agreement
		/// Admins will have special privileges w.r.t.
		/// modifying payments
		pub administrator_id: T::AccountId,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	/// An instance of a payment that is to be issued and claimed
	pub struct ScheduledPayment<T: Config> {
		/// When the payment will be eligible for claiming
		pub payment_date: u64,

		/// How much of the total amount can be claimed with
		/// this instance of payment
		pub amount_per_claim: BalanceOf<T>,

		/// If false, this instance is not eligible for claim
		pub released: bool,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, Eq, TypeInfo, Copy, MaxEncodedLen)]
	/// Whether the payment is coming from a personal or an
	/// escrow account
	pub enum PaymentSource {
		#[default]
		PersonalAccount,
		EscrowAccount,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, Copy, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct PaymentMethod<T: Config> {
		pub payment_source: PaymentSource,

		/// The account from which the transfer is to be drawn
		pub account_id: T::AccountId,
	}

	pub type BalanceOf<T> = <<T as Config>::PaymentCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_escrow::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type PaymentId: Member + Parameter + From<u32> + Clone + Eq + Copy + MaxEncodedLen;
		type RFPReferenceId: Member + Parameter + MaxEncodedLen + From<u32> + Copy + Clone + Eq + TypeInfo;
		type PaymentCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber> + Clone + Eq;
		type TimeProvider: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn payment_agreements)]
	/// Here we store all payment agreements
	/// Key: (payer, payee, payment_id)
	/// Value: Payment Details
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
		/// Payment has successfully been initialized
		/// [payer, payee, total_payment_amount]
		PaymentInitialized(T::AccountId, T::AccountId, BalanceOf<T>),

		/// The next available payment has been claimed
		/// [payee, amount_claimed]
		PartOfPaymentClaimed(T::AccountId, BalanceOf<T>),

		/// The next available payment has been released or frozen
		/// [payer, payment_id, payment_made_avialble_for_claim]
		NextPaymentReleaseStatusChanged(T::AccountId, T::PaymentId, bool),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Payment doesn't exist in storage with the specified key
		PaymentDetailsNonExistent,

		/// There is no scheduled payment in the payment agreements
		NoScheduledPaymentRecorded,

		/// The payment has not been released, or has been blocked by
		/// the payer
		PaymentNotReleased,

		/// A payment agreement with the specified key already exists
		PaymentAlreadyInitialized,

		/// The scheduled date for payment has not passed yet, 
		/// meaning the payment cannot be claimed
		PaymentNotAvailable,

		/// A payment is specified for Escrow, but no associated
		/// escrow account was found
		NoEscrowAccountFound,

		/// The Escrow account has been frozen
		Frozen,

		/// The payer accessing the escrow is not an admin
		Unauthorized,

		/// Attempting to claim payment from escrow for oneself
		SelfDistributionAttempt,

		/// Trying to claim more funds than exist in an escrow
		InsufficientEscrowFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		/// An extrinsic that transfers the next scheduled payment
		/// to the payee's account, if the payment is available
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
					if payment_method.payment_source == PaymentSource::PersonalAccount {
						Pallet::<T>::transfer_funds_from_personal_account(
							payment_account_id,
							&payee,
							payment_amount,
						)?;
					} else {
						Pallet::<T>::transfer_funds_from_escrow_account(
							payment_account_id,
							&payer_id,
							&payee,
							payment_amount
						)?
					}
					
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
		/// An extrinsic that initializes a payment and commits
		/// it to storage
		pub fn initialize_payment (
			origin: OriginFor<T>, 
			payment_details: PaymentDetails<T>,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			let payee = payment_details.payee.clone();
			let payment_id = payment_details.payment_id;
			let payment_details_exists = <PaymentAgreements<T>>::get(
				(&payer, &payee, &payment_id)
			);
			ensure!(
				payment_details_exists.is_none(),
				Error::<T>::PaymentAlreadyInitialized
			);
			let total_payment_amount = payment_details.total_payment_amount;
			let paying_account = payment_details.payment_method.account_id.clone();
			<PaymentAgreements<T>>::insert(
				(&payer, &payee, payment_id), 
				payment_details
			);
			Self::deposit_event(
				Event::PaymentInitialized(
					paying_account, 
					payee, 
					total_payment_amount
				)
			);			
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// An extrinsic that blocks the next payment from release
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
		/// An extrinsic that unblocks the next payment, 
		/// allowing it to be claimed
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
							*payment_id, 
							released
						)
					);
					Ok(())
				}
			)?;
			Ok(())
		}

		pub fn transfer_funds_from_personal_account(
			payment_account_id: &T::AccountId, 
			payee: &T::AccountId,
			payment_amount: BalanceOf<T>,
		) -> DispatchResult {
			T::PaymentCurrency::transfer(
				payment_account_id,
				payee,
				payment_amount,
				AllowDeath,
			)
		}

		pub fn transfer_funds_from_escrow_account(
			escrow_account_id: &T::AccountId,
			admin_account_id: &T::AccountId, 
			payee: &T::AccountId,
			payment_amount: BalanceOf<T>,
		) -> DispatchResult {
			<pallet_escrow::Escrow<T>>::try_mutate(
				escrow_account_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoEscrowAccountFound)?;
						
					ensure!(
						!escrow_details.is_frozen,
						Error::<T>::Frozen
					);
					// Make sure the payer is an Admin and the
					// transfer can be completed
					ensure!(
						escrow_details.admins.iter().any(|x| *x == admin_account_id.clone()),
						Error::<T>::Unauthorized
					);

					// Unlock funds
					T::EscrowCurrency::remove_lock(pallet_escrow::ESCROW_LOCK, escrow_account_id);
					
					// Transfer the unlocked funds
					T::PaymentCurrency::transfer(
						escrow_account_id,
						payee,
						payment_amount,
						AllowDeath,
					)?;

					let payment_amount_as_128: u128 = 
						TryInto::<u128>::try_into(payment_amount).ok().unwrap();
					//let amount_as_128: u128 = TryInto::<u128>::try_into(escrow_details.amount.clone()).ok().unwrap();
					escrow_details.amount -= payment_amount_as_128.try_into().ok().unwrap();
					
					// Lock the remaining funds
					T::EscrowCurrency::set_lock(
						pallet_escrow::ESCROW_LOCK,
						escrow_account_id,
						escrow_details.amount,
						WithdrawReasons::all(),
					);
					Ok(())
				}
			)?;
			Ok(())
		}
	}
}