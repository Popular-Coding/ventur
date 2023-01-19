use crate::*;
use frame_support::{
    assert_ok, assert_noop, bounded_vec
};
use mock::*;

const ACCOUNT_ID: u64 = 24601;
const BIDDER_ID: u64 = 22222;
const RFP_ID: u32 = 1410;
const OTHER_RFP_ID: u32 = 999;
const BID_ID: u32 = 1111;
const OTHER_BID_ID: u32 = 12345;
const BID_AMOUNT: u128 = 1999;
const NEW_BID_AMOUNT: u128 = 1525;
const PAYMENT_ID: u32 = 0001;
const RFP_REFERENCE_ID: u32 = 1410;
const RFP_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq";
const OTHER_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptpg";
const BID_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkpabc";
const OTHER_BID_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkpdef";

#[test]
fn test_create_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        assert!(System::events().is_empty());
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::CreateRFP(
                    ACCOUNT_ID, 
                    RFP_ID,
                )
        ));
        let stored_details = 
            RFPModule::get_rfps(ACCOUNT_ID, RFP_ID).unwrap();
        assert_eq!(stored_details, rfp_details);
    })
}

#[test]
fn test_re_create_rfp_fails() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        assert!(System::events().is_empty());
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        assert_noop!(
            RFPModule::create_rfp(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                rfp_details.clone(),
            ),
            Error::<Test>::RFPAlreadyExists
        );
    })
}

#[test]
fn test_update_rfp_succeeds() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let new_cid: Vec<u8> = OTHER_CID.as_bytes().to_vec();
        let new_ipfs_hash: [u8; 59] = new_cid.try_into().unwrap();
        let new_rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash: new_ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::update_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            new_rfp_details.clone(),
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::UpdateRFP(
                    ACCOUNT_ID, 
                    RFP_ID,
                )
        ));
        let stored_details = 
            RFPModule::get_rfps(ACCOUNT_ID, RFP_ID).unwrap();
        assert_eq!(stored_details, new_rfp_details);
    })
}

#[test]
fn test_update_rfp_fails_if_rfp_doesnt_exist() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_noop!(
            RFPModule::update_rfp(
                Origin::signed(ACCOUNT_ID),
                RFP_ID, 
                rfp_details
            ), 
            Error::<Test>::UpdatingNonExistentRFP
        );
    })
}

#[test]
fn test_cancel_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        assert_ok!(RFPModule::cancel_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::CancelRFP(
                    ACCOUNT_ID, 
                    RFP_ID,
                )
        ));
        assert!(
            RFPModule::get_rfps(ACCOUNT_ID, RFP_ID).is_none()
        );
    })
}

#[test]
fn test_cancel_rfp_fails_if_not_existent() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        assert_noop!(
            RFPModule::cancel_rfp(
                Origin::signed(ACCOUNT_ID),
                RFP_ID
            ),
            Error::<Test>::CancelingNonExistentRFP
        );
    })
}

#[test]
fn test_bid_on_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::BidOnRFP(
                    BIDDER_ID, 
                    RFP_ID,
                    BID_ID,
                )
        ));
        let stored_bid = 
            RFPModule::all_bids(BID_ID).unwrap();
        assert_eq!(stored_bid, bid_details);
        let bids_for_rfp = 
            RFPModule::rfp_to_bids(RFP_ID).unwrap();
        assert!(bids_for_rfp.contains(&BID_ID));
    })
}

#[test]
fn test_bid_on_rfp_fails_if_rfp_doesnt_exist() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };

        assert_noop!(
            RFPModule::bid_on_rfp(
                Origin::signed(BIDDER_ID),
                ACCOUNT_ID,
                RFP_ID,
                BID_ID,
                bid_details.clone()
            ),
            Error::<Test>::NonExistentRFP
        );
    })
}

#[test]
fn test_bid_on_rfp_fails_with_existing_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));

        assert_noop!(
            RFPModule::bid_on_rfp(
                Origin::signed(BIDDER_ID),
                ACCOUNT_ID,
                RFP_ID,
                BID_ID,
                bid_details.clone()
            ),
            Error::<Test>::BidAlreadyExists
        );
    })
}

#[test]
fn test_bid_on_rfp_fails_if_rfp_not_accepting_bids() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::NotAcceptingNewBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_noop!(
            RFPModule::bid_on_rfp(
                Origin::signed(BIDDER_ID),
                ACCOUNT_ID,
                RFP_ID,
                BID_ID,
                bid_details.clone()
            ),
            Error::<Test>::RFPNotAcceptingBids
        );
    })
}

