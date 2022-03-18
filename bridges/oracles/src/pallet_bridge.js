import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Channel } from 'async-channel';
import { readFileSync } from 'fs';
import { dirname, join, normalize, format } from 'path';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const node_address = 'ws://127.0.0.1:9944';
export const SECRETSEED = process.env.PALLET_MNEMONIC || "//Alice";
export const TOKEN = process.env.TOKEN || "BBB";
const get_custom_types = async () => {
    const relative_path = join('..', '..', '..');
    const root_path = normalize(join(__dirname, relative_path));
    const asset_path = join(root_path, 'assets');
    const types_file = format({
        root: '/ignored',
        dir: asset_path,
        base: 'types.json'
    });
    console.log('path \t ', types_file);
    const custom_types = JSON.parse(readFileSync(types_file, 'utf8'));
    return custom_types;
};
export const setup_substrate = async () => {
    const wsProvider = new WsProvider(node_address);
    const custom_types = await get_custom_types();
    const api = await ApiPromise.create({
        provider: wsProvider, "types": custom_types
    });
    return api;
};

export const destination = async (api, block, index) => {
    try {
        const blockHash = await api.rpc.chain.getBlockHash(block);
        const signedBlock = await api.rpc.chain.getBlock(blockHash);
        const { method: { args, method, section } } = signedBlock.block.extrinsics[index];
        if (section === 'bridge' && method === 'request') {
            return args[1].toHex();
        } else {
            return null;
        }
    }
    catch (err) {
        console.error('Error', err);
        return null;
    }
}

export const native_transfer = async (api, keyspair, amount, address) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.balances.transfer(address, amount)
        .signAndSend(keyspair, ({ status, events, dispatchError }) => {
            console.log(`Current status is ${status}`);
            try {
                // status would still be set, but in the case of error we can shortcut
                // to just check it (so an error would indicate InBlock or Finalized)
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { name, section } = decoded;
                        console.log(`${section}.${name}`);
                    }
                    else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                    chan.push(`isInBlock.`);
                }
                else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    // chan.push(`Event report end.`);
                }
            }
            catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(value => console.log(value), error => console.error(error));
    chan.close();
};
export const pallet_assets_create = async (api, keyspair, id, admin, maxZombies, minBalance) => {
    const unsub = await api.tx.assets.create(id, maxZombies, minBalance)
        .signAndSend(keyspair, ({ status, events, dispatchError }) => {
            try {
                // status would still be set, but in the case of error we can shortcut
                // to just check it (so an error would indicate InBlock or Finalized)
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { name, section } = decoded;
                        console.log(`${section}.${name}`);
                    }
                    else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(dispatchError.toString());
                    }
                }
            }
            catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
};
export const pallet_assets_force_create = async (api, keyspair, id, admin, maxZombies, minBalance) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.sudo
        .sudo(api.tx.assets.forceCreate(id, admin, maxZombies, minBalance))
        .signAndSend(keyspair, ({ status, events, dispatchError }) => {
            console.log(`Current status is ${status}`);
            try {
                // status would still be set, but in the case of error we can shortcut
                // to just check it (so an error would indicate InBlock or Finalized)
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { name, section } = decoded;
                        console.log(`${section}.${name}`);
                    }
                    else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                }
                else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
                    events
                        // We know this tx should result in `Sudid` event.
                        .filter(({ event }) =>
                            api.events.sudo.Sudid.is(event)
                        )
                        // We know that `Sudid` returns just a `Result`
                        .forEach(({ event: { data: [result] } }) => {
                            // Now we look to see if the extrinsic was actually successful or not...
                            if (result.isError) {
                                let error = result.asError;
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = api.registry.findMetaError(error.asModule);
                                    const { docs, name, section } = decoded;

                                    console.log(`${section}.${name}: ${docs.join(' ')}`);
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString());
                                }
                                unsub();
                                chan.push("error");
                            } else {

                            }
                        });
                    unsub();
                    chan.push(`Event report end.`);
                }
            }
            catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(value => console.log(value), error => console.error(error));
    chan.close();
};
export const pallet_bridge_create_settings = async (api, keyspair, key, data) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.sudo
        .sudo(api.tx.bridge.createSettings(key, data))
        .signAndSend(keyspair, ({ status, events, dispatchError }) => {
            console.log(`Current status is ${status}`);
            try {
                // status would still be set, but in the case of error we can shortcut
                // to just check it (so an error would indicate InBlock or Finalized)
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { name, section } = decoded;
                        console.log(`${section}.${name}`);
                    }
                    else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                }
                else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
                    events
                        // We know this tx should result in `Sudid` event.
                        .filter(({ event }) =>
                            api.events.sudo.Sudid.is(event)
                        )
                        // We know that `Sudid` returns just a `Result`
                        .forEach(({ event: { data: [result] } }) => {
                            // Now we look to see if the extrinsic was actually successful or not...
                            if (result.isError) {
                                let error = result.asError;
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = api.registry.findMetaError(error.asModule);
                                    const { docs, name, section } = decoded;

                                    console.log(`${section}.${name}: ${docs.join(' ')}`);
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString());
                                }
                                unsub();
                                chan.push("error");
                            } else {

                            }
                        });

                    unsub();
                    chan.push(`Event report end.`);
                }
            }
            catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(value => console.log(value), error => console.error(error));
    chan.close();
};
export const pallet_bridge_destroy_settings = async (api, keyspair, key) => {
    const unsub = await api.tx.bridge.destroy_settings(key)
        .signAndSend(keyspair, ({ status, events, dispatchError }) => {
            try {
                // status would still be set, but in the case of error we can shortcut
                // to just check it (so an error would indicate InBlock or Finalized)
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { name, section } = decoded;
                        console.log(`${section}.${name}`);
                    }
                    else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(dispatchError.toString());
                    }
                }
            }
            catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
};

