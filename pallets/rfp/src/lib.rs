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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type RFPId: Member + Parameter + MaxEncodedLen + Copy;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Creates an RFP
		/// [account, rfp]
		CreateRFP(T::AccountId, T::RFPId),
		/// Updates an RFP
		/// [account, rfp]
		UpdateRFP(T::AccountId, T::RFPId),
	}

	#[pallet::error]
	pub enum Error<T> {

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// An extrinsic that transfers the next scheduled payment
		// to the payee's account, if the payment is available
		pub fn claim (
		) -> DispatchResult {
			Self::deposit_event(
				Event::PartOfPaymentClaimed(payee, payment_amount)
			);
			Ok(())
		}
	}
}