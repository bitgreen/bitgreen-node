// Bitgreen API Server

/* import packages */
import express, { Express, Request, Response } from "express";
import cors, { CorsOptions } from "cors";
import { PrismaClient } from "@prisma/client";
import * as dotenv from "dotenv";
import { initApi } from "./methods";
import { BlockHash } from "@polkadot/types/interfaces";

/* config */
dotenv.config();
const port = process.env.API_PORT || 3000;

const prisma = new PrismaClient();

// array of all allowed origins
// TODO: Add list of origins
const allowed_origins = ["*"];
const cors_options: CorsOptions = {
	origin: allowed_origins,
};

// main function
const mainLoop = async () => {
	/* setup app */
	const app: Express = express();

	app.use(express.urlencoded({ extended: true }));
	app.use(express.json());

	app.use(cors());

	app.get("/", function (req: Request, res: Response) {
		res.send("Hello from BitGreen!");
	});

	app.get("/get-block", async (req: Request, res: Response) => {
		let block_number = req.query.block_number as string;

		const api = await initApi();

		const block_hash = (await api.rpc.chain.getBlockHash(block_number)) as BlockHash;

		let [signed_block, block_events] = await Promise.all([
			api.rpc.chain.getBlock(block_hash),
			api.query.system.events.at(block_hash),
		]);

		res.json({
			signed_block: signed_block.toHuman(),
			block_events: block_events.toHuman(),
		});
	});

	// app.get('/analyze-data', db.getAnalyzeData)

	app.get("/vcu/projects", async (req: Request, res: Response) => {
		const { originator = undefined } = req.query;

		const or = originator
			? {
					OR: [{ originator: originator as string }],
			  }
			: {};

		const vcu_projects = await prisma.vcu_projects.findMany({
			where: {
				...or,
			},
			select: {
				id: true,
				asset_id: true,
				originator: true,
				name: true,
				description: true,
				registry_name: true,
				registry_summary: true,
				approved: true,
				total_supply: true,
				minted: true,
				retired: true,
				unit_price: true,

				batches: {
					select: {
						name: true,
						uuid: true,
						issuance_year: true,
						start_date: true,
						end_date: true,
						total_supply: true,
						minted: true,
						retired: true,
					},
				},
				documents: {
					select: {
						url: true,
					},
				},
				images: {
					select: {
						url: true,
					},
				},
				locations: {
					select: {
						latitude: true,
						longitude: true,
					},
				},
				royalties: {
					select: {
						account: true,
						fee_percent: true,
					},
				},
				sdgs: {
					select: {
						type: true,
						description: true,
						references: true,
					},
				},
				videos: {
					select: {
						url: true,
					},
				},
			},
		});

		res.json({
			projects: vcu_projects,
		});
	});

	app.get("/transactions", async (req: Request, res: Response) => {
		const { account, date_start = "2000-01-01", date_end = "2200-01-01" } = req.query;

		const account_query = account
			? {
					OR: [{ sender: account as string }, { recipient: account as string }],
			  }
			: {};

		let transactions;
		try {
			transactions = await prisma.transactions.findMany({
				where: {
					created_at: {
						gte: new Date(date_start as string),
						lte: new Date(date_end as string),
					},
					...account_query,
				},
				select: {
					block_number: true,
					hash: true,
					sender: true,
					recipient: true,
					amount: true,
					created_at: true,
				},
				orderBy: {
					created_at: "desc",
				},
			});
		} catch (e) {
			transactions = null;
		}

		res.json({
			transactions: transactions,
		});
	});

	app.get("/transaction", async (req: Request, res: Response) => {
		const { hash = "" } = req.query;

		const transaction = await prisma.transactions.findUnique({
			where: {
				hash: hash as string,
			},
			select: {
				block_number: true,
				hash: true,
				sender: true,
				recipient: true,
				amount: true,
				created_at: true,
			},
		});

		res.json({
			transaction: transaction,
		});
	});

	app.get("/assets", async (req: Request, res: Response) => {
		const assets = await prisma.assets.findMany();

		res.json({
			assets: assets,
		});
	});

	app.get("/asset", async (req: Request, res: Response) => {
		const { asset_id, project_id } = req.query;

		let asset: any = {};

		if (asset_id) {
			asset = await prisma.assets.findUnique({
				where: {
					id: Number(asset_id),
				},
			});
		} else if (project_id) {
			let project = await prisma.vcu_projects.findUnique({
				where: {
					id: Number(project_id),
				},
				select: {
					asset: true,
				},
			});

			asset = project?.asset;
		}

		res.json({
			asset: asset,
		});
	});

	app.get("/assets/transactions", async (req: Request, res: Response) => {
		let {
			account = undefined,
			date_start = "2000-01-01",
			date_end = "2200-01-01",
			asset_id = undefined,
			project_id = undefined,
		} = req.query;

		const account_query = account
			? {
					OR: [{ sender: account as string }, { recipient: account as string }],
			  }
			: {};

		// filter by project only if asset_id is not present
		if (!asset_id && project_id) {
			let project = await prisma.vcu_projects.findUnique({
				where: {
					id: Number(project_id),
				},
				select: {
					asset: true,
				},
			});

			// @ts-ignore
			asset_id = project?.asset?.id || asset_id;
		}

		const asset_query = asset_id
			? {
					AND: [{ asset_id: Number(asset_id) }],
			  }
			: {};

		let transactions;
		try {
			transactions = await prisma.asset_transactions.findMany({
				where: {
					created_at: {
						gte: new Date(date_start as string),
						lte: new Date(date_end as string),
					},
					...account_query,
					...asset_query,
				},
				orderBy: {
					created_at: "desc",
				},
			});
		} catch (e) {
			transactions = null;
		}

		res.json({
			transactions: transactions,
		});
	});

	app.get("/assets/transaction", async (req: Request, res: Response) => {
		const { hash = "" } = req.query;

		const transaction = await prisma.asset_transactions.findUnique({
			where: {
				hash: hash as string,
			},
		});

		res.json({
			transaction: transaction,
		});
	});

	// app.get('/impact_actions', db.getImpactActions)
	// app.get('/impact_actions/auditors', db.getImpactActionsAuditors)
	// app.get('/impact_actions/categories', db.getImpactActionsCategories)
	// app.get('/impact_actions/oracles', db.getImpactActionsOracles)
	// app.get('/impact_actions/proxies', db.getImpactActionsProxies)
	// app.get('/impact_actions/approval_requests', db.getImpactActionsApprovalRequests)
	// app.get('/impact_actions/approval_request', db.getImpactActionsApprovalRequest)
	// app.get('/impact_actions/approval_requests/auditors', db.getImpactActionsApprovalRequestsAuditors)
	// app.get('/impact_actions/approval_requests/auditors/votes', db.getImpactActionsApprovalRequestsAuditorsVotes)

	/* serve api */
	const server = app.listen(port, function () {
		console.log(`API server is listening at: http://localhost:${port}.`);
	});
};

// run main function
mainLoop().catch(console.error);
