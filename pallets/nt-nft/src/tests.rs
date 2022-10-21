use crate::{mock::*, Error, CollectionDetails};
use frame_support::{assert_noop,  assert_ok};

const ACCOUNT_ID: u64 = 1;
const OTHER_ACCOUNT_ID: u64 = 2;
const COLLECTION_ID: u128 = 101;

/// Test Create Collection Dispatchable
#[test]
fn create_collection_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
	});
}

#[test]
fn create_collection_fails_on_repeat_collection_id() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_noop!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID), Error::<Test>::CollectionIdAlreadyExists);
	});
}

#[test]
fn correct_storage_for_create_collection() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		// Read pallet storage and assert an expected result.
		let collection_details = CollectionDetails{
			owner: ACCOUNT_ID,
			amount: 0,
			is_frozen: false
		};
		assert_eq!(NTNFTModule::collection(COLLECTION_ID), Some(collection_details));
	});
}

// Test Freeze Collection
#[test]
fn freeze_collection_successfully_executes() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_ok!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
	});
}

#[test]
fn freeze_collection_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NTNFTModule::create_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_noop!(NTNFTModule::freeze_collection(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID), Error::<Test>::Unauthorized);
	});
}