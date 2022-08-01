// Bitgreen block fetcher
// This program will process all blocks from X to Y
// and store them in a local Postgresql database.

import { Command } from "commander";
import { processBlock, initApi } from "./methods";

const program = new Command();

program
	.description("Bitgreen crawler to fetch custom blocks.")
	.version("1.0.0", "-v, --version")
	.usage("[OPTIONS]...")
	.option("-bs, --block-start <value>", "starting block number", "1")
	.option("-be, --block-end <value>", "ending block number - stop fetching at this block", false)
	.option("-a, --analyze-only", "analyze only, dont crawl all data", false)
	.parse(process.argv);

program.parse();

const options = program.opts();

async function main() {
	const api = await initApi();

	const block_start = !isNaN(options.blockStart) ? parseInt(options.blockStart) : 1;
	const block_end =
		!isNaN(options.blockEnd) && parseInt(options.blockEnd) >= block_start
			? parseInt(options.blockEnd)
			: 99999999999999;

	console.log(`Blocks to fetch: ${block_start} to ${block_end}`);

	for (let block_number = block_start; block_number <= block_end; block_number++) {
		const block_processed = await processBlock(api, block_number);

		if (!block_processed) {
			// TODO: Subscribe here
			break;
		}
	}
}

main().catch(console.error);
