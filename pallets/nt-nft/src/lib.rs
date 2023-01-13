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

// Attribution - Parity - Uniques Pallet
// This pallet's approach to interfaces/extrinsics for nfts is directly 
// inspired by Parity's uniques pallet.  Below is a link to their source 
// code, as well as the Apache 2.0 license that applies to it.

// https://github.com/paritytech/substrate/blob/master/frame/uniques/src/lib.rs
// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
//! - Updating NT-NFTs metadata.
//! - Assigning NT-NFTs.
//! - Burning NT-NFTs.
//! - Discarding NT-NFTs.
//! 
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create_collection` - Creates a collection administered by the calling AccountId.
//! - `freeze_collection` - Freeze a collection, preventing any further actions from being performed on the collection until it is thawed.
//! - `thaw_collection` - Thaw a frozen collection, allowing actions to be performed on the collection.
//! - `destroy_collection` - Destroy a collection. <This extrinsic is not yet implemented.>
//! - `assign_ntnft` - Assign a NT-NFT to an AccountId.
//! - `accept_assignment` - Accept an NT-NFT assignment.
//! - `cancel_assignment` - Cancel an NT-NFT assignment.
//! - `mint_ntnft` - Mint a new NT-NFT in a collection.
//! - `update_ntnft` - Update the metadata of a NT-NFT. <This extrinsic is not yet implemented.>
//! - `burn_ntnft` - Burn a minted unassigned NT-NFT from a collection.
//! - `discard_ntnft` - Discard an assigned NT-NFT.

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
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct CollectionDetails<AccountId> {
		pub(super) owner: AccountId,
		pub(super) amount: u32,
		pub(super) is_frozen: bool,
		pub(super) image_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
		pub(super) metadata_ipfs_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>>,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub struct ItemDetails<AccountId> {
		// maybe differentiate minter from owner
		// On assignment but not yet accepted, who is the owner?
		pub(super) owner: AccountId,
		pub(super) is_assigned: bool,
		pub(super) is_accepted: bool,
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
	pub(super) type Collection<T: Config> = 
		StorageMap<_, 
			Blake2_128Concat, 
			T::CollectionId, 
			CollectionDetails<T::AccountId>, 
			OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn assignment)]
	pub(super) type Assignment<T: Config> = 
		StorageDoubleMap<_, 
			Blake2_128Concat, 
			T::CollectionId, 
			Blake2_128Concat, 
			T::AccountId, 
			T::ItemId, 
			OptionQuery>;
			
	#[pallet::storage]
	#[pallet::getter(fn proposed_assignment)]
	pub(super) type ProposedAssignment<T: Config> = 
		StorageDoubleMap<_, 
			Blake2_128Concat, 
			T::CollectionId, 
			Blake2_128Concat, 
			T::AccountId, 
			T::ItemId, 
			OptionQuery>;
	
	// ToDo: Evaluate whether keeping cancelled assignments is worth the storage cost.
	#[pallet::storage]
	#[pallet::getter(fn canceled_assignment)]
	pub(super) type CanceledAssignment<T: Config> = 
		StorageDoubleMap<_, 
			Blake2_128Concat, 
			T::CollectionId, 
			Blake2_128Concat, 
			T::AccountId, 
			T::ItemId, 
			OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn item)]
	pub(super) type Item<T: Config> = 
		StorageDoubleMap<_, 
			Blake2_128Concat, 
			T::CollectionId, 
			Blake2_128Concat, 
			T::ItemId, 
			ItemDetails<T::AccountId>, 
			OptionQuery>;
	
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
		/// CollectionId already exists
		CollectionIdAlreadyExists,
		/// CollectionId does not exist
		CollectionIdDoesNotExist,
		/// Collection Is Frozen
		CollectionFrozen,
		/// Collection is not Frozen (so thaw will error)
		CollectionNotFrozen,
		/// ItemId does not exist
		ItemIdDoesNotExist,
		/// ItemId already exists (error on repeated minting of same id)
		ItemIdAlreadyExists,
		/// Item is not assigned, accepting assignment fails
		ItemIsNotAssigned,
		/// Item is assigned, cannot be assigned
		ItemIsAlreadyAssigned,
		/// Caller is not authorized to perform this action
		Unauthorized,
		/// Caller is attempting to accept an ntnft they do not have an assignment for
		NoAssignmentForThisAccount,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        /// A dispatchable to create an NT-NFT Collection
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

		/// A dispatchable to freeze an NT-NFT Collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn freeze_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			// Ensure transaction signed, collection exists, and caller is authorized
			let who = ensure_signed(origin)?;
			
			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;
			
			// Check that collection is not frozen
			ensure!(!collection_details.is_frozen, <Error<T>>::CollectionFrozen);

			// Ensure that the caller is the owner
			ensure!(who == collection_details.owner, <Error<T>>::Unauthorized);

			// Freeze the account
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					collection_details.is_frozen = true;
					Ok(())
				}
			)?;
			Self::deposit_event(Event::FreezeCollection(collection_id, who));
			Ok(())
		}

		/// A dispatchable to thaw an NT-NFT Collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn thaw_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			// Ensure transaction signed, collection exists, and caller is authorized
			let who = ensure_signed(origin)?;
		
			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;
			
			// Check that collection is frozen
			ensure!(collection_details.is_frozen, <Error<T>>::CollectionNotFrozen);

			// Ensure that the caller is the owner
			ensure!(who == collection_details.owner, <Error<T>>::Unauthorized);

			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					collection_details.is_frozen = false;
					Ok(())
				}
			)?;
			Self::deposit_event(Event::ThawCollection(collection_id, who));
			Ok(())
		}

		/// A dispatchable to mint an NT-NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn mint_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// Ensure transaction signed, collection exists, and caller is authorized
			let who = ensure_signed(origin)?;

			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;

			// Check that collection is not frozen
			ensure!(!collection_details.is_frozen, <Error<T>>::CollectionFrozen);

			// Ensure that the caller is the owner
			ensure!(who == collection_details.owner, <Error<T>>::Unauthorized);

			// Check that item does not already exist
			ensure!(!<Item<T>>::contains_key(&collection_id, &ntnft_id), <Error<T>>::ItemIdAlreadyExists);

			// Insert Item and Update Collection
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					let new_amount = 
						collection_details.amount.checked_add(1).ok_or(<Error<T>>::StorageOverflow)?;
					collection_details.amount = new_amount;
					let item = ItemDetails{
						owner: who.clone(),
						is_assigned: false,
						is_accepted: false,
					};
					<Item::<T>>::insert(&collection_id, &ntnft_id, item);
					Ok(())
				}
			)?;
			
			// Deposit Event
			Self::deposit_event(Event::MintNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}

		/// A dispatchable to burn an NT-NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn burn_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// Ensure transaction signed
			let who = ensure_signed(origin)?;

			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;

			// Check that collection is not frozen
			ensure!(!collection_details.is_frozen, <Error<T>>::CollectionFrozen);

			// Ensure that the caller is the owner
			ensure!(who == collection_details.owner, <Error<T>>::Unauthorized);

			// Check that item exists
			let item = <Item<T>>::get(&collection_id, &ntnft_id).ok_or(<Error<T>>::ItemIdDoesNotExist)?;

			// Update Collection
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					let new_amount = 
						collection_details.amount.checked_sub(1).ok_or(<Error<T>>::StorageOverflow)?;
					collection_details.amount = new_amount;
					Ok(())
				}
			)?;

			// Remove Item
			if item.is_accepted {
				<Assignment<T>>::remove(&collection_id, &item.owner);
			} else if item.is_assigned {
				<ProposedAssignment<T>>::remove(&collection_id, &item.owner);
			}
			<Item<T>>::remove(&collection_id, &ntnft_id);
			
			// Deposit Event
			Self::deposit_event(Event::BurnNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}

		/// A dispatchable to assign an NT-NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1).ref_time())]
		pub fn assign_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId, target_address: T::AccountId) -> DispatchResult {
			// Ensure transaction is signed
			let who = ensure_signed(origin)?;

			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;
			
			// Check that collection is not frozen
			ensure!(!collection_details.is_frozen, <Error<T>>::CollectionFrozen);

			// Ensure that the caller is the owner
			ensure!(who == collection_details.owner, <Error<T>>::Unauthorized);

			// Ensure the Item exists
			ensure!(<Item<T>>::contains_key(&collection_id, &ntnft_id), <Error<T>>::ItemIdDoesNotExist);

			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::ItemIdDoesNotExist)?;
					ensure!(!item_details.is_accepted && !item_details.is_assigned, <Error<T>>::ItemIsAlreadyAssigned);
					item_details.is_assigned = true;
					<ProposedAssignment<T>>::insert(&collection_id, &target_address, ntnft_id);
					Ok(())
				}
			)?;

			// Deposit Event
			Self::deposit_event(Event::AssignNTNFT(who, collection_id, ntnft_id, target_address));
			Ok(())
		}

		/// A dispatchable to accept an NT-NFT assignment
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn accept_assignment(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// Ensure transaction is signed
			let who = ensure_signed(origin)?;

			// Check that target has a proposed assignment
			ensure!(<ProposedAssignment<T>>::contains_key(&collection_id, &who), <Error<T>>::NoAssignmentForThisAccount);
			
			// Update item assignment
			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::ItemIdDoesNotExist)?;
					ensure!(
						!item_details.is_accepted && 
						item_details.is_assigned, 
						<Error<T>>::ItemIsNotAssigned
					);
					item_details.is_accepted = true;
					<ProposedAssignment<T>>::remove(&collection_id, &who);
					<Assignment<T>>::insert(&collection_id, &who, ntnft_id);
					Ok(())
				}
			)?;

			// Deposit Event
			Self::deposit_event(Event::AcceptAssignment(collection_id, ntnft_id, who));
			Ok(())
		}

		/// A dispatchable to reject an NT-NFT assignment
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1).ref_time())]
		pub fn cancel_assignment(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId, target_address: T::AccountId) -> DispatchResult {
			// Ensure transaction is signed
			let who = ensure_signed(origin)?;

			// Check that target has a proposed assignment
			ensure!(<ProposedAssignment<T>>::contains_key(&collection_id, &target_address), <Error<T>>::NoAssignmentForThisAccount);

			// Check that collection exists
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::CollectionIdDoesNotExist)?;

			// Check that collection is not frozen
			ensure!(!collection_details.is_frozen, <Error<T>>::CollectionFrozen);

			// Check that caller is authorized to call cancel (either collection owner, or asignee)
			ensure!(<ProposedAssignment<T>>::contains_key(&collection_id, &who)||who == collection_details.owner, <Error<T>>::Unauthorized);

			// Update item and cancel assignment
			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::ItemIdDoesNotExist)?;
					ensure!(
						!item_details.is_accepted && 
						item_details.is_assigned,   
						<Error<T>>::ItemIsNotAssigned
					);
					item_details.is_accepted = false;
					item_details.is_assigned = false;
					<ProposedAssignment<T>>::remove(&collection_id, &who);
					<CanceledAssignment<T>>::insert(&collection_id, &target_address, ntnft_id);
					Ok(())
				}
			)?;

			
			Self::deposit_event(Event::CancelAssignment(who, collection_id, ntnft_id, target_address));
			Ok(())
		}

		/// A dispatchable to discard an NT-NFT assignment
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn discard_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// Ensure transaction is signed
			let who = ensure_signed(origin)?;

			// Check that the caller has the ntnft
			ensure!(<Assignment<T>>::contains_key(&collection_id, &who), <Error<T>>::NoAssignmentForThisAccount);

			// Update item to unassign ntnft from the caller
			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::ItemIdDoesNotExist)?;
					item_details.is_accepted = false;
					item_details.is_assigned = false;
					<Assignment<T>>::remove(&collection_id, &who);
					Ok(())
				}
			)?;

			// Deposit Event
			Self::deposit_event(Event::DiscardNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}
	}
}