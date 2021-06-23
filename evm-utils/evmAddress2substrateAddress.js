// Utility to convert an EVM Address to its Substrate Address for funds deposits
// for example a valid EVM Address is: 0x8E8b284E2a582cA3B49eCcfC7B548436cE9bccCa
// you can launch the utility by: 
// node evmAddress2substrateAddress.js 0x8E8b284E2a582cA3B49eCcfC7B548436cE9bccCa

// Import required modules
const { decodeAddress } = require("@polkadot/keyring");
const { encodeAddress, blake2AsHex } = require ('@polkadot/util-crypto');

const { hexToU8a, isHex } = require("@polkadot/util");

console.log("Utility to convert an EVM Address to Substrate to receive funds");
// read the address from command line
const evmAddress=process.argv[2];
console.log("Converting Evm Address:",evmAddress);
const addressBytesA = Buffer.from(evmAddress.slice(2), 'hex');
const prefixBytes = Buffer.from('evm:');
const convertBytes = Uint8Array.from(Buffer.concat([ prefixBytes, addressBytesA ]));
const finalAddressHex = blake2AsHex(convertBytes, 256);
const finalAddressb58 = encodeAddress(finalAddressHex);
console.log("You can use this substrate address, to transfer funds to the EVM address above using the Pallet \"Balances\".")
console.log("Computed Balance Address for EVM deposit Base58: ",finalAddressb58);

