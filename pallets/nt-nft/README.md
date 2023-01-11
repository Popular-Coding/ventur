# NT-NFT Pallet

<div align="center">

[![License](https://img.shields.io/github/license/Popular-Coding/ventur?color=green)](https://github.com/Popular-Coding/ventur/blob/main/LICENSE)
[![rustdoc](https://img.shields.io/badge/rustdoc-nt_nft_pallet-informational)](https://docs.ventur.network/pallet_ntnft/index.html)
</div>

## NT-NFT Pallet Setup and Testing Guide (Ubuntu)

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
git clone https://github.com/PopularCoding/ventur

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

_Confirm that you can see the recent blocks listed._
If you are not able to access the block explorer on polkadot.js.org, you should:

1. Confirm that your Ventur node is running
2. Check if your Ventur node is running the JSON-RPC WS server on an address and port other than ```127.0.0.1:9944```
    a. If your node is running on a different address and port, update the custom endpoint in polkadot.js.org to the address and port number your node is serving

    | _Setting your Development Node Endpoint in polkadot.js.org_ |
    |:--:|
    |![Setting your Custom Endpoint](docs/setting-custom-endpoint.png)|

#### 3. Test Creating an NT-NFT Collection

| _1. Create an NT-NFT Collection_ |
|:--:|
|![Creating an NT-NFT Collection](docs/create-collection.png)|
Create an NT-NFT Collection.
Example values:
collectionId: ```150```
imageIpfsCid: ```QmaG1CtUr74GPQwZeAnFhpiSgwtwGyR3zK2BRYh4DPDw3c```
metadataIpfsCid: ```Qmb232AquR57EMUGgU92TxeZ8QyAJF5nERjdPZRNNJoh6z```

| _2. Verify created NT-NFT Collection_ |
|:--:|
|![Verify NT-NFT Collection in Chain State](docs/verify-create-collection.png)|
#### 4. Test Minting an NT-NFT


| _1. Mint an NT-NFT to the previously created Collection_ |
|:--:|
|![Minting an NT-NFT](docs/mint.png)|

| _2. Verify minting NT-NFT to Collection_ |
|:--:|
|![Verify NT-NFT Mint](docs/verify-mint.png)|

#### 5. Test Burning an NT-NFT

| _1. Burn the previously created NT-NFT_ |
|:--:|
|![Burning an NT-NFT](docs/burn.png)|

| _2. Verify burning the NT-NFT_ |
|:--:|
|![Verify NT-NFT Burn](docs/verify-burn.png)|

#### 6. Test Assigning an NT-NFT to an Address

| _1. Mint another NT-NFT to the previously created Collection_ |
|:--:|
|![Minting an NT-NFT](docs/second-mint.png)|

| _2. Assign the new NT-NFT to an Account_ |
|:--:|
|![Proposing an NT-NFT Assignment](docs/assign.png)|

| _3. Verify the NT-NFT Proposed Assignment_ |
|:--:|
|![Verify the proposed assignment of an NT-NFT](docs/verify-assign.png)|

#### 7. Test Accepting an Assigned NT-NFT

| _1. Accept the NT-NFT assignment from the assigned account_ |
|:--:|
|![Accept an NT-NFT Assignment](docs/accept-assignment.png)|

| _2. Verify the Accepted NT-NFT_ |
|:--:|
|![Verify the accepted NT-NFT](docs/verify-assign-final.png)|

#### 8. Test Rejecting an Assigned NT-NFT

| _1. Mint another NT-NFT to the previously created Collection_ |
|:--:|
|![Minting an NT-NFT](docs/.png)|

| _2. Assign the new NT-NFT to an Account_ |
|:--:|
|![Proposing an NT-NFT Assignment](docs/.png)|

| _3. Verify the NT-NFT Proposed Assignment_ |
|:--:|
|![Verify the proposed assignment of an NT-NFT](docs/.png)|

| _3. Reject NT-NFT Proposed Assignment_ |
|:--:|
|![Reject the proposed assignment of an NT-NFT](docs/.png)|

| _3. Verify the NT-NFT Rejection_ |
|:--:|
|![Verify the rejected assignment of an NT-NFT](docs/.png)|

#### 9. Test Discarding an Assigned NT-NFT

| _1. Mint another NT-NFT to the previously created Collection_ |
|:--:|
|![Minting an NT-NFT](docs/.png)|

| _2. Assign the new NT-NFT to an Account_ |
|:--:|
|![Proposing an NT-NFT Assignment](docs/.png)|

| _3. Accept the new NT-NFT from the Account_ |
|:--:|
|![Accept an NT-NFT Assignment](docs/.png)|

| _2. Verify the Accepted NT-NFT_ |
|:--:|
|![Verify the accepted NT-NFT](docs/.png)|

| _2. Discard the NT-NFT_ |
|:--:|
|![Discard the NT-NFT](docs/.png)|

| _2. Verify Discarding the NT-NFT_ |
|:--:|
|![Verify discarding the NT-NFT](docs/.png)|

#### 10. Test Freezing an NT-NFT Collection

| _1. Freeze the previously created Collection_ |
|:--:|
|![Freezing a Collection](docs/.png)|

| _2. Attempt to mint another NT-NFT to the Collection_ |
|:--:|
|![Attempt to mint an NT-NFT](docs/.png)|

| _2. Verify that the mint failed_ |
|:--:|
|![Verify Failed Mint](docs/.png)|

#### 11. Test Thawing an NT-NFT Collection

| _1. Thaw the frozen Collection_ |
|:--:|
|![Thaw a Collection](docs/.png)|

| _2. Attempt to mint another NT-NFT to the Collection_ |
|:--:|
|![Attempt to mint an NT-NFT](docs/.png)|

| _2. Verify that the mint succeeded_ |
|:--:|
|![Verify Successful Mint](docs/.png)|

#### 12. Test Retiring an NT-NFT Collection

1. Destroy
2. Confirm
