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

1. Create
2. Confirm

#### 4. Test Minting an NT-NFT

1. Create
2. Mint
3. Confirm

#### 5. Test Burning an NT-NFT

1. Create
2. Mint
3. Burn
4. Confirm

#### 6. Test Assigning an NT-NFT to an Address

1. Create
2. Mint
3. Assign
4. Confirm

#### 7. Test Accepting an Assigned NT-NFT

1. Assign
2. Accept

#### 8. Test Rejecting an Assigned NT-NFT

1. Assign
2. Cancel

#### 9. Test Discarding an Assigned NT-NFT

1. Assign
2. Discard

#### 10. Test Freezing an NT-NFT Collection

1. Freeze
2. Mint - Fail

#### 11. Test Thawing an NT-NFT Collection

1. Thaw
2. Mint - Succeed

#### 12. Test Destroying an NT-NFT Collection

1. Destroy
2. Confirm