export const pallet_bridge_request = async (api, keyspair, token, destination, amount) => {
    const txhash = await api.tx.bridge.request(token, destination, amount)
        .signAndSend(keyspair, { nonce: -1 });
    return txhash;
};

export const pallet_bridge_mint = async (api, keyspair, token, recipient, transaction_id, amount) => {
    const txhash = await api.tx.bridge.mint(token, recipient, transaction_id, amount)
        .signAndSend(keyspair, { nonce: -1 });
    return txhash;
};


export const pallet_bridge_burn = async (api, keyspair, token, recipient, transaction_id, amount) => {
    const txhash = await api.tx.bridge.burn(token, recipient, transaction_id, amount)
        .signAndSend(keyspair, { nonce: -1 });
    return txhash;
};

export const pallet_bridge_set_lockdown = async (api, keyspair, token) => {
    const txhash = await api.tx.bridge.set_lockdown(token)
        .signAndSend(keyspair, { nonce: -1 });
    return txhash;
};

export const pallet_bridge_set_unlockdown = async (api, keyspair) => {
    return await api.tx.bridge.set_unlockdown().signAndSend(keyspair, { nonce: -1 });
};

export const subscription_pallet_bridge = async (api) => {
    // Subscribe to system events via storage
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);
        // Loop through the Vec<EventRecord>
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;
            // Show what we are busy with
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\tmeta:\t${event.meta}`);
            console.log(`\tsection:\t${event.section}`);
            console.log(`\tmethod:\t${event.method}`);
            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data, index) => {
                console.log(`\tindex[${index}]:\t\t${types[index].type}: ${data.toString()}`);
                if (types[index].type === 'DispatchResult') {
                    const result = api.createType('DispatchResult', data);
                    if (result.isErr) {
                        const dispatchError = result.asErr;
                        if (dispatchError.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(dispatchError.asModule);
                            const { name, section } = decoded;
                            console.log(`error: \t${section}.${name}`);
                        }
                        else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            console.log(`other: ${dispatchError.toString()}`);
                        }
                    }
                }
            });
            if (event.section === 'bridge') {
                if (event.method === 'SettingsCreated') {
                    console.log("bridge.SettingsCreated.Action");
                }
                else if (event.method === 'Minted') {
                    console.log("bridge.Minted.Action");
                }
                else if (event.method === 'Burned') {
                    console.log("bridge.Burned.Action");
                }
                else {
                    console.log("bridge.other.Action");
                }
            }
        });
    });
};
export const setup_bridge_test = async (api, keyspair) => {
    console.log("before pallet_assets_force_create");
    await pallet_assets_force_create(api, keyspair, 1, keyspair.address, 1, 1);
    console.log("after pallet_assets_force_create");
    await bridge_create_settings_smoke_test(api, keyspair);
    console.log("after bridge_create_settings_smoke_test");
};
export const setup_basic_bridge_test = async (api, keyspair) => {
    // const keyring = new Keyring({ type: 'sr25519' });
    // const charlie = keyring.addFromUri('//Charlie');
    // const dave = keyring.addFromUri('//Dave');
    // await native_transfer(api, keyspair, 10000 , charlie.address);
    // await native_transfer(api, keyspair, 1, dave.address);
    await pallet_assets_force_create(api, keyspair, 1, keyspair.address, 1, 1);
    await bridge_create_basic_settings_weth_with3(api, keyspair);
    // await bridge_create_settings_smoke_test(api, keyspair);
    // const eve = keyring.addFromUri('//Eve');
    // const recipient = api.createType('AccountId', eve.address);
    // const bitg_token_bytes = api.createType('Bytes', TOKEN);
    // const transaction_id_bytes = api.createType('Bytes', "a123");
    // const balance = api.createType('Balance', "1000");
    // await pallet_bridge_mint(api, charlie, bitg_token_bytes, recipient, transaction_id_bytes, balance);
};
const bridge_create_basic_settings_test = async (api, keyspair) => {
    const bitg_key_bytes = api.createType('Bytes', TOKEN);
    const keyring = new Keyring({ type: 'sr25519' });
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');
    const dave = keyring.addFromUri('//Dave');
    const json_data = {
        chainid: 1,
        description: "xxxxxxxxxx",
        address: keyspair.address,
        assetid: 1,
        internalthreshold: 3,
        externathreshold: 3,
        internalkeepers: [bob.address, charlie.address, dave.address],
        externalkeepers: [bob.address, charlie.address, dave.address],
        internalwatchdogs: [bob.address, charlie.address, dave.address],
        externalwatchdogs: [bob.address, charlie.address, dave.address],
        internalwatchcats: [bob.address, charlie.address, dave.address],
        externalwatchcats: [bob.address, charlie.address, dave.address]
    };
    const json_data_bytes = api.createType('Bytes', JSON.stringify(json_data));
    await pallet_bridge_create_settings(api, keyspair, bitg_key_bytes, json_data_bytes);
};
const bridge_create_basic_settings_weth_with3 = async (api, keyspair) => {
    const bitg_key_bytes = api.createType('Bytes', 'WETH');
    const keyring = new Keyring({ type: 'sr25519' });
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');
    const dave = keyring.addFromUri('//Dave');
    const json_data = {
        chainid: 1,
        description: "WETH",
        address: keyspair.address,
        assetid: 1,
        internalthreshold: 3,
        externathreshold: 3,
        internalkeepers: [bob.address, charlie.address, dave.address],
        externalkeepers: [bob.address, charlie.address, dave.address],
        internalwatchdogs: [bob.address, charlie.address, dave.address],
        externalwatchdogs: [bob.address, charlie.address, dave.address],
        internalwatchcats: [bob.address, charlie.address, dave.address],
        externalwatchcats: [bob.address, charlie.address, dave.address]
    };
    const json_data_bytes = api.createType('Bytes', JSON.stringify(json_data));
    await pallet_bridge_create_settings(api, keyspair, bitg_key_bytes, json_data_bytes);
};
const bridge_create_settings_smoke_test = async (api, keyspair) => {
    const bitg_key_bytes = api.createType('Bytes', TOKEN);
    const json_data = {
        chainid: 1,
        description: "xxxxxxxxxx",
        address: keyspair.address,
        assetid: 1,
        internalthreshold: 1,
        externathreshold: 1,
        internalkeepers: [keyspair.address],
        externalkeepers: [keyspair.address],
        internalwatchdogs: [keyspair.address],
        externalwatchdogs: [keyspair.address],
        internalwatchcats: [keyspair.address],
        externalwatchcats: [keyspair.address]
    };
    const json_data_bytes = api.createType('Bytes', JSON.stringify(json_data));
    await pallet_bridge_create_settings(api, keyspair, bitg_key_bytes, json_data_bytes);
};
const bridge_mint_smoke_test = async (api, keyspair, recipient) => {
    const bitg_token_bytes = api.createType('Bytes', TOKEN);
    const transaction_id_bytes = api.createType('Bytes', "a123");
    const balance = api.createType('Balance', "1");
    await pallet_bridge_mint(api, keyspair, bitg_token_bytes, recipient, transaction_id_bytes, balance);
};
const bridge_burn_smoke_test = async (api, keyspair, recipient) => {
    const bitg_token_bytes = api.createType('Bytes', TOKEN);
    const transaction_id_bytes = api.createType('Bytes', "a123");
    const balance = api.createType('Balance', "1");
    await pallet_bridge_burn(api, keyspair, bitg_token_bytes, recipient, transaction_id_bytes, balance);
};
