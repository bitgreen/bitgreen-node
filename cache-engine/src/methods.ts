import "dotenv/config";
import { PrismaClient } from "@prisma/client";
import { WsProvider, ApiPromise } from "@polkadot/api";
import { ProviderInterface } from "@polkadot/rpc-provider/types";
import { ApiOptions } from "@polkadot/api/types";
import {
	BlockNumber,
	BlockHash,
	SignedBlock,
	Extrinsic,
	EventRecord,
} from "@polkadot/types/interfaces";
import { hexToString, u8aToString } from "@polkadot/util";

const types = require("../assets/types.json");
const rpc = require("../assets/rpc.json");

const prisma = new PrismaClient();

type ProjectDetails = {
	originator: string;
	name: string;
	description: string;
	registryDetails: RegistryDetails;
	totalSupply: number;
	unitPrice: number;
	minted: number;
	retired: number;
	batches: ProjectDetailsBatch[] | undefined;
	documents: string[] | undefined;
	images: string[] | undefined;
	location: [] | undefined;
	royalties: ProjectDetailsRoyalty[] | undefined;
	sdgDetails: ProjectDetailsSdg[] | undefined;
	videos: string[] | undefined;
};
type RegistryDetails = {
	name: string;
	id: string;
	summary: string;
};
type ProjectDetailsBatch = {
	name: string;
	uuid: string;
	issuanceYear: number;
	startDate: string;
	endDate: string;
	totalSupply: number;
	minted: number;
	retired: number;
};
type ProjectDetailsRoyalty = {
	accountId: string;
	percentOfFees: number;
};
type ProjectDetailsSdg = {
	sdgType: string;
	description: string;
	references: string;
};

export async function initApi() {
	// Initialise the provider to connect to the local node
	const provider = new WsProvider(process.env.RPC_PROVIDER) as ProviderInterface;

	// Create the API and wait until ready
	const api = await ApiPromise.create({
		provider: provider,
		types: types,
		rpc: rpc,
	} as ApiOptions);
	await api.isReady;

	return api;
}

