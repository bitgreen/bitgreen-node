# EVM Utils  
  
Evm Pallet is a complex module that requires specific data conversion.  
in this folder you can find some utility to work on EVM.  

## Requirements  
Install [Node 14.7](https://nodejs.org/en/)  
Install [Yarn](https://yarnpkg.com/)  

## Dependencies Installation  
```bash
yarn add @polkadot/api
```

## Account conversion
EVM requires the usage of Ethereum Address and such address must have some funds deposited to load and execute a smart contract.  
The utility:  
```bash
substrateAddress2EvmAddress.js 
```
computes the EVM address required:  
For for example a valid Substrate Address is: 5D22LQopHohbANebYDftPRinrm3SVPpZZW6PK6RT2zj4shyx or the hex version 0x2a30b3682d0591387a016735959b7db04dc4ff4bbacf46d6f780bf29a1818c04  
you can launch the utility by:  
```bash
node substrateAddress2EvmAddress.js 5D22LQopHohbANebYDftPRinrm3SVPpZZW6PK6RT2zj4shyx
```
and obtain the following output:  
```bash
Converting Substrate Address: 5D22LQopHohbANebYDftPRinrm3SVPpZZW6PK6RT2zj4shyx
Computed Evm Address:  0x2a30b3682d0591387a016735959b7db04dc4ff4b
You can use this substrate address, to transfer funds to the EVM address above using the Pallet "Balanaces".
Computed Balance Address for EVM deposit Base58:  5FRmUWiFLmqz9PM5TKk3JUNiYW7k2VgvFd6PxDHUftMHkLmG
Computed Balance Address for EVM deposit Hex:  0x94c5087c11bb8dc193cfba7e9e7303e759881702c57e5438b196c7d6f31b32d9
```

## Transfer Funds to EVM Address
For each EVM address we can compute a "Substrate" address to receive funds using the "Balance" module.
  
The utility:  
```bash
evmAddress2SubstrateAddress.js 
```
computes the EVM address required.  
For example a valid EVM Address is: 0x8E8b284E2a582cA3B49eCcfC7B548436cE9bccCa  
you can launch the utility by: 
```bash
node evmAddress2substrateAddress.js 0x8E8b284E2a582cA3B49eCcfC7B548436cE9bccCa
```
and obtain the followin result:  
```bash
Utility to convert an EVM Address to Substrate to receive funds
Converting Evm Address: 0x8E8b284E2a582cA3B49eCcfC7B548436cE9bccCa
You can use this substrate address, to transfer funds to the EVM address above using the Pallet "Balances".
Computed Balance Address for EVM deposit Base58:  5F8K5uYqkMJLjcw1doimf8rmE5fnxhcVPpSKwuJh52sSn1vx
```
## Withdraw funds from EVM Address

You can withdraw funds from the EVM balance to the "matching" Substrate Address using the module "evm","withdraw".  
The transaction must be signed from the Substrate address used to compute the address by substrateAddress2EvmAddress.js. 

## Technical Information:

In order to use Ethereum contracts on a Substrate chain using EVM pallet, there are the following assumptions:  
1) A (32-byte) Substrate address must have a corresponding (20-byte) Ethereum address.  
2) Each (20-byte) Ethereum address must have its balance (deposit) maintained.  
The EVM module satisfies step 1 by simply truncating the source Substrate address into an Ethereum address, taking the first 20 bytes.   
To satisfy step 2, the chain uses the "Balances" module to manage each Ethereum address, by converting Ethereum addresses back into "EVM addresses", which are 32-byte Substrate addresses.  
  
Note that these EVM addresses have no inherent relationship to the original truncated Substrate address.  
  
For Example:  
Consider a 32-byte Substrate address: 0x1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF.  
Truncate this to create a 20-byte Ethereum address: 0x1234567890ABCDEF1234567890ABCDEF12345678  
This Ethereum address's balance comes from the Substrate balances module's state for its corresponding EVM address, produced by hashing the above bytes with an evm: prefix (0x65766D3A).  
So we perform Hash(0x65766D3A1234567890ABCDEF1234567890ABCDEF1234) = 0xAF8536395A1EEC8EDA6FB9CF36739ECF75BECF6FEA04CEEC108BBB6AA15B7CB3, whose balance in the Balances module will be used for EVM-related operations.  
The hashing algorithm is Blake2.  
  
Note that these actions are not reversible: we cannot convert from an EVM address back to its Ethereum address, nor can we convert from an Ethereum address back to its "source" Substrate address.  

