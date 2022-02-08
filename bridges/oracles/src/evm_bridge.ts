import Web3 from 'web3';
// import HDWalletProvider from '@truffle/hdwallet-provider';

import { readFileSync } from 'fs';
import { dirname, join, normalize, format } from 'path';
import { fileURLToPath } from 'url';
import { AbiItem } from 'web3-utils';
import { Contract } from 'web3-eth-contract';

export const NODE_ADDRESS = process.env.NODE_ADDRESS || Web3.givenProvider || 'ws://127.0.0.1:8545';
const ROUTER_ADDRESS = process.env.ROUTER_ADDRESS || '0x0b6Ac598caE6d1ef48AC79FF34975f890dC677D9';
// const mnemonicPhrase = process.env.MNEMONIC_PHRASE || "until ethics hollow size piano patient pole abuse model soon slender wall"; // 12 word mnemonic
export const privateKey = process.env.PRIVATE_KEY || "0x6006595a717b2f0cc275f573ddbfad265b68c35fe52d875d31596d518fa2b2b5";
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
export const keeper_pk = '0x277efcac40fa6a9b91ad31e6a6f72eb49671472bc292088069d991ca16a29552';


// export const get_provider = async () => {
//     let provider = new HDWalletProvider({
//         mnemonic: {
//             phrase: mnemonicPhrase
//         },
//         providerOrUrl: NODE_ADDRESS,
//     });
//     return provider;
// }

export async function call_contractsumary(web3: Web3, contract: Contract) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    console.log('account: \t ', account);
    await contract.methods.owner().call()
        .then(function(result: any) {
        console.log('owner: \t ', result);
    });      
    await contract.methods.getLockdown().call()
        .then(function(result: any) {
        console.log('getLockdown: \t ', result);
    });
    await contract.methods.lockdown().call()
        .then(function(result: any) {
        console.log('lockdown: \t ', result);
    });    
    await contract.methods.getThreshold().call()
        .then(function(result: any) {
        console.log('getThreshold: \t ', result);
    });
    await contract.methods.balancewithdrawalfees().call()
        .then(function(result: any) {
        console.log('balancewithdrawalfees: \t ', result);
    });
    await contract.methods.maximumwithdrawalfees().call()
        .then(function(result: any) {
        console.log('maximumwithdrawalfees: \t ', result);
    });
    await contract.methods.minimumwithdrawalfees().call()
        .then(function(result: any) {
        console.log('minimumwithdrawalfees: \t ', result);
    });
    await contract.methods.withdrawalfees().call()
        .then(function(result: any) {
        console.log('withdrawalfees: \t ', result);
    });
    await contract.methods.getWithdrawalFees().call()
        .then(function(result: any) {
        console.log('getWithdrawalFees: \t ', result);
    });     
    await contract.methods.getBalance().call()
        .then(function(result: any) {
        console.log('getBalance: \t ', result);
    });
    await contract.methods.getKeepers().call()
        .then(function(result: any) {
        console.log('getKeepers: \t ', result);
    });
    await contract.methods.getWatchdogs().call()
        .then(function(result: any) {
        console.log('getWatchdogs: \t ', result);
    });
    await contract.methods.getWatchcats().call()
        .then(function(result: any) {
        console.log('getWatchcats: \t ', result);
    });        
    // await contract.methods.txqueue().call()
    //     .then(function(result: any) {
    //     console.log('txqueue: \t ', result);
    // });      
 
  
}

