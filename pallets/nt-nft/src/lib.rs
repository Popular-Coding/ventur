// This file is part of Ventur, it implements an NT-NFT, 
// Non-Transferable NFT, Substrate Pallet.

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

//! # NT-NFT Pallet
//!
//! The NT-NFT pallet provides functionality for creation, distribution, and management of NT-NFTs.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The NT-NFT pallet provides functions for:
//!
//! - Creating NT-NFT Collections.
//! - Freezing Collections.
//! - Thawing Collections.
//! - Destroying Collections.
//! - Minting NT-NFTs.
//! - Assigning NT-NFTs.
//! - Burning NT-NFTs.
//! - Discarding NT-NFTs.
//! 
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create_collection` - 
//! - `freeze_collection` - 
//! - `thaw_collection` - 
//! - `destroy_collection` - 
//! - `assign_ntnft` - 
//! - `accept_assignment` - 
//! - `cancel_assignment` - 
//! - `mint_ntnft` - 
//! - `burn_ntnft` - 
//! - `discard_ntnft` - 

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

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CollectionDetails<AccountId> {
		pub(super) owner: AccountId,
		pub(super) amount: u32,
		pub(super) is_frozen: bool,
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

	#[pallet::storage]
	#[pallet::getter(fn collection)]
	pub(super) type Collection<T: Config> = StorageMap<_, Blake2_128Concat, T::CollectionId, CollectionDetails<T::AccountId>, OptionQuery>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create a Collection of NT-NFTs
		/// [collection, account]
		CreateCollection(T::CollectionId, T::AccountId),
        /// Destroy a Collection of NT-NFTs
		/// [collection, account]
        DestroyCollection(T::CollectionId, T::AccountId),
        /// Freeze a Collection of NT-NFTs
		/// [collection, account]
        FreezeCollection(T::CollectionId, T::AccountId),
        /// Thaw a Collection of NT-NFTs
		/// [collection, account]
		ThawCollection(T::CollectionId, T::AccountId),

        /// Mint an NT-NFT
		/// [collection, ntnft, account]
        MintNTNFT(T::CollectionId, T::ItemId, T::AccountId),
        /// Burn an NT-NFT
		/// [collection, ntnft, account]
		BurnNTNFT(T::CollectionId, T::ItemId, T::AccountId),
        /// Assign an NT-NFT
		/// [who, collection, ntnft, account]
        AssignNTNFT(T::AccountId, T::CollectionId, T::ItemId, T::AccountId),
        /// Accept an NT-NFT
		/// [collection, ntnft, account]
		AcceptAssignment(T::CollectionId, T::ItemId, T::AccountId),
        /// Cancel an NT-NFT assignment
		/// [who, collection, ntnft, account]
		CancelAssignment(T::AccountId, T::CollectionId, T::ItemId, T::AccountId),
        /// Discard an NT-NFT
		/// [collection, ntnft, account]
		DiscardNTNFT(T::CollectionId, T::ItemId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
        /// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        /// A dispatchable to create an NT-NFT Collection
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Collection<T>>::contains_key(&collection_id), <Error<T>>::NoneValue);
			<Collection<T>>::insert(
				collection_id, 
				CollectionDetails {
					owner: who.clone(),
					amount: 0,
					is_frozen: false,
				});

			Self::deposit_event(Event::CreateCollection(collection_id, who));
			Ok(())
		}
	}
}