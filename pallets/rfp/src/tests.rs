use crate::*;
use frame_support::{
    assert_ok, assert_noop
};
use mock::*;

const ACCOUNT_ID: u64 = 24601;
const RFP_ID: u64 = 1410;
const BID_AMOUNT: u128 = 1999;
const NEW_BID_AMOUNT: u128 = 1525;
const RFP_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq";
const OTHER_CID: &str = "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptpg";

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
        assert_ok!(RFPModule::bid_on_rfp(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            BID_AMOUNT
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::BidOnRFP(
                    ACCOUNT_ID, 
                    RFP_ID,
                    BID_AMOUNT
                )
        ));
    })
}

#[test]
fn test_shortlist_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        assert_ok!(RFPModule::shortlist_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::ShortlistBid(
                    ACCOUNT_ID, 
                    RFP_ID,
                )
        ));
    })
}

#[test]
fn test_update_rfp_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        assert_ok!(RFPModule::update_rfp_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
            NEW_BID_AMOUNT
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::UpdateRFPBid(
                    ACCOUNT_ID, 
                    RFP_ID,
                    NEW_BID_AMOUNT
                )
        ));
    })
}

#[test]
fn test_accept_rfp_bid() {
    let mut t = test_externalities();
    t.execute_with(||
    {
        assert!(System::events().is_empty());
        assert_ok!(RFPModule::accept_rfp_bid(
            Origin::signed(ACCOUNT_ID),
            RFP_ID,
        ));
        System::assert_last_event(
            mock::Event::RFPModule(
                crate::Event::AcceptRFPBid(
                    ACCOUNT_ID, 
                    RFP_ID,
                )
        ));
    })
}