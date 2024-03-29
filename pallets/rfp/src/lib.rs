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
		},
		storage::bounded_vec::BoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use pallet_payments;

	pub const VEC_LIMIT: u32 = u32::MAX;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_payments::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type RFPId: Member + Parameter + MaxEncodedLen + From<u32> + Copy + Clone + Eq + TypeInfo;
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		type Cid: MaxEncodedLen + TypeInfo + Decode + Encode + Clone + Eq + sp_std::fmt::Debug;
		type BidId: Member + Parameter + MaxEncodedLen + From<u32> + Copy + Clone + Eq + TypeInfo;
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct RFPDetails<T: Config>{
		pub(super) rfp_owner: T::AccountId,

		pub(super) ipfs_hash: T::Cid,

		pub(super) rfp_status: RFPStatus,
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, Eq, TypeInfo, Copy, MaxEncodedLen)]
	/// Describes whether the RPF Owner is accepting bids, not accepting new bids,
	/// or if a bid has already been accepted for this RFP
	pub enum RFPStatus {
		#[default]
		AcceptingBids,
		NotAcceptingNewBids,
		AcceptedBid
	}

	#[derive(Default, Clone, Encode, Decode, RuntimeDebugNoBound, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct BidDetails<T: Config>{
		// TODO: Add feature of having multiple admins
		pub(super) bid_owner: T::AccountId,

		pub(super) ipfs_hash: T::Cid,

		pub(super) bid_amount: BalanceOf<T>

		// TODO: Add status (bid, shortlisted, accepted)
		// TODO: Add rfpid associated with this bid
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

	#[pallet::storage]
	#[pallet::getter(fn all_bids)]
	pub type AllBids<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::BidId, // bid_id
		BidDetails<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn rfp_to_bids)]
	pub type RFPToBids<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::RFPId, // rfP_id
		BoundedVec<
				T::BidId, ConstU32<{VEC_LIMIT}>
			>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn rfp_to_shortlisted_bids)]
	pub type RFPToShortlistedBids<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::RFPId, // rfp_id
		BoundedVec<
				T::BidId, ConstU32<{VEC_LIMIT}>
			>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn rfp_to_accepted_bid)]
	pub type RFPToAcceptedBid<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::RFPId, // rfp_id
		T::BidId, // bid_id
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
		/// [account, rfp, bid_id]
		BidOnRFP(T::AccountId, T::RFPId, T::BidId),
		/// RFP Admin creates a shortlist of the bids on an RFP
		/// [account, rfp, bid_id]
		ShortlistBid(T::AccountId, T::RFPId, T::BidId),
		/// Updates a bid on an RFP
		/// [account, rfp, bid_id]
		UpdateRFPBid(T::AccountId, T::RFPId, T::BidId),
		/// Accepts a bid on an RFP
		/// [account, rfp, bid_id]
		AcceptRFPBid(T::AccountId, T::RFPId, T::BidId),
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

		/// Trying to create a bid under an ID that already exists
		BidAlreadyExists,

		/// Reached maximum amount of bids
		TooManyBids,

		/// Did not find Bids for RFP
		NoBidsForRFPFound,

		/// Trying to update a non-existent bid
		UpdatingNonExistentBid,

		/// Someone other than the bid owner attempted
		/// to update the bid details
		UnauthorizedUpdateOfBid,

		/// General error for non existent RFP
		NonExistentRFP,

		/// Trying to shortlist a bid that doesn't exist
		ShortlistingNonExistentBid,

		/// Trying to shortlist for an RFP that has no bids
		NoBidsForRFP,

		/// Trying to shortlist a bid that was not made for this RFP
		NoSuchBidForRFP,

		/// Trying to accept a bid that was not in the shortlist
		AcceptedBidNotShortlisted,

		/// Error if the RFP has no shortlist
		RFPHasNoShortlist,

		/// Accepting a bid for an RFP that has already had a bid accepted
		BidAlreadyAccepted,

		/// Bid was accepted, but there was an error with the payment initialization
		PaymentInitializationFailed,

		/// Bid on an RFP that is not currently accepting new bids
		RFPNotAcceptingBids
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
			let rfps_to_bids: BoundedVec<
				T::BidId, ConstU32<{VEC_LIMIT}>
			> = BoundedVec::<
					T::BidId, ConstU32<{VEC_LIMIT}>
				>::default();
			<RFPToBids<T>>::insert::<&T::RFPId, BoundedVec<
				T::BidId, ConstU32<{VEC_LIMIT}>
			>>(
				&rfp_id,
				rfps_to_bids.into()
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
			// TODO: Delete bids associated with this RFP as well
			Self::deposit_event(Event::CancelRFP(rfp_owner, rfp_id));
			Ok(())
		}

		/// A dispatchable to Bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn bid_on_rfp(
			origin: OriginFor<T>, 
			rfp_owner: T::AccountId, 
			rfp_id: T::RFPId, 
			bid_id: T::BidId,
			bid_details: BidDetails<T>
		) -> DispatchResult {
			let bid_owner = ensure_signed(origin)?;
			let rfp_details = <RFPs<T>>::get(
				&rfp_owner,
				&rfp_id
			).ok_or(<Error<T>>::NonExistentRFP)?;
			ensure!(
				rfp_details.rfp_status == RFPStatus::AcceptingBids,
				<Error<T>>::RFPNotAcceptingBids
			);
			ensure!(
				!<AllBids<T>>::contains_key(
					&bid_id
				),
				<Error<T>>::BidAlreadyExists
			);
			<AllBids<T>>::insert(
				&bid_id,
				bid_details
			);
			<RFPToBids<T>>::try_mutate(
				&rfp_id,
				| maybe_bids_for_rfp | -> DispatchResult {
					let bids_for_rfp = 
						maybe_bids_for_rfp.as_mut()
							.ok_or(
								<Error<T>>::NoBidsForRFPFound
							)?;
					bids_for_rfp
						.try_push(bid_id)
						.ok()
						.ok_or(
							<Error<T>>::TooManyBids
						)?;
					Ok(())
				}
			)?;

			Self::deposit_event(Event::BidOnRFP(bid_owner, rfp_id, bid_id));
			Ok(())
		}

		/// A dispatchable to create a shortlist of bids
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn shortlist_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			bid_id: T::BidId,
		) -> DispatchResult {
			let rfp_owner = ensure_signed(origin)?;
			let maybe_rfp_details = <RFPs<T>>::get(
				&rfp_owner,
				&rfp_id,
			);
			ensure!(maybe_rfp_details.is_some(),
				Error::<T>::NonExistentRFP
			);
			let maybe_bid_details = <AllBids<T>>::get(
				&bid_id
			);
			ensure!(maybe_bid_details.is_some(),
				Error::<T>::ShortlistingNonExistentBid
			);

			let all_bids_for_rfp = <RFPToBids<T>>::get(
				&rfp_id,
			).ok_or(
				Error::<T>::NoBidsForRFP
			)?;
			
			ensure!(
				all_bids_for_rfp.contains(&bid_id),
				Error::<T>::NoSuchBidForRFP
			);
			let maybe_shortlisted_bids = <RFPToShortlistedBids<T>>::get(
				&rfp_id,
			);
			if let Some(mut _shortlisted_bids) = maybe_shortlisted_bids {
				<RFPToShortlistedBids<T>>::mutate(
					&rfp_id,
					| maybe_shortlisted_bids | -> DispatchResult {
						let shortlisted_bids = maybe_shortlisted_bids.as_mut().unwrap();
						shortlisted_bids.try_push(bid_id)
							.ok()
							.ok_or(
								<Error<T>>::TooManyBids
							)?;
						Ok(())
					}
				)?;
			} else {
				let mut shortlisted_bids = BoundedVec::<
					T::BidId, ConstU32<{VEC_LIMIT}>
				>::default();
				shortlisted_bids.try_push(bid_id)
					.ok()
					.ok_or(
						<Error<T>>::TooManyBids
					)?;
				<RFPToShortlistedBids<T>>::insert::<&T::RFPId, BoundedVec<
					T::BidId, ConstU32<{VEC_LIMIT}>
				>>(
					&rfp_id,
					shortlisted_bids.into()
				);
			}
			Self::deposit_event(
				Event::ShortlistBid(
					rfp_owner,
					rfp_id,
					bid_id
				)
			);
			Ok(())
		}

		/// A dispatchable to update a bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn update_rfp_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			bid_id: T::BidId,
			updated_bid_details: BidDetails<T>
		) -> DispatchResult {
			let updater_id = ensure_signed(origin)?;
			<AllBids<T>>::try_mutate(
				&bid_id,
				| maybe_bid_details | -> DispatchResult {
					let bid_details = 
						maybe_bid_details.as_mut()
							.ok_or(
								<Error<T>>::UpdatingNonExistentBid
							)?;
					ensure!(
						updater_id == bid_details.bid_owner,
						<Error<T>>::UnauthorizedUpdateOfBid,
					);
					*bid_details = updated_bid_details;
					Ok(())
				}
			)?;
			Self::deposit_event(
				Event::UpdateRFPBid(
					updater_id, 
					rfp_id, 
					bid_id
				)
			);
			Ok(())
		}

		/// A dispatchable to accept a bid on an RFP
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn accept_rfp_bid(
			origin: OriginFor<T>, 
			rfp_id: T::RFPId, 
			bid_id: T::BidId,
			payment_details: pallet_payments::PaymentDetails::<T>
		) -> DispatchResult {
			let rfp_owner = ensure_signed(origin.clone())?;

			<RFPs<T>>::try_mutate(
				&rfp_owner,
				&rfp_id,
				| maybe_rfp_details | -> DispatchResult {
					let rfp_details = 
						maybe_rfp_details.as_mut()
							.ok_or(
								<Error<T>>::NonExistentRFP
							)?;
					ensure!(
						rfp_details.rfp_status != RFPStatus::AcceptedBid,
						<Error<T>>::BidAlreadyAccepted
					);

					let shortlisted_bids = <RFPToShortlistedBids<T>>::get(
						&rfp_id
					);

					if shortlisted_bids.is_some() {
						// If there has been a shortlisting process, 
						// ensure that the bid to be accepted has been
						// shortlisted
						ensure!(
							shortlisted_bids.unwrap().contains(&bid_id),
							Error::<T>::AcceptedBidNotShortlisted
						);
					} else {
						// Otherwise, just make sure the bid exists
						let all_bids_for_rfp = <RFPToBids<T>>::get(
							&rfp_id,
						).ok_or(
							Error::<T>::NoBidsForRFP
						)?;
						ensure!(
							all_bids_for_rfp.contains(&bid_id),
							Error::<T>::NoSuchBidForRFP
						);
					}
		
					ensure!(
						<RFPToAcceptedBid<T>>::get(&rfp_id).is_none(),
						Error::<T>::BidAlreadyAccepted
					);
		
					<pallet_payments::Pallet<T>>::initialize_payment(
						origin,
						payment_details
					).ok().ok_or(Error::<T>::PaymentInitializationFailed)?;

					<RFPToAcceptedBid<T>>::insert(
						&rfp_id,
						&bid_id,
					);
					rfp_details.rfp_status = RFPStatus::AcceptedBid;
					Ok(())
				}
			)?;

			Self::deposit_event(
				Event::AcceptRFPBid(
					rfp_owner, 
					rfp_id,
					bid_id,
				)
			);
			Ok(())
		}
	}
}