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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub const VEC_LIMIT: u32 = u32::MAX;

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
	#[pallet::getter(fn get_subscription_payments_calendar)]
	pub type SubscriptionPaymentsCalendar<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u64, // payment date
		Blake2_128Concat,
		T::SubscriptionServiceId,
		BoundedVec<
				T::SubscriptionId, ConstU32<{VEC_LIMIT}>
			>,
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
    }

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreateSubscription(),
		ClaimSubscriptionPayment(),
		CancelSubscription(),

	}

	#[pallet::error]
	pub enum Error<T> {
        /// Error names should be descriptive.
		NoneValue,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        
	}
}