import { Keyring } from '@polkadot/api';
import '@polkadot/api-augment';
import { Channel } from 'async-channel';
import { setup_substrate, SECRETSEED, pallet_bridge_burn, pallet_bridge_mint, destination } from './pallet_bridge.js';
import { NODE_ADDRESS, get_bitgreen_bridge_contract, privateKey, send_transfer, get_erc20 } from './evm_bridge.js';
import Web3 from 'web3';

export const QUEUE_MODE = process.env.QUEUE || false;

let api;
const handle_events = async (api, keypair, web3, BitgreenBridge, value) => {
    // console.log(value);
    switch (value['method']) {
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
                    const asset_id = await api.query.bridge.transactionBurnTracker(transaction_id, account);
                    console.log(`asset_id: \t ${asset_id}`);
                    if (asset_id.toNumber() == 0) {
                        const txid = await pallet_bridge_burn(api, keypair, token, recipient, transaction_id, balance);
                        console.log(`burn txid: \t ${txid}`);
                    }
                }
            }
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
                    const asset_id = await api.query.bridge.transactionMintTracker(transaction_id, account);
                    console.log(asset_id);
                    console.log(`asset_id: \t ${asset_id}`);
                    if (asset_id.toNumber() == 0) {
                        const txid = await pallet_bridge_mint(api, keypair, token, recipient, transaction_id, balance);
                        console.log(`mint txid: \t ${txid}`);
                    }
                }
            }
        }
            break;
        case 'Request': {
            const transaction_id = api.createType('Bytes',value['txid']);
            console.log(`transaction_id: \t ${transaction_id.toString()}`);
            const signer = value[0];
            console.log(`signer: \t ${signer}`);
            const token = value[1];
            const destination = value[2].toHex();
            console.log(`destination: \t ${destination}`);
            if (web3.utils.isAddress(destination)) {
                // TODO: make it search for encoded key
                const threshold = await api.query.bridge.burnConfirmation(token);
                const balance = value[3];
                console.log(`burn threshold: \t ${threshold}`);
                if (threshold.eq(false)) {
                    const account = api.createType('AccountId', keypair.address);
                    console.log(`account: \t ${account}`);
                    const asset_id = await api.query.bridge.transactionBurnTracker(transaction_id, account);
                    console.log(`asset_id: \t ${asset_id}`);
                    if (asset_id.toNumber() == 0) {
                        const txid = await pallet_bridge_burn(api, keypair, token, signer, transaction_id, balance);
                        console.log(`request burn txid: \t ${txid}`);
                    }
                }
            }
        }
            break;
        case 'Burned': {
            const signer = value[0];
            console.log(`signer: \t ${signer}`);
            const asset_id = value[1];
            console.log(`asset_id: \t ${asset_id}`);
            const amount = value[3];
            console.log(`amount: \t ${amount}`);
            const txid = value[4].toHex();
            console.log(`txid: \t ${txid}`);


            if (asset_id.toNumber() == 1) { // WETH TO ETH
                const gasPrice = await web3.eth.getGasPrice();
                const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
                const voted = await BitgreenBridge.methods.txvotes(txid, account).call();
                if (!voted) {
                    const threshold = await BitgreenBridge.methods.getThreshold().call();            
                    const queue = await BitgreenBridge.methods.txqueue(txid).call();
                    const cnt = queue.cnt;
            
                    if (cnt < threshold) {
                        const transaction_id = api.createType('(u64,u16)',txid);
                        const [block_number, index] = transaction_id;
                        const recipient = await destination(api, block_number, index);
                        console.log('recipient: \t ', recipient);
                        if (recipient) {
                            const balance = amount.toBigInt();
                            // const balance = amount;
                            console.log('evm transfer balance: \t ', balance);
                            const erc20 = await get_erc20(asset_id);
                            console.log('erc20: \t ', erc20);
                            const receipt = await send_transfer(web3, gasPrice, BitgreenBridge, privateKey, txid, recipient, balance, erc20);
                            console.log(receipt);
                        }
                    }
                }    
            }
        }
        default:
            // break;
            console.log(value);
    }
};
const keepers_subscription_pallet_bridge = async (api, keypair, web3, BitgreenBridge) => {
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
                    const extrinsicsHash = ex.hash;
                    // console.log(header.number.toRawType());
                    console.log(`#${header.number}: ${header.author}`);
                    console.log(`#${index}: ${extrinsicsHash}`);
                    // console.log(`index: ${index}`);
                    // console.log(ex.toHuman());
    
                    let value = {};
    
                    allRecords
                      // filter the specific events based on the phase and then the
                      // index of our extrinsic in the block
                      .filter(({ phase }) =>
                        phase.isApplyExtrinsic &&
                        phase.asApplyExtrinsic.eq(index)              )
                      // test the events against the specific types we are looking for
                      .forEach((record) => {
                        let { event } = record;
                        const types = event.typeDef;
                        if (api.events.system.ExtrinsicSuccess.is(event)) {
                          // extract the data for this event
                          // (In TS, because of the guard above, these will be typed)
                          if (section !== 'timestamp') {
                            const [dispatchInfo] = event.data;            
                            console.log(`${section}.${method}:: ExtrinsicSuccess:: ${JSON.stringify(dispatchInfo.toHuman())} \t args:: ${args}`);
                          }
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
                            console.log(`${section}.${method}:: `);
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
        
                    if (value['success'] && value['method']) {
                        if (value['method'] === 'Request') {
                            const bi_number = header.number.toBigInt();
                            // console.log(bi_number);
                            const txid = api.createType('(u64,u16)',[bi_number,index]);
                            // console.log(txid);
                            value['txid'] = txid.toHex();
                            console.log(txid.toHex());
                        }
    
                        chan.push(value);                
                    }
    
                }
            }
            catch (err) {
                console.error('Error', err);
            }  
          });        
        //   console.log(blockHash.toHex());          
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