#[test]
fn test_shortlist_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::ShortlistBid(
                    ACCOUNT_ID, 
                    RFP_ID,
                    BID_ID
                )
        ));
        let shortlisted_bids = 
            RFPModule::rfp_to_shortlisted_bids(RFP_ID).unwrap();
        assert!(shortlisted_bids.contains(&BID_ID));
    })
}

#[test]
fn test_shortlist_multiple_bids() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let other_bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let other_bid_cid_hash: [u8; 59] = other_bid_cid.try_into().unwrap();
        let other_bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: other_bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            OTHER_BID_ID,
            other_bid_details.clone()
        ));
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID
        ));
        
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            OTHER_BID_ID
        ));
        let shortlisted_bids = 
            RFPModule::rfp_to_shortlisted_bids(RFP_ID).unwrap();
        assert!(shortlisted_bids.contains(&BID_ID));
        assert!(shortlisted_bids.contains(&OTHER_BID_ID));
    })
}

#[test]
fn test_shortlist_bid_fails_if_rfp_non_existent() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        assert_noop!(
            RFPModule::shortlist_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                BID_ID
            ),
            Error::<Test>::NonExistentRFP
        );
    })
}

#[test]
fn test_shortlist_bid_fails_if_bid_non_existent() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        assert_noop!(
            RFPModule::shortlist_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                BID_ID
            ),
            Error::<Test>::ShortlistingNonExistentBid
        );
    })
}

#[test]
fn test_shortlist_bid_fails_if_rfp_has_no_such_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let new_cid: Vec<u8> = OTHER_CID.as_bytes().to_vec();
        let new_ipfs_hash: [u8; 59] = new_cid.try_into().unwrap();
        let new_rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash: new_ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            OTHER_RFP_ID,
            new_rfp_details.clone(),
        ));
        let other_bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let other_bid_cid_hash: [u8; 59] = other_bid_cid.try_into().unwrap();
        let other_bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: other_bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            OTHER_RFP_ID,
            OTHER_BID_ID,
            other_bid_details.clone()
        ));
        assert_noop!(
            RFPModule::shortlist_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                OTHER_BID_ID,
            ),
            Error::<Test>::NoSuchBidForRFP
        );
    })
}

#[test]
fn test_no_bids_for_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let new_cid: Vec<u8> = OTHER_CID.as_bytes().to_vec();
        let new_ipfs_hash: [u8; 59] = new_cid.try_into().unwrap();
        let new_rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash: new_ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            OTHER_RFP_ID,
            new_rfp_details.clone(),
        ));
        assert_noop!(
            RFPModule::shortlist_bid(
                Origin::signed(ACCOUNT_ID),
                OTHER_RFP_ID,
                BID_ID,
            ),
            Error::<Test>::NoSuchBidForRFP
        );
    })
}

#[test]
fn test_update_rfp_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let other_bid_cid: Vec<u8> = OTHER_BID_CID.as_bytes().to_vec();
        let other_bid_hash: [u8; 59] = other_bid_cid.try_into().unwrap();
        let updated_bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: other_bid_hash,
            bid_amount: NEW_BID_AMOUNT,
        };
        assert_ok!(RFPModule::update_rfp_bid(
            Origin::signed(BIDDER_ID),
            RFP_ID,
            BID_ID,
            updated_bid_details.clone(),
        ));
        let stored_bid = 
            RFPModule::all_bids(BID_ID).unwrap();
        assert_eq!(stored_bid, updated_bid_details);
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::UpdateRFPBid(
                    BIDDER_ID, 
                    RFP_ID,
                    BID_ID
                )
        ));
    })
}

#[test]
fn test_update_fails_if_updater_not_owner() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let other_bid_cid: Vec<u8> = OTHER_BID_CID.as_bytes().to_vec();
        let other_bid_hash: [u8; 59] = other_bid_cid.try_into().unwrap();
        let updated_bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: other_bid_hash,
            bid_amount: NEW_BID_AMOUNT,
        };
        assert_noop!(
            RFPModule::update_rfp_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                BID_ID,
                updated_bid_details.clone(),
            ),
            Error::<Test>::UnauthorizedUpdateOfBid
        );
    })
}

