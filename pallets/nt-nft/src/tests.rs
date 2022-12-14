use crate::{mock::*, Error, CollectionDetails};
use frame_support::{assert_noop,  assert_ok};
use hex_literal::hex;

const ACCOUNT_ID: u64 = 1;
const OTHER_ACCOUNT_ID: u64 = 2;
const COLLECTION_ID: u128 = 101;
const OTHER_COLLECTION_ID: u128 = 102;
const NTNFT_ID: u128 = 1001;

#[test]
fn create_collection_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
	});
}

#[test]
fn create_collection_fails_on_repeat_collection_id() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_noop!(
			NTNFTModule::create_collection(
				Origin::signed(ACCOUNT_ID), 
				COLLECTION_ID, 
				image_cid,
				meta_cid
			), 
			Error::<Test>::CollectionIdAlreadyExists
		);
	});
}

#[test]
fn correct_storage_for_create_collection() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		// Read pallet storage and assert an expected result.
		let collection_details = CollectionDetails{
			owner: ACCOUNT_ID,
			amount: 0,
			is_frozen: false,
			image_ipfs_cid: image_cid,
			metadata_ipfs_cid: meta_cid,
		};
		assert_eq!(NTNFTModule::collection(COLLECTION_ID), Some(collection_details));
	});
}

#[test]
fn freeze_collection_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
	});
}

#[test]
fn freeze_collection_fails_on_collectionid_does_not_exist() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_noop!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), OTHER_COLLECTION_ID), Error::<Test>::CollectionIdDoesNotExist);
	});
}

#[test]
fn freeze_collection_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_noop!(NTNFTModule::freeze_collection(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID), Error::<Test>::Unauthorized);
	});
}

// Test Thaw Collection
#[test]
fn thaw_collection_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_ok!(NTNFTModule::thaw_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
	});
}

#[test]
fn thaw_collection_fails_on_collectionid_does_not_exist() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_noop!(NTNFTModule::thaw_collection(Origin::signed(ACCOUNT_ID), OTHER_COLLECTION_ID), Error::<Test>::CollectionIdDoesNotExist);
	});
}

#[test]
fn thaw_collection_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::freeze_collection(Origin::signed(ACCOUNT_ID), COLLECTION_ID));
		assert_noop!(NTNFTModule::thaw_collection(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn mint_ntnft_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
	});
}

#[test]
fn mint_ntnft_fails_on_collectionid_does_not_exist() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_noop!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), OTHER_COLLECTION_ID, NTNFT_ID), Error::<Test>::CollectionIdDoesNotExist);
	});
}

#[test]
fn mint_ntnft_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_noop!(NTNFTModule::mint_ntnft(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID, NTNFT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn burn_ntnft_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_ok!(NTNFTModule::burn_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
	});
}

#[test]
fn burn_ntnft_fails_on_collectionid_does_not_exist() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_noop!(NTNFTModule::burn_ntnft(Origin::signed(ACCOUNT_ID), OTHER_COLLECTION_ID, NTNFT_ID), Error::<Test>::CollectionIdDoesNotExist);
	});
}

#[test]
fn burn_ntnft_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_noop!(NTNFTModule::burn_ntnft(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID, NTNFT_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn assign_ntnft_successfully_executes() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_ok!(NTNFTModule::assign_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID, OTHER_ACCOUNT_ID));
	});
}

#[test]
fn assign_ntnft_fails_on_collectionid_does_not_exist() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_noop!(NTNFTModule::assign_ntnft(Origin::signed(ACCOUNT_ID), OTHER_COLLECTION_ID, NTNFT_ID, OTHER_ACCOUNT_ID), Error::<Test>::CollectionIdDoesNotExist);
	});
}

#[test]
fn assign_ntnft_fails_on_unauthorized() {
	new_test_ext().execute_with(|| {
		let image_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		let meta_cid = 
			sp_core::H256(
				hex!(
					"5198bfa9da6f9c9d22dd2b0ce301dde9fd3c5826a117705d3ffa415d83a6bde8"
				)
			);
		assert_ok!(NTNFTModule::create_collection(
			Origin::signed(ACCOUNT_ID), 
			COLLECTION_ID,
			image_cid,
			meta_cid,
		));
		assert_ok!(NTNFTModule::mint_ntnft(Origin::signed(ACCOUNT_ID), COLLECTION_ID, NTNFT_ID));
		assert_noop!(NTNFTModule::assign_ntnft(Origin::signed(OTHER_ACCOUNT_ID), COLLECTION_ID, NTNFT_ID, OTHER_ACCOUNT_ID), Error::<Test>::Unauthorized);
	});
}