# Attribution: https://github.com/paritytech/scripts/blob/master/dockerfiles/ci-linux/Dockerfile
# Copyright 2022 Parity Technologies

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at

# 	http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Changes: Created temp image for build, Specified ventur node build folders, Copied binaries to ubuntu image

FROM rust as temp

# Build dependencies
RUN apt-get update && apt-get install -y git clang libclang-dev curl libssl-dev llvm libudev-dev pkg-config make cmake libprotobuf-dev protobuf-compiler

# Install rust and tools 
RUN set -eux && \
	# install `rust-src` component for ui test
	rustup component add rust-src rustfmt clippy && \
	# install specific Rust nightly, default is stable, use minimum components
	rustup toolchain install nightly-2022-07-25 --profile minimal --component rustfmt clippy && \
	# "alias" pinned nightly-2022-07-25 toolchain as nightly
	ln -s /usr/local/rustup/toolchains/nightly-2022-07-25-x86_64-unknown-linux-gnu /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \
	# install wasm toolchain
	rustup target add wasm32-unknown-unknown && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	# install cargo tools
	cargo install cargo-web wasm-pack cargo-deny cargo-nextest && \
  	cargo install --locked cargo-spellcheck && \
	cargo install --version 0.4.2 diener && \
	# wasm-bindgen-cli version should match the one pinned in substrate
	cargo install --version 0.2.73 wasm-bindgen-cli && \
	# install wasm-gc. It's useful for stripping slimming down wasm binaries (polkadot)
	cargo +nightly install wasm-gc && \
	# versions
	rustup show && \
	cargo --version && \
	# apt clean up
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* && \
	# cargo clean up
	rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache

# Compile binaries for ventur node 
WORKDIR /ventur-node
COPY . /ventur-node
RUN cargo build --release --locked 

# Ventur Node Image
FROM ubuntu:20.04
COPY --from=temp /ventur-node/target/release/ventur-node /usr/local/bin
EXPOSE 9944
ENTRYPOINT /usr/local/bin/ventur-node --dev --ws-external
STOPSIGNAL SIGTERM