import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from '@polkadot/keyring';
import assert from 'assert';
import { pallet_vcu_checks } from './vcu.js';

const wait_for_tx = () => new Promise((resolve) => setTimeout(resolve, 10000));

class BitgreenNode {
  api = null;

  constructor(url = "ws://127.0.0.1:9944") {
    this.url = url;
  }

  async connect_to_chain() {
    // if connection already exists return
    if (this.api) return this.api;
    const provider = new WsProvider(this.url);

    // Create the API and wait until ready
    const api = await ApiPromise.create({ provider });

    // Retrieve the chain & node information information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version(),
    ]);

    this.api = api;
    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    return api;
  }

  // query balance from balances pallet
  async query_balance(account) {
    const accountInfo = await this.api.query.system.account(account);
    const { data } = accountInfo.toJSON();
    return data.free / 1e6;
  }

  // return dev account for chain
  async get_dev_account() {
      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519' });
      // Alice is the dev account boss
      return keyring.addFromUri('//Alice');
  }

  // get the chain sudo key
  async get_sudo_key() {
    let result = await this.api.query.sudo.key();
    return result;
  }
}

async function main() {
  console.log("######## Starting Integration tests");
  // establish connection to chain
  let chain = new BitgreenNode();

  // lets do some sanity checks to ensure we have setup ready
  console.log("######## Checking evironment setup");
  await chain.connect_to_chain();
  let dev_account = await chain.get_dev_account();

  // ensure dev account has some balance
  assert(await chain.query_balance(dev_account.address) > 10000);

  // ensure dev account is the sudo account
  assert(await chain.get_sudo_key() == dev_account.address);

  // run checks for vcu pallet
  await pallet_vcu_checks(chain);

}

main().catch(console.error);
