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

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub struct SubscriptionDetails<AccountId> {
		// maybe differentiate minter from owner
		// On assignment but not yet accepted, who is the owner?
		pub(super) owner: AccountId,
		pub(super) is_active: bool,
		pub(super) cost: u32,
		pub(super) frequency: u32,
		pub(super) upfront_discount: StorageMap<_, Blake2_128Concat, u32, u32>, // MAP of(#_of_periods, discounted_cost)
		pub(super) metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
	}
	
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub struct SubscriptionInstance<AccountId> {
		pub(super) owner: AccountId,
		pub(super) is_active: bool,
		pub(super) cost: u32,
		pub(super) frequency: u32,
		pub(super) upfront_discount: StorageMap<_, Blake2_128Concat, u32, u32>, // MAP of(#_of_periods, discounted_cost)
		pub(super) metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CollectionId: Member + Parameter + MaxEncodedLen + Copy;
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;
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
        /// A dispatchable to create a Subscription
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_collection(
			origin: OriginFor<T>, 
			collection_id: T::CollectionId,
			image_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
			metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Collection<T>>::contains_key(&collection_id), <Error<T>>::CollectionIdAlreadyExists);
			<Collection<T>>::insert(
				collection_id, 
				CollectionDetails {
					owner: who.clone(),
					amount: 0,
					is_frozen: false,
					image_ipfs_cid: image_ipfs_cid,
					metadata_ipfs_cid: metadata_ipfs_cid,
				});

			Self::deposit_event(Event::CreateCollection(collection_id, who));
			Ok(())
		}
	}
}