// This file is part of Ventur, it implements an RFP process as a Substrate Pallet.

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

//! # RFP Pallet
//!
//! The RFP pallet provides functionality for creation, distribution, and management of RFP.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The RFP pallet provides functions for:
//!
//! - Creating RFPs.
//! - Bidding on RFPs.
//! - Shortlisting RFP bids.
//! - Updating RFP bids.
//! - Accepting RFP bids.
//! - Updating RFPs.
//! - Canceling RFPs.
//! 
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create_rfp` - 
//! - `update_rfp` - 
//! - `cancel_rfp` - 
//! - `bid_on_rfp` - 
//! - `shortlist_bid` -
//! - `update_rfp_bid` -
//! - `accept_rfp_bid` -

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
		pallet_prelude::*,
		traits::{
			Currency,
			LockableCurrency
		}
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type RFPId: Member + Parameter + MaxEncodedLen + From<u32> + Copy + Clone + Eq + TypeInfo;
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		type Cid: MaxEncodedLen + TypeInfo + Decode + Encode + Clone + Eq + std::fmt::Debug;
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct RFPDetails<T: Config>{
		pub(super) rfp_owner: T::AccountId,

		pub(super) ipfs_hash: T::Cid,
	}

	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::storage]
	#[pallet::getter(fn get_rfps)]
	pub type RFPs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // rfp owner
		Blake2_128Concat,
		T::RFPId, // rfp_id
		RFPDetails<T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Creates an RFP
		/// [account, rfp]
		CreateRFP(T::AccountId, T::RFPId),
		/// Updates an RFP
		/// [account, rfp]
		UpdateRFP(T::AccountId, T::RFPId),
		/// Cancels an RFP
		/// [account, rfp]
		CancelRFP(T::AccountId, T::RFPId),
		/// Bids on an RFP
		/// [account, rfp, amount]
		BidOnRFP(T::AccountId, T::RFPId, BalanceOf<T>),
		/// RFP Admin creates a shortlist of the bids on an RFP
		/// [account, rfp]
		ShortlistBid(T::AccountId, T::RFPId),
		/// Updates a bid on an RFP
		/// [account, rfp, amount]
		UpdateRFPBid(T::AccountId, T::RFPId, BalanceOf<T>),
		/// Accepts a bid on an RFP
		/// [account, rfp]
		AcceptRFPBid(T::AccountId, T::RFPId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Trying to create an RFP that already exists
		/// under that id
		RFPAlreadyExists,

		/// Trying to update an RFP that hasn't been created yet
		UpdatingNonExistentRFP,

		/// Trying to cancel an RFP that doesn't exist
		CancelingNonExistentRFP,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A dispatchable to create an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_rfp(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId,
			rfp_details: RFPDetails<T>
		) -> DispatchResult {
			let rfp_owner = ensure_signed(origin)?;
			
			// Assert rfp doesn't already exist
			let rfp_exists = <RFPs<T>>::get(
				&rfp_owner,
				&rfp_id,
			);

			ensure!(
				rfp_exists.is_none(),
				Error::<T>::RFPAlreadyExists
			);

			// Insert the RFP details into storage
			<RFPs<T>>::insert(
				&rfp_owner, 
				&rfp_id,
				rfp_details
			);

			Self::deposit_event(Event::CreateRFP(rfp_owner, rfp_id));
			Ok(())
		}

		/// A dispatchable to modify an existing RFP
		 #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn update_rfp(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			new_rfp_details: RFPDetails<T>
		) -> DispatchResult {
			let rfp_owner = ensure_signed(origin)?;

			// Update the stored value with the new details
			<RFPs<T>>::try_mutate(
				&rfp_owner,
				&rfp_id,
				| maybe_rfp_details | -> DispatchResult {
					let rfp_details = 
						maybe_rfp_details.as_mut()
							.ok_or(
								<Error<T>>::UpdatingNonExistentRFP
							)?;
					*rfp_details = new_rfp_details;
					Ok(())
				}
			)?;
			Self::deposit_event(
				Event::UpdateRFP(
					rfp_owner, 
					rfp_id
				)
			);
			Ok(())
		}
		
		/// A dispatchable to cancel an existing RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn cancel_rfp(origin: OriginFor<T>, rfp_id: T::RFPId) -> DispatchResult {
			let rfp_owner = ensure_signed(origin)?;
			ensure!(
				<RFPs<T>>::contains_key(
					&rfp_owner,
					&rfp_id
				),
				<Error<T>>::CancelingNonExistentRFP
			);

			<RFPs<T>>::remove(&rfp_owner, &rfp_id);
			Self::deposit_event(Event::CancelRFP(rfp_owner, rfp_id));
			Ok(())
		}

		/// A dispatchable to Bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn bid_on_rfp(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			amount: BalanceOf<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: Bid on RFP
			Self::deposit_event(Event::BidOnRFP(who, rfp_id, amount));
			Ok(())
		}

		/// A dispatchable to create a shortlist of bids
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn shortlist_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: Bid on RFP
			Self::deposit_event(Event::ShortlistBid(who, rfp_id));
			Ok(())
		}

		/// A dispatchable to update a bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn update_rfp_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			updated_amount: BalanceOf<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: update bid
			Self::deposit_event(Event::UpdateRFPBid(who, rfp_id, updated_amount));
			Ok(())
		}

		/// A dispatchable to accept a bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn accept_rfp_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: accept bid
			Self::deposit_event(Event::AcceptRFPBid(who, rfp_id));
			Ok(())
		}
	}
}