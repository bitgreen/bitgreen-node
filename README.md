
<p align="center">
  <img src="./doc/images/bitgreen-logo.png" width="460">
</p>

# Bitgreen Parachain

This repository contains the core logic and associated tools to issue and manage verified carbon credit units on the Bitgreen parachain.

<!-- TOC -->

- [1. Introduction](#1-introduction)
- [2. Overview](#2-overview)
  - [2.1. Carbon Credits Pallet](#21-carbon-credits-pallet)
  - [2.2. Carbon Credits Pool](#22-carbon-credits-pool)
  - [2.2. Cache Engine](#23-cache-engine)
- [3. Building](#3-building)
- [4. Run local testnet](#4-run-local-testnet)
- [5. Development](#5-development)
- [6. Run a Collator](#6-run-a-collator)
- [7. Bug Bounty :bug:](#6-bug-bounty-bug)
- [8. Contribute](#7-contribute)

<!-- /TOC -->

# 1. Introduction
Bitgreen is an open and permissionless blockchain built to meet the needs of NGOs, corporate ESG groups, and purpose-driven innovation in Web3. Bitgreen makes it easy to finance, originate and purchase high quality, transparent Carbon Credits that conserve nature, remove atmospheric CO2 and send financial benefits to local communities.

# 2. Overview
Bitgreen provides a convenient and easy-to-use platform for the aforementioned Carbon Credit management processes and the Bitgreen Impact Investment Platform delivers the first blockchain marketplace for discovering and buying digital green bonds.

## 2.1. Carbon Credits Pallet
Each Carbon Credits represents a reduction or removal of one tonne of carbon dioxide equivalent (CO2e) achieved by a project. Carbon Creditss are characterized by a number of quality assurance principles which are confirmed through the project validation and verification process. Carbon Credits are ultimately purchased and retired by an end user as a means of offsetting their emissions. The Carbon Credits pallet manages the creation and retirement of Carbon Credits units for the bitgreen runtime. Credits in a project are represented in terms of batches, these batches are usually seperated in terms of 'vintages'. The vintage refers to the `age` of the credit. For more details, see the pallet [documentation](./pallets/carbon-credits/README.md)

## 2.2. Carbon Credits Pool
 The Carbon Credits Pool pallet lets users create and manage Carbon Credits pools. A Carbon Credits pool is a collection of Carbon Credits tokens of different types represented by a common pool token. A user holding any Carbon Credits tokens (subject to the Carbon Credits pool config) can deposit Carbon Credits tokens to the pool and receive equivalent pool tokens in return. These pool tokens can be transferred freely and can be retired. When retire function is called, the underlying Carbon Credits credits are retired starting from the oldest in the pool. For more details, see the pallet [documentation](./pallets/carbon-credits-pool/README.md)

## 2.3. Cache Engine
 We developed a light cache engine to query the transactions by account date/time limits and transaction id (txhash).  
It's reachable at: [https://testnet.bitgreen.org:9443](https://testnet.bitgreen.org:9443). You can install in your node as from [instructions here](cache-engine/README.md)

# 3. Building

Follow these steps to get started with the BitGreen parachain:  

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

You may need additional dependencies, checkout [substrate.io](https://docs.substrate.io/v3/getting-started/installation) for more info

```bash
sudo apt-get install -y git clang curl libssl-dev llvm libudev-dev
```
You can build without launching:

```sh
git clone https://github.com/bitgreen/bitgreen-node
cd bitgreen-node
cargo build --release -p bitgreen-parachain
```

# 4. Run local testnet

Since Bitgreen is a parachain, you can only run it alongside a relaychain, the following steps describe how to get the parachain and relaychain running with [polkadot-launch](https://github.com/paritytech/polkadot-launch)

1. Install polkadot-launch  `npm i polkadot-launch -g`

2. Clone the polkadot repo and build the relaychain, follow instructions in [cumulus repo](https://github.com/paritytech/cumulus#launch-the-relay-chain)

2. Ensure the relaychain build path matches the path insde `polkadot-launch/config.json`

2. Run `polkadot-launch ./polkadot-launch/config.json`


# 5. Development

Bitgreen is following the [Substrate code style](https://github.com/paritytech/substrate/blob/master/docs/STYLE_GUIDE.md).

In addition, we incorporate several tools to improve code quality. These are integrated into our CI
and are expected to pass before a PR is considered mergeable. They can also be run locally.

* [clippy](https://github.com/rust-lang/rust-clippy) - run with `cargo clippy --release --workspace`
* [rustfmt](https://github.com/rust-lang/rustfmt) - run with `cargo fmt -- --check`
* [dprint](https://dprint.dev/plugins/toml/) - run with `dprint fmt`

### Directory Structure

The following is a list of directories of interest in development.

|Directory              |Purpose                                                                     |
| --------------------- | -------------------------------------------------------------------------- |
|doc/                   | High level documentation                                                   |
|cache-engine/          | Cache engine source (js)                                                   |
|parachain/             | Bitgreen's main node (rust)                                                |
|pallets/               | Bitgreen's Substrate runtime pallets (rust)                                |
|primitives/            | Base types used in runtime                                                 |
|runtime/               | Bitgreen's runtime (on-chain) code (rust, compiled to WASM)                |
|scripts/               | Utilities for launching and interacting with a Bitgreen chain (typescript) |
|tools/                 | Various tools generally related to development (typescript)                |

### Build the node 
```
git clone https://github.com/bitgreen/bitgreen-node
cd bitgreen-node

# Build the node (The first build will be long (~30min))
cargo build --release
```

### Run tests
```
(Run tests for entire chain)
cargo test

(Run tests for pallet)
cargo test -p <pallet-name>
```

### Run benchmark tests
```
(Run benchmark tests for entire chain)
cargo test --features runtime-benchmarks

(Run benchmark tests for pallet)
cargo test -p <pallet-name>  --features runtime-benchmarks
```

# 6. Run a collator

Collators aggregate transactions on parachains into blocks and then propose these to the relay chain validators for finalization. Collators do not contribute to network security and only fill the role of a proposer and aggregator. Collators in theory have a power to censor transactions by not including them in their block proposals. 

The Bitgreen network is a parachain and therefore only requires collators as it is secured by the Polkadot relay chain validators. You can build and run a bitgreen parachain collator, for detailed instructions, refer [collator guide](./doc/collator_guide.md).


# 7. Bugs Reporting
For bug reporting, please open an issue in our repo : [https://github.com/bitgreen/bitgreen-node/issues](https://github.com/bitgreen/bitgreen-node/issues). For security issues, kindly report issues to info@bitgreen.org.

# 8. Contribute 
Bitgreen is open source under the terms of the MIT. We welcome contributions. Please review our CONTRIBUTIONS.md document for more information.