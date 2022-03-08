// Bitgreen block crawler
// This program will listen for new blocks and store 
// them in a local Postgresql database. 

// import required dependancies
const { ApiPromise, WsProvider } = require('@polkadot/api')
const db = require('./queries')
const types = require("./assets/types.json");

require('dotenv').config()

// main function (must be async)
async function main () {

    // Initialise the provider to connect to the local node
    const provider = new WsProvider(process.env.RPC_PROVIDER);

    // Create the API and wait until ready
    const api = await ApiPromise.create({
        provider: provider,
        types: types
    });
    await api.isReady;

    // Retrieve the chain & node information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    // log message to console 
    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);

    // We only display a couple, then unsubscribe
    let count = 0;

    // Subscribe to the new headers on-chain. The callback is fired when new headers
    // are found, the call itself returns a promise with a subscription that can be
    // used to unsubscribe from the newHead subscription
    const unsubscribe = await api.rpc.chain.subscribeNewHeads(async(header) => {
        console.log(`Chain is at block: #${header.number}`);
        // console.log(header);
        console.log('Block Hash: ' + header.hash.toHex());

        // let signed_block = await api.rpc.chain.getBlock(header.hash);
        let [signed_block, block_events] = await Promise.all([
            api.rpc.chain.getBlock(header.hash),
            api.query.system.events.at(header.hash)
        ]);

        let current_time

        // parse block
        signed_block.block.extrinsics.map((ex, index) => {
            const method = ex.method.method.toString()
            const section = ex.method.section.toString()
            const isSigned = ex.isSigned
            const txHash = ex.hash.toHex()
            let recipient, amount

            let signedByAddress = null
            if(isSigned) {
                signedByAddress = ex.signer.toString()
            }

            if(section === 'timestamp' && method === 'set') {
                ex.args.map(( arg, d ) => {
                    current_time = arg.toString();
                });
            }

            // (await api.rpc.eth.getTransactionReceipt(txHash)).toJSON()

            block_events
                .filter(({ phase }) =>
                    phase.isApplyExtrinsic &&
                    phase.asApplyExtrinsic.eq(index)
                )
                .map(({ event }) => {
                    if(api.events.system.ExtrinsicSuccess.is(event)) {
                        console.log('Transaction Hash: ' + txHash);

                        if(section === 'balances' && (method === 'transferKeepAlive' || method === 'transfer')) {
                            // console.log(api.eth.rpc.inspect())
                            // console.log(await api.rpc.eth.getTransactionReceipt('0x3b3c01dad5e131c63b549944bbb0fe93f6e3b2a783d01ee18f2ffbe2a5075ed2'));

                            ex.args.map(( arg, d ) => {
                                if(d === 0) {
                                    recipient = arg.toHuman()['Id'];
                                } else if(d === 1) {
                                    amount = arg.toString();
                                }
                            });

                            db.storeTransaction({
                                block_number: header.number,
                                hash: txHash,
                                sender: signedByAddress,
                                recipient: recipient,
                                amount: amount,
                                gas_fees: 0,
                                date: current_time,
                            })
                        }

                        console.log(section);

                        if(section === 'sudo' && method === 'sudo') {
                            console.log('sudo');
                        }
                    }
                })

        });

        db.storeBlock({
            number: header.number,
            hash: header.hash.toHex(),
            date: current_time,
        })

        console.log('-----------------------------------------------------');

        if (++count === 20) {
            unsubscribe();
            process.exit(0);
        }
    });
}

main().catch(console.error);