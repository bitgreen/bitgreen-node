import { ApiPromise, Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { Channel } from 'async-channel';
import { KeyringPair } from '@polkadot/keyring/types';
import { setup_substrate, SECRETSEED, TOKEN, pallet_bridge_burn } from './pallet_bridge.js';
import { privateKey, subscription_contract, NODE_ADDRESS, get_bitgreen_bridge_contract, send_transfer, get_erc20 } from './evm_bridge.js';
import Web3 from 'web3';
import { Contract } from 'web3-eth-contract';

let api: ApiPromise;

const handle_events = async (api: ApiPromise, web3: Web3, BitgreenBridge: Contract, keypair: KeyringPair, value: any) => {
    switch (value.method) {
        case 'BurnQueued':
            const transaction_id = value[4];
            const signer = value[0];
            const account = api.createType('AccountId', keypair.address);
            let exist = false;
            if (signer !== account) {
                const __exist = await api.query.bridge.burn_tracker_contains(transaction_id, signer);
                console.log(__exist);
                if (__exist) {
                    exist = true;
                }
            }
            if (!exist) {
                const bitg_token_bytes = api.createType('Bytes', TOKEN);
                const threshold = await api.query.bridge.is_threshold_burn(bitg_token_bytes);
                const balance = value[3];
                const recipient = value[2];
                if (!threshold) {                
                    await pallet_bridge_burn(api, keypair, bitg_token_bytes, recipient, transaction_id, balance);    
                }
                const asset_id = value[1];
                const erc20 = get_erc20(asset_id);
                const gasPrice = await web3.eth.getGasPrice();
                await send_transfer(web3, gasPrice, BitgreenBridge, privateKey, transaction_id.toString(), recipient.toString(), balance.toString(), erc20);
                
            }
            break;
        case 'MintQueued':
            break;
        default:
            console.log(value);
    }
}

const keepers_subscription_pallet_bridge = async (api: ApiPromise, web3: Web3, BitgreenBridge: Contract) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const keypair = keyring.addFromUri(SECRETSEED);

    const chan = new Channel(1 /* default */);
    // Subscribe to system events via storage
    api.query.system.events((events: any[]) => {
        // console.log(`\nReceived ${events.length} events:`);

        // Loop through the Vec<EventRecord>
        events.forEach((record: { event: any; phase: any; }) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            if (event.section === 'bridge') {
                let value: { [key: string | number]: any[] } = {};
                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data: any, index: string | number) => {
                    console.log(`\tindex[${index}]:\t\t${types[index].type}: ${data.toString()}`);
                    if (types[index].type === 'DispatchResult') {
                        const result = api.createType('DispatchResult', data)
                        if (result.isErr) {
                            const dispatchError = result.asErr;
                            if (dispatchError.isModule) {
                                // for module errors, we have the section indexed, lookup
                                const decoded = api.registry.findMetaError(dispatchError.asModule);
                                const { name, section } = decoded;
                                console.log(`error: \t${section}.${name}`);
                            } else {
                                // Other, CannotLookup, BadOrigin, no extra info
                                console.log(`other: ${dispatchError.toString()}`);
                            }
                            return;                            
                        }
                    }
                    value[index] = api.createType(types[index].type, data);
                    value['method'] = event.method;
                    chan.push(value);
                });
            }
        });
    });
    for await (const value of chan) {
        await handle_events(api, web3, BitgreenBridge, keypair, value);
    }
    console.log('after events');
}


const main = async () => {
    try {
        api = await setup_substrate();
        const web3 = new Web3(NODE_ADDRESS);
        // Wait until we are ready and connected
        await api.isReady;
        await subscription_contract(web3);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        BitgreenBridge.events.allEvents([], function (error: any, event: any) {
            if (error) {
                console.error('error: \t ', error);
            }
            console.log('event: \t ', event);
        })
            .on('error', console.error);
        await keepers_subscription_pallet_bridge(api, web3, BitgreenBridge);

    } catch (err) {
        console.error('Error', err);
    }
}

main().catch(console.error).finally(() => {
    console.log('end');
    // process.exit();
});

