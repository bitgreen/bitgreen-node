import { ApiPromise, Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { KeyringPair } from '@polkadot/keyring/types';
import { subscription_pallet_bridge, setup_substrate, setup_bridge_test, setup_basic_bridge_test, pallet_bridge_burn } from './pallet_bridge.js';
import { subscription_contract, NODE_ADDRESS, get_bitgreen_bridge_contract, smoke_test, smoke_restore_ownership, smoke_transfer, call_contractsumary} from './evm_bridge.js';
import Web3 from 'web3';

let api: ApiPromise;

const main = async () => {
  // let provider = null;
  try {

    api = await setup_substrate();
    const keyring = new Keyring({ type: 'sr25519' });
    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');
    const dave = keyring.addFromUri('//Dave');
    // const eve = keyring.addFromUri('//Eve');
    // const ferdie = keyring.createFromUri('//Ferdie');

    // Wait until we are ready and connected
    await api.isReady;

    const web3 = new Web3(NODE_ADDRESS);
    const BitgreenBridge = await get_bitgreen_bridge_contract(web3);

    const eve = keyring.addFromUri('//Eve');
    const recipient = api.createType('AccountId', eve.address);
    const bitg_token_bytes = api.createType('Bytes', "BBB");
    const transaction_id_bytes = api.createType('Bytes', "a123");
    const balance = api.createType('Balance', "1");

    await pallet_bridge_burn(api, charlie, bitg_token_bytes, recipient, transaction_id_bytes, balance);

  } catch (err) {
    console.error('Error', err);
  } finally {
    // provider!.engine.stop();
  }
}

main().catch(console.error).finally(() => {
  console.log('end');
  process.exit();
});

