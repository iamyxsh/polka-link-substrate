#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use crate::pallet::vec::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_user_identity::UserInfo;
	use scale_info::prelude::vec;

	pub type LimitedVec = BoundedVec<u8, ConstU32<15>>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_user_identity::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug, MaxEncodedLen)]
	pub struct ProviderInfo {
		pub name: LimitedVec,
		pub votes: u128,
		pub verified: bool,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug, MaxEncodedLen)]
	pub struct VotingInfo<T> {
		pub votes: Vec<T>,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_provider)]
	pub type AccountIdToProviderInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ProviderInfo, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_votes)]
	pub type AccountIdToVotingInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, VotingInfo<T::AccountId>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProviderRegistered { name: LimitedVec, who: T::AccountId },
		Vote { provider: LimitedVec, who: T::AccountId },
		ProviderVerified { provider: LimitedVec },
		UserVerified { provider: T::AccountId, user: UserInfo },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyRegistered,
		NotRegistered,
		AlreadyVerified,
		AlreadyVoted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn register_provider(origin: OriginFor<T>, name: LimitedVec) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let provider_info = ProviderInfo { name: name.clone(), verified: false, votes: 0 };

			if <AccountIdToProviderInfo<T>>::get(&who).is_some() {
				return Err(Error::<T>::AlreadyRegistered.into());
			}

			<AccountIdToProviderInfo<T>>::insert(&who, provider_info);

			Self::deposit_event(Event::ProviderRegistered { name, who });
			Ok(())
		}

		#[pallet::call_index(1)]
		pub fn vote(origin: OriginFor<T>, provider: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let provider_info = <AccountIdToProviderInfo<T>>::get(&provider);
			let voting_info = <AccountIdToVotingInfo<T>>::get(&who);

			if provider_info.is_none() {
				return Err(Error::<T>::NotRegistered.into());
			}

			if voting_info.is_none() {
				let voting_info = VotingInfo { votes: vec![provider.clone()] };
				<AccountIdToVotingInfo<T>>::insert(&who, voting_info);
			} else {
				if voting_info.unwrap().votes.contains(&provider) {
					return Err(Error::<T>::AlreadyVoted.into());
				} else {
					<AccountIdToVotingInfo<T>>::mutate(&who, |q| {
						let info = q.as_mut().unwrap();
						info.votes.push(provider.clone());
					});
				}
			}

			<AccountIdToProviderInfo<T>>::mutate(&provider, |q| {
				let info = q.as_mut().unwrap();
				info.votes = info.votes + 1;
				if info.votes == 2 {
					info.verified = true;
					Self::deposit_event(Event::ProviderVerified {
						provider: provider_info.clone().unwrap().name,
					});
				}
			});

			Self::deposit_event(Event::Vote { provider: provider_info.unwrap().name, who });
			Ok(())
		}

		#[pallet::call_index(2)]
		pub fn verify(origin: OriginFor<T>, user: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user_info = pallet_user_identity::Pallet::<T>::get_identity(&user);

			if user_info.clone().is_none() {
				return Err(Error::<T>::NotRegistered.into());
			}

			if user_info.clone().unwrap().verified {
				return Err(Error::<T>::AlreadyVerified.into());
			}

			pallet_user_identity::AccountIdToUserInfo::<T>::mutate(&user, |q| {
				let info = q.as_mut().unwrap();
				info.verified = true;
			});

			Self::deposit_event(Event::UserVerified { provider: who, user: user_info.unwrap() });
			Ok(())
		}
	}
}
