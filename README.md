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
 The following command pulls the ventur-node code from our GitHub repo. 

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

## Running a Ventur Node (Docker Compose)

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Clone the repo.
```
git clone https://github.com/PopularCoding/ventur

cd ventur
```


Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

## Running a Ventur Node (Docker File)
Clone the repo.
```
git clone https://github.com/PopularCoding/ventur

cd ventur
```
Then run this command to build the dockerfile.
```
docker build . -t ventur-node
```
To run the docker file with localhost:9944 published so it can interact with the Polkadot-JS front-end. (single node development chain)
```
docker run -p 9944:9944 ventur-node
```


## Connect with Polkadot-JS Apps Front-end

Once the node is running locally, you can connect it with **Polkadot-JS App** front-end
to interact with the development chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)



## Run Tests

Unit tests can be run locally using the ``` cargo test ``` command.

## Runtime Architecture
Pallets:
- Payment-Contracts
- Escrow
- NT-NFT (Targeted for Milestone 2)
- RFP-Process (Targeted for Milestone 2)
