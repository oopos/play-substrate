#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence

use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	ensure,
	dispatch::{DispatchResult},
};
use frame_system::{
	self as system,
	ensure_signed,
};
use sp_std::vec::Vec;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proof): map hasher(twox_64_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		DuplicateClaim,
		ClaimNotExist,
		NotClaimOwner,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::DuplicateClaim);
			
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

	}
}
