use crate::*;
use mock::*;
use crate::Config as MyConfig;
use frame_support::{
    assert_ok, 
    assert_noop,
    traits::{
        Currency,
    },
};
use sp_runtime::BoundedVec;

const OWNER_ACCOUNT_ID: u64 = 11111;
const SUBSCRIBER_ID: u64 = 1234;
const SUBSCRIPTION_SERVICE_ID: u128 = 128;
const SUBSCRIPTION_ID: u128 = 9;
const BASE_SUBSCRIPTION_FEE: u128 = 96;

#[test]
fn test_create_subscription() {
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
                SubscriptionFeeFrequency::Monthly,
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
    
        let subscription_service_details = 
            SubscriptionsModule::get_subscription_services(
                SUBSCRIPTION_SERVICE_ID
            );
        assert!(subscription_service_details.is_some());
        let subscriptions = 
            SubscriptionsModule::get_subscription_services_to_subscription_ids(
                OWNER_ACCOUNT_ID, SUBSCRIPTION_SERVICE_ID
            ).unwrap();
        
        assert!(subscriptions.is_empty());
    });  
}

#[test]
fn test_fail_on_reused_id() {
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
                SubscriptionFeeFrequency::Monthly,
                meta_cid.clone(),
            )
        );
        assert_noop!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Monthly,
                meta_cid,
            ),
            Error::<Test>::SubscriptionIdExists
        );
    });  
}

#[test]
fn test_initiate_subscription() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE
        );
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Monthly,
                meta_cid.clone(),
            )
        );
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
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &OWNER_ACCOUNT_ID
            ), 
            BASE_SUBSCRIPTION_FEE
        );
        
        let subscriptions_for_service = 
            SubscriptionsModule::get_subscription_services_to_subscription_ids(
                OWNER_ACCOUNT_ID, SUBSCRIPTION_SERVICE_ID
            ).unwrap();
        let subscription = subscriptions_for_service.first().unwrap();
        assert_eq!(subscription, &SUBSCRIPTION_ID);
        
        let subscription = 
            SubscriptionsModule::get_subscriptions(
                SUBSCRIPTION_ID
            ).unwrap();
        assert_eq!(subscription.subscription_id, SUBSCRIPTION_ID);
    }
    );  
}

#[test]
fn test_initiate_subscription_fails_with_no_service() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE
        );
        assert_noop!(
            SubscriptionsModule::initiate_subscription(
                Origin::signed(SUBSCRIBER_ID),
                90,
                SUBSCRIPTION_ID,
                OWNER_ACCOUNT_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Monthly,
            ),
            Error::<Test>::NonExistentSubscriptionService
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &OWNER_ACCOUNT_ID
            ), 
            0
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &SUBSCRIBER_ID
            ), 
            BASE_SUBSCRIPTION_FEE
        );
    }
    );  
}

#[test]
fn test_claim_subscription_payment_fails_for_uninstantiated_service() {
    let mut t = test_externalities();
    t.execute_with(||
    { 
        assert!(System::events().is_empty());
        assert_noop!(
            SubscriptionsModule::claim_subscription_payment(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
            ),
            Error::<Test>::NonExistentSubscriptionService
        );
    }
    );  
}

#[test]
fn test_claim_subscription_payment_fails_for_uninstantiated_subscription() {
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
                SubscriptionFeeFrequency::Monthly,
                meta_cid,
            )
        );
        assert_noop!(
            SubscriptionsModule::claim_subscription_payment(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
            ),
            Error::<Test>::NoSubscriptionForService
        );
    }
    );  
}

#[test]
fn test_claim_subscription_fails_before_payment_date() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE
        );
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Monthly,
                meta_cid.clone(),
            )
        );
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
        assert_noop!(
            SubscriptionsModule::claim_subscription_payment(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
            ),
            Error::<Test>::ClaimingPaymentBeforeDueDate
        );
    }
    );  
}

#[test]
fn test_claim_payment() {
    let mut t = test_externalities();
    t.execute_with(||
    {   
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE * 2
        );
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
                meta_cid.clone(),
            )
        );
        assert_ok!(
            SubscriptionsModule::initiate_subscription(
                Origin::signed(SUBSCRIBER_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
                OWNER_ACCOUNT_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
            )
        );
        assert_ok!(
            SubscriptionsModule::claim_subscription_payment(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
            )
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &OWNER_ACCOUNT_ID
            ), 
            2 * BASE_SUBSCRIPTION_FEE
        );
    }
    );  
}


#[test]
fn test_cancel_subscription_fails_with_non_owner_cancelling() {
    let mut t = test_externalities();
    t.execute_with(||
    { 
        assert!(System::events().is_empty());
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE * 2
        );
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
                meta_cid.clone(),
            )
        );
        assert_ok!(
            SubscriptionsModule::initiate_subscription(
                Origin::signed(SUBSCRIBER_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
                OWNER_ACCOUNT_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
            )
        );
        assert_noop!(
            SubscriptionsModule::cancel_subscription(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_ID,
            ),
            Error::<Test>::NonOwnerModifyingSubscription
        );
    }
    );  
}

#[test]
fn test_cancel_subscription() {
    let mut t = test_externalities();
    t.execute_with(||
    { 
        assert!(System::events().is_empty());
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &SUBSCRIBER_ID, 
            BASE_SUBSCRIPTION_FEE * 2
        );
		let meta_cid: BoundedVec<u8, ConstU32<{VEC_LIMIT}>> = b"Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z".to_vec().try_into().unwrap();
        assert_ok!(
            SubscriptionsModule::create_subscription_service(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
                meta_cid.clone(),
            )
        );
        assert_ok!(
            SubscriptionsModule::initiate_subscription(
                Origin::signed(SUBSCRIBER_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
                OWNER_ACCOUNT_ID,
                BASE_SUBSCRIPTION_FEE,
                SubscriptionFeeFrequency::Adhoc,
            )
        );
        assert_ok!(
            SubscriptionsModule::cancel_subscription(
                Origin::signed(SUBSCRIBER_ID),
                SUBSCRIPTION_ID,
            ),
        );
        System::assert_last_event(
            mock::Event::SubscriptionsModule(
                crate::Event::CancelSubscription(
                    SUBSCRIBER_ID, 
                    SUBSCRIPTION_ID,
                )
        ));
        assert_noop!(
            SubscriptionsModule::claim_subscription_payment(
                Origin::signed(OWNER_ACCOUNT_ID),
                SUBSCRIPTION_SERVICE_ID,
                SUBSCRIPTION_ID,
            ),
            Error::<Test>::ClaimingPaymentFromInactiveSubscription
        );
    }
    );  
}