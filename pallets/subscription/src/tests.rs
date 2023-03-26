use crate::*;
use mock::*;
use frame_support::assert_ok;
use sp_runtime::BoundedVec;

const OWNER_ACCOUNT_ID: u64 = 11111;
const SUBSCRIBER_ID: u64 = 1234;
const SUBSCRIPTION_SERVICE_ID: u128 = 128;
const SUBSCRIPTION_ID: u128 = 9;
const BASE_SUBSCRIPTION_FEE: u64 = 96;


#[test]
fn test_create_rfp() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert!(System::events().is_empty());
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                meta_cid,
            )
        );
        System::assert_last_event(
            mock::Event::SubscriptionsModule(
                crate::Event::CreateSubscription(
                    OWNER_ACCOUNT_ID, 
                    SUBSCRIPTION_SERVICE_ID,
                )
        ));
    }
    );  
}

#[test]
fn test_initiate_subscription() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        assert!(System::events().is_empty());
        assert_ok!(
            SubscriptionsModule::initiate_subscription(
                Origin::signed(SUBSCRIBER_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
                OWNER_ACCOUNT_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Monthly,
            )
        );
        System::assert_last_event(
            mock::Event::SubscriptionsModule(
                crate::Event::InitiateSubscription(
                    SUBSCRIBER_ID, 
                    SUBSCRIPTION_ID,
                )
        ));
    }
    );  
}