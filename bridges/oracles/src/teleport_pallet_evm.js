import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { setup_substrate, pallet_bridge_burn } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, get_erc20, send_transfer, privateKey } from './evm_bridge.js';
import Web3 from 'web3';
const destination_address = '0xE617985640737D2AB30e9eA0F484195eF106C983';
let api;
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
        const transaction_id_bytes = api.createType('Bytes', "a135");
        const balance = api.createType('Balance', "1");
        const gasPrice = await web3.eth.getGasPrice();
        let txid = await pallet_bridge_burn(api, charlie, bitg_token_bytes, recipient, transaction_id_bytes, balance);
        console.log(`txid: \t ${txid}`);
        const asset_id = '1';
        const erc20 = await get_erc20(asset_id);
        await send_transfer(web3, gasPrice, BitgreenBridge, privateKey, txid.toString(), destination_address, balance.toString(), erc20);
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
