import { Keyring } from '@polkadot/api';
import '@polkadot/api-augment';
import { Channel } from 'async-channel';
import { setup_substrate, SECRETSEED, pallet_bridge_burn, pallet_bridge_mint, destination, pallet_bridge_set_lockdown } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, privateKey, send_transfer, get_erc20, send_setLockdown } from './evm_bridge.js';
import Web3 from 'web3';

const total_lockdown = async (api, keypair, web3, BitgreenBridge, token) => {
    const gasPrice = await web3.eth.getGasPrice();
    const txid = await pallet_bridge_set_lockdown(api, keypair, token);
    console.log(txid);
    const receipt = await send_setLockdown(web3, gasPrice, BitgreenBridge);
    console.log(receipt);
}

const handle_events = async (api, keypair, web3, BitgreenBridge, value) => {
    switch (value['method']) {
        case 'Minted': {
            // console.log(value);
            const signer = value[0];
            console.log(`signer: \t ${signer}`);
            const asset_id = value[1];
            console.log(`asset_id: \t ${asset_id}`);
            const recipient = value[2];
            console.log(`recipient: \t ${recipient}`);
            const amount = value[3];
            console.log(`amount: \t ${amount}`);
            const txid = value[4].toHex();
            console.log(`txid: \t ${txid}`);
            const token = value[5];
            console.log(`token: \t ${token}`);


            if (asset_id.toNumber() == 1) { // ETH TO WETH
                const lockdown = await BitgreenBridge.methods.getLockdown().call();
                if (lockdown == false) {
                    const tx = await web3.eth.getTransaction(txid);
                    if (tx) {
                        console.log(tx);
                        const value = tx['value'];
                        console.log(value);
                        const input = tx['input'];
                        const param_ABI = [{
                            type: "bytes32",
                            name: "destination"
                        }];
                        const destination = await web3.eth.abi.decodeParameters(
                            param_ABI,
                            input.slice(10));
                        console.log(destination);

                        if (recipient.eq(destination[0])) {
                            if (!amount.eq(value)) {
                                console.log(`FATAL : \t amount ${amount} <> value ${value}`);
                                await total_lockdown(api, keypair, web3, BitgreenBridge, token);
                                console.log('end');
                                process.exit();
                            }
                            // else {
                            // console.log('all right');
                            // await total_lockdown(api, keypair, web3, BitgreenBridge, token);                            
                            // }
                        } else {
                            console.log('WARNING: recipient not equal to destination');
                        }

                    }

                    // const receipt = await web3.eth.getTransactionReceipt(txid);
                    // if (receipt) {
                    //     console.log(receipt);
                    //     receipt.logs[0].topics.shift();
                    //     const bridge_deposit_request_event = web3.eth.abi.decodeLog([{
                    //         type: 'bytes32',
                    //         name: 'destination',
                    //         indexed: true
                    //     }, {
                    //         type: 'uint',
                    //         name: 'amount',
                    //         indexed: true
                    //     }, {
                    //         type: 'address',
                    //         name: 'sender',
                    //         indexed: true
                    //     }],
                    //         receipt.logs[0],
                    //         receipt.logs[0].topics);

                    //     console.log(bridge_deposit_request_event);
                    // }

                }
            }
        }
        default:
            break;
        // console.log(value);
    }
};


const watchdog_subscription_pallet_bridge = async (api, keypair, web3, BitgreenBridge) => {
    const chan = new Channel(1 /* default */);

    // subscribe to all new headers (with extended info)
    api.derive.chain.subscribeNewHeads(async (header) => {
        const blockHash = await api.rpc.chain.getBlockHash(header.number);
        const signedBlock = await api.rpc.chain.getBlock(blockHash);
        const allRecords = await api.query.system.events.at(blockHash);

        signedBlock.block.extrinsics.forEach((ex, index) => {
            try {
                const { method: { args, method, section } } = ex;
                if (section == 'bridge') {
                    // const extrinsicsHash = ex.hash;
                    // console.log(`#${header.number}: ${header.author}`);
                    // console.log(`#${index}: ${extrinsicsHash}`);

                    let value = {};

                    allRecords
                        // filter the specific events based on the phase and then the
                        // index of our extrinsic in the block
                        .filter(({ phase }) =>
                            phase.isApplyExtrinsic &&
                            phase.asApplyExtrinsic.eq(index))
                        // test the events against the specific types we are looking for
                        .forEach((record) => {
                            let { event } = record;
                            const types = event.typeDef;
                            if (api.events.system.ExtrinsicSuccess.is(event)) {
                                value['success'] = true;
                            } else if (api.events.system.ExtrinsicFailed.is(event)) {
                                // extract the data for this event
                                const [dispatchError, dispatchInfo] = event.data;
                                let errorInfo;

                                // decode the error
                                if (dispatchError.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    // (For specific known errors, we can also do a check against the
                                    // api.errors.<module>.<ErrorName>.is(dispatchError.asModule) guard)
                                    const decoded = api.registry.findMetaError(dispatchError.asModule);

                                    errorInfo = `${decoded.section}.${decoded.name}`;
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    errorInfo = dispatchError.toString();
                                }

                                console.log(`${section}.${method}:: ExtrinsicFailed:: ${errorInfo}`);
                                value['success'] = false;
                            } else {
                                // TODO finish event handling refactoring to deal with substrate block, index, tx hash
                                // console.log(`${section}.${method}:: `);
                                // console.log(event);

                                event.data.forEach((data, index) => {
                                    // console.log(`\tindex[${index}]:\t\t${types[index].type}: ${data.toString()}`);
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
                                            value['success'] = false;
                                            return;
                                        }
                                    }
                                    value[index] = api.createType(types[index].type, data);
                                });
                                // console.log(event.method);
                                value['method'] = event.method;
                            }
                        });

                    if (value['success'] && value['method']) {
                        if (value['method'] === 'Request') {
                            const bi_number = header.number.toBigInt();
                            const txid = api.createType('(u64,u16)', [bi_number, index]);
                            value['txid'] = txid.toHex();
                            // console.log(txid.toHex());
                        }

                        chan.push(value);
                    }

                }
            }
            catch (err) {
                console.error('Error', err);
            }
        });
    });

    for await (const value of chan) {
        try {
            await handle_events(api, keypair, web3, BitgreenBridge, value);
        }
        catch (err) {
            console.error('Error', err);
        }
    }
};

