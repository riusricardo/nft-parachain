// Copyright 2020 Parity Technologies (UK) Ltd.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use orml_nft::CID;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct ClassData {
	pub data: Vec<u8>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct TokenData {
	pub data: u32,
}

pub trait Trait: frame_system::Trait  + orml_nft::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		pub TokensOf get(fn tokens_of): map hasher(twox_64_concat) T::AccountId  => Vec<T::TokenId>;
	}
}

decl_event!(
	pub enum Event<T> where
		AccountId = <T as frame_system::Trait>::AccountId,
		TokenId = <T as orml_nft::Trait>::TokenId,
	{
		MintedToken(AccountId, TokenId, CID),
		BurnedToken(AccountId, TokenId),
		TransferredToken(AccountId, AccountId, TokenId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn mint(origin, metadata: CID, data: T::TokenData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::mint_nft(who, metadata, data)?;
			Ok(())
		}

		#[weight = 10_000]
		pub fn transfer(origin, to: T::AccountId, token_id: T::TokenId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::transfer_nft(who, to, token_id)?;
			Ok(())
		}

		#[weight = 10_000]
		pub fn burn(origin, token_id: T::TokenId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::burn_nft(who, token_id)?;
			Ok(())
		}
	}
}


impl<T: Trait> Module<T> {

	pub fn mint_nft(from: T::AccountId, metadata: CID, data: T::TokenData) -> dispatch::DispatchResult {
		let token_class: T::ClassId = 0.into();
		let token_id = <orml_nft::Module<T>>::mint(&from, token_class, metadata, data)?;

		let mut owned_tokens = Self::tokens_of(&from);
		owned_tokens.push(token_id.clone());
		TokensOf::<T>::insert(&from, owned_tokens);

		Self::deposit_event(RawEvent::MintedToken(from, token_id, metadata));
		Ok(())
	}

	pub fn transfer_nft(from: T::AccountId, to: T::AccountId, token_id: T::TokenId) -> dispatch::DispatchResult {
		let mut owned_tokens_from = Self::tokens_of(&from);
		let token = owned_tokens_from.binary_search(&token_id).ok().ok_or(Error::<T>::NoneValue)?;
		owned_tokens_from.remove(token);
		TokensOf::<T>::insert(&from, owned_tokens_from);

		let token_class: T::ClassId = 0.into();
		<orml_nft::Module<T>>::transfer(&from, &to, (token_class, token_id))?;

		let mut owned_tokens_to = Self::tokens_of(&to);
		owned_tokens_to.push(token_id);
		TokensOf::<T>::insert(&to, owned_tokens_to);

		Self::deposit_event(RawEvent::TransferredToken(from, to, token_id));
		Ok(())
	}

	pub fn burn_nft(from: T::AccountId, token_id: T::TokenId) -> dispatch::DispatchResult {
		let mut owned_tokens_from = Self::tokens_of(&from);
		let token = owned_tokens_from.binary_search(&token_id).ok().ok_or(Error::<T>::NoneValue)?;
		owned_tokens_from.remove(token);
		TokensOf::<T>::insert(&from, owned_tokens_from);

		let token_class: T::ClassId = 0.into();
		<orml_nft::Module<T>>::burn(&from, (token_class, token_id))?;

		Self::deposit_event(RawEvent::BurnedToken(from, token_id));
		Ok(())
	}
}