const handle_evm_transfer_events = async (web3, BitgreenBridge, returnValues) => {
    const gasPrice = await web3.eth.getGasPrice();
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const txid = returnValues['0'];
    const voted = await BitgreenBridge.methods.txvotes(txid, account).call();
    if (!voted) {
        const threshold = await BitgreenBridge.methods.getThreshold().call();

        const queue = await BitgreenBridge.methods.txqueue(txid).call();
        const cnt = queue.cnt;

        if (cnt < threshold) {
            const recipient = returnValues['1'];
            console.log('recipient: \t ', recipient);
            const balance = returnValues['2'];
            console.log('balance: \t ', balance);
            const erc20 = await get_erc20(asset_id);            
            console.log('erc20: \t ', erc20);
            await send_transfer(web3, gasPrice, BitgreenBridge, privateKey, txid, recipient, balance, erc20);
        }
    }
}

const handle_evm_deposit_events = async (api, keypair, returnValues, transaction_id) => {
    const recipient = returnValues['0'];
    // const transaction_id = '0x0000000000000000000000000000000000000000';
    const signer = returnValues['2'];
    console.log(`signer: \t ${signer}`);
    const token = api.createType('Bytes', 'WETH');
    console.log(`token: \t ${token}`);
    // TODO: make it search for encoded key
    const threshold = await api.query.bridge.mintConfirmation(token);
    const balance = api.createType('Balance', returnValues['1']);
    console.log(`mint threshold: \t ${threshold}`);
    if (threshold.eq(false)) {
        const account = api.createType('AccountId', keypair.address);
        console.log(`account: \t ${account}`);
        const asset_id = await api.query.bridge.transactionMintTracker(transaction_id, account);
        console.log(asset_id);
        console.log(`asset_id: \t ${asset_id}`);
        if (asset_id.toNumber() == 0) {
            const txid = await pallet_bridge_mint(api, keypair, token, recipient, transaction_id, balance);
            console.log(`mint txid: \t ${txid}`);
        }
    }

}

const main = async () => {
    try {
        api = await setup_substrate();
        const web3 = new Web3(NODE_ADDRESS);
        // Wait until we are ready and connected
        await api.isReady;

        const keyring = new Keyring({ type: 'sr25519' });
        const keypair = keyring.addFromUri(SECRETSEED);

        // await subscription_contract(web3);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        if (QUEUE_MODE) {
            BitgreenBridge.events.BridgeTransferQueued()
                .on('data', function (event) {
                    handle_evm_transfer_events(web3, BitgreenBridge, event.returnValues);
                }).on('changed', function (event) {
                    console.log('event changed BridgeTransferQueued: \t ', event);
                }).on('error', console.error);
        }
        BitgreenBridge.events.BridgeDepositRequest()
            .on('data', function (event) {
                console.log('event BridgeDepositRequest: \t ', event);
                handle_evm_deposit_events(api, keypair, event.returnValues, event.transactionHash);
            }).on('changed', function (event) {
                console.log('event changed BridgeDepositRequest: \t ', event);
            }).on('error', console.error);

        await keepers_subscription_pallet_bridge(api, keypair, web3, BitgreenBridge);
    }
    catch (err) {
        console.error('Error', err);
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    process.exit();
});
