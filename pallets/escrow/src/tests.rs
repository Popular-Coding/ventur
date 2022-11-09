use crate::{mock::*, Error, /* Escrow, */ EscrowDetails};
use frame_support::{assert_noop,  assert_ok, /* BoundedVec */};

const ACCOUNT_ID: u64 = 1;
const OTHER_ACCOUNT_ID: u64 = 2;
const YET_ANOTHER_ACCOUNT_ID: u64 = 3;
const AMOUNT: u128 = 10000;
const GREATER_AMOUNT: u128 = 10001;

/// Create Escrow Tests
#[test]
fn create_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
	});
}

#[test]
fn error_on_duplicate_escrow() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)), Error::<Test>::EscrowAlreadyCreated);
	});
}

#[test]
fn correct_storage_for_create_escrow() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		// Read pallet storage and assert an expected result.
		let admins = vec![ACCOUNT_ID].try_into().unwrap();
		let contributions = vec![].try_into().unwrap();
		let escrow_details = EscrowDetails{
			admins: admins,
			contributions: contributions,
			amount: 0,
			total_contributed: 0,
			is_frozen: false,
			is_open: false,
		};
		assert_eq!(EscrowModule::escrow(ACCOUNT_ID), Some(escrow_details.clone()));
		assert!(EscrowModule::administrator(ACCOUNT_ID, ACCOUNT_ID).is_some());
	});
}

/// Fund Escrow Tests
#[test]
fn fund_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
	});
}

#[test]
fn correct_error_for_unauthorized_fund_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::fund_escrow(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID, AMOUNT), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_fund_escrow_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, AMOUNT), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_fund_escrow_with_frozen_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT), Error::<Test>::Frozen);
	});
}

/// Test Payout Escrow
#[test]
fn payout_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
		assert_ok!(EscrowModule::payout_escrow(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID, AMOUNT));
	});
}

#[test]
fn correct_error_for_payout_escrow_with_frozen_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::payout_escrow(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID, AMOUNT), Error::<Test>::Frozen);
	});
}

#[test]
fn correct_error_for_payout_escrow_unauthorized() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
		assert_noop!(EscrowModule::payout_escrow(Origin::signed(OTHER_ACCOUNT_ID), YET_ANOTHER_ACCOUNT_ID, ACCOUNT_ID, AMOUNT), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_payout_escrow_self_distribution() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
		assert_noop!(EscrowModule::payout_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, ACCOUNT_ID, AMOUNT), Error::<Test>::SelfDistributionAttempt);
	});
}

#[test]
fn correct_error_for_payout_escrow_lack_funds() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::fund_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID, AMOUNT));
		assert_noop!(EscrowModule::payout_escrow(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID, GREATER_AMOUNT), Error::<Test>::InsufficientEscrowFunds);
	});
}

/// Test Close Escrow
#[test]
fn close_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::close_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_close_escrow_frozen() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::close_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

#[test]
fn correct_error_for_unauthorized_close_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::close_escrow(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_close_escrow_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::close_escrow(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID), Error::<Test>::NoSuchEscrow);
	});
}

/// Test Enable Open Contribution
#[test]
fn enable_open_contribution_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_unauthorized_enable_open_contribution() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::enable_open_contribution(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_enable_open_contribution_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_enable_open_contribution_frozen() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

/// Test Disable Open Contribution
#[test]
fn disable_open_contribution_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_ok!(EscrowModule::disable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_unauthorized_disable_open_contribution() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::disable_open_contribution(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_disable_open_contribution_with_invalid_escrow() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::disable_open_contribution(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID), Error::<Test>::NoSuchEscrow);
	});
}

#[test]
fn correct_error_for_disable_open_contribution_frozen() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::enable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::disable_open_contribution(Origin::signed(ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

/// Test Freeze Escrow
#[test]
fn freeze_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_freeze_escrow_on_already_frozen() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

#[test]
fn correct_error_for_freeze_escrow_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_noop!(EscrowModule::freeze_escrow(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

/// Test Thaw Escrow
#[test]
fn thaw_escrow_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_ok!(EscrowModule::thaw_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_thaw_escrow_on_not_frozen() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_ok!(EscrowModule::thaw_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::thaw_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::AlreadyNotFrozen);
	});
}

#[test]
fn correct_error_for_thaw_escrow_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::thaw_escrow(Origin::signed(OTHER_ACCOUNT_ID), ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

/// Test Add Admin
#[test]
fn add_admin_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_add_admin_on_frozen() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

#[test]
fn correct_error_for_add_admin_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_noop!(EscrowModule::add_admin(Origin::signed(YET_ANOTHER_ACCOUNT_ID), YET_ANOTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_add_admin_on_already_present_admin() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_noop!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::AdminAlreadyPresent);
	});
}

/// Test Remove Admin
#[test]
fn remove_admin_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_ok!(EscrowModule::remove_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
	});
}

#[test]
fn correct_error_for_remove_admin_on_frozen() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_ok!(EscrowModule::freeze_escrow(Origin::signed(ACCOUNT_ID), ACCOUNT_ID));
		assert_noop!(EscrowModule::remove_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::Frozen);
	});
}

#[test]
fn correct_error_for_remove_admin_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_noop!(EscrowModule::remove_admin(Origin::signed(YET_ANOTHER_ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn correct_error_for_remove_admin_on_non_present_admin() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::create_escrow(Origin::signed(ACCOUNT_ID)));
		assert_ok!(EscrowModule::add_admin(Origin::signed(ACCOUNT_ID), OTHER_ACCOUNT_ID, ACCOUNT_ID));
		assert_noop!(EscrowModule::remove_admin(Origin::signed(ACCOUNT_ID), YET_ANOTHER_ACCOUNT_ID, ACCOUNT_ID), Error::<Test>::AdminNotPresent);
	});
}
