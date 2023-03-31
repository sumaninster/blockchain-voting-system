use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_register_new_election() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		println!("{:?}", RuntimeOrigin::root());
		assert_ok!(ElectionModule::register_new_election(RuntimeOrigin::root()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ElectionModule::election_open_for_voter_registration(1), Some(false));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::ElectionIdRegistered { election_id: 1 }.into());
	});
}

#[test]
fn correct_error_for_invalid_election_id() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			ElectionModule::open_for_voter_registration(RuntimeOrigin::root(), 5),
			Error::<Test>::InvalidElectionId
		);
	});
}
