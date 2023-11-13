use crate::{mock::*, Error, Event, UserInfo};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
pub fn it_can_register_user() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let username = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let twitter = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		assert_ok!(ServiceProvider::register_user(
			RuntimeOrigin::signed(1),
			username.clone(),
			twitter.clone()
		));

		let user_info = UserInfo { username, twitter, verified: false };

		match ServiceProvider::get_identity(1) {
			Some(info) => assert_eq!(info, user_info),
			None => assert!(false, "get_identity does not return any value"),
		}

		System::assert_last_event(Event::UserAdded { user: user_info, who: 1 }.into());
	});
}

#[test]
pub fn it_can_deny_double_registration() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let username = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let twitter = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		assert_ok!(ServiceProvider::register_user(
			RuntimeOrigin::signed(1),
			username.clone(),
			twitter.clone()
		));
		assert_noop!(
			ServiceProvider::register_user(
				RuntimeOrigin::signed(1),
				username.clone(),
				twitter.clone()
			),
			Error::<Test>::AlreadyRegistered
		);
	});
}

#[test]
pub fn it_can_update_info() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let username = BoundedVec::try_from("iamyxsh".as_bytes().to_vec()).unwrap();
		let twitter = BoundedVec::try_from("yash".as_bytes().to_vec()).unwrap();
		let _ = ServiceProvider::register_user(
			RuntimeOrigin::signed(1),
			twitter.clone(),
			username.clone(),
		);

		assert_ok!(ServiceProvider::update_info(
			RuntimeOrigin::signed(1),
			username.clone(),
			twitter.clone()
		));
		let user_info = UserInfo { username, twitter, verified: false };

		match ServiceProvider::get_identity(1) {
			Some(info) => assert_eq!(info, user_info),
			None => assert!(false, "get_identity does not return any value"),
		}

		System::assert_last_event(Event::UserAdded { user: user_info, who: 1 }.into());
	});
}