#[test]
fn test_update_fails_if_bid_doesnt_exist() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let other_bid_cid: Vec<u8> = OTHER_BID_CID.as_bytes().to_vec();
        let other_bid_hash: [u8; 59] = other_bid_cid.try_into().unwrap();
        let updated_bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: other_bid_hash,
            bid_amount: NEW_BID_AMOUNT,
        };
        assert_noop!(
            RFPModule::update_rfp_bid(
                Origin::signed(BIDDER_ID),
                RFP_ID,
                BID_ID,
                updated_bid_details.clone(),
            ),
            Error::<Test>::UpdatingNonExistentBid
        );
    })
}

#[test]
fn test_accept_rfp_bid_fails_if_no_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let payment_schedule = bounded_vec![];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: ACCOUNT_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: ACCOUNT_ID,
            payee: BIDDER_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: BID_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ACCOUNT_ID,
        };
        assert_noop!(
            RFPModule::accept_rfp_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                BID_ID,
                payment_details
            ),
            Error::<Test>::NonExistentRFP
        );
    })
}

#[test]
fn test_accept_rfp_bid_fails_if_bid_not_shortlisted() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID
        ));
        let payment_schedule = bounded_vec![];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: ACCOUNT_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: ACCOUNT_ID,
            payee: BIDDER_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: BID_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ACCOUNT_ID,
        };
        assert_noop!(
            RFPModule::accept_rfp_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                OTHER_BID_ID,
                payment_details.clone()
            ),
            Error::<Test>::AcceptedBidNotShortlisted
        );
    })
}

#[test]
fn test_accept_rfp_bid_fails_if_bid_doesnt_exist() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        let payment_schedule = bounded_vec![];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: ACCOUNT_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: ACCOUNT_ID,
            payee: BIDDER_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: BID_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ACCOUNT_ID,
        };
        assert_noop!(
            RFPModule::accept_rfp_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                OTHER_BID_ID,
                payment_details.clone()
            ),
            Error::<Test>::NoSuchBidForRFP
        );
    })
}

#[test]
fn test_accept_rfp_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID
        ));
        let payment_schedule = bounded_vec![];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: ACCOUNT_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: ACCOUNT_ID,
            payee: BIDDER_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: BID_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ACCOUNT_ID,
        };
        assert_ok!(RFPModule::accept_rfp_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID, 
            payment_details
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::AcceptRFPBid(
                    ACCOUNT_ID, 
                    RFP_ID,
                    BID_ID
                )
        ));
        assert!(
            PalletPayments::payment_agreements(
                (ACCOUNT_ID, BIDDER_ID, PAYMENT_ID)
            ).is_some()
        );
        let accepted_bid = 
            RFPModule::rfp_to_accepted_bid(RFP_ID).unwrap();
        assert_eq!(accepted_bid, BID_ID);
        
        let stored_details = 
            RFPModule::get_rfps(ACCOUNT_ID, RFP_ID).unwrap();
        assert_eq!(stored_details.rfp_status, RFPStatus::AcceptedBid);
    })
}

#[test]
fn test_accept_rfp_bid_again_fails() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        let cid: Vec<u8> = RFP_CID.as_bytes().to_vec();
        let ipfs_hash: [u8; 59] = cid.try_into().unwrap();
        let rfp_details = RFPDetails::<Test> {
            rfp_owner: ACCOUNT_ID,
            ipfs_hash,
            rfp_status: RFPStatus::AcceptingBids
        };
        assert_ok!(RFPModule::create_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            rfp_details.clone(),
        ));
        let bid_cid: Vec<u8> = BID_CID.as_bytes().to_vec();
        let bid_cid_hash: [u8; 59] = bid_cid.clone().try_into().unwrap();
        let bid_details = BidDetails::<Test> {
            bid_owner: BIDDER_ID,
            ipfs_hash: bid_cid_hash,
            bid_amount: BID_AMOUNT,
        };
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(BIDDER_ID),
            ACCOUNT_ID,
            RFP_ID,
            BID_ID,
            bid_details.clone()
        ));
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID
        ));
        let payment_schedule = bounded_vec![];
        let payment_method = pallet_payments::PaymentMethod::<Test> {
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: ACCOUNT_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: ACCOUNT_ID,
            payee: BIDDER_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: BID_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ACCOUNT_ID,
        };
        assert_ok!(RFPModule::accept_rfp_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_ID,
            payment_details.clone()
        ));
        assert_noop!(
            RFPModule::accept_rfp_bid(
                Origin::signed(ACCOUNT_ID),
                RFP_ID,
                BID_ID, 
                payment_details.clone()
            ),
            Error::<Test>::BidAlreadyAccepted
        );
    })
}