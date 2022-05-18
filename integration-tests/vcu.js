import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from '@polkadot/keyring';
import assert from 'assert';

export async function pallet_vcu_checks(chain) {
  const keyring = new Keyring({ type: 'sr25519' });
  let Bob = keyring.addFromUri('//Bob');

  let dev_account = await chain.get_dev_account();

  // test adding authorised accounts
  console.log("VCU : Test adding authorised accounts");
  const add_auth = await chain.api.tx.sudo.sudo(
    chain.api.tx.vcu.addAuthorizedAccount(Bob.address)
  ).signAndSend(dev_account, (result) => { console.log(result) });

  // test removing authorised accounts
  console.log("VCU : Test destroy authorised accounts");
  const destroy_auth = await chain.api.tx.sudo.sudo(
    chain.api.tx.vcu.destroyAuthorizedAccount(Bob.address)
  ).signAndSend(dev_account, (result) => { console.log(result) });

}
