// This file is part of Ventur, it implements a multi admin locked fund 
// account, with configurations for open contribution.

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

//! # Escrow Pallet
//!
//! The Escrow pallet provides functionality for multi admin accounts, with lockable funds, and options for open contribution to the locked funds.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Escrow pallet provides functions for:
//!
//! - Creating an Escrow within an AccountId.
//! - Assigning and Removing Admins that are allowed to manage the Escrow.
//! - Contributing funds to the Escrow.
//! - Distributing funds from the Escrow.
//! - Enabling and Disabling Open (Non Admin) Contributions to the Locked Escrow Funds
//! - Freezing and Thawing the Escrow.
//! - Closing the Escrow, and proportionally disbursing the remaining funds back to contributors.
//! 
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create_escrow` - Creates an Escrow, tied to the calling AccountId.
//! - `fund_escrow` - Transfers funds to the Escrow Account and Locks the transfered amount.
//! - `payout_escrow` - Distributes funds from the otherwise locked Escrow funds in an Account.
//! - `close_escrow` - Closes out an Escrow, by distributing all locked funds out to the contributors, proportionately to their contributions.
//! - `enable_open_contribution` - Enables non admins to contribute to locked funds using the fund_escrow dispatchable.
//! - `disable_open_contribution` - Prevents non admins from contributing to locked funds using the fund_escrow dispatchable.
//! - `freeze_escrow` - Freezes an Escrow, preventing any distributions, contributions, or changes in configuration.
//! - `thaw_escrow` - Removes a freeze from an Escrow, enabling distributions, contributions, and changes in configuration.
//! - `add_admin` - Adds an admin to the Escrow's admins.
//! - `remove_admin` - Removes an admin from the Escrow's admins.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*, 
		sp_std::vec, 
		traits::{
			Currency,
			LockIdentifier,
			LockableCurrency,
			WithdrawReasons, 
			ExistenceRequirement::AllowDeath
		}, 
		sp_runtime::{traits::{Zero, CheckedSub}}
	};
	use frame_system::pallet_prelude::*;

	pub const VEC_LIMIT: u32 = u32::MAX;
	const ESCROW_LOCK: LockIdentifier = *b"Escrowed";

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type PaymentCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Contribution<AccountId, T: Config>  {
		pub(super) contributor: AccountId,
		pub(super) amount: BalanceOf<T>,
	}
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct EscrowDetails<AccountId, T:Config> {
		pub(super) admins: BoundedVec<AccountId, ConstU32<{VEC_LIMIT}>>,
		pub(super) contributions: BoundedVec<Contribution<AccountId, T>, ConstU32<{VEC_LIMIT}>>,
		pub(super) amount: BalanceOf<T>,
		pub(super) total_contributed: BalanceOf<T>,
		pub(super) is_frozen: bool,
		pub(super) is_open: bool,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn escrow)]
	pub(super) type Escrow<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, EscrowDetails<T::AccountId, T>, OptionQuery>;

	// The Administrator storage map needs to be reevaluated, 
	// originally intended to provide a quick means of querying all of the escrows tied to an account
	// need to confirm if storage costs are worthwhile functionally, or if it makes sense to use an indexer for this
	#[pallet::storage]
	#[pallet::getter(fn administrator)]
	pub(super) type Administrator<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, T::BlockNumber, OptionQuery>;

	pub type BalanceOf<T> = <<T as Config>::PaymentCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Creates Escrow Object, notes created Escrow and Admin account
		/// [account]
		CreateEscrow(T::AccountId),
		/// Adds Funds to Escrow Object, notes Escrow Id, contributing Account Id, and the amount contributed
		/// [escrow, who, amount]
		FundEscrow(T::AccountId, T::AccountId, BalanceOf<T>),
		/// Paysout Funds from Escrow Object, notes Escrow Id, receiving Account Id, and the amount distributed
		/// [escrow, who, payee, amount]
		PayoutEscrow(T::AccountId, T::AccountId, T::AccountId, BalanceOf<T>),
		/// Closes the escrow, notes the escrow Id and admin id (this results in the dispersment of remaining funds among contributors proportionate to contributions)
		/// [escrow, who]
		CloseEscrow(T::AccountId, T::AccountId),
		/// Sets the open bool to true and allows for any account to Fund the Escrow
		/// [escrow, who]
		EnableOpenContribution(T::AccountId, T::AccountId),
		/// Sets the open bool to false and only allows for admin accounts to fund the Escrow
		/// [escrow, who]
		DisableOpenContribution(T::AccountId, T::AccountId),
		/// Freezes the escrow
		/// [escrow, who]
		FreezeEscrow(T::AccountId, T::AccountId),
		/// Thaws the escrow
		/// [escrow, who]
		ThawEscrow(T::AccountId, T::AccountId),
		/// Adds Administrator
		/// [escrow, who, new_admin]
		AddAdministrator(T::AccountId, T::AccountId, T::AccountId),
		/// Remove Administrator
		/// [escrow, who, admin_to_remove]
		RemoveAdministrator(T::AccountId, T::AccountId, T::AccountId)
	}

	// Errors inform users that escrow went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error on transaction unsigned
		Unsigned,
		/// Error on None value
		NoneValue,
		/// Error on Storage Overflow
		StorageOverflow,
		/// Can not create Escrow that already exists
		EscrowAlreadyCreated,
		/// No Escrow exists for the referenced AccountId
		NoSuchEscrow,
		/// User is not authorized to perform this action on the escrow
		Unauthorized,
		/// Escrow is frozen, no configurations, contributions, or distributions are possible until thawed
		Frozen,
		/// Open configuration already enabled
		OpenAlreadyEnabled,
		/// Open configuration already disabled
		OpenAlreadyDisabled,
		/// Unfrozen Escrow cannot be thawed
		AlreadyNotFrozen,
		/// Admins are not allowed to distribute funds to themselves
		SelfDistributionAttempt,
		/// The AccountId that was attempted to be added is already an admin
		AdminAlreadyPresent,
		/// The AccountId that was attempted to be removed from admin, is already not an admin
		AdminNotPresent,
		/// Admin storage mutation failed
		ErrorOnPushAdmin,
		/// The Escrow has insufficient funds locked to fulfill the intended transfer
		InsufficientEscrowFunds,
		/// Funder has insufficient balance for the intended transfer into the escrow
		InsufficientBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A dispatchable to create an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_escrow(origin: OriginFor<T>) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&who);
			ensure!(
				escrow_details.is_none(),
				Error::<T>::EscrowAlreadyCreated
			);

			// Insert new Escrow and Administrator into Storage
			let admins: BoundedVec<T::AccountId, ConstU32<{VEC_LIMIT}>> = vec![who.clone()].try_into().unwrap();
			let contributions: BoundedVec<Contribution<T::AccountId, T>, ConstU32<{VEC_LIMIT}>> = vec![].try_into().unwrap();
			<Escrow<T>>::insert(
				who.clone(), 
				EscrowDetails {
					admins,
					contributions,
					amount: BalanceOf::<T>::zero(),
					total_contributed: BalanceOf::<T>::zero(),
					is_frozen: false,
					is_open: false,
				});
			<Administrator<T>>::insert(
				who.clone(),
				who.clone(),
				<frame_system::Pallet<T>>::block_number(),
			);

			T::PaymentCurrency::set_lock(
				ESCROW_LOCK,
				&who,
				BalanceOf::<T>::zero(),
				WithdrawReasons::all(),
			);

			// Emit an event.
			Self::deposit_event(Event::CreateEscrow(who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to fund an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn fund_escrow(origin: OriginFor<T>, escrow_id: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			// Check that our caller has signed the transaction
			let funder = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// If escrow isn't open, confirm that origin is an admin
			if !escrow_details.is_open {
			ensure!(
				escrow_details.admins.iter().any(|x| *x == funder.clone()),
				Error::<T>::Unauthorized
			);
			}

			// Confirm contribution is smaller than escrow amount
			T::PaymentCurrency::ensure_can_withdraw(
				&funder,
				amount,
				WithdrawReasons::all(),
				T::PaymentCurrency::free_balance(&funder).checked_sub(&amount).unwrap()
			)?;
			ensure!(
				T::PaymentCurrency::free_balance(&funder) >= amount,
				Error::<T>::InsufficientBalance
			);

			// Update Escrow storage
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.amount += amount;
					escrow_details.total_contributed += amount;

					let contribution = Contribution {
						contributor: funder.clone(),
						amount,
					};
					escrow_details.contributions.try_push(contribution).ok();

					T::PaymentCurrency::transfer(
						&funder,
						&escrow_id,
						amount,
						AllowDeath,
					)?;
					T::PaymentCurrency::set_lock(
						ESCROW_LOCK,
						&escrow_id,
						escrow_details.amount,
						WithdrawReasons::all(),
					);

					Ok(())
				}
			)?;

			// Emit an event.
			Self::deposit_event(Event::FundEscrow(escrow_id, funder, amount));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to payout from an escrow 
		/// --This functionality may be limitted to payouts from RFPs only once the RFP pallet is implemented
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn payout_escrow(origin: OriginFor<T>, payee: T::AccountId, escrow_id: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Confirm that payee is not an admin
			ensure!(
				!escrow_details.admins.iter().any(|x| *x == payee),
				Error::<T>::SelfDistributionAttempt
			);

			// Confirm distribution is smaller than escrow amount
			ensure!(
				(escrow_details.amount >= amount),
				Error::<T>::InsufficientEscrowFunds
			);
			
			T::PaymentCurrency::remove_lock(ESCROW_LOCK, &escrow_id);

			// Send funds to payee
			T::PaymentCurrency::transfer(
				&escrow_id,
				&payee,
				amount,
				AllowDeath,
			)?;

			// Update Escrow storage
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					escrow_details.amount -= amount;
					
					T::PaymentCurrency::set_lock(
						ESCROW_LOCK,
						&escrow_id.clone(),
						escrow_details.amount,
						WithdrawReasons::all(),
					);
					Ok(())
				}
			)?;

			// Emit an event.
			Self::deposit_event(Event::PayoutEscrow(escrow_id, who, payee, amount));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to close an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn close_escrow(origin: OriginFor<T>, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Cast the Total Contributed and Current Balance from Escrow to u128s 
				// for use in calculating the distribution of the remaining balance
			let escrow_total_at_closing: u128 = 
				TryInto::<u128>::try_into(escrow_details.amount).ok().unwrap();
			let escrow_total_contributed: u128 = 
				TryInto::<u128>::try_into(escrow_details.total_contributed).ok().unwrap();
			
			// Unlock Escrow for Distribution
			T::PaymentCurrency::remove_lock(ESCROW_LOCK, &escrow_id);
			
			// Distribute remaining funds to contributors proportionately to their contributions
			escrow_details.contributions.iter().for_each(|contribution|{
				let contributed_amount: u128 = TryInto::<u128>::try_into(contribution.amount).ok().unwrap();
				// Calculate their disbursement
				let close_disbursement: u128 = 
					(escrow_total_at_closing as f64 * (contributed_amount as f64/escrow_total_contributed as f64)) as u128;
				// Transfer the funds to the contributor
				T::PaymentCurrency::transfer(
					&escrow_id.clone(),
					&contribution.contributor,
					close_disbursement.try_into().ok().unwrap(),
					AllowDeath,
				).ok();
			});

			// Remove Escrow and Administrator from Storage
			<Escrow<T>>::remove(escrow_id.clone());
			// Remove all Admins
			escrow_details.admins.iter().for_each(|admin|{
				<Administrator<T>>::remove(
					admin.clone(),
					escrow_id.clone()
				);
			});


			// Emit an event.
			Self::deposit_event(Event::CloseEscrow(escrow_id, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to open an account for contributions from non admins
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn enable_open_contribution(origin: OriginFor<T>, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);

			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);
			
			// Update Escrow storage to set is_open
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					escrow_details.is_open = true;
					Ok(())
				}
			)?;
			
			// Emit event
			Self::deposit_event(Event::EnableOpenContribution(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to disable contributions from non admins to the locked escrow funds.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn disable_open_contribution(origin: OriginFor<T>, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_open
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					escrow_details.is_open = false;
					Ok(())
				}
			)?;

			// Emit event
			Self::deposit_event(Event::DisableOpenContribution(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to freeze an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn freeze_escrow(origin: OriginFor<T>, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_frozen
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					escrow_details.is_frozen = true;
					Ok(())
				}
			)?;

			// Emit event
			Self::deposit_event(Event::FreezeEscrow(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to thaw an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn thaw_escrow(origin: OriginFor<T>, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;

			// Check escrow is frozen
			ensure!(
				escrow_details.is_frozen,
				Error::<T>::AlreadyNotFrozen
			);

			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_frozen
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					escrow_details.is_frozen = false;
					Ok(())
				}
			)?;
			
			// Emit event
			Self::deposit_event(Event::ThawEscrow(escrow_id, who));
			Ok(())
		}

		/// A dispatchable to add an administrator
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn add_admin(origin: OriginFor<T>, new_admin: T::AccountId, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Confirm that Admin is not already present
			ensure!(
				!escrow_details.admins.iter().any(|x| *x == new_admin.clone()),
				Error::<T>::AdminAlreadyPresent
			);

			// Insert new Escrow and Administrator into Storage
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					// Add admin to vector
					escrow_details.admins.try_push(new_admin.clone()).ok();
					Ok(())
				}
			)?;
			<Administrator<T>>::insert(
				new_admin.clone(),
				escrow_id.clone(),
				<frame_system::Pallet<T>>::block_number(),
			);

			// Emit an event.
			Self::deposit_event(Event::AddAdministrator(escrow_id, who, new_admin));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		
		/// A dispatchable to remove an administrator
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn remove_admin(origin: OriginFor<T>, admin_to_remove: T::AccountId, escrow_id: T::AccountId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id).ok_or(<Error<T>>::NoSuchEscrow)?;
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);
			
			// Confirm Admin is present to be removed
			ensure!(
				escrow_details.admins.iter().any(|x| *x == admin_to_remove.clone()),
				Error::<T>::AdminNotPresent
			);

			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details = maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoSuchEscrow)?;
					
					// Remove admin from vector
					escrow_details.admins.remove(
						escrow_details.admins.iter().position(|x| *x == admin_to_remove.clone()).unwrap()
					);
					Ok(())
				}
			)?;
			// Remove Admin
			<Administrator<T>>::remove(
				who.clone(),
				escrow_id.clone(),
			);

			// Emit an event.
			Self::deposit_event(Event::RemoveAdministrator(escrow_id, who, admin_to_remove));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
