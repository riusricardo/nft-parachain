// Copyright 2020 Parity Technologies (UK) Ltd.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait  + orml_nft::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		pub TokensOf get(fn tokens_of): map hasher(twox_64_concat) T::AccountId  => Vec<T::TokenId>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn mint_nft(origin, metadata: Vec<u8>, data: T::TokenData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let token_class: T::ClassId = 0.into();
			let token_id = <orml_nft::Module<T>>::mint(&who, token_class, metadata, data)?;

			let mut owned_tokens = Self::tokens_of(&who);
			owned_tokens.push(token_id);
			TokensOf::<T>::insert(who, owned_tokens);

			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		#[weight = 10_000]
		pub fn transfer_nft(origin, to: T::AccountId, token_id: T::TokenId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let mut owned_tokens_from = Self::tokens_of(&who);
			let token = owned_tokens_from.binary_search(&token_id).ok().ok_or(Error::<T>::NoneValue)?;
			owned_tokens_from.remove(token);
			TokensOf::<T>::insert(&who, owned_tokens_from);

			let token_class: T::ClassId = 0.into();
			<orml_nft::Module<T>>::transfer(&who, &to, (token_class, token_id))?;

			let mut owned_tokens_to = Self::tokens_of(&to);
			owned_tokens_to.push(token_id);
			TokensOf::<T>::insert(to, owned_tokens_to);

			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		#[weight = 10_000]
		pub fn burn_nft(origin, token_id: T::TokenId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let mut owned_tokens_from = Self::tokens_of(&who);
			let token = owned_tokens_from.binary_search(&token_id).ok().ok_or(Error::<T>::NoneValue)?;
			owned_tokens_from.remove(token);
			TokensOf::<T>::insert(&who, owned_tokens_from);

			let token_class: T::ClassId = 0.into();
			<orml_nft::Module<T>>::burn(&who, (token_class, token_id))?;

			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}
	}
}