export async function processBlock(api: ApiPromise, block_number: BlockNumber | number) {
	const block_hash = (await api.rpc.chain.getBlockHash(block_number)) as BlockHash;

	let signed_block: SignedBlock, block_events: any;

	console.log(`Chain is at block: #${block_number}`);
	// console.log('Block Hash: ' + block_hash.toHex());

	try {
		[signed_block, block_events] = await Promise.all([
			api.rpc.chain.getBlock(block_hash),
			api.query.system.events.at(block_hash),
		]);
	} catch (error) {
		return false;
	}

	const current_time: string | number | Date = Number(
		signed_block.block.extrinsics.at(0)?.args.at(0)?.toString()
	);

	// parse block
	signed_block.block.extrinsics.map(async (ex: Extrinsic, index: number) => {
		const method = ex.method.method.toString();
		const section = ex.method.section.toString();
		const is_signed = ex.isSigned;
		const hash = ex.hash.toString();
		let extrinsic_success = false,
			new_asset_id: number | undefined;

		let signed_by_address: string | undefined;
		if (is_signed) {
			signed_by_address = ex.signer.toString();
		}

		// console.log(`${section}:${method}`);

		// (await api.rpc.eth.getTransactionReceipt(hash)).toJSON()

		// Check if extrinsic was a success first
		block_events
			.filter(
				({ phase }: EventRecord) =>
					phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index)
			)
			.map(async ({ event }: EventRecord) => {
				extrinsic_success = !!api.events.system.ExtrinsicSuccess.is(event);

				// extract asset_id and assign it to a project
				if (
					section === "vcu" &&
					method === "mint" &&
					event.section === "assets" &&
					event.method === "ForceCreated"
				) {
					event.data.map(async (arg: any, d: number) => {
						if (d === 0) {
							new_asset_id = arg.toNumber();
						}
					});
				}
			});

		// Start processing extrinsic and it's data
		block_events
			.filter(
				({ phase }: EventRecord) =>
					phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index)
			)
			.map(async ({ event }: EventRecord) => {
				if (extrinsic_success) {
					let event_section = event.section;
					let event_method = event.method;
					// console.log(`${event_section}:${event_method}`)
					// console.log('Transaction Hash: ' + hash);

					if (event_section === "vcu") {
						if (event_method === "ProjectCreated") {
							let project_id: number | undefined, details: ProjectDetails | undefined;

							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									project_id = arg.toNumber();
								} else if (d === 1) {
									details = arg.toJSON();
								}
							});

							const batches: object[] = [];
							if (details && details.batches) {
								details.batches.map(async (batch: ProjectDetailsBatch) => {
									batches.push({
										name: hexToString(batch.name),
										uuid: hexToString(batch.uuid),
										issuance_year: batch.issuanceYear,
										start_date: String(batch.startDate),
										end_date: String(batch.endDate),
										total_supply: batch.totalSupply,
										minted: batch.minted,
										retired: batch.retired,
									});
								});
							}

							const documents: object[] = [];
							if (details && details.documents) {
								details.documents.map(async (document: string) => {
									documents.push({
										url: hexToString(document),
									});
								});
							}

							const images: object[] = [];
							if (details && details.images) {
								details.images.map(async (image: string) => {
									images.push({
										url: hexToString(image),
									});
								});
							}

							const locations: object[] = [];
							if (details && details.location) {
								details.location.map(async (location) => {
									locations.push({
										latitude: location[0],
										longitude: location[1],
									});
								});
							}

							const royalties: any[] = [];
							if (details && details.royalties) {
								details.royalties.map(async (royalty: ProjectDetailsRoyalty) => {
									royalties.push({
										account: royalty.accountId,
										fee_percent: royalty.percentOfFees,
									});
								});
							}

							const sdgs: object[] = [];
							if (details && details.sdgDetails) {
								details.sdgDetails.map(async (sdg: ProjectDetailsSdg) => {
									sdgs.push({
										type: sdg.sdgType,
										description: hexToString(sdg.description),
										references: hexToString(sdg.references),
									});
								});
							}

							const videos: object[] = [];
							if (details && details.videos) {
								details.videos.map(async (video: string) => {
									videos.push({
										url: hexToString(video),
									});
								});
							}

							try {
								if (project_id && details) {
									await prisma.vcu_projects.create({
										data: {
											id: project_id,
											block_number: block_number as number,
											hash: hash,
											originator: details.originator,
											name: hexToString(details.name),
											description: hexToString(details.description),
											registry_name: hexToString(
												details.registryDetails.name
											),
											registry_id: hexToString(details.registryDetails.id),
											registry_summary: hexToString(
												details.registryDetails.summary
											),

											total_supply: details.totalSupply,
											minted: details.minted,
											retired: details.retired,
											unit_price: details.unitPrice,

											batches: {
												create: batches,
											},
											documents: {
												create: documents,
											},
											images: {
												create: images,
											},
											locations: {
												create: locations,
											},
											royalties: {
												create: royalties,
											},
											sdgs: {
												create: sdgs,
											},
											videos: {
												create: videos,
											},

											created_at: new Date(current_time).toISOString(),
										},
									});
								}
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}

						if (event_method === "ProjectApproved") {
							let project_id,
								is_approved: boolean = false;

							ex.args.map(async (arg: any, d: number) => {
								if (d === 0) {
									project_id = arg.toNumber();
								} else if (d === 1) {
									is_approved = arg.toString() === "true";
								}
							});

							try {
								await prisma.vcu_projects.update({
									where: {
										id: project_id,
									},
									data: {
										approved: is_approved,
										updated_at: new Date(current_time).toISOString(),
									},
								});
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}

						if (event_method === "VCUMinted") {
							// console.log(api.events.vcu.VCUMinted.is(event))
							let project_id, account, amount;

							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									project_id = arg.toNumber();
								} else if (d === 1) {
									account = arg.toString();
								} else if (d === 2) {
									amount = arg.toNumber();
								}
							});

							if (new_asset_id) {
								// connect asset id with vcu project
								try {
									await prisma.vcu_projects.update({
										where: {
											id: project_id,
										},
										data: {
											asset_id: new_asset_id,
											updated_at: new Date(current_time).toISOString(),
										},
									});
								} catch (e) {
									// @ts-ignore
									console.log(`Error occurred: ${e.message}`);
								}
							}
						}
					}

					if (event_section == "assets") {
						if (event_method == "ForceCreated") {
							let asset_id: number | undefined;
							let owner: string | undefined;
							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									asset_id = arg.toNumber();
								} else if (d === 1) {
									owner = arg.toString();
								}
							});

							try {
								await prisma.assets.create({
									data: {
										id: asset_id as number,
										block_number: block_number as number,
										hash: hash as string,
										owner: owner as string,
										created_at: new Date(current_time).toISOString(),
									},
								});
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}

						if (event_method == "MetadataSet") {
							let asset_id: number | undefined;
							let name: string | undefined;
							let symbol: string | undefined;
							let decimals: number | undefined;
							let is_frozen: boolean | undefined;
							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									asset_id = arg.toNumber();
								} else if (d === 1) {
									name = arg.toString();
								} else if (d === 2) {
									symbol = arg.toString();
								} else if (d === 3) {
									decimals = arg.toNumber();
								} else if (d === 4) {
									is_frozen = arg.toString() === "true";
								}
							});

							// wait half a second, prisma throws record not found if we do it too fast.
							// investigate why await is not doing good job, maybe try/catch?
							await new Promise((resolve) => setTimeout(resolve, 500));

							try {
								await prisma.assets.update({
									where: {
										id: asset_id,
									},
									data: {
										name: hexToString(name),
										symbol: hexToString(symbol),
										decimals: decimals as number,
										is_frozen: is_frozen as boolean,
										updated_at: new Date(current_time).toISOString(),
									},
								});
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}

						if (event_method == "Transferred") {
							let asset_id: number | undefined;
							let sender: string | undefined;
							let recipient: string | undefined;
							let amount: number | undefined;

							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									asset_id = arg.toNumber();
								} else if (d === 1) {
									sender = arg.toString();
								} else if (d === 2) {
									recipient = arg.toString();
								} else if (d === 3) {
									amount = arg.toNumber();
								}
							});

							try {
								await prisma.asset_transactions.create({
									data: {
										block_number: block_number as number,
										hash: hash,
										sender: sender as string,
										recipient: recipient as string,
										amount: amount as number,
										asset_id: asset_id as number,
										created_at: new Date(current_time).toISOString(),
									},
								});
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}
					}

					if (
						section === "balances" &&
						(method === "transferKeepAlive" || method === "transfer")
					) {
						if (event_section === "balances" && event_method === "Transfer") {
							let sender: string | undefined;
							let recipient: string | undefined;
							let amount: number | undefined;

							// TODO: check why api.rpc.eth is empty
							// console.log(api.rpc.eth.getTransactionReceipt(hash));

							event.data.map(async (arg: any, d: number) => {
								if (d === 0) {
									sender = arg.toString();
								} else if (d === 1) {
									recipient = arg.toString();
								} else if (d === 2) {
									amount = arg.toString();
								}
							});

							try {
								await prisma.transactions.create({
									data: {
										block_number: block_number as number,
										hash: hash as string,
										recipient: recipient as string,
										sender: sender as string,
										amount: amount as number,
										created_at: new Date(current_time).toISOString(),
									},
								});
							} catch (e) {
								// @ts-ignore
								console.log(`Error occurred: ${e.message}`);
							}
						}
					}
				}
			});
	});

	// store block in db
	try {
		await prisma.blocks.create({
			data: {
				number: block_number as number,
				hash: block_hash.toHex() as string,
				created_at: new Date(current_time).toISOString(),
			},
		});
	} catch (e) {
		// @ts-ignore
		console.log(`Error occurred: ${e.message}`);
	}

	console.log("-----------------------------------------------------");

	return true;
}
