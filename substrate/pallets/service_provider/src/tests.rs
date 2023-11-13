use crate::{mock::*, Error, Event, ProviderInfo};
use frame_support::{assert_noop, assert_ok};
use pallet_user_identity::UserInfo;
use sp_runtime::BoundedVec;

#[test]
pub fn it_can_register_provider() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		assert_ok!(ServiceProvider::register_provider(RuntimeOrigin::signed(1), name.clone()));

		let provider_info = ProviderInfo { name: name.clone(), verified: false, votes: 0 };

		match ServiceProvider::get_provider(1) {
			Some(info) => assert_eq!(info, provider_info),
			None => assert!(false, "get_identity does not return any value"),
		}

		System::assert_last_event(Event::ProviderRegistered { name, who: 1 }.into());
	});
}

#[test]
pub fn it_can_deny_double_registration() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let _ = ServiceProvider::register_provider(RuntimeOrigin::signed(1), name.clone());

		assert_noop!(
			ServiceProvider::register_provider(RuntimeOrigin::signed(1), name.clone(),),
			Error::<Test>::AlreadyRegistered
		);
	});
}

#[test]
pub fn it_can_allow_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let provider = 1;
		let voter = 2;
		let _ = ServiceProvider::register_provider(RuntimeOrigin::signed(provider), name.clone());

		assert_ok!(ServiceProvider::vote(RuntimeOrigin::signed(voter), 1));

		match ServiceProvider::get_provider(provider) {
			Some(info) => assert_eq!(info.votes, 1),
			None => assert!(false, "get_provider does not return any value"),
		}

		match ServiceProvider::get_votes(voter) {
			Some(info) => assert!(info.votes.contains(&provider)),
			None => assert!(false, "get_votes does not return any value"),
		}

		System::assert_last_event(Event::Vote { provider: name, who: 2 }.into());
	});
}

#[test]
pub fn it_can_deny_double_voting() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let provider = 1;
		let voter = 2;

		let _ = ServiceProvider::register_provider(RuntimeOrigin::signed(provider), name.clone());

		let _ = ServiceProvider::vote(RuntimeOrigin::signed(voter), provider);
		assert_noop!(
			ServiceProvider::vote(RuntimeOrigin::signed(voter), provider),
			Error::<Test>::AlreadyVoted
		);
	});
}

#[test]
pub fn it_can_verify_after_2_votes() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let provider = 1;
		let voter1 = 2;
		let voter2 = 3;

		let _ = ServiceProvider::register_provider(RuntimeOrigin::signed(provider), name.clone());

		let _ = ServiceProvider::vote(RuntimeOrigin::signed(voter1), provider);
		let _ = ServiceProvider::vote(RuntimeOrigin::signed(voter2), provider);
		match ServiceProvider::get_provider(provider) {
			Some(info) => assert!(info.verified),
			None => assert!(false, "get_provider does not return any value"),
		}
		System::assert_has_event(Event::ProviderVerified { provider: name }.into());
	});
}

#[test]
pub fn it_can_verify_user() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let name = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let twitter = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let provider = 1;
		let voter1 = 2;
		let voter2 = 3;

		let user_info =
			UserInfo { twitter: twitter.clone(), username: name.clone(), verified: false };

		let _ = pallet_user_identity::Pallet::<Test>::register_user(
			RuntimeOrigin::signed(voter1),
			name.clone(),
			twitter.clone(),
		);

		let _ = ServiceProvider::register_provider(RuntimeOrigin::signed(provider), name.clone());

		let _ = ServiceProvider::vote(RuntimeOrigin::signed(voter1), provider);
		let _ = ServiceProvider::vote(RuntimeOrigin::signed(voter2), provider);

		assert_ok!(ServiceProvider::verify(RuntimeOrigin::signed(provider), voter1));

		match pallet_user_identity::Pallet::<Test>::get_identity(voter1) {
			Some(info) => assert!(info.verified),
			None => assert!(false, "get_identity does not return any value"),
		}

		System::assert_has_event(Event::UserVerified { provider, user: user_info.clone() }.into());
	});
}
