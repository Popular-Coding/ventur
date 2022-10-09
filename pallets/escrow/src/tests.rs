use crate::{mock::*, Error, Escrow, EscrowDetails};
use frame_support::{assert_noop,  assert_ok, /* BoundedVec */};

const ESCROW_ID: u64 = 11201;
const OTHER_ESCROW_ID: u64 = 11232;
const ACCOUNT_ID: u64 = 1;
const OTHER_ACCOUNT_ID: u64 = 2;
const AMOUNT: u128 = 10000;

/// Trivial Extrinsic Execution Tests
#[test]
fn create_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

#[test]
fn fund_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID, AMOUNT));
	});
}

#[test]
fn close_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::close_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

#[test]
fn enable_open_contribution_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

#[test]
fn disable_open_contribution_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::disable_open_contribution(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

#[test]
fn freeze_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

#[test]
fn thaw_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::thaw_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
	});
}

/// Storage Validation Tests
#[test]
fn correct_storage_for_create_escrow() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		// Read pallet storage and assert an expected result.
		let admins = vec![ACCOUNT_ID].try_into().unwrap();
		let contributions = vec![].try_into().unwrap();
		let escrow_details = EscrowDetails{
			owner: ACCOUNT_ID,
			admins: admins,
			contributions: contributions,
			amount: 0,
			total_contributed: 0,
			is_frozen: false,
			is_open: false,
		};
		assert_eq!(EscrowModule::escrow(ESCROW_ID), Some(escrow_details.clone()));
		assert!(EscrowModule::administrator(ACCOUNT_ID, ESCROW_ID).is_some());
	});
}

/// Errors
/// Authorization Errors
#[test]
fn correct_error_for_unauthorized_fund_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::fund_escrow(Origin::signed(OTHER_ACCOUNT_ID), ESCROW_ID, AMOUNT), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_unauthorized_close_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::close_escrow(Origin::signed(OTHER_ACCOUNT_ID), ESCROW_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_unauthorized_enable_open_contribution() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::enable_open_contribution(Origin::signed(OTHER_ACCOUNT_ID), ESCROW_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_unauthorized_disable_open_contribution() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::disable_open_contribution(Origin::signed(OTHER_ACCOUNT_ID), ESCROW_ID), Error::<Test>::Unauthorized);
	});
}

/// Missing Escrow Errors
#[test]
fn correct_error_for_fund_escrow_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), OTHER_ESCROW_ID, AMOUNT), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_close_escrow_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::close_escrow(Origin::signed(ACCOUNT_ID), OTHER_ESCROW_ID), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_enable_open_contribution_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), OTHER_ESCROW_ID), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_disable_open_contribution_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ESCROW_ID));
		assert_noop!(EscrowModule::disable_open_contribution(Origin::signed(ACCOUNT_ID), OTHER_ESCROW_ID), Error::<Test>::NoSuchEscrow);
	});
}
