# Payment Pallet Testing Guide (Ubuntu)

<div align="center">

[![License](https://img.shields.io/github/license/Popular-Coding/ventur?color=green)](https://github.com/Popular-Coding/ventur/blob/main/LICENSE)
[![rustdoc](https://img.shields.io/badge/rustdoc-payment_pallet-informational)](https://docs.ventur.network/pallet_payments/index.html)
[![Payments Unit Test](https://github.com/Popular-Coding/ventur/actions/workflows/test-payments.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/test-payments.yml)

</div>

## Payment Pallet Setup and Testing Guide (Ubuntu)

### Prerequisite Setup

#### Install Dependencies

```bash
sudo apt install build-essential
sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```

#### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source ~/.cargo/env

rustup default stable

rustup update stable

rustup update nightly

rustup install nightly-2022-09-19 

rustup override set nightly-2022-09-19

rustup target add wasm32-unknown-unknown

rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Deploy a Local Ventur Node

#### Fetch the code

The following command pulls the ventur-node code from our github repo:

```bash
git clone https://github.com/Popular-Coding/ventur.git

cd ventur
```

#### Run the node

The following command builds the node. (This may take some time):

```bash
cargo run --release -- --dev
```

### Run Unit Tests

Unit tests can be run locally using the following command:

```bash
cargo test
```

### Manual Test Guide

#### 1. Start the node

```bash
cargo run --release -- --dev
```

| _Running your local node_ |
|:--:|
|![Running the Node](docs/running-node.png)|

#### 2. Access the Node through the polkadot.js.org interface

Once you have a ventur node running locally, follow this link:
[https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer)

| _Accessing your Development Node Endpoint in polkadot.js.org_ |
|:--:|
|![Accessing the Node](docs/access-polkadot-js-org.png)|

Confirm that you can see the recent blocks listed.
If you are not able to access the block explorer on polkadot.js.org, you should:

1. Confirm that your Ventur node is running
2. Check if your Ventur node is running the JSON-RPC WS server on an address and port other than ```127.0.0.1:9944```
    a. If your node is running on a different address and port, update the custom endpoint in polkadot.js.org to the address and port number your node is serving

    | _Setting your Development Node Endpoint With the Payments Pallet_ |
    |:--:|
    |![Setting your Custom Endpoint](docs/setting-custom-endpoint.png)|

#### 3. Test Out Functionality

##### Initialize A One-Time Payment

1. Select `payments` in the dropdown under `submit the following extrinsic`
2. Select the `initializePayment` extrinsic in the next dropdown
3. Select the appropriate actor to be the payer id
4. Fill out the fields with the desired values (Note: if you'd like to test out claiming a payment, make sure the unixtime is soon enough to collect the payment within this testing session)
5. Submit the transaction
    | _Initializing a One-Time Payment in polkadot.js.org_ |
    |:--:|
    |![Initializing a One-Time Payment](docs/one-time-payment-init.png)|

    | _Transaction Succeeded_ |
    |:--:|
    |![Initialization Success](docs/init-success.png)|

##### Claim a Payment

1. Select `payments` in the dropdown under `submit the following extrinsic`
2. Select the `claim` extrinsic in the next dropdown
3. Make sure the acting account matches the `payee_id` field from your payment initialization
4. Make sure the `payer_id` field matches the acting account from your payment initialization
5. Submit the transaction. You should see a successful transaction

    | _Claiming Payment With the Payments Pallet_ |
    |:--:|
    |![Claiming a Payment](docs/claim-payment.png)|

#### Initialize Scheduled Payments

1. Follow the same steps lined out in the Initialize One-Time Payment Section
2. In Scheduled Payments, click on `Add Item`
3. Add as many payments as you'd like to test, choosing the scheduled unixtime for payment release
4. Submit the transaction

    | _Scheduling Multiple Payments_ |
    |:--:|
    |![Multiple Payments](docs/multiple-payments.png)|

#### Claiming Unavailable Payments

1. Follow the same steps lined out in the Claim a Payment Section
2. Try to claim as many payments as you'd like
3. If you have scheduled a payment for a future time, the transaction should fail

    | _Claiming A Payment That Isn't Due Yet_ |
    |:--:|
    |![Payment Failure](docs/payment-failure.png)|

#### Initialize a Payment from Escrow

1. Create an escrow

    | _Create an Escrow_ |
    |:--:|
    |![Create an escrow](docs/create-escrow.png)|

2. Fund an Escrow
Call the `fundEscrow` extrinsic with a balance of:
`1000000000000000000`

    | _Fund an Escrow_ |
    |:--:|
    |![Fund Escrow](docs/fund-escrow.png)|

3. Check Account Balances

    | _Note Account Balances_ |
    |:--:|
    |![Account Balances](docs/note-accounts.png)|

4. Initialize a payment contract

    | _Initializing a Payment From Escrow_ |
    |:--:|
    |![Initialize Payment Contract](docs/init-escrow-pay.png)|

5. Claim a payment

    | _Claiming A Payment From Escrow_ |
    |:--:|
    |![Claim payment from escrow](docs/claim-escrow-pay.png)|

6. Confirm Expected Changes to Account Balances

    | _Confirm Payment has Come from the Payer's Escrow_ |
    |:--:|
    |![Payment From Escrow](docs/balances-pay-from-escrow.png)|
