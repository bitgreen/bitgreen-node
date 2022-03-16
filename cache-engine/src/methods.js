const db = require("./queries");
const {WsProvider, ApiPromise} = require("@polkadot/api");
const types = require("../assets/types.json");

require('dotenv').config()

async function initApi() {
    // Initialise the provider to connect to the local node
    const provider = new WsProvider(process.env.RPC_PROVIDER);

    // Create the API and wait until ready
    const api = await ApiPromise.create({
        provider: provider,
        types: types
    });
    await api.isReady;

    return api
}

async function processBlock(api, block_number, analyze_only = false) {
    const block_hash = await api.rpc.chain.getBlockHash(block_number);

    console.log(`Chain is at block: #${block_number}`);
    console.log('Block Hash: ' + block_hash.toHex());

    let [signed_block, block_events] = await Promise.all([
        api.rpc.chain.getBlock(block_hash),
        api.query.system.events.at(block_hash)
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

                    if(!analyze_only) {
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
                                block_number: block_number,
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
                    } else {
                        // console.log(`${section}:${method}`);

                        db.storeAnalyzeData({
                            block_number: block_number,
                            block_hash: block_hash.toHex(),
                            tx_hash: txHash,
                            section: section,
                            method: method
                        })
                    }
                }
            })
    });

    // store block in db
    db.storeBlock({
        number: block_number,
        hash: block_hash.toHex(),
        date: current_time,
    })

    console.log('-----------------------------------------------------');
}

const getBlock = async(request, response) => {
    let block_number = request.query.block_number

    const api = await initApi()

    const block_hash = await api.rpc.chain.getBlockHash(block_number)

    let [signed_block, block_events] = await Promise.all([
        api.rpc.chain.getBlock(block_hash),
        api.query.system.events.at(block_hash)
    ]);

    response.json({
        signed_block: signed_block.toHuman(),
        block_events: block_events.toHuman(),
    })
}

module.exports = {
    initApi,
    processBlock,
    getBlock
}