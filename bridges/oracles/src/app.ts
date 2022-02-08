import { ApiPromise, Keyring } from '@polkadot/api';
// import type { AccountId } from '@polkadot/types/interfaces';
import '@polkadot/api-augment';
import { KeyringPair } from '@polkadot/keyring/types';
import { subscription_pallet_bridge } from './pallet_bridge.js';
import { subscription_contract, NODE_ADDRESS, get_bitgreen_bridge_contract, smoke_test, smoke_restore_ownership, smoke_transfer, call_contractsumary} from './evm_bridge.js';
import Web3 from 'web3';

// let api: ApiPromise;

const main = async () => {
  // let provider = null;
  try {

    // api = await setup_substrate();
    // const keyring = new Keyring({ type: 'sr25519' });
    // create Alice based on the development seed
    // const alice = keyring.addFromUri('//Alice');
    // const bob = keyring.addFromUri('//Bob');
    // const charlie = keyring.addFromUri('//Charlie');
    // const dave = keyring.addFromUri('//Dave');
    // const eve = keyring.addFromUri('//Eve');
    // const ferdie = keyring.createFromUri('//Ferdie');

    // Wait until we are ready and connected
    // await api.isReady;

    // const recipient = api.createType('AccountId', charlie.address);

    // await setup_bridge_test(api, alice);
    // await bridge_mint_smoke_test(api, alice, recipient);
    // await bridge_burn_smoke_test(api, alice, recipient);

    // provider = await get_provider();
    const web3 = new Web3(NODE_ADDRESS);
    // const addresses = provider.getAddresses();
    // console.log(`addresses: \t ${addresses}`);
    await subscription_contract(web3);
    const BitgreenBridge = await get_bitgreen_bridge_contract(web3);
    BitgreenBridge.events.allEvents([], function (error: any, event: any) {
      if (error) {
        console.error('error: \t ', error);
      }
      console.log('event: \t ', event);
    })
      .on('error', console.error);

      // const filter = web3.eth.filter('latest');
      // filter.watch((err, res) => {
      //   if (err) {
      //     console.log(`Watch error: ${err}`);
      //   } else {
      //     // Update balance
      //     web3.eth.getBalance(address, (err, bal) => {
      //       if (err) {
      //         console.log(`getBalance error: ${err}`);
      //       } else {
      //         balance = bal;
      //         console.log(`Balance [${address}]: ${web3.fromWei(balance, "ether")}`);
      //       }
      //     });
      //   }
      // });      

    // await smoke_test(web3, BitgreenBridge);
    // await smoke_restore_ownership(web3, BitgreenBridge);
    await smoke_transfer(web3, BitgreenBridge);

  } catch (err) {
    console.error('Error', err);
  } finally {

    // provider!.engine.stop();
  }
}

main().catch(console.error).finally(() => {
  console.log('end');
  // process.exit();
});

