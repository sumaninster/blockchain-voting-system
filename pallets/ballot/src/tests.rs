use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let election_id = 1;
		let candidate = 1;
        let zkp_value = 42;
        let transcript = b"ZKPExample";
        let zkp_randomness = b"hello world";
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::cast_vote(RuntimeOrigin::signed(1), election_id, candidate, zkp_value, transcript, zkp_randomness));
		// Read pallet storage and assert an expected result.
		//assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		//System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}
/*
#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}*/