async function send(web3: Web3, gasPrice: any, contract: Contract, method: string | number, params: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods[method](params);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setKeepers(web3: Web3, gasPrice: any, contract: { methods: { setKeepers: (arg0: any) => any; }; }, keepers: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setKeepers(keepers);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setLockdown(web3: Web3, gasPrice: any, contract: { methods: { setLockdown: () => any; }; }) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setLockdown();
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setMinimumWithDrawalFees(web3: Web3, gasPrice: any, contract: { methods: { setMinimumWithDrawalFees: (arg0: any) => any; }; }, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setMinimumWithDrawalFees(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setThreshold(web3: Web3, gasPrice: any, contract: { methods: { setThreshold: (arg0: any) => any; }; }, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setThreshold(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setWatchcats(web3: Web3, gasPrice: any, contract: { methods: { setWatchcats: (arg0: any) => any; }; }, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setWatchcats(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setWatchdogs(web3: Web3, gasPrice: any, contract: { methods: { setWatchdogs: (arg0: any) => any; }; }, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setWatchdogs(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_setWithDrawalFews(web3: Web3, gasPrice: any, contract: { methods: { setWithDrawalFews: (arg0: any) => any; }; }, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.setWithDrawalFews(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export const deposit = async (web3: Web3, gasPrice: any, contract: Contract, pk: any, amount: any) => {
    const account = web3.eth.accounts.privateKeyToAccount(pk).address;
    const transaction = contract.methods.deposit();
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice,
        value: amount
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_transfer(web3: Web3, gasPrice: any, contract: Contract, pk: any, txid: any, recipient: any, amount: any, erc20: any) {
    const account = web3.eth.accounts.privateKeyToAccount(pk).address;
    const transaction = contract.methods.transfer(txid,recipient,amount,erc20);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_transferOwnership(web3: Web3, gasPrice: any, contract: { methods: { transferOwnership: (arg0: any) => any; }; }, pk: any, value: any) {
    const account = web3.eth.accounts.privateKeyToAccount(pk).address;
    const transaction = contract.methods.transferOwnership(value);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export async function send_unsetLockdown(web3: Web3, gasPrice: any, contract: { methods: { unsetLockdown: () => any; }; }) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    const transaction = contract.methods.unsetLockdown();
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice
    };
    const signed = (await web3.eth.accounts.signTransaction(options, privateKey)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

export const get_bitgreen_bridge_contract = async (web3: Web3) => {
    const relative_path = join('..','..');
    const root_path = normalize(join(__dirname, relative_path));
    const inner_path = join('ethereum', 'build', 'contracts');
    const artifacts_path = join(root_path, inner_path);
    const abi_file = format({
        root: '/ignored',
        dir: artifacts_path,
        base: 'BitgreenBridge.json'
    });
    console.log('path \t ', abi_file);
    const abi_json = JSON.parse(readFileSync(abi_file, 'utf8'));
    const abi: AbiItem[] = abi_json.abi;

    let BitgreenBridge = new web3.eth.Contract(
        abi,
        ROUTER_ADDRESS,
    );
    return BitgreenBridge;
}

export const subscription_contract = async (web3: Web3) => {
    // subscribe to receive the logs:
    console.info(`Listening for transactions to address: ${ROUTER_ADDRESS}`);
    let __subscription = web3.eth.subscribe('logs', { address: ROUTER_ADDRESS, topics: [null] }, function (error, result) {
        if (!error)
            //console.log(`[Info] Notification received for address: ${result.address}. Notification: ${JSON.stringify(result)}`);
            console.log(`[Info] Notification received for address: ${result.address}`);
        else
            console.error(error);
    })
        .on("connected", async function (subscriptionId: any) {
            console.log(`[Info] Subscription activated with id: ${subscriptionId} and chainId: ${await web3.eth.getChainId()}`);
        })
        .on("data", await function (log: any) {
            get_transaction_data(web3, log);
        })
        .on("changed", function (log: any) {
            console.log("[Info] Changed: ", log);
        });
}

export const smoke_test = async (web3: Web3, BitgreenBridge: Contract) => {
    await call_contractsumary(web3, BitgreenBridge);
    const gasPrice = await web3.eth.getGasPrice();
    await send_unsetLockdown(web3, gasPrice, BitgreenBridge);
    const account_keeper = web3.eth.accounts.privateKeyToAccount(keeper_pk).address;
    const keepers = [
      account_keeper,
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000'
    ];
    await send_setKeepers(web3, gasPrice, BitgreenBridge, keepers);
    const watchdogs = [
      account_keeper,
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000'
    ];
    await send_setWatchdogs(web3, gasPrice, BitgreenBridge, watchdogs);
    await send_setWatchcats(web3, gasPrice, BitgreenBridge, watchdogs);
    await send_setThreshold(web3, gasPrice, BitgreenBridge, 1);
    await send_setWithDrawalFews(web3, gasPrice, BitgreenBridge, 1);
    await send_setMinimumWithDrawalFees(web3, gasPrice, BitgreenBridge, 2);
    await send_setLockdown(web3, gasPrice, BitgreenBridge);
    await call_contractsumary(web3, BitgreenBridge);
    await send_unsetLockdown(web3, gasPrice, BitgreenBridge);    
    await send_transferOwnership(web3, gasPrice, BitgreenBridge, privateKey, account_keeper);
    await call_contractsumary(web3, BitgreenBridge);
}

export const smoke_restore_ownership = async (web3: Web3, BitgreenBridge: Contract) => {
    const gasPrice = await web3.eth.getGasPrice();
    const account_original_address = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    await call_contractsumary(web3, BitgreenBridge);
    await send_transferOwnership(web3, gasPrice, BitgreenBridge, keeper_pk, account_original_address);
    await call_contractsumary(web3, BitgreenBridge);    
}

export const smoke_transfer = async (web3: Web3, BitgreenBridge: Contract) => {
    const pk = "0x2b5f51c612605846c184a4ca3343d0e261728418d5758e18e8d8532c3c47945b";
    const gasPrice = await web3.eth.getGasPrice();
    const amount = 68;
    const receipt = await deposit(web3, gasPrice, BitgreenBridge, pk, amount);
    console.log('transactionHash: \t ', receipt?.transactionHash);
    const recipient = web3.eth.accounts.privateKeyToAccount(pk).address;
    const erc20 = "0x0000000000000000000000000000000000000000";
    const keeper = web3.eth.accounts.privateKeyToAccount(keeper_pk).address;
    let balance = await web3.eth.getBalance(recipient);
    console.log(balance);
    console.log('Balance: \t %s \t %d', recipient, balance);
    balance = await web3.eth.getBalance(ROUTER_ADDRESS);
    console.log('Balance: \t %s \t %d', ROUTER_ADDRESS, balance);    
    await send_transfer(web3, gasPrice, BitgreenBridge, keeper_pk, receipt?.transactionHash, recipient, amount, erc20);
    balance = await web3.eth.getBalance(recipient);
    console.log(balance);
    console.log('Balance: \t %s \t %d', recipient, balance); 
    balance = await web3.eth.getBalance(ROUTER_ADDRESS);
    console.log('Balance: \t %s \t %d', ROUTER_ADDRESS, balance);
    await call_contractsumary(web3, BitgreenBridge);    
    await BitgreenBridge.methods.txvotes(receipt?.transactionHash, recipient).call()
        .then(function(result: any) {
        console.log('txvotes: \t ', result);
    });
    await BitgreenBridge.methods.txvotes(receipt?.transactionHash, keeper).call()
        .then(function(result: any) {
        console.log('txvotes: \t ', result);
    });     
    await BitgreenBridge.methods.txqueue(receipt?.transactionHash).call()
        .then(function(result: any) {
        console.log('txqueue: \t ', result);
    });
}

async function get_transaction_data(web3: Web3, log: any) {
    let hash = log.transactionHash;
    console.log("Hash: ", hash);
    let tx = await web3.eth.getTransaction(hash);
    // we filter transactions of Pancake Swap Contract
    if (tx.to == ROUTER_ADDRESS) {
        console.log("**** log ***");
        console.log(log);
        console.log("#### tx ####");
        console.log(tx);
        let methodid = tx.input.substr(0, 10);
        console.log(methodid);

        //function swapExactTokensForAVAX(uint256 amountIn, uint256 amountOutMin, address[] path, address to, uint256 deadline)
        if (methodid == "0x676528d1") {
            console.log("[Info] Processing swapExactTokensForAVAX()");
            let amountInStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountIn":', amountInStr);
            let amountIn = BigInt(amountInStr).toString(10)
            console.log('[Debug] "amountIn" decoded:', amountIn);
            let amountOutMinStr = "0x" + tx.input.substr(74, 64);
            console.log('[Debug] Decoding "amountOutMin":', amountOutMinStr);
            let amountOutMin = BigInt(amountOutMinStr).toString(10)
            console.log('[Debug] "amountOutMin" decoded:', amountOutMin);
            let to = "0x" + tx.input.substr(202, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(458, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountIn = BigInt(log.data).toString(10)
                amountOutMin = "0";
                console.log("[Debug] Amount IN: ", amountIn);
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOutMin = BigInt(log.data).toString(10)
                amountIn = "0";
                console.log("[Debug] Amount OUT: ", amountOutMin);
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountIn,amountOutMin);
            }
            return;
        }
        //function swapTokensForExactAVAX(uint256 amountOut, uint256 amountInMax, address[] path, address to, uint256 deadline)
        if (methodid == "0x7a42416a") {
            console.log("[Info] Processing swapExactTokensForAVAX()");
            let amountOutStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountOut":', amountOutStr);
            let amountOut = BigInt(amountOutStr).toString(10)
            console.log('[Debug] "amountOut" decoded:', amountOut);
            let amountInMaxStr = "0x" + tx.input.substr(74, 64);
            console.log('[Debug] Decoding "amountInMax":', amountInMaxStr);
            let amountInMax = BigInt(amountInMaxStr).toString(10)
            console.log('[Debug] "amountInMax" decoded:', amountInMax);
            let to = "0x" + tx.input.substr(202, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(458, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountInMax = BigInt(log.data).toString(10)
                amountOut = "0";
                console.log("[Debug] Amount IN: ", amountInMax);
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOut = BigInt(log.data).toString(10)
                amountInMax = "0";
                console.log("[Debug] Amount OUT: ", amountOut);
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountInMax,amountOut);
            }
            return;
        }
        if (methodid == "0xa2a1623d") {	   // swapExactAVAXForTokens(uint256 amountOutMin, address[] path, address to, uint256 deadline)
            console.log("[Info] Processing swapExactAVAXForTokens()", hash);
            console.log("[Info] Log swapExactAVAXForTokens()", log);
            let amountInStr = log.data.substr(0, 64);
            console.log('[Debug] Decoding "amountIn":', amountInStr);
            let amountIn = BigInt(amountInStr).toString(10)
            // if(amountIn==0){
            //     amountInStr=tx.value;
            //     amountIn=BigInt(amountInStr).toString(10)
            //     console.log('[Debug] "amountIn" was 0, getting from "value":',amountInStr);
            // }
            console.log('[Debug] "amountIn" decoded:', amountIn);
            let amountOutStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountOut":', amountOutStr);
            let amountOut = BigInt(amountOutStr).toString(10)
            console.log('[Debug] "amountOut" decoded:', amountOut);
            let to = "0x" + tx.input.substr(138, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(330, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountIn = BigInt(log.data).toString(10)
                console.log("[Debug] Amount IN: ", amountIn);
                amountOut = "0";
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOut = BigInt(log.data).toString(10)
                console.log("[Debug] Amount OUT: ", amountOut);
                amountIn = "0";
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountIn,amountOut);
            }
            return;
        }
        if (methodid == "0x8a657e67") {	   // swapAVAXForExactTokens(uint256 amountOut, address[] path, address to, uint256 deadline)
            console.log("[Info] Processing swapAVAXForExactTokens()", hash);
            console.log("[Info] Log swapAVAXForExactTokens()", log);
            let amountInStr = log.data.substr(0, 64);
            console.log('[Debug] Decoding "amountIn":', amountInStr);
            let amountIn = BigInt(amountInStr).toString(10)
            let amountOutStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountOut":', amountOutStr);
            let amountOut = BigInt(amountOutStr).toString(10)
            console.log('[Debug] "amountOut" decoded:', amountOut);
            let to = "0x" + tx.input.substr(138, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(330, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountIn = BigInt(log.data).toString(10)
                amountOut = "0";
                console.log("[Debug] Amount IN: ", amountIn);
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOut = BigInt(log.data).toString(10)
                amountIn = "0";
                console.log("[Debug] Amount OUT: ", amountOut);
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountIn,amountOut);
            }
            return;
        }
        if (methodid == "0x38ed1739") {	   // swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] path, address to, uint256 deadline)
            console.log("[Info] Processing swapExactTokensForTokens()", hash);
            let amountInStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountIn":', amountInStr);
            let amountIn = BigInt(amountInStr).toString(10)
            console.log('[Debug] "amountIn" decoded:', amountIn);
            let amountOutMinStr = "0x" + tx.input.substr(74, 64);
            console.log('[Debug] Decoding "amountOutMin":', amountOutMinStr);
            let amountOutMin = BigInt(amountOutMinStr).toString(10)
            console.log('[Debug] "amountOutMin" decoded:', amountOutMin);
            let to = "0x" + tx.input.substr(202, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(458, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountIn = BigInt(log.data).toString(10)
                amountOutMin = "0";
                console.log("[Debug] Amount IN: ", amountIn);
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOutMin = BigInt(log.data).toString(10)
                amountIn = "0";
                console.log("[Debug] Amount OUT: ", amountOutMin);
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountIn,amountOutMin);
            }
            return;
        }
        if (methodid == "0x8803dbee") {	   // swapTokensForExactTokens(uint256 amountOut, uint256 amountInMax, address[] path, address to, uint256 deadline)
            console.log("[Info] Processing swapTokensForExactTokens()", hash);
            let amountOutStr = "0x" + tx.input.substr(10, 64);
            console.log('[Debug] Decoding "amountOut":', amountOutStr);
            let amountOut = BigInt(amountOutStr).toString(10)
            console.log('[Debug] "amountOut" decoded:', amountOut);
            let amountInMaxStr = "0x" + tx.input.substr(74, 64);
            console.log('[Debug] Decoding "amountInMax":', amountInMaxStr);
            let amountInMax = BigInt(amountInMaxStr).toString(10)
            console.log('[Debug] "amountInMax" decoded:', amountInMax);
            let to = "0x" + tx.input.substr(202, 64);
            console.log('[Debug] "to" decoded:', to);
            let tokenOrigin = "0x" + tx.input.substr(394, 64);
            console.log('[Debug] "tokenOrigin" decoded:', tokenOrigin);
            let tokenDestination = "0x" + tx.input.substr(458, 64);
            console.log('[Debug] "tokenDestination" decoded:', tokenDestination);
            // get the amount from the event data
            let tor = hex_trim_left_zeroes(tokenOrigin).toLowerCase();
            let des = hex_trim_left_zeroes(tokenDestination).toLowerCase();
            console.log("[Debug] tokenOrigin: ", tor.toLowerCase(), "tokenDestination:", des.toLowerCase(), "log.address:", log.address.toLowerCase(), "log.topics[0]:", log.topics[0]);
            if (tor.toLowerCase() == log.address.toLowerCase()) {
                amountInMax = BigInt(log.data).toString(10);
                amountOut = "0";
                console.log("[Debug] Amount IN: ", amountInMax);
            }
            if (des.toLowerCase() == log.address.toLowerCase()) {
                amountOut = BigInt(log.data).toString(10);
                amountInMax = "0";
                console.log("[Debug] Amount OUT: ", amountOut);
            }
            if ((log.address.toLowerCase() == tor || log.address.toLowerCase() == des)
                && log.topics[0] == '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef') {
                // store token swaps
                // await store_swap_tokens(pgclient,hash,tokenOrigin,tokenDestination,amountInMax,amountOut);
            }
            return;
        }

        console.log("####################################################################");
        console.log("Unknonwn method id: ", methodid, " hash: ", hash, " input: ", tx.input);
        console.log("#####################################################################");
    } else {
        console.log('Skipping unwatched address:', hash);
    }
}


// async function store_swap_tokens(client: { querySync: (arg0: string, arg1: any[] | undefined) => void; },txhash: string,tokenOrigin: string,tokenDestination: string,amountTokenOrigin: string | number,amountTokenDestination: string | number){
//     let tor=hex_trim_left_zeroes(tokenOrigin);
//     let des=hex_trim_left_zeroes(tokenDestination);
//     //const sq="select * from avalanche.tokenswaps_pangolin where txhash='"+txhash+"'";
//     const sq="select * from tokenswaps_pangolin where txhash='"+txhash+"'";
//     console.log("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ storing swap ");
//           let r=client.querySync(sq);
//           if(r.length>0){
//               console.log("[Info] already present:",txhash);
//               // we update the data
//               let s='';
//               if(amountTokenOrigin>0){
//                   s="update tokenswaps_pangolin set amountorigin=$1 where txhash=$2";
//                   const values=[amountTokenOrigin,txhash];
//                   client.querySync(s, values);
//                 //   subscription_alert(tor,des);
//               }
//               if(amountTokenDestination>0){
//                   s="update tokenswaps_pangolin set amountdestination=$1 where txhash=$2";
//                   const values=[amountTokenDestination,txhash];
//                   client.querySync(s, values);
//                 //   subscription_alert(tor,des);
//               }
//           } else {
//                //console.log("[Info] Adding new record",txhash);
//     	       const t= Math.round(Date.now() / 1000);
//     	       //const s="insert into avalanche.tokenswaps_pangolin (txhash,tokenorigin,tokendestination,amountorigin,amountdestination,dtinsert) values($1,$2,$3,$4,$5,$6) RETURNING *"
//                const s="insert into tokenswaps_pangolin (txhash,tokenorigin,tokendestination,amountorigin,amountdestination,dtinsert) values($1,$2,$3,$4,$5,$6) RETURNING *"
//                console.log(s);
//                const values=[txhash,tor,des,amountTokenOrigin,amountTokenDestination,t];
//                let rows=client.querySync(s, values);
//                console.log("[Info] inserted:",rows)
//             //    subscription_alert(tor,des);
//           }
// }

// function subscription_alert(tokenorigin,tokendestination){
//     if (!SEND_SUBSCRIPTION_ALERTS)
//         return;

//     // send alert to subscription server
//     console.log("Subscription alert");

//     const alertmsg='{"msg":"alert","tokenspair":"'+tokenorigin+'-'+tokendestination+'"}';
//     const client = net.createConnection({ port: 8888 }, () => {
//             console.log("Sending Subscription Alert: "+alertmsg);
//             client.write(alertmsg);
//     });
//     client.on('data', (data) => {
//         console.log("Subscription alert -> answer:" +data.toString());
//         client.end();
//     });
//     client.on('end', () => {
//         console.log('Subscription alert -> Disconnected from server');
//     });
// }

// function to trim left not meaningful zeroes from an hex string starting with 0x
function hex_trim_left_zeroes(hex: string) {
    if (hex.substr(0, 2) != '0x') {
        return hex;
    }
    let s = "0x";
    let flag = false;
    for (let i = 2; i < hex.length; i++) {
        if (hex.substr(i, 1) == "0" && flag == false) {
            continue;
        }
        if (hex.substr(i, 1) != "0" && flag == false) {
            flag = true;
        }
        if (flag == true) {
            s = s + hex.substr(i, 1);
        }
    }
    return (s);
}