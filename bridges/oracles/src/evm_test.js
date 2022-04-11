import { ROUTER_ADDRESS, privateKey, send_transfer, send_unsetLockdown, send_setKeepers, send_setWatchdogs, send_setWatchcats, send_setThreshold, send_setWithDrawalFews, send_setMinimumWithDrawalFees, send_transferOwnership, deposit } from './evm_bridge.js';

export const keeper_pk1 = '0x1d5b454dd5885cab88c648873a0a45a28beeb52313c493b2e81560474b2bf8a9';
export const keeper_pk2 = '0x20c56d95edaa777bb405f49ab57710a78d9b2bb5726cb43ff04c3446aa32b842';
export const keeper_pk3 = '0xd6a123956fe58dabd7072e8c6e325ff37bf04500914c5986287887cfbe5700e1';

export const basic_evm_setup_test = async (web3, BitgreenBridge) => {
    await call_contractsumary(web3, BitgreenBridge);
    const gasPrice = await web3.eth.getGasPrice();
    await send_unsetLockdown(web3, gasPrice, BitgreenBridge);
    const account_keeper1 = web3.eth.accounts.privateKeyToAccount(keeper_pk1).address;
    const account_keeper2 = web3.eth.accounts.privateKeyToAccount(keeper_pk2).address;
    const account_keeper3 = web3.eth.accounts.privateKeyToAccount(keeper_pk3).address;
    const keepers = [
        account_keeper1,
        account_keeper2,
        account_keeper3,
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
        account_keeper1,
        account_keeper2,
        account_keeper3
    ];
    await send_setWatchdogs(web3, gasPrice, BitgreenBridge, watchdogs);
    await send_setWatchcats(web3, gasPrice, BitgreenBridge, watchdogs);
    await send_setThreshold(web3, gasPrice, BitgreenBridge, 2);
    await send_setWithDrawalFews(web3, gasPrice, BitgreenBridge, 1);
    await send_setMinimumWithDrawalFees(web3, gasPrice, BitgreenBridge, 1);
    const amount = 1000;
    // const receipt = await deposit(web3, gasPrice, BitgreenBridge, privateKey, amount, null);
    // console.log('transactionHash: \t ', receipt?.transactionHash);
    await call_contractsumary(web3, BitgreenBridge);
};

export const smoke_test = async (web3, BitgreenBridge) => {
    await call_contractsumary(web3, BitgreenBridge);
    const gasPrice = await web3.eth.getGasPrice();
    await send_unsetLockdown(web3, gasPrice, BitgreenBridge);
    const account_keeper = web3.eth.accounts.privateKeyToAccount(keeper_pk1).address;
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
};

export const smoke_restore_ownership = async (web3, BitgreenBridge) => {
    const gasPrice = await web3.eth.getGasPrice();
    const account_original_address = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    await call_contractsumary(web3, BitgreenBridge);
    await send_transferOwnership(web3, gasPrice, BitgreenBridge, keeper_pk1, account_original_address);
    await call_contractsumary(web3, BitgreenBridge);
};

export async function call_contractsumary(web3, contract) {
    const account = web3.eth.accounts.privateKeyToAccount(privateKey).address;
    console.log('account: \t ', account);
    await contract.methods.owner().call()
        .then(function (result) {
            console.log('owner: \t ', result);
        });
    await contract.methods.getLockdown().call()
        .then(function (result) {
            console.log('getLockdown: \t ', result);
        });
    await contract.methods.lockdown().call()
        .then(function (result) {
            console.log('lockdown: \t ', result);
        });
    await contract.methods.getThreshold().call()
        .then(function (result) {
            console.log('getThreshold: \t ', result);
        });
    await contract.methods.balancewithdrawalfees().call()
        .then(function (result) {
            console.log('balancewithdrawalfees: \t ', result);
        });
    await contract.methods.maximumwithdrawalfees().call()
        .then(function (result) {
            console.log('maximumwithdrawalfees: \t ', result);
        });
    await contract.methods.minimumwithdrawalfees().call()
        .then(function (result) {
            console.log('minimumwithdrawalfees: \t ', result);
        });
    await contract.methods.withdrawalfees().call()
        .then(function (result) {
            console.log('withdrawalfees: \t ', result);
        });
    await contract.methods.getWithdrawalFees().call()
        .then(function (result) {
            console.log('getWithdrawalFees: \t ', result);
        });
    await contract.methods.getBalance().call()
        .then(function (result) {
            console.log('getBalance: \t ', result);
        });
    await contract.methods.getKeepers().call()
        .then(function (result) {
            console.log('getKeepers: \t ', result);
        });
    await contract.methods.getWatchdogs().call()
        .then(function (result) {
            console.log('getWatchdogs: \t ', result);
        });
    await contract.methods.getWatchcats().call()
        .then(function (result) {
            console.log('getWatchcats: \t ', result);
        });
}

export const smoke_transfer = async (web3, BitgreenBridge) => {
    const pk = "0x16eea8ef343d6138e563991c23a80310d8b9cbc3b9dd9e11ebdd4005d4773a1f";
    const gasPrice = await web3.eth.getGasPrice();
    const amount = 68;
    const receipt = await deposit(web3, gasPrice, BitgreenBridge, pk, amount);
    console.log('transactionHash: \t ', receipt?.transactionHash);
    const recipient = web3.eth.accounts.privateKeyToAccount(pk).address;
    const erc20 = "0x0000000000000000000000000000000000000000";
    const keeper = web3.eth.accounts.privateKeyToAccount(keeper_pk1).address;
    let balance = await web3.eth.getBalance(recipient);
    console.log(balance);
    console.log('Balance: \t %s \t %d', recipient, balance);
    balance = await web3.eth.getBalance(ROUTER_ADDRESS);
    console.log('Balance: \t %s \t %d', ROUTER_ADDRESS, balance);
    await send_transfer(web3, gasPrice, BitgreenBridge, keeper_pk1, receipt?.transactionHash, recipient, amount, erc20);
    balance = await web3.eth.getBalance(recipient);
    console.log(balance);
    console.log('Balance: \t %s \t %d', recipient, balance);
    balance = await web3.eth.getBalance(ROUTER_ADDRESS);
    console.log('Balance: \t %s \t %d', ROUTER_ADDRESS, balance);
    await call_contractsumary(web3, BitgreenBridge);
    await BitgreenBridge.methods.txvotes(receipt?.transactionHash, recipient).call()
        .then(function (result) {
            console.log('txvotes: \t ', result);
        });
    await BitgreenBridge.methods.txvotes(receipt?.transactionHash, keeper).call()
        .then(function (result) {
            console.log('txvotes: \t ', result);
        });
    await BitgreenBridge.methods.txqueue(receipt?.transactionHash).call()
        .then(function (result) {
            console.log('txqueue: \t ', result);
        });
};

