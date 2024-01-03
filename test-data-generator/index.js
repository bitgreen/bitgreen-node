import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import assert from "assert";

const wait_for_tx = () => new Promise((resolve) => setTimeout(resolve, 10_000));

// The account used to create project, mint and create sell order
// Ensure this account is an authorised account and kyc verified
const SIGNER_SEED = "truth flip carbon state security adapt lion work broken senior royal order";

// The account used to buy the sell order listing
// Ensure this account is kyc verified
const BUYER_SEED = "army empower forward rare pen click host kid ribbon photo duck poem";

// Accounts to validate the payment
// Do not change these values, these are used for test validation
const TEST_VALIDATOR_ONE = "task genius festival hope bind chase shy item connect wife budget pipe";
const TEST_VALIDATOR_TWO =
	"gun zebra clever surprise cement bench swear blanket push dust fame gather";

var PROJECT_ID = 3;
var ASSET_ID = PROJECT_ID;

class BitgreenNode {
	api = null;

	constructor(url = "wss://testnet.bitgreen.org") {
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

	// get asset id for last project
	async get_next_asset_id() {
		const assetId = await this.api.query.carbonCredits.nextAssetId();
		return assetId.toHuman();
	}

	// return dev account for chain
	async get_dev_account() {
		// Constuct the keyring after the API (crypto has an async init)
		const keyring = new Keyring({ type: "sr25519" });
		// Create pair and add Alice to keyring pair dictionary (with account seed)
		const pairSigner = keyring.createFromUri(SIGNER_SEED, { name: "sr25519" });
		return pairSigner;
	}

	// return buyer account for chain
	async get_buyer_account() {
		// Constuct the keyring after the API (crypto has an async init)
		const keyring = new Keyring({ type: "sr25519" });
		// Create pair and add Alice to keyring pair dictionary (with account seed)
		const pairSigner = keyring.createFromUri(BUYER_SEED, { name: "sr25519" });
		return pairSigner;
	}

	// create the project
	async create_project(account) {
		let project_data = {
			name: "Carbon Preservation Project: Sustaining the Earth's Future",
			description:
				"At its core, the project employs a multifaceted approach, combining innovative technologies with sustainable practices",
			location: [(100, 100), (100, 100)],
			images: [
				"https://bitgreen-gateway.infura-ipfs.io/ipfs/bafybeigp2twcwpmtmohtl4y54npwoczg5sypip6ei2t5a4cwxwd6yuwlru",
			],
			videos: ["ipfs/link-to-video"],
			documents: ["ipfs/link-to-document"],
			registryDetails: [
				{
					registry: "Verra",
					name: "ProjectNameInregistry",
					id: "projectIdInRegistry",
					summary: "thisprojectwillsavetheworld",
				},
			],
			sdgDetails: [
				{
					sdgType: "NoPoverty",
					description:
						"No Poverty SDG employs a multi-faceted approach that encompasses social protection programs, sustainable economic growth, and targeted interventions to uplift marginalized communities",
					refrences: "RefrencetoSDG",
				},
			],
			batchGroups: [
				{
					name: "MainBatchGroup",
					uuid: "MainBatchGroup",
					assetId: 200,
					total_supply: 100_000,
					minted: 0,
					retired: 0,
					batches: [
						{
							name: "Batch2020",
							uuid: "Batch2020",
							issuanceYear: 2020,
							total_supply: 50_000,
							startDate: 2020,
							endDate: 2021,
							minted: 0,
							retired: 0,
						},
						{
							name: "Batch2019",
							uuid: "Batch2019",
							issuanceYear: 2019,
							total_supply: 50_000,
							startDate: 2019,
							endDate: 2020,
							minted: 0,
							retired: 0,
						},
					],
				},
			],
			royalties: null,
		};
		const tx = await this.api.tx.carbonCredits.create(project_data);
		const hash = await tx.signAndSend(account);
		console.log("Create Project transaction sent with hash", hash.toHex());
	}

	// get the chain sudo key
	async approve_project(account) {
		const nonce = await this.api.rpc.system.accountNextIndex(account.address);
		const assetId = await this.api.query.carbonCredits.nextAssetId();
		let asset_id = assetId.toHuman();
		console.log("ProjectId is ", asset_id - 1);
		PROJECT_ID = asset_id - 1;
		const tx = await this.api.tx.carbonCredits.approveProject(PROJECT_ID, true);
		const hash = await tx.signAndSend(account, { nonce });
		console.log("Approve transaction sent with hash", hash.toHex());
	}

	// get the chain sudo key
	async mint_tokens(account) {
		const nonce = await this.api.rpc.system.accountNextIndex(account.address);
		const tx = await this.api.tx.carbonCredits.mint(PROJECT_ID, 0, 10_000, false);
		const hash = await tx.signAndSend(account, { nonce });
		console.log("Mint transaction sent with hash", hash.toHex());
	}

	// create sell order
	async create_sell_order(account) {
		const nonce = await this.api.rpc.system.accountNextIndex(account.address);
		const tx = await this.api.tx.dex.createSellOrder(ASSET_ID, 10_000, 1000);
		const hash = await tx.signAndSend(account, { nonce });
		console.log("Create sell order sent with hash", hash.toHex());
	}

	// create buy order
	async create_buy_order(account) {
		const buyOrderId = await this.api.query.dex.orderCount();
		let buy_order = buyOrderId.toHuman() - 1;
		console.log("buy_order is ", buy_order);
		const nonce = await this.api.rpc.system.accountNextIndex(account.address);
		const tx = await this.api.tx.dex.createBuyOrder(buy_order, ASSET_ID, 10, 10_000_000);
		const hash = await tx.signAndSend(account, { nonce });
		console.log("Create buy order sent with hash", hash.toHex());
	}

	// validate payment
	async validate_payment(account) {
		// Constuct the keyring after the API (crypto has an async init)
		const keyring = new Keyring({ type: "sr25519" });
		// Create pair and add Alice to keyring pair dictionary (with account seed)
		const pairSignerOne = keyring.createFromUri(TEST_VALIDATOR_ONE, { name: "sr25519" });
		// Create pair and add Alice to keyring pair dictionary (with account seed)
		const pairSignerTwo = keyring.createFromUri(TEST_VALIDATOR_TWO, { name: "sr25519" });

		const buyOrderId = await this.api.query.dex.buyOrderCount();
		let buy_order = buyOrderId.toHuman() - 1;
		console.log("buy_order is ", buy_order);
		// simulate a stripe payment
		const tx = await this.api.tx.dex.validateBuyOrder(
			buy_order,
			0,
			"0x848f743b40ecfdf1d3ff85fdcebf57f526e924c0ca2e329a4624caab722ca47a"
		);

		// sent from validator one
		const nonce = await this.api.rpc.system.accountNextIndex(pairSignerOne.address);
		const hash = await tx.signAndSend(pairSignerOne, { nonce });
		console.log("Validate buy order sent with hash", hash.toHex());

		// sent from validator two
		const nonce2 = await this.api.rpc.system.accountNextIndex(pairSignerTwo.address);
		const hash2 = await tx.signAndSend(pairSignerTwo, { nonce });
		console.log("Validate buy order sent with hash2", hash2.toHex());
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
	let signer_balance = await chain.query_balance(dev_account.address);
	console.log("Signer address balance is ", signer_balance);
	assert(signer_balance > 10000);

	// create a project
	await chain.create_project(dev_account);
	await wait_for_tx();

	// approve project
	await chain.approve_project(dev_account);
	await wait_for_tx();

	// mint tokens
	await chain.mint_tokens(dev_account);
	await wait_for_tx();

	// list tokens in marketplace
	await chain.create_sell_order(dev_account);
	await wait_for_tx();

	// buy tokens from buyer account
	let buyer_account = await chain.get_buyer_account();
	await chain.create_buy_order(buyer_account);
	await wait_for_tx();

	// validate as paid
	await chain.validate_payment();
}

main().catch(console.error);
