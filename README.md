# ![Ventur](media/ventur-cover.webp)

<div align="center">

[![License](https://img.shields.io/github/license/Popular-Coding/ventur?color=green)](https://github.com/Popular-Coding/ventur/blob/main/LICENSE)
[![Unit Tests](https://github.com/Popular-Coding/ventur/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/test.yml) 
[![Cargo Check Release](https://github.com/Popular-Coding/ventur/actions/workflows/check-release.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/check-release.yml)
[![Check Docker Compose and Docker Build](https://github.com/Popular-Coding/ventur/actions/workflows/check-docker.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/check-docker.yml)
[![Escrow Unit Test](https://github.com/Popular-Coding/ventur/actions/workflows/test-escrow.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/test-escrow.yml)
[![Payments Unit Test](https://github.com/Popular-Coding/ventur/actions/workflows/test-payments.yml/badge.svg?branch=main)](https://github.com/Popular-Coding/ventur/actions/workflows/test-payments.yml)

</div>

**A Business and Professional Enablement [Parachain](https://polkadot.network/technology/) built with [Substrate](https://substrate.dev).**

## Running a Ventur Node (Ubuntu)

### Setup - Environment Setup

#### Install Dependencies

```bash
sudo apt install build-essential
sudo apt install -y git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
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

## Build a Ventur Node

### Fetch the code

The following command pulls the ventur-node code from our GitHub repo.

```bash
git clone https://github.com/Popular-Coding/ventur.git

cd ventur
```

### Build the node

The following command builds the node. (This may take some time)

```bash
cargo build --release
```

## Run the node

### Single-Node

The ``` cargo run ``` command will perform an initial build and launch the node.   If you built the node with the ``` cargo build --release ``` command in the previous build the node step. Use the following code to run the node with the build you have already completed.

This command will start a single node with a persistent state:

```bash
./target/release/ventur-node --dev
```

To purge the development chain's state:

```bash
./target/release/ventur-node purge-chain --dev
```

To Start the development chain with detailed logging.

```bash
RUST_BACKTRACE=1 ./target/release/ventur-node -ldebug --dev
```

## Running a Ventur Node (Docker)

### Running a Ventur Node (Docker Compose)

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Clone the repo.

```bash
git clone https://github.com/Popular-Coding/ventur.git

cd ventur
```

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

### Running a Ventur Node (Dockerfile)

Clone the repo.

```bash
git clone <https://github.com/Popular-Coding/ventur.git>

cd ventur

```

Then run this command to build the docker image.

```bash
docker build . -t ventur-node
```

To run the container with localhost:9944 published so it can interact with the Polkadot-JS front-end. (single node development chain)

```bash
docker run -p 9944:9944 ventur-node
```

## Connect with Polkadot-JS Apps Front-end

Once the node is running locally, you can connect it with **Polkadot-JS App** front-end
to interact with the development chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)

## Run Tests

Unit tests can be run locally using the ``` cargo test ``` command.

### Manual Test Guides

[![Test Guide](https://img.shields.io/badge/Test_Guide-payment_pallet-informational)](/pallets/payments/README.md)

[![Test Guide](https://img.shields.io/badge/Test_Guide-escrow_pallet-informational)](/pallets/escrow/README.md)

## Runtime Architecture

Pallets:

- Payment-Contracts

[![rustdoc](https://img.shields.io/badge/rustdoc-payment_pallet-informational)](https://docs.ventur.network/pallet_payment/index.html)

- Escrow

[![rustdoc](https://img.shields.io/badge/rustdoc-escrow_pallet-informational)](https://docs.ventur.network/pallet_escrow/index.html)

- NT-NFT (Targeted for Milestone 2)
- RFP-Process (Targeted for Milestone 2)
