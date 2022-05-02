import { Channel } from 'async-channel';
import { NODE_ADDRESS, ROUTER_ADDRESS, get_bitgreen_bridge_contract, privateKey, get_bitgreen_bridge_abi, send_transfer, get_erc20, send_setLockdown, build_setLockdown } from './evm_bridge.js';
import Web3 from 'web3';
import { addABI, decodeMethod } from 'abi-decoder';
import { MistxSocket } from '@alchemist-coin/mistx-connect';

let surveillance = {};
const threshold = process.env.BLOCK_THRESHOLD || 1;
let maxPriorityFeePerGas_tracked = 0;

const main = async () => {
    const chan = new Channel(0 /* default */);
    try {

        if (!privateKey) {
            console.error("PRIVATE_KEY should be defined");
            process.exit(2);
        }

        if (!ROUTER_ADDRESS) {
            console.error("ROUTER_ADDRESS should be defined");
            process.exit(3);
        }

        const web3 = new Web3(NODE_ADDRESS);

        // Create a socket connection to goerli or mainnet
        const socket = new MistxSocket('api-goerli.mistx.io'); //> goerli
        // const socket = new MistxSocket('api.mistx.io') > mainnet
        // const socket = new MistxSocket(); // defaults to mainnet

        // init the socket connection with callbacks
        const disconnect = socket.init({
            onConnect: () => { },
            onConnectError: err => { },
            onDisconnect: err => { },
            onError: err => { },
            onFeesChange: (fees) => { },
            onTransactionResponse: (response) => { }
        })



        const abi = get_bitgreen_bridge_abi();
        addABI(abi);
        const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
        const jsonInterface = BitgreenBridge.options.jsonInterface;
        // console.log(jsonInterface);
        const watchdogs = await BitgreenBridge.methods.getWatchdogs().call();
        console.log(watchdogs);

        var subscription_pendings = web3.eth.subscribe('pendingTransactions')
            .on("data", async function (transactionHash) {
                // console.log(transactionHash);
                const lockdown = await BitgreenBridge.methods.getLockdown().call();
                if (!lockdown) {
                    const t_data = await web3.eth.getTransaction(transactionHash);
                    if (t_data) {
                        const gasPrice = t_data.gasPrice;
                        // console.log(gasPrice);
                        if (t_data.to == ROUTER_ADDRESS) {
                            console.log(t_data);
                            const watchdogs = await BitgreenBridge.methods.getWatchdogs().call();
                            console.log(watchdogs);
                            const from = t_data.from;
                            if (watchdogs.includes(from)) {
                                const gas = t_data.gas;
                                const maxFeePerGas = t_data.maxFeePerGas;
                                const maxPriorityFeePerGas = t_data.maxPriorityFeePerGas;
                                const input = t_data.input;

                                const decodedData = decodeMethod(input);
                                console.log(decodedData);
                                if (decodedData.name === 'setLockdown') {
                                    const keys = Object.keys(surveillance);
                                    if (keys.length == 0 && !surveillance[transactionHash]) {
                                        const number = await web3.eth.getBlockNumber();
                                        surveillance[transactionHash] =
                                        {
                                            'number': number,
                                            'nonce': 1,
                                            'gas': gas,
                                            'maxFeePerGas': maxFeePerGas,
                                            'maxPriorityFeePerGas': maxPriorityFeePerGas
                                        };
                                    } else {
                                        let hash = keys[0];
                                        const stored_maxFeePerGas = surveillance[hash].maxFeePerGas;
                                        if (stored_maxFeePerGas < maxFeePerGas) {
                                            delete surveillance[hash];

                                            const number = await web3.eth.getBlockNumber();
                                            surveillance[transactionHash] =
                                            {
                                                'number': number,
                                                'nonce': 1,
                                                'gas': gas,
                                                'maxFeePerGas': maxFeePerGas,
                                                'maxPriorityFeePerGas': maxPriorityFeePerGas
                                            };
                                        }

                                    }
                                }

                            }
                        }
                    } else {
                        console.log(transactionHash);
                    }
                } else {
                    if (Object.keys(surveillance).length > 0) {
                        surveillance = {};
                    }
                }
            });

        var subscription_blocks = web3.eth.subscribe('newBlockHeaders')
            .on("data", async function (blockHeader) {
                // console.log(blockHeader);
                // const hash = blockHeader.hash;
                const number = await web3.eth.getBlockNumber();
                // console.log(number);
                const lockdown = await BitgreenBridge.methods.getLockdown().call();
                const keys = Object.keys(surveillance);
                if (!lockdown && keys.length > 0) {
                    let hash = keys[0];
                    if (number - surveillance[hash].number >= threshold) {
                        let maxPriorityFeePerGas = surveillance[hash].maxPriorityFeePerGas;
                        maxPriorityFeePerGas = maxPriorityFeePerGas * 2;
                        const maxFeePerGas = 1; // TODO: get that
                        const serializedRawTx = await build_setLockdown(web3, maxFeePerGas, maxPriorityFeePerGas, BitgreenBridge);

                        // emit a transaction
                        const bundle = {
                            transactions: [serializedRawTx]
                        }
                        let receipt = await socket.emitBundleRequest(bundle);
                        console.log(receipt);


                    }
                }
                maxPriorityFeePerGas_tracked = 1;
            })
            .on("error", console.error);

        var subscription_syncing = web3.eth.subscribe('syncing', function (error, sync) {
            if (!error)
                console.log(sync);
        })
            .on("data", function (sync) {
                // show some syncing stats
            })
            .on("changed", function (isSyncing) {
                if (isSyncing) {
                    console.log("Syncing");
                    // stop app operation
                } else {
                    // regain app operation
                }
            });

        await chan.get().then(value => console.log(value), error => console.error(error));
    }
    catch (err) {
        console.error('Error', err);
    }
    finally {
        chan.close();
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    process.exit(0);
});