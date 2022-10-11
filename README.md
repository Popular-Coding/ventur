# ![Ventur](media/ventur-cover.webp)

<div align="center">

[![License](https://img.shields.io/github/license/Popular-Coding/ventur?color=green)](https://github.com/Popular-Coding/ventur/blob/main/LICENSE)
[![Unit Tests](https://github.com/Popular-Coding/ventur/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/test.yml) [![Cargo Check Release](https://github.com/Popular-Coding/ventur/actions/workflows/check-release.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/check-release.yml)

</div>

**A Business and Professional Enablement [Parachain](https://polkadot.network/technology/) built with [Substrate](https://substrate.dev).**



## Running a Ventur Node (Ubuntu)
 
### Setup - Environment Setup



#### Install Dependencies 
```
sudo apt install build-essential
sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```

#### Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source ~/.cargo/env

rustup default stable

rustup update nightly

rustup update stable
 
rustup target add wasm32-unknown-unknown --toolchain nightly
```
 
## Build a Ventur Node
 ### Fetch the code
 The following command pulls the ventur-node code from our github repo. 

```
git clone https://github.com/PopularCoding/ventur

cd ventur
 ```
 
 ### Build the node
 The following command builds the node. (This may take some time)
 ```
cargo build --release
```

## Run the node

#### Single-Node


The ``` cargo run ``` command will perform an initial build and launch the node.   If you built the node with the ``` cargo build --release ``` command in the previous build the node step. Use the following code to run the node with the build you have already completed. 



This command will start a single node with a persistent state:

``` 
./target/release/ventur-node --dev
```

To purge the development chain's state:
```
./target/release/ventur-node purge-chain --dev
```

To Start the development chain with detailed logging. 

```
RUST_BACKTRACE=1 ./target/release/ventur-node -ldebug --dev
```

## Running a Ventur Node (Docker)

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

## Run Tests

Unit tests can be run locally using the ``` cargo test ``` command

## Runtime Architecture
Pallets:
- Payment-Contracts
- Escrow
- NT-NFT (Targeted for Milestone 2)
- RFP-Process (Targeted for Milestone 2)
