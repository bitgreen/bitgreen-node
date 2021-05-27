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
## Accounts  
### Address Format  
The address format used  is SS58. SS58 is a modification of Base-58-check from Bitcoin with some minor modifications. 
Notably, the format contains an address type prefix that identifies an address as belonging to a specific substrate network.  
We are using the prefix= "5".
For example a valid address is: 5DFJF7tY4bpbpcKPJcBTQaKuCDEPCpiz8TRjpmLeTtweqmXL  

### Address Generation  
A valid account only requires a private key that can sign on one of the supported curves and signature schemes. 
Most wallets take many steps from a mnemonic phrase to derive the account key, which affects the ability to use the same mnemonic phrase in multiple wallets.  
Wallets that use different steps will arrive at a different set of addresses from the same mnemonic.  

### Seed Generation  
Most wallets generate a mnemonic phrase for users to back up their wallets and generate a private key from the mnemonic.  
Not all wallets use the same algorithm to convert from mnemonic phrase to private key.  
Polkadot-js library uses the BIP39 dictionary for mnemonic generation, but use the entropy byte array to generate the private key, while full BIP39 wallets (like Ledger) use 2048 rounds of PBKDF2 on the mnemonic.  As such, the same mnemonic will not generate the same private keys. See Substrate BIP39 for more information.  

### Cryptography  
Bitgreen supports the following cryptographic key pairs and signing algorithms:  
- Ed25519  
- Sr25519 - Schnorr signatures on the Ristretto group  
- ECDSA signatures on secp256k1  
Note that the address for a secp256k1 key is the SS58 encoding of the hash of the public key in order to reduce the public key from 33 bytes to 32 bytes.  

### Account Data  
Account balance information is stored in a strructure "AccountData". Bitgreen primarily deals with two types of balances: free and reserved.  
For most operations, free balance is what you are interested in. It is the "power" of an account in staking and governance.   
Reserved balance represents funds that have been set aside by some operation and still belong to the account holder, but cannot be used.  
Locks are an abstraction over free balance that prevent spending for certain purposes.   
Several locks can operate on the same account, but they overlap rather than add.  
Locks are automatically added onto accounts when tasks are done on the network (e.g. leasing a parachain slot or voting).   
For example, an account could have a free balance of 200 BITG with two locks on it: 150 BITG for Transfer purposes and 100 BITG for Reserve purposes.  
The account could not make a transfer that brings its free balance below 150 BITG, but an operation could result in reserving BITG such that the free balance is below BITG, but above 100 BITG.  
Bonding tokens for staking and voting in governance referenda both utilize locks.  
Vesting is another abstraction that uses locks on free balance.  
Vesting sets a lock that decreases over time until all the funds are transferable.  

### Balances Module
The Balances module provides functionality for handling accounts and balances.  
The Balances module provides functions for:  

- Getting and setting free balances.  
- Retrieving total, reserved and unreserved balances.  
- Repatriating a reserved balance to a beneficiary account that exists.  
- Transferring a balance between accounts (when not reserved).  
- Slashing an account balance.  
- Account creation and removal.  
- Managing total issuance.  
- Setting and managing locks.  
[Further details are available here](https://substrate.dev/rustdocs/v3.0.0/pallet_balances/index.html)  


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


## Development Tools

You can interact with our testing node:
``` 
testnode.bitg.org 
```
using the web app hosted here:    
[https://polkadot.js.org/apps](https://polkadot.js.org/apps)  
To configure, click on the top left menu option and set the "Custom Node" field with 'wss://testnode.bitg.org'  
(Attention: Safari browser does not work with this app)

## Development Libraries

[JavaScript - Polkadot-JS API](https://polkadot.js.org/docs/api/)  
The Polkadot-JS API is a javascript library of interfaces for communicating with Substrate nodes like BitGreen node.  
The API provides application developers the ability to query a node and submit signed transaction using Javascript.  

[JavaScript Polkadot-JS Extension](https://github.com/polkadot-js/apps)    
The Polkadot-JS Extension is a simple extension for managing accounts in a browser extension and allowing the signing of extrinsics using these accounts.  
It also provides simple interface for interacting with extension-compliant dApps.  

[Python - py-substrate-interface](https://github.com/polkascan/py-substrate-interface)   
py-substrate-interface is a Python library for interacting with the BitGreen RPC.   
It supports a wide range of capabilities and powers the Polkascan multi-chain block explorer. This library is maintained by Polkascan Foundation.  

[Rust - Substrate-subtxt](https://github.com/paritytech/substrate-subxt)
A Rust library to submit extrinsics to BitGreen node via RPC.  

[Kotlin - Substrate-client-Kotlin](https://github.com/NodleCode/substrate-client-kotlin)
Substrate-client-kotlin is client library to interact with a substrate-based chain like BitGreen. 
It uses the API available from the RPC endpoint only (no sidecar). As of today it provides the following functionality:
- compatible with substrate 3.0 
- ed25519 wallet creation
- get account info (balance)
- sign extrinsic and send (immortal era)
- estimate fee








 
