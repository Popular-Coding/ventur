
use crate::*;
use crate::{
    self as pallet_payments,
    Config as MyConfig
};
use frame_support::{
    assert_noop, 
    assert_ok,
    traits::{
        Currency,
    },
};
use mock::*;
use frame_support::bounded_vec;
use pallet_timestamp::{self as timestamp};

const PAYEE_ID: u64 = 1234;
const PAYMENT_ID: u32 = 0001;
const RFP_REFERENCE_ID: u32 = 966;
const TOTAL_PAYMENT_AMOUNT: u128 = 24601;
// const ESCROW_ACCOUNT_ID: u64 = 1999;
const ADMINISTRATOR_ID: u64 = 1410;
const PAYER_ID: u64 = 124;

#[test]
fn test_initialize_payment() {
    let mut t = test_externalities();
    t.execute_with(|| {
        assert!(System::events().is_empty());
        assert!(
            Payments::payment_agreements(
                (PAYER_ID, PAYEE_ID, PAYMENT_ID)
            ).is_none()
        );
        let time: u64 = <timestamp::Pallet<Test>>::now();
        let scheduled_payment_1 = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let scheduled_payment_2 = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time + 200,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let payment_schedule = bounded_vec![
            scheduled_payment_1, 
            scheduled_payment_2
        ];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: PAYER_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: PAYER_ID,
            payee: PAYEE_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: TOTAL_PAYMENT_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ADMINISTRATOR_ID,
        };
        assert_ok!(Payments::initialize_payment(
            Origin::signed(PAYER_ID),
            payment_details
        ));
        assert!(
            Payments::payment_agreements(
                (PAYER_ID, PAYEE_ID, PAYMENT_ID)
            ).is_some()
        );
        let expected_event = 
            crate::Event::PaymentInitialized(
                PAYER_ID, 
                PAYEE_ID, 
                TOTAL_PAYMENT_AMOUNT
            );
        System::assert_last_event(mock::Event::Payments(expected_event));
    });
}

#[test]
fn test_claim_successful_payment() {
    let mut t = test_externalities();
    t.execute_with(|| {
        assert!(System::events().is_empty());
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &PAYER_ID, 
            TOTAL_PAYMENT_AMOUNT
        );

        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &PAYER_ID
            ), 
            TOTAL_PAYMENT_AMOUNT
        );
        let time: u64 = <timestamp::Pallet<Test>>::now();
        let scheduled_payment_1 = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let scheduled_payment_2 = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time + 500,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let payment_schedule = bounded_vec![
            scheduled_payment_1, 
            scheduled_payment_2.clone()
        ];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: PAYER_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: PAYER_ID,
            payee: PAYEE_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: TOTAL_PAYMENT_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ADMINISTRATOR_ID,
        };
        assert_ok!(Payments::initialize_payment(
            Origin::signed(PAYER_ID),
            payment_details
        ));
        assert_ok!(
            Payments::claim(
                Origin::signed(PAYEE_ID),
                PAYER_ID, 
                PAYMENT_ID
            )
        );
        let expected_event = 
            crate::Event::PartOfPaymentClaimed(
                PAYEE_ID, 
                TOTAL_PAYMENT_AMOUNT / 2
            );
        System::assert_last_event(mock::Event::Payments(expected_event));
        let payment_agreements = Payments::payment_agreements(
            (PAYER_ID, PAYEE_ID, PAYMENT_ID)
        ).unwrap();
        let remaining_scheduled_payments = payment_agreements.payment_schedule;
        assert_eq!(remaining_scheduled_payments.len(), 1);
        assert_eq!(
            remaining_scheduled_payments.first().unwrap(), 
            &scheduled_payment_2,
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &PAYER_ID
            ), 
            TOTAL_PAYMENT_AMOUNT - (TOTAL_PAYMENT_AMOUNT / 2),
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &PAYEE_ID
            ), 
            TOTAL_PAYMENT_AMOUNT / 2,
        );
    });
}

    
#[test]
fn test_claim_fails_before_payment_date() {
    let mut t = test_externalities();
    t.execute_with(|| {
        assert!(System::events().is_empty());
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &PAYER_ID, 
            TOTAL_PAYMENT_AMOUNT
        );
        let time: u64 = <timestamp::Pallet<Test>>::now();
        let scheduled_payment = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time + 500,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let payment_schedule = bounded_vec![
            scheduled_payment.clone(), 
        ];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: PAYER_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: PAYER_ID,
            payee: PAYEE_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: TOTAL_PAYMENT_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ADMINISTRATOR_ID,
        };
        assert_ok!(Payments::initialize_payment(
            Origin::signed(PAYER_ID),
            payment_details
        ));
        assert_noop!(
            Payments::claim(
                Origin::signed(PAYEE_ID),
                PAYER_ID, 
                PAYMENT_ID
            ),
            Error::<Test>::PaymentNotAvailable
        );
        let payment_agreements = Payments::payment_agreements(
            (PAYER_ID, PAYEE_ID, PAYMENT_ID)
        ).unwrap();
        let remaining_scheduled_payments = payment_agreements.payment_schedule;
        assert_eq!(remaining_scheduled_payments.len(), 1);
        assert_eq!(
            remaining_scheduled_payments.first().unwrap(), 
            &scheduled_payment,
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &PAYER_ID
            ), 
            TOTAL_PAYMENT_AMOUNT,
        );
        assert_eq!(
            <Test as MyConfig>::PaymentCurrency::total_balance(
                &PAYEE_ID
            ), 
            0,
        );
    });
}

