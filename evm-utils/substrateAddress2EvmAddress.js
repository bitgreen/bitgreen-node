// Utility to convert a Substrate Address in EVM Address
// for example a valid Substrate Address is: 5D22LQopHohbANebYDftPRinrm3SVPpZZW6PK6RT2zj4shyx or the hex version 0x2a30b3682d0591387a016735959b7db04dc4ff4bbacf46d6f780bf29a1818c04
// you can launch the utility by: 
// node substrateAddress2EvmAddress.js 5D22LQopHohbANebYDftPRinrm3SVPpZZW6PK6RT2zj4shyx

// Import required modules
const { decodeAddress } = require("@polkadot/keyring");
const { encodeAddress, blake2AsHex } = require ('@polkadot/util-crypto');

const { hexToU8a, isHex } = require("@polkadot/util");

console.log("Utility to convert a Substrate Address to EVM format");
// read the address from command line
const substrateAddress=process.argv[2];
console.log("Converting Substrate Address:",substrateAddress);
const addressBytes = decodeAddress(substrateAddress);
//cut the address to 20 bytes and convert to Hex
let evmAddress='0x' + Buffer.from(addressBytes.subarray(0, 20)).toString('hex');
// show the result
console.log("Computed Evm Address: ",evmAddress);
// compute Substrate Balance Address of the derived EVM address
const addressBytesA = Buffer.from(evmAddress.slice(2), 'hex');
const prefixBytes = Buffer.from('evm:');
const convertBytes = Uint8Array.from(Buffer.concat([ prefixBytes, addressBytesA ]));
const finalAddressHex = blake2AsHex(convertBytes, 256);
const finalAddressb58 = encodeAddress(finalAddressHex);
console.log("You can use this substrate address, to transfer funds to the EVM address above using the Pallet \"Balanaces\".")
console.log("Computed Balance Address for EVM deposit Base58: ",finalAddressb58);

console.log("Computed Balance Address for EVM deposit Hex: ",finalAddressHex);


