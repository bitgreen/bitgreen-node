// Bitgreen block fetcher
// This program will process all blocks from X to Y
// and store them in a local Postgresql database.

const { program } = require('commander');
const { initApi, processBlock } = require("./src/methods")

program
    .description('Bitgreen crawler to fetch custom blocks.')
    .version('1.0.0', '-v, --version')
    .usage('[OPTIONS]...')
    .option('-bs, --block-start <value>', 'starting block number', '1')
    .option('-be, --block-end <value>', 'ending block number - stop fetching at this block', false)
    .option('-a, --analyze-only', 'analyze only, dont crawl all data', false)
    .parse(process.argv);

program.parse()

const options = program.opts();

async function main() {
    const api = await initApi()

    const block_start = !isNaN(options.blockStart) ? parseInt(options.blockStart) : 1;
    const block_end = !isNaN(options.blockEnd) && parseInt(options.blockEnd) >= block_start ? parseInt(options.blockEnd) : 99999999999999;
    const analyze_only = options.analyzeOnly;

    console.log(`Blocks to fetch: ${block_start} to ${block_end}`);

    for(let block_number = block_start; block_number <= block_end; block_number++) {
        await processBlock(api, block_number, analyze_only)
    }
}

main().catch(console.error);