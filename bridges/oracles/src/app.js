import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { setup_substrate, setup_bridge_test } from './pallet_bridge.js';
import { subscription_contract, NODE_ADDRESS, get_bitgreen_bridge_contract, deposit, privateKey } from './evm_bridge.js';
import Web3 from 'web3';
let api;
const main = async () => {
    // let provider = null;
    try {
        api = await setup_substrate();
        const keyring = new Keyring({ type: 'sr25519' });
        // create Alice based on the development seed
        const alice = keyring.addFromUri('//Alice');
        // const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');
        // const dave = keyring.addFromUri('//Dave');
        // const eve = keyring.addFromUri('//Eve');
        // const ferdie = keyring.createFromUri('//Ferdie');
        // Wait until we are ready and connected
        await api.isReady;
        const recipient = api.createType('AccountId', charlie.address);
        // await setup_bridge_test(api, alice);
        // provider = await get_provider();
        const web3 = new Web3(NODE_ADDRESS);
        // const addresses = provider.getAddresses();
        // console.log(`addresses: \t ${addresses}`);
        await subscription_contract(web3);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        // BitgreenBridge.events.allEvents([], function (error, event) {
        //     if (error) {
        //         console.error('error: \t ', error);
        //     }
        //     console.log('event: \t ', event);
        // })
        //     .on('error', console.error);
        // await smoke_test(web3, BitgreenBridge);
        // await smoke_restore_ownership(web3, BitgreenBridge);
        // await smoke_transfer(web3, BitgreenBridge);
        // await bridge_mint_smoke_test(api, alice, recipient);
        // await bridge_burn_smoke_test(api, alice, recipient);

        const amount = 10;
        const gasPrice = await web3.eth.getGasPrice();
        const destination = web3.utils.asciiToHex('0x' + '5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw'); //Eve
        const receipt = await deposit(web3, gasPrice, BitgreenBridge, privateKey, amount, destination);
        console.log(receipt);
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
    // process.exit();
});
