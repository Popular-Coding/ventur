// This file is part of Ventur, it implements an Subscription, 
// Subscription, Substrate Pallet.

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

//! # Subscription Pallet
//!
//! The Subscription pallet provides functionality for creation, distribution, and management of Subscriptions.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Subscription pallet provides functions for:
//!
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! 
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 
//! - 

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			Currency, 
			UnixTime, 
			LockableCurrency,
			ExistenceRequirement::AllowDeath, 
		},
		storage::bounded_vec::BoundedVec,
	};
	use chrono::naive::NaiveDateTime;
	use chronoutil::relative_duration::RelativeDuration;

	pub const VEC_LIMIT: u32 = u32::MAX;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub enum SubscriptionFeeFrequency {
		Weekly,
		#[default]
		Monthly,
		Yearly,
		Adhoc,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct SubscriptionService<T: Config> {
		pub(super) service_owner: T::AccountId,
		pub(super) subscription_service_id: T::SubscriptionServiceId,
		pub(super) is_active: bool,
		pub(super) base_subscription_fee: BalanceOf<T>,
		pub(super) default_frequency: SubscriptionFeeFrequency,
		pub(super) metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
	}
	
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Subscription<T: Config> {
		pub(super) subscriber: T::AccountId,
		pub(super) subscription_service_owner: T::AccountId,
		pub(super) subscription_service_id: T::SubscriptionServiceId,
		pub(super) subscription_id: T::SubscriptionId,
		pub(super) is_active: bool,
		pub(super) subscription_fee: BalanceOf<T>,
		pub(super) payment_frequency: SubscriptionFeeFrequency,
		pub(super) most_recent_payment_date: u64,
		pub(super) next_payment_due_date: u64
	}

	#[pallet::storage]
	#[pallet::getter(fn get_subscription_services)]
	pub type SubscriptionServices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::SubscriptionServiceId,
		SubscriptionService<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_subscription_services_to_subscription_ids)]
	pub type SubscriptionServicesToSubscriptionIds<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, 
		Blake2_128Concat,
		T::SubscriptionServiceId,
		BoundedVec<
				T::SubscriptionId, ConstU32<{VEC_LIMIT}>
			>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_subscriptions)]
	pub type Subscriptions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::SubscriptionId,
		Subscription<T>,
		OptionQuery,
	>;

	pub type BalanceOf<T> = <<T as Config>::PaymentCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type SubscriptionServiceId: Member + Parameter + MaxEncodedLen + Copy;
		type SubscriptionId: Member + Parameter + MaxEncodedLen + Copy;
		type TimeProvider: UnixTime;
		type PaymentCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber> + Clone + Eq;
    }

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// [owner_id, subscription_id]
		CreateSubscription(T::AccountId, T::SubscriptionServiceId),
		// [subscriber_id, subscription_id]
		InitiateSubscription(T::AccountId, T::SubscriptionId),
		// [owner_id, subscription_id]
		ClaimSubscriptionPayment(T::AccountId, T::SubscriptionId),
		// [owner_id, subscription_id]
		CancelSubscription(T::AccountId, T::SubscriptionId),
	}

	#[pallet::error]
	pub enum Error<T> {
		// Subscription with ID already created
		SubscriptionIdExists,
		// Subscription Service not found in storage
		NonExsitantSubscriptionService,
		// Too many services outside the bound of the bounded vec
		TooManyServices,
		// No Subscription for given service id
		NoSubscriptionForService,
		// No Subscription found for id
		NoSubscriptionFound,
		// Trying to claim a payment from an inactive subscription
		ClaimingPaymentFromInactiveSubscription,
		// Trying to claim a payment before the due date
		ClaimingPaymentBeforeDueDate
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_subscription_service (
			origin: OriginFor<T>, 
			subscription_service_id: T::SubscriptionServiceId,
			base_subscription_fee: BalanceOf<T>,
			default_frequency: SubscriptionFeeFrequency,
			metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
		) -> DispatchResult {
			let service_owner = ensure_signed(origin)?;
			ensure!(
				<SubscriptionServices<T>>::get(
					&subscription_service_id
				).is_none(),
				Error::<T>::SubscriptionIdExists
			);
			let subscription_service_details = SubscriptionService {
				service_owner: service_owner.clone(),
				subscription_service_id,
				is_active: true,
				base_subscription_fee,
				default_frequency,
				metadata_ipfs_cid
			};
			<SubscriptionServices<T>>::insert(
				&subscription_service_id, 
				subscription_service_details
			);

			let empty_subscriptions_vec: BoundedVec<
				T::SubscriptionId, ConstU32<{VEC_LIMIT}>
			> = BoundedVec::<
					T::SubscriptionId, ConstU32<{VEC_LIMIT}>
				>::default();
			<SubscriptionServicesToSubscriptionIds<T>>::insert::<
				&T::AccountId, &T::SubscriptionServiceId,
				BoundedVec::<
					T::SubscriptionId, ConstU32<{VEC_LIMIT}
				>
			>
				> (
				&service_owner, 
				&subscription_service_id, 
				empty_subscriptions_vec.into()
			);
			Self::deposit_event(
				Event::CreateSubscription(
					service_owner, subscription_service_id
				)
			);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn initiate_subscription (
			origin: OriginFor<T>, 
			subscription_service_id: T::SubscriptionServiceId,
			subscription_id: T::SubscriptionId,
			service_owner: T::AccountId,
			subscription_fee: BalanceOf<T>,
			payment_frequency: SubscriptionFeeFrequency,
		) -> DispatchResult {
			let subscriber = ensure_signed(origin)?;
			T::PaymentCurrency::transfer(
				&subscriber,
				&service_owner,
				subscription_fee,
				AllowDeath,
			)?;
			ensure!(
				!<SubscriptionServicesToSubscriptionIds<T>>::get(
					&service_owner,
					&subscription_service_id,
				).is_none(),
				Error::<T>::NonExsitantSubscriptionService
			);
			let time_now: u64 = T::TimeProvider::now().as_secs();
			let next_payment_due_date = Pallet::<T>::calculate_next_payment_date(
				time_now,
				payment_frequency.clone()
			);
			let subscription_detail: Subscription<T> = Subscription {
				subscriber: subscriber.clone(),
				subscription_service_owner: service_owner.clone(),
				subscription_service_id,
				subscription_id,
				is_active: true,
				subscription_fee,
				payment_frequency: payment_frequency.clone(),
				most_recent_payment_date: time_now,
				next_payment_due_date
			};
			<SubscriptionServicesToSubscriptionIds<T>>::try_mutate(
				&service_owner,
				&subscription_service_id,
				| maybe_subscription_ids | -> DispatchResult {
					let subscription_ids = 
						maybe_subscription_ids.as_mut()
							.ok_or(
								<Error<T>>::NonExsitantSubscriptionService
							)?;
					subscription_ids
						.try_push(subscription_id)
						.ok()
						.ok_or(
							<Error<T>>::TooManyServices
						)?;
					Ok(())
				}
			)?;
			<Subscriptions<T>>::insert(
				&subscription_id, subscription_detail
			);
			Self::deposit_event(
				Event::InitiateSubscription(
					subscriber, subscription_id
				)
			);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn claim_subscription_payment (
			origin: OriginFor<T>, 
			subscription_service_id: T::SubscriptionServiceId,
			subscription_id: T::SubscriptionId,
		) -> DispatchResult {
			let subscription_owner_id = ensure_signed(origin)?;
			ensure!(
				<SubscriptionServicesToSubscriptionIds<T>>::get(
					&subscription_owner_id,
					&subscription_service_id,
				).ok_or(
					Error::<T>::NonExsitantSubscriptionService
				)?.contains(&subscription_id),
				Error::<T>::NoSubscriptionForService
			);
			<Subscriptions::<T>>::try_mutate(
				&subscription_id, 
				| maybe_subscription_details | -> DispatchResult {
					let subscription_details = 
						maybe_subscription_details
						.as_mut()
						.ok_or(
							Error::<T>::NoSubscriptionFound
						)?;
						ensure!(
							subscription_details.is_active,
							Error::<T>::ClaimingPaymentFromInactiveSubscription
						);
					// Deny the payment if it is before the due date
					let time_now: u64 = T::TimeProvider::now().as_secs();
					ensure!(
						time_now >= subscription_details.next_payment_due_date,
						Error::<T>::ClaimingPaymentBeforeDueDate,
					);
					let subscription_fee = subscription_details.subscription_fee;
					let subscriber = &subscription_details.subscriber;
					T::PaymentCurrency::transfer(
						subscriber,
						&subscription_owner_id,
						subscription_fee,
						AllowDeath,
					)?;
					subscription_details.most_recent_payment_date = time_now;
					let next_payment_due_date = Pallet::<T>::calculate_next_payment_date(
						time_now,
						subscription_details.payment_frequency.clone()
					);
					subscription_details.next_payment_due_date = next_payment_due_date;
					Ok(())
				}
			)?;

			Self::deposit_event(
				Event::ClaimSubscriptionPayment(
					subscription_owner_id, subscription_id
				)
			);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn cancel_subscription (
			origin: OriginFor<T>, 
			subscription_id: T::SubscriptionId,
		) -> DispatchResult {
			let subscription_owner_id = ensure_signed(origin)?;
			Self::deposit_event(
				Event::CancelSubscription(
					subscription_owner_id, subscription_id
				)
			);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn calculate_next_payment_date(
			last_payment_date: u64,
			payment_frequency: SubscriptionFeeFrequency,
		) -> u64 {
			let last_payment_date_as_naive_date = 
				NaiveDateTime::from_timestamp_opt(
					last_payment_date.try_into().unwrap(), 
					0
				).unwrap();
			let delta = match payment_frequency {
				SubscriptionFeeFrequency::Weekly=>RelativeDuration::weeks(1),
				SubscriptionFeeFrequency::Monthly=>RelativeDuration::months(1),
				SubscriptionFeeFrequency::Yearly=>RelativeDuration::years(1),
				SubscriptionFeeFrequency::Adhoc=>RelativeDuration::weeks(0)
			};
			let next_payment_date = last_payment_date_as_naive_date + delta;
			return next_payment_date.timestamp().try_into().unwrap();
		}
	}
}