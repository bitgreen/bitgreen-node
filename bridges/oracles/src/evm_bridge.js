/// Module for evm compatible client integration based on web3js

import Web3 from 'web3';
import { readFileSync } from 'fs';
import { dirname, join, normalize, format } from 'path';
import { fileURLToPath } from 'url';

export const options = {
    timeout: 30000, // ms

    // Useful for credentialed urls, e.g: ws://username:password@localhost:8546
    // headers: {
    //   authorization: 'Basic username:password'
    // },

    clientConfig: {
      // Useful if requests are large
      maxReceivedFrameSize: 100000000,   // bytes - default: 1MiB
      maxReceivedMessageSize: 100000000, // bytes - default: 8MiB

      // Useful to keep a connection alive
      keepalive: true,
      keepaliveInterval: 60000 // ms
    },

    // Enable auto reconnection
    reconnect: {
        auto: true,
        delay: 5000, // ms
        maxAttempts: 5,
        onTimeout: false
    }
};

export const NODE_ADDRESS = process.env.NODE_ADDRESS || Web3.givenProvider || new Web3.providers.WebsocketProvider('ws://127.0.0.1:8546', options);
export const ROUTER_ADDRESS = process.env.ROUTER_ADDRESS;
export const privateKey = process.env.PRIVATE_KEY;
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

var nonce_block = {};

const get_nonce = async (web3, account) => {    
    if (nonce_block[account]) {
        nonce_block[account] = nonce_block[account] + 1;
    } else {
        nonce_block[account] = await web3.eth.getTransactionCount(account);
    }
    return nonce_block[account]
}


export const get_erc20 = async (asset_id) => {
    const erc20 = '0x0000000000000000000000000000000000000000';
    return erc20;
};

async function send(web3, gasPrice, contract, method, params) {
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

/// send transaction with call to method setKeepers from contract BitgreenBridge.sol
export async function send_setKeepers(web3, gasPrice, contract, keepers) {
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

/// send transaction with call to method setLockdown from contract BitgreenBridge.sol
export async function send_setLockdown(web3, gasPrice, contract) {
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

/// send transaction with call to method setMinimumWithDrawalFees from contract BitgreenBridge.sol
export async function send_setMinimumWithDrawalFees(web3, gasPrice, contract, value) {
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

/// send transaction with call to method setThreshold from contract BitgreenBridge.sol
export async function send_setThreshold(web3, gasPrice, contract, value) {
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

/// send transaction with call to method setWatchcats from contract BitgreenBridge.sol
export async function send_setWatchcats(web3, gasPrice, contract, value) {
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

/// send transaction with call to method setWatchdogs from contract BitgreenBridge.sol
export async function send_setWatchdogs(web3, gasPrice, contract, value) {
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

/// send transaction with call to method setWithDrawalFews from contract BitgreenBridge.sol
export async function send_setWithDrawalFews(web3, gasPrice, contract, value) {
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

/// send transaction with call to method deposit from contract BitgreenBridge.sol
/// this call will trigger keepers looking for cross bridge transactions
export const deposit_method = async (web3, gasPrice, contract, pk, amount, destination, nonce) => {
    const account = web3.eth.accounts.privateKeyToAccount(pk).address;
    const transaction = contract.methods.deposit(destination);
    const gas = await transaction.estimateGas({ from: account });

    let new_nonce = await get_nonce(web3, account);
    if (nonce) {
        new_nonce = new_nonce + nonce;
    }
    console.log('nonce \t ', new_nonce);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas:  gas,
        gasPrice: gasPrice,
        value: amount,
        nonce: new_nonce
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
};

/// transfer amount to contract address
export const deposit = async (web3, gasPrice, pk, amount, destination) => {
    const options = {
        to: ROUTER_ADDRESS,
        gas:  22496,
        gasPrice: gasPrice,
        value: amount,
        data: destination
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
};

/// send transaction with call to method transfer from contract BitgreenBridge.sol
/// this call from keepers will reach consensus threshold abount transfer from pallet bridge
export async function send_transfer(web3, gasPrice, contract, pk, txid, recipient, amount, erc20) {
    const account = web3.eth.accounts.privateKeyToAccount(pk).address;
    const transaction = contract.methods.transfer(txid, recipient, amount, erc20);
    const nonce = await get_nonce(web3, account);
    const options = {
        to: transaction._parent._address,
        data: transaction.encodeABI(),
        gas: await transaction.estimateGas({ from: account }),
        gasPrice: gasPrice,
        nonce: nonce
    };
    const signed = (await web3.eth.accounts.signTransaction(options, pk)).rawTransaction;
    let receipt = null;
    if (signed) {
        receipt = await web3.eth.sendSignedTransaction(signed);
    }
    return receipt;
}

/// send transaction with call to method transferOwnership from contract BitgreenBridge.sol
export async function send_transferOwnership(web3, gasPrice, contract, pk, value) {
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

/// send transaction with call to method unsetLockdown from contract BitgreenBridge.sol
export async function send_unsetLockdown(web3, gasPrice, contract) {
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

export const get_bitgreen_bridge_abi = () => {
    const relative_path = join('..', '..');
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
    const abi = abi_json.abi;
    return abi;
}

// resolve relative path to json types definition file
export const get_bitgreen_bridge_contract = async (web3) => {
    const relative_path = join('..', '..');
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
    const abi = abi_json.abi;
    let BitgreenBridge = new web3.eth.Contract(abi, ROUTER_ADDRESS);
    return BitgreenBridge;
};

