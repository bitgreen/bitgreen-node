import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { SECRETSEED, setup_substrate, setup_basic_bridge_test } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, basic_evm_setup_test } from './evm_bridge.js';
import Web3 from 'web3';
let api;
const main = async () => {
    // let provider = null;
    try {
        api = await setup_substrate();
        const keyring = new Keyring({ type: 'sr25519' });
        // create Alice based on the development seed
        const keyspair = keyring.addFromUri(SECRETSEED);
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        // Wait until we are ready and connected
        await api.isReady;
        const web3 = new Web3(NODE_ADDRESS);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        // await setup_basic_bridge_test(api, keyspair);
        await basic_evm_setup_test(web3, BitgreenBridge);
        // await native_transfer(api, alice, 100, charlie.address);
        // await native_transfer(api, alice, 100, dave.address);
        // const eve = keyring.addFromUri('//Eve');
        // const recipient = api.createType('AccountId', eve.address);
        // const bitg_token_bytes = api.createType('Bytes', "BBB");
        // const transaction_id_bytes = api.createType('Bytes', "a123");
        // const balance = api.createType('Balance', "1000");
        // await pallet_bridge_mint(api, alice, bitg_token_bytes, recipient, transaction_id_bytes, balance);
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
