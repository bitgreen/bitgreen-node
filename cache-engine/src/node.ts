// Bitgreen block crawler
// This program will listen for new blocks and store
// them in a local Postgresql database.

// import required dependencies
import "dotenv/config";
import { processBlock, initApi } from "./methods";
import { Header } from "@polkadot/types/interfaces";

// main function (must be async)
async function main() {
	const api = await initApi();

	// Retrieve the chain & node information via rpc calls
	const [chain, nodeName, nodeVersion] = await Promise.all([
		api.rpc.system.chain(),
		api.rpc.system.name(),
		api.rpc.system.version(),
	]);

	// log message to console
	console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);

	// We only display a couple, then unsubscribe
	let count = 0;

	// Subscribe to the new headers on-chain. The callback is fired when new headers
	// are found, the call itself returns a promise with a subscription that can be
	// used to unsubscribe from the newHead subscription
	const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header: Header) => {
		await processBlock(api, header.number.toNumber());

		if (++count === 20) {
			// unsubscribe();
			// process.exit(0);
		}
	});
}

main().catch(console.error);
