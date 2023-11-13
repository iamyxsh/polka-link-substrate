#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::{Error, *};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {

	use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	pub type LimitedVec = BoundedVec<u8, ConstU32<15>>;

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug, MaxEncodedLen)]
	pub struct UserInfo {
		pub username: LimitedVec,
		pub twitter: LimitedVec,
		pub verified: bool,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_identity)]
	pub type AccountIdToUserInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserAdded { user: UserInfo, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyRegistered,
		NotRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn register_user(
			origin: OriginFor<T>,
			username: LimitedVec,
			twitter: LimitedVec,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user_info = UserInfo { username, twitter, verified: false };

			if <AccountIdToUserInfo<T>>::get(&who).is_some() {
				return Err(Error::<T>::AlreadyRegistered.into());
			}

			<AccountIdToUserInfo<T>>::insert(&who, user_info.clone());

			Self::deposit_event(Event::UserAdded { user: user_info, who });
			Ok(())
		}

		#[pallet::call_index(1)]
		pub fn update_info(
			origin: OriginFor<T>,
			username: LimitedVec,
			twitter: LimitedVec,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user_info = UserInfo { username, twitter, verified: false };

			if <AccountIdToUserInfo<T>>::get(&who).is_none() {
				return Err(Error::<T>::NotRegistered.into());
			}

			<AccountIdToUserInfo<T>>::mutate(&who, |q| {
				let info = q.as_mut().unwrap();
				*info = user_info.clone();
			});

			Self::deposit_event(Event::UserAdded { user: user_info, who });
			Ok(())
		}
	}
}
