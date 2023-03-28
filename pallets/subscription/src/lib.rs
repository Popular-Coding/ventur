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
		traits::UnixTime,
	};
	pub const VEC_LIMIT: u32 = u32::MAX;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub enum SubscriptionFeeFrequency {
		Weekly,
		#[default]
		Monthly,
		Yearly
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub struct SubscriptionService<T: Config> {
		pub(super) service_owner: T::AccountId,
		pub(super) subscription_service_id: T::SubscriptionServiceId,
		pub(super) is_active: bool,
		pub(super) base_subscription_fee: u64,
		pub(super) default_frequency: SubscriptionFeeFrequency,
		pub(super) metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
	}
	
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Subscription<T: Config> {
		pub(super) subscriber: T::AccountId,
		pub(super) subscription_service_id: T::SubscriptionServiceId,
		pub(super) subscription_id: T::SubscriptionId,
		pub(super) is_active: bool,
		pub(super) subscription_fee: u64,
		pub(super) payment_frequency: SubscriptionFeeFrequency,
		pub(super) most_recent_payment_date: u64,
		pub(super) next_payment_due_date: u64
	}

	#[pallet::storage]
	#[pallet::getter(fn get_subscription_services)]
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

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type SubscriptionServiceId: Member + Parameter + MaxEncodedLen + Copy;
		type SubscriptionId: Member + Parameter + MaxEncodedLen + Copy;
		type TimeProvider: UnixTime;
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
		CancelSubscription(T::AccountId, T::SubscriptionServiceId),
	}

	#[pallet::error]
	pub enum Error<T> {
        /// Error names should be descriptive.
		NoneValue,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_subscription_service (
			origin: OriginFor<T>, 
			subscription_service_id: T::SubscriptionServiceId,
			_base_subscription_fee: u64,
			_metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
		) -> DispatchResult {
			let service_owner = ensure_signed(origin)?;
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
			_subscription_service_id: T::SubscriptionServiceId,
			subscription_id: T::SubscriptionId,
			_service_owner: T::AccountId,
			_subscription_fee: u64,
			_payment_frequencey: SubscriptionFeeFrequency,
		) -> DispatchResult {
			let subscriber_id = ensure_signed(origin)?;
			Self::deposit_event(
				Event::InitiateSubscription(
					subscriber_id, subscription_id
				)
			);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn claim_subscription_payment (
			origin: OriginFor<T>, 
			subscription_id: T::SubscriptionId,
		) -> DispatchResult {
			let subscriber_owner_id = ensure_signed(origin)?;
			Self::deposit_event(
				Event::ClaimSubscriptionPayment(
					subscriber_owner_id, subscription_id
				)
			);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn cancel_subscription_service (
			origin: OriginFor<T>, 
			subscription_service_id: T::SubscriptionServiceId,
		) -> DispatchResult {
			let subscription_owner_id = ensure_signed(origin)?;
			Self::deposit_event(
				Event::CancelSubscription(
					subscription_owner_id, subscription_service_id
				)
			);
			Ok(())
		}
	}
}