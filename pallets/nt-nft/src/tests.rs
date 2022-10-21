use crate::{mock::*, Error, CollectionDetails};
use frame_support::{assert_noop,  assert_ok};

const ACCOUNT_ID: u64 = 1;
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