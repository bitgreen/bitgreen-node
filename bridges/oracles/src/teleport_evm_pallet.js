import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { setup_substrate, pallet_bridge_mint } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, privateKey, deposit_method} from './evm_bridge.js';
import Web3 from 'web3';
let api;

export const AMOUNT = process.env.AMOUNT || 10;
export const NONCE = process.env.NONCE || 0;

const main = async () => {
    // let provider = null;
    try {
        api = await setup_substrate();
        const keyring = new Keyring({ type: 'sr25519' });
        // create Alice based on the development seed
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        // const ferdie = keyring.createFromUri('//Ferdie');
        // Wait until we are ready and connected
        await api.isReady;
        const web3 = new Web3(NODE_ADDRESS);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        const recipient = api.createType('AccountId', eve.address);
        const bitg_token_bytes = api.createType('Bytes', 'WETH');
        const gasPrice = await web3.eth.getGasPrice();

        const amount = AMOUNT;

        const receipt = await deposit_method(web3, gasPrice, BitgreenBridge, privateKey, amount, recipient.toHex(), NONCE);
        console.log('transactionHash: \t ', receipt.transactionHash);
        // const transaction_id_bytes = api.createType('Bytes', receipt.transactionHash);
        // const balance = api.createType('Balance', amount);
    
        // let txid = await pallet_bridge_mint(api, charlie, bitg_token_bytes, recipient, transaction_id_bytes, balance);
        // console.log(`txid: \t ${txid}`);
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
