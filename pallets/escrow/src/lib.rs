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
	use frame_support::{pallet_prelude::*, sp_std::vec, traits::{Currency, ExistenceRequirement::AllowDeath}, sp_runtime::{traits::Zero/* , SaturatedConversion */}};
	use frame_system::pallet_prelude::*;

	pub const VEC_LIMIT: u32 = u32::MAX;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type EscrowId: Member + Parameter + MaxEncodedLen + Clone + Eq + TypeInfo + IsType<<Self as frame_system::Config>::AccountId>;
		type PaymentCurrency: Currency<Self::AccountId>;
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
		pub(super) owner: AccountId,
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
	pub(super) type Escrow<T: Config> = StorageMap<_, Blake2_128Concat, T::EscrowId, EscrowDetails<T::AccountId, T>, OptionQuery>;

	// The Administrator storage map needs to be reevaluated, 
	// originally intended to provide a quick means of querying all of the escrows tied to an account
	// need to confirm if storage costs are worthwhile functionally, or if it makes sense to use an indexer for this
	#[pallet::storage]
	#[pallet::getter(fn administrator)]
	pub(super) type Administrator<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::EscrowId, T::BlockNumber, OptionQuery>;

	pub type BalanceOf<T> = <<T as Config>::PaymentCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [escrow, who]

		/// Escrow Events
		// Creates Escrow Object, notes created Escrow and Admin account
		CreateEscrow(T::EscrowId, T::AccountId),
		// Adds Funds to Escrow Object, notes Escrow Id, contributing Account Id, and the amount contributed
		FundEscrow(T::EscrowId, T::AccountId, BalanceOf<T>),
		// Paysout Funds from Escrow Object, notes Escrow Id, receiving Account Id, and the amount distributed
		PayoutEscrow(T::EscrowId, T::AccountId, T::AccountId, BalanceOf<T>),
		// Closes the escrow, notes the escrow Id and admin id (this results in the dispersment of remaining funds among contributors proportionate to contributions)
		CloseEscrow(T::EscrowId, T::AccountId),
		// Sets the open bool to true and allows for any account to Fund the Escrow
		EnableOpenContribution(T::EscrowId, T::AccountId),
		// Sets the open bool to false and only allows for admin accounts to fund the Escrow
		DisableOpenContribution(T::EscrowId, T::AccountId),
		// Freezes the escrow
		FreezeEscrow(T::EscrowId, T::AccountId),
		// Thaws the escrow
		ThawEscrow(T::EscrowId, T::AccountId),
		// Adds Administrator
		AddAdministrator(T::EscrowId, T::AccountId, T::AccountId),
		// Remove Administrator
		RemoveAdministrator(T::EscrowId, T::AccountId, T::AccountId)
	}

	// Errors inform users that escrow went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		EscrowAlreadyCreated,
		NoSuchEscrow,
		Unauthorized,
		Frozen,
		OpenAlreadyEnabled,
		OpenAlreadyDisabled,
		AlreadyNotFrozen,
		SelfDistributionAttempt,
		AdminAlreadyPresent,
		AdminNotPresent,
		ErrorOnPushAdmin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A dispatchable to create an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_escrow(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_none(),
				Error::<T>::EscrowAlreadyCreated
			);

			// Insert new Escrow and Administrator into Storage
			let admins: BoundedVec<T::AccountId, ConstU32<{VEC_LIMIT}>> = vec![who.clone()].try_into().unwrap();
			let contributions: BoundedVec<Contribution<T::AccountId, T>, ConstU32<{VEC_LIMIT}>> = vec![].try_into().unwrap();
			<Escrow<T>>::insert(
				escrow_id.clone(), 
				EscrowDetails {
					owner: who.clone(),
					admins: admins.clone(),
					contributions: contributions.clone(),
					amount: BalanceOf::<T>::zero(),
					total_contributed: BalanceOf::<T>::zero(),
					is_frozen: false,
					is_open: false,
				});
			<Administrator<T>>::insert(
				who.clone(),
				escrow_id.clone(),
				<frame_system::Pallet<T>>::block_number(),
			);

			// Emit an event.
			Self::deposit_event(Event::CreateEscrow(escrow_id, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to fund an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn fund_escrow(origin: OriginFor<T>, escrow_id: T::EscrowId, amount: BalanceOf<T>) -> DispatchResult {
			// Check that our caller has signed the transaction
			let funder = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// If escrow isn't open, confirm that origin is an admin
			if !escrow_details.as_ref().unwrap().is_open {
			ensure!(
				escrow_details.unwrap().admins.iter().any(|x| *x == funder.clone()),
				Error::<T>::Unauthorized
			);
			}
			

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
						amount: amount,
					};
					escrow_details.contributions.try_push(contribution).ok();
					Ok(())
				}
			)?;

			T::PaymentCurrency::transfer(
				&funder,
				&escrow_id.clone().try_into().unwrap(),
				amount,
				AllowDeath,
			)?;

			// Emit an event.
			Self::deposit_event(Event::FundEscrow(escrow_id, funder, amount));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to payout from an escrow 
		/// --This functionality may be limitted to payouts from RFPs only once the RFP pallet is implemented
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn payout_escrow(origin: OriginFor<T>, payee: T::AccountId, escrow_id: T::EscrowId, amount: BalanceOf<T>) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Confirm that payee is not an admin
			ensure!(
				!escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == payee),
				Error::<T>::SelfDistributionAttempt
			);

			// Confirm distribution is smaller than escrow amount
			ensure!(
				(escrow_details.as_ref().unwrap().amount >= amount.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.amount -= amount;
					Ok(())
				}
			)?;
			
			// Send funds to payee
			T::PaymentCurrency::transfer(
				&escrow_id.clone().try_into().unwrap(),
				&payee,
				amount,
				AllowDeath,
			)?;

			// Emit an event.
			Self::deposit_event(Event::PayoutEscrow(escrow_id, who, payee, amount));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable to close an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn close_escrow(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Cast the Total Contributed and Current Balance from Escrow to u128s 
				// for use in calculating the distribution of the remaining balance
			let escrow_total_at_closing: u128 = 
				TryInto::<u128>::try_into(escrow_details.clone().unwrap().amount).ok().unwrap();
			let escrow_total_contributed: u128 = 
				TryInto::<u128>::try_into(escrow_details.clone().unwrap().total_contributed).ok().unwrap();
			
			// Distribute remaining funds to contributors proportionately to their contributions
			escrow_details.as_ref().unwrap().contributions.iter().for_each(|contribution|{
					let contributed_amount: u128 = TryInto::<u128>::try_into(contribution.amount).ok().unwrap();
					// Calculate their disbursement
					let close_disbursement: u128 = escrow_total_at_closing * (contributed_amount/escrow_total_contributed);
					// Transfer the funds to the contributor
					T::PaymentCurrency::transfer(
					&escrow_id.clone().try_into().unwrap(),
					&contribution.contributor,
				    close_disbursement.try_into().ok().unwrap(), //ToDo Fix the clone wars
					AllowDeath,
					).ok();
			});

			// Remove Escrow and Administrator from Storage
			<Escrow<T>>::remove(escrow_id.clone());
			// Remove all Admins
			escrow_details.unwrap().admins.iter().for_each(|admin|{
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

		/// Dispatchable which allows an escrow admin to freeze an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn enable_open_contribution(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);

			// Confirm that origin is an admin
			ensure!(
				escrow_details.unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);
			
			// Update Escrow storage to set is_open
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.is_open = true;
					Ok(())
				}
			)?;
			
			// Emit event
			Self::deposit_event(Event::EnableOpenContribution(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to thaw an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn disable_open_contribution(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_open
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.is_open = false;
					Ok(())
				}
			)?;

			// Emit event
			Self::deposit_event(Event::DisableOpenContribution(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to freeze an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn freeze_escrow(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_frozen
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.is_frozen = true;
					Ok(())
				}
			)?;

			// Emit event
			Self::deposit_event(Event::FreezeEscrow(escrow_id, who));
			Ok(())
		}

		/// Dispatchable which allows an escrow admin to thaw an escrow
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn thaw_escrow(origin: OriginFor<T>, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);

			// Check escrow is frozen
			ensure!(
				escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::AlreadyNotFrozen
			);

			// Confirm that origin is an admin
			ensure!(
				escrow_details.unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			// Update Escrow storage to set is_frozen
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					escrow_details.is_frozen = false;
					Ok(())
				}
			)?;
			
			// Emit event
			Self::deposit_event(Event::ThawEscrow(escrow_id, who));
			Ok(())
		}

		/// A dispatchable to add an administrator
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_admin(origin: OriginFor<T>, new_admin: T::AccountId, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);

			ensure!(
				!escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == new_admin.clone()),
				Error::<T>::AdminAlreadyPresent
			);

			// Insert new Escrow and Administrator into Storage
			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
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
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn remove_admin(origin: OriginFor<T>, admin_to_remove: T::AccountId, escrow_id: T::EscrowId) -> DispatchResult {
			// Check that our caller has signed the transaction
			let who = ensure_signed(origin)?;
			
			// Check that the passed in escrow exists
			let escrow_details = <Escrow<T>>::get(&escrow_id);
			ensure!(
				escrow_details.is_some(),
				Error::<T>::NoSuchEscrow
			);
			
			// Check escrow isn't frozen
			ensure!(
				!escrow_details.as_ref().unwrap().is_frozen,
				Error::<T>::Frozen
			);
			
			// Confirm that origin is an admin
			ensure!(
				escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == who.clone()),
				Error::<T>::Unauthorized
			);
			
			// Confirm Admin is present to be removed
			ensure!(
				!escrow_details.as_ref().unwrap().admins.iter().any(|x| *x == admin_to_remove.clone()),
				Error::<T>::AdminNotPresent
			);

			<Escrow<T>>::try_mutate(
				&escrow_id, 
				| maybe_escrow_details | -> DispatchResult {
					let escrow_details =
						maybe_escrow_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
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
