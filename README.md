# BITGREEN Node

This is the BitGreen Node based on Substrate Framework 3.x

## New Features
- Smart Contract Support in native [!Ink language](https://substrate.dev/docs/en/knowledgebase/smart-contracts/ink-development) a Rust based embedded domain specific language. 


## Installation

Follow these steps to get started with the BitGreen Node:  

- First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

- Use Rust's native `cargo` command to build and launch the BitGreen Node:

```sh
cargo run --release -- --dev --tmp
```
You can build without launching:

```sh
cargo build --release
```


### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/bitg-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/bitg-node --dev
```

Purge the development chain's state:

```bash
./target/release/bitg-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/bitg-node -lruntime=debug --dev
```
 
## Smart Contracts
BitGreen has now a strong support for [smart contracts written in RUST language](doc/smartcontracts.md)  

## Assets (Fungible Tokens)
A specific module for [fungible tokens (ERC20)](doc/assets.md) is included in the blockchain node.

## Staking (for Validators)
A specific module is available to stake and un-stake funds to qualify as validator of the blockchain.  
A validator gains BITG for every new block written. The block writing allow to the top 250 validators for stakes locked.  
[Here is the documentation.](doc/staking.md)  

## Decentralized Autonomous Organization
BitGreen blockchain has specific support for the [Decentralized Autonoumous Organization.](doc/dao.md)  



 
