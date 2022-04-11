/// Test to trigger crossing the bridge from evm to pallet

import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { setup_substrate, pallet_bridge_mint } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, privateKey, deposit_method} from './evm_bridge.js';
import Web3 from 'web3';
let api;

export const AMOUNT = process.env.AMOUNT || 10;
export const NONCE = process.env.NONCE || 0;

/// Trigger simulation test by call to deposit_method 
/// on evm based smartcontract, running keepers should
/// listen to this and make it go through the bridge into
/// bridge pallet
const main = async () => {
    // let provider = null;
    try {
        api = await setup_substrate();
        const keyring = new Keyring({ type: 'sr25519' });
        const eve = keyring.addFromUri('//Eve');
        // Wait until we are ready and connected
        await api.isReady;
        const web3 = new Web3(NODE_ADDRESS);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        const recipient = api.createType('AccountId', eve.address);
        const gasPrice = await web3.eth.getGasPrice();

        const amount = AMOUNT;

        // call to deposit_method on evm contract should trigger the bridge
        const receipt = await deposit_method(web3, gasPrice, BitgreenBridge, privateKey, amount, recipient.toHex(), NONCE);
        console.log('transactionHash: \t ', receipt.transactionHash);
    }
    catch (err) {
        console.error('Error', err);
    }
    finally {
        // provider!.engine.stop();
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    process.exit();
});
