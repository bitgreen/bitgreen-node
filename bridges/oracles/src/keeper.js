import { Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { Channel } from 'async-channel';
import { setup_substrate, SECRETSEED, pallet_bridge_burn, pallet_bridge_mint } from './pallet_bridge.js';
import { subscription_contract, NODE_ADDRESS, get_bitgreen_bridge_contract } from './evm_bridge.js';
import Web3 from 'web3';
let api;
const handle_events = async (api, web3, BitgreenBridge, keypair, value) => {
    console.log(value.method);
    switch (value.method) {
        case 'BurnQueued': {
            const transaction_id = value[4];
            const signer = value[0];
            console.log(`signer: \t ${signer}`);
            const token = value[5];
            const recipient = value[2];
            // TODO: make it search for encoded key
            const threshold = await api.query.bridge.burnConfirmation(token);
            const balance = value[3];
            console.log(`burn threshold: \t ${threshold}`);
            if (threshold.eq(false)) {
                const account = api.createType('AccountId', keypair.address);
                console.log(`account: \t ${account}`);
                if (!signer.eq(account)) {
                    const asset_id = await api.query.bridge.transactionBurnTracker(transaction_id, signer);
                    console.log(`asset_id: \t ${asset_id}`);
                    if (asset_id.toNumber() > 0) {
                        const txid = await pallet_bridge_burn(api, keypair, token, recipient, transaction_id, balance);
                        console.log(`txid: \t ${txid}`);
                    }
                }
            }
            // const asset_id = value[1];
            // const erc20 = get_erc20(asset_id);
            // const gasPrice = await web3.eth.getGasPrice();
            // await send_transfer(web3, gasPrice, BitgreenBridge, privateKey, transaction_id.toString(), recipient.toString(), balance.toString(), erc20);
        }
            break;
        case 'MintQueued': {
            const transaction_id = value[4];
            const signer = value[0];
            console.log(`signer: \t ${signer}`);
            const token = value[5];
            console.log(`token: \t ${token}`);
            const recipient = value[2];
            // TODO: make it search for encoded key
            const threshold = await api.query.bridge.mintConfirmation(token);
            const balance = value[3];
            console.log(`mint threshold: \t ${threshold}`);
            if (threshold.eq(false)) {
                const account = api.createType('AccountId', keypair.address);
                console.log(`account: \t ${account}`);
                if (!signer.eq(account)) {
                    const asset_id = await api.query.bridge.transactionMintTracker(transaction_id, signer);
                    console.log(asset_id);
                    console.log(`asset_id: \t ${asset_id}`);
                    if (asset_id.toNumber() > 0) {
                        const txid = await pallet_bridge_mint(api, keypair, token, recipient, transaction_id, balance);
                        console.log(`txid: \t ${txid}`);
                    }
                }
            }
        }
            break;
        default:
            console.log(value);
    }
};
const keepers_subscription_pallet_bridge = async (api, web3, BitgreenBridge) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const keypair = keyring.addFromUri(SECRETSEED);
    const chan = new Channel(1 /* default */);
    // Subscribe to system events via storage
    api.query.system.events((events) => {
        // console.log(`\nReceived ${events.length} events:`);
        // Loop through the Vec<EventRecord>
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;
            let value = {};
            if (event.section === 'bridge') {
                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data, index) => {
                    console.log(`\tindex[${index}]:\t\t${types[index].type}: ${data.toString()}`);
                    if (types[index].type === 'DispatchResult') {
                        const result = api.createType('DispatchResult', data);
                        if (result.isErr) {
                            const dispatchError = result.asErr;
                            if (dispatchError.isModule) {
                                // for module errors, we have the section indexed, lookup
                                const decoded = api.registry.findMetaError(dispatchError.asModule);
                                const { name, section } = decoded;
                                console.log(`error: \t${section}.${name}`);
                            }
                            else {
                                // Other, CannotLookup, BadOrigin, no extra info
                                console.log(`other: ${dispatchError.toString()}`);
                            }
                            return;
                        }
                    }
                    value[index] = api.createType(types[index].type, data);
                    value['method'] = event.method;
                });
                chan.push(value);
            }
        });
    });
    for await (const value of chan) {
        await handle_events(api, web3, BitgreenBridge, keypair, value);
    }
    console.log('after events');
};
const main = async () => {
    try {
        api = await setup_substrate();
        const web3 = new Web3(NODE_ADDRESS);
        // Wait until we are ready and connected
        await api.isReady;
        await subscription_contract(web3);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        BitgreenBridge.events.allEvents([], function (error, event) {
            if (error) {
                console.error('error: \t ', error);
            }
            console.log('event: \t ', event);
        })
            .on('error', console.error);
        await keepers_subscription_pallet_bridge(api, web3, BitgreenBridge);
    }
    catch (err) {
        console.error('Error', err);
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    // process.exit();
});
