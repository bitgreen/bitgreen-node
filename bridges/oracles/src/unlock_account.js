import Web3 from 'web3';
import { NODE_ADDRESS } from './evm_bridge.js';

const ACCOUNT_PASSWORD = process.env.ACCOUNT_PASSWORD || '';

const unlockAccounts = async (web3) => {
    const accounts = await web3.eth.personal.getAccounts();
    console.log(accounts);
    accounts.forEach(function (account) {
        console.log('Unlocking ' + account + '...');
        unlockAccountsIfNeeded(account, ACCOUNT_PASSWORD, 3000);
    });
}

const unlockAccountsIfNeeded = async (accounts, passwords, unlock_duration_sec) => {
    if (typeof (unlock_duration_sec) === 'undefined') unlock_duration_sec = 300;

    for (let i = 0; i < accounts.length; i++) {
        if (isAccountLocked(accounts[i])) {
            console.log("Account " + accounts[i] + " is locked. Unlocking")
            web3.eth.personal.unlockAccount(accounts[i], passwords[i], unlock_duration_sec);
        }
    }
}  

const main = async () => {
    try {
        const web3 = new Web3(NODE_ADDRESS);
        await unlockAccounts(web3);
    }
    catch (err) {
        console.error('Error', err);
    }
};
main().catch(console.error).finally(() => {
    console.log('end');
    process.exit();
});