const handle_evm_transfer_events = async (api, keypair, web3, BitgreenBridge, returnValues) => {
    try {
        // const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
        const txid = returnValues['0'];
        console.log(txid);
        const recipient = returnValues['1'];
        console.log(recipient);
        const amount = returnValues['2'];
        console.log(amount);
        const erc20 = returnValues['3'];
        console.log(erc20);
        const wdf = returnValues['4'];
        console.log(wdf);
        const total = wdf + amount;

        const transaction_id = api.createType('(u64,u16)', txid);
        const [block_number, index] = transaction_id;
        console.log(block_number);
        console.log(index);

        const blockHash = await api.rpc.chain.getBlockHash(block_number);
        // const signedBlock = await api.rpc.chain.getBlock(blockHash);
        const allRecords = await api.query.system.events.at(blockHash);

        let value = {};
        allRecords
            // filter the specific events based on the phase and then the
            // index of our extrinsic in the block
            .filter(({ phase }) =>
                phase.isApplyExtrinsic &&
                phase.asApplyExtrinsic.eq(index))
            // test the events against the specific types we are looking for
            .forEach((record) => {
                let { event } = record;
                const types = event.typeDef;
                if (api.events.system.ExtrinsicSuccess.is(event)) {
                    value['success'] = true;
                } else if (api.events.system.ExtrinsicFailed.is(event)) {
                    // extract the data for this event
                    const [dispatchError, dispatchInfo] = event.data;
                    let errorInfo;

                    // decode the error
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        // (For specific known errors, we can also do a check against the
                        // api.errors.<module>.<ErrorName>.is(dispatchError.asModule) guard)
                        const decoded = api.registry.findMetaError(dispatchError.asModule);

                        errorInfo = `${decoded.section}.${decoded.name}`;
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        errorInfo = dispatchError.toString();
                    }

                    console.log(`${section}.${method}:: ExtrinsicFailed:: ${errorInfo}`);
                    value['success'] = false;
                } else {
                    // console.log(`${section}.${method}:: `);
                    // console.log(event);

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
                                value['success'] = false;
                                return;
                            }
                        }
                        value[index] = api.createType(types[index].type, data);
                    });
                    // console.log(event.method);
                    value['method'] = event.method;
                }
            });

        const token = value[1];
        const destination = value[2];
        const balance = value[3];

        if (destination.eq(recipient)) {
            if (!balance.eq(total)) {
                console.log(`FATAL : \t total ${total} <> balance ${balance}`);
                await total_lockdown(api, keypair, web3, BitgreenBridge, token);
                console.log('end');
                process.exit();
            }
            else {
                console.log('all right');
                // await total_lockdown(api, keypair, web3, BitgreenBridge, token);                            
            }
        } else {
            console.log('WARNING: recipient not equal to destination');
        }
    }
    catch (err) {
        console.error('Error', err);
    }

}

const main = async () => {
    try {
        const api = await setup_substrate();
        const web3 = new Web3(NODE_ADDRESS);
        // Wait until we are ready and connected
        await api.isReady;

        const keyring = new Keyring({ type: 'sr25519' });
        const keypair = keyring.addFromUri(SECRETSEED);

        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        BitgreenBridge.events.BridgeTransfer()
            .on('data', function (event) {
                console.log('event BridgeTransfer: \t ', event);
                handle_evm_transfer_events(api, keypair, web3, BitgreenBridge, event.returnValues, event.transactionHash);
            }).on('changed', function (event) {
                console.log('event changed BridgeTransfer: \t ', event);
            }).on('error', console.error);

        await watchdog_subscription_pallet_bridge(api, keypair, web3, BitgreenBridge);
    }
    catch (err) {
        console.error('Error', err);
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    process.exit();
});