#[test]
fn test_block_and_unblock_payment() {
    let mut t = test_externalities();
    t.execute_with(|| {
        assert!(System::events().is_empty());
        let _ = <Test as MyConfig>::PaymentCurrency::deposit_creating(
            &PAYER_ID, 
            TOTAL_PAYMENT_AMOUNT
        );
        let time: u64 = <timestamp::Pallet<Test>>::now();
        let scheduled_payment = pallet_payments::ScheduledPayment::<Test> {
            payment_date: time,
            amount_per_claim: TOTAL_PAYMENT_AMOUNT / 2,
            released: true,
        };
        let payment_schedule = bounded_vec![
            scheduled_payment.clone(), 
        ];
        let payment_method = pallet_payments::PaymentMethod::<Test>{
            payment_source: pallet_payments::PaymentSource::PersonalAccount,
            account_id: PAYER_ID,
        };
        let payment_details = pallet_payments::PaymentDetails::<Test> {
            payer: PAYER_ID,
            payee: PAYEE_ID,
            payment_id: PAYMENT_ID,
            rfp_reference_id: RFP_REFERENCE_ID,
            total_payment_amount: TOTAL_PAYMENT_AMOUNT.into(),
            payment_schedule,
            payment_method: payment_method.clone(),
            administrator_id: ADMINISTRATOR_ID,
        };
        assert_ok!(Payments::initialize_payment(
            Origin::signed(PAYER_ID),
            payment_details
        ));
        assert_ok!(
            Payments::block_next_payment(
                Origin::signed(PAYER_ID),
                PAYEE_ID,
                PAYMENT_ID,
            )
        );
        let expected_event = 
            crate::Event::NextPaymentReleaseStatusChanged(
                PAYER_ID, 
                PAYMENT_ID,
                false
            );
        
        System::assert_last_event(mock::Event::Payments(expected_event));
        assert_noop!(
            Payments::claim(
                Origin::signed(PAYEE_ID),
                PAYER_ID, 
                PAYMENT_ID
            ),
            Error::<Test>::PaymentNotReleased
        );
        let payment_agreements = Payments::payment_agreements(
            (PAYER_ID, PAYEE_ID, PAYMENT_ID)
        ).unwrap();
        let remaining_scheduled_payments = payment_agreements.payment_schedule;
        assert_eq!(remaining_scheduled_payments.len(), 1);
        assert_eq!(
            remaining_scheduled_payments.first().unwrap().released, 
            false,
        );

        assert_ok!(
            Payments::release_next_payment(
                Origin::signed(PAYER_ID),
                PAYEE_ID,
                PAYMENT_ID,
            )
        );
        assert_ok!(
            Payments::claim(
                Origin::signed(PAYEE_ID),
                PAYER_ID, 
                PAYMENT_ID
            )
        );
    });
}

