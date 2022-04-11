/// Test to trigger crossing the bridge from pallet to evm

import { Keyring } from '@polkadot/api';
import '@polkadot/api-augment';
import { setup_substrate, pallet_bridge_request } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, get_erc20, send_transfer, privateKey } from './evm_bridge.js';

const destination_address = '0xB1f5e50686d94f21b0D4488e629C46E8Ccb84160';
let api;

/// Trigger simulation test by call to request 
/// on pallet bridge, running keepers should
/// listen for this and make it go through the bridge into
/// bridge evm contract and finally to recipient
const main = async () => {
    try {
        api = await setup_substrate();
        const keyring = new Keyring({ type: 'sr25519' });
        const eve = keyring.addFromUri('//Eve');
        // Wait until we are ready and connected
        await api.isReady;
        const bitg_token_bytes = api.createType('Bytes', 'WETH');
        const balance = api.createType('Balance', "2");
        const destination_address_bytes = api.createType('Bytes', destination_address);

        /// call on pallet bridge that trigger the running keepers
        let txid = await pallet_bridge_request(api, eve, bitg_token_bytes, destination_address_bytes, balance);
        console.log(`txid: \t ${txid}`);
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
