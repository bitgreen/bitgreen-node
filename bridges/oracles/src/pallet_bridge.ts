import { ApiPromise, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Channel } from 'async-channel';
import { readFileSync } from 'fs';
import { dirname, join, normalize, format } from 'path';
import { fileURLToPath } from 'url';
import type { AccountId } from '@polkadot/types/interfaces';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const node_address = 'ws://127.0.0.1:9944';

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
}

export const setup_substrate = async () => {
    const wsProvider = new WsProvider(node_address);
    const custom_types = await get_custom_types();
    const api = await ApiPromise.create({
        provider: wsProvider, "types": custom_types
    });
    return api;
}

export const pallet_assets_create = async (api: ApiPromise, keyspair: KeyringPair, id: any, admin: any, maxZombies: any, minBalance: any) => {
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(dispatchError.toString());
                    }
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
}

export const pallet_assets_force_create = async (api: ApiPromise, keyspair: KeyringPair, id: any, admin: any, maxZombies: any, minBalance: any) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.sudo
        .sudo(
            api.tx.assets.forceCreate(id, admin, maxZombies, minBalance)
        )
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const pallet_bridge_create_settings = async (api: ApiPromise, keyspair: KeyringPair, key: any, data: any) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.sudo
        .sudo(
            api.tx.bridge.createSettings(key, data)
        )
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const pallet_bridge_destroy_settings = async (api: ApiPromise, keyspair: KeyringPair, key: any) => {
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(dispatchError.toString());
                    }
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
}

export const pallet_bridge_mint = async (api: ApiPromise, keyspair: KeyringPair, token: any, recipient: any, transaction_id: any, amount: any) => {
    // console.log(api.tx.bridge.mint);
    // console.log(token);
    // console.log(recipient);
    // console.log(transaction_id);
    // console.log(amount);
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.bridge.mint(token, recipient, transaction_id, amount)
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const pallet_bridge_burn = async (api: ApiPromise, keyspair: KeyringPair, token: any, recipient: any, transaction_id: any, amount: any) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.bridge.burn(token, recipient, transaction_id, amount)
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const pallet_bridge_set_lockdown = async (api: ApiPromise, keyspair: KeyringPair, token: any) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.bridge.set_lockdown(token)
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const pallet_bridge_set_unlockdown = async (api: ApiPromise, keyspair: KeyringPair) => {
    const chan = new Channel(0 /* default */);
    const unsub = await api.tx.bridge.set_unlockdown()
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
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(`other: ${dispatchError.toString()}`);
                    }
                    unsub();
                    chan.push("error");
                }

                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

                    // Loop through Vec<EventRecord> to display all events
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                    });
                    unsub();
                    chan.push(`Event report end.`);
                }
            } catch {
                console.log("[Error] Too many transactions in short time");
            }
        });
    await chan.get().then(
        value => console.log(value),
        error => console.error(error)
    );
    chan.close();
}

export const subscription_pallet_bridge = async (api: ApiPromise) => {
    // Subscribe to system events via storage
    api.query.system.events((events: any[]) => {
        console.log(`\nReceived ${events.length} events:`);

        // Loop through the Vec<EventRecord>
        events.forEach((record: { event: any; phase: any; }) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // Show what we are busy with
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\tmeta:\t${event.meta}`);
            console.log(`\tsection:\t${event.section}`);
            console.log(`\tmethod:\t${event.method}`);

            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data: any, index: string | number) => {
                console.log(`\tindex[${index}]:\t\t${types[index].type}: ${data.toString()}`);
                if (types[index].type === 'DispatchResult') {
                    const result = api.createType('DispatchResult', data)
                    if (result.isErr) {
                        const dispatchError = result.asErr;
                        if (dispatchError.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(dispatchError.asModule);
                            const { name, section } = decoded;
                            console.log(`error: \t${section}.${name}`);
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            console.log(`other: ${dispatchError.toString()}`);
                        }
                    }
                }
            });

            if (event.section === 'bridge') {
                if (event.method === 'SettingsCreated') {
                    console.log("bridge.SettingsCreated.Action");
                } else
                    if (event.method === 'Minted') {
                        console.log("bridge.Minted.Action");
                    } else
                        if (event.method === 'Burned') {
                            console.log("bridge.Burned.Action");
                        } else {
                            console.log("bridge.other.Action");
                        }
            }

        });
    });

}

const setup_bridge_test = async (api: ApiPromise, keyspair: KeyringPair) => {
    console.log("before pallet_assets_force_create");
    await pallet_assets_force_create(api, keyspair, 1, keyspair.address, 1, 1);
    console.log("after pallet_assets_force_create");
    await bridge_create_settings_smoke_test(api, keyspair);
    console.log("after bridge_create_settings_smoke_test");
}

const bridge_create_settings_smoke_test = async (api: ApiPromise, keyspair: KeyringPair) => {
    const bitg_key_bytes = api.createType('Bytes', "BITG");
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
}

const bridge_mint_smoke_test = async (api: ApiPromise, keyspair: KeyringPair, recipient: AccountId) => {
    const bitg_token_bytes = api.createType('Bytes', "BITG");
    const transaction_id_bytes = api.createType('Bytes', "a123");
    const balance = api.createType('Balance', "1");

    await pallet_bridge_mint(api, keyspair, bitg_token_bytes, recipient, transaction_id_bytes, balance);
}

const bridge_burn_smoke_test = async (api: ApiPromise, keyspair: KeyringPair, recipient: AccountId) => {
    const bitg_token_bytes = api.createType('Bytes', "BITG");
    const transaction_id_bytes = api.createType('Bytes', "a123");
    const balance = api.createType('Balance', "1");

    await pallet_bridge_burn(api, keyspair, bitg_token_bytes, recipient, transaction_id_bytes, balance);
}

