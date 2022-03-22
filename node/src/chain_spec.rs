use bitg_runtime::{
	get_all_module_accounts, opaque::SessionKeys, AccountId, AuthorityDiscoveryConfig,
	AuthorityDiscoveryId, BabeConfig, BalancesConfig, BridgeConfig, ContractsConfig, CurrencyId,
	EvmConfig, GenesisConfig, GrandpaConfig, ImOnlineId, IndicesConfig,
	MaxNativeTokenExistentialDeposit, SessionConfig, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TokenSymbol, TokensConfig, BBB, WASM_BINARY,
};
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Bytes, Pair, Public, H160};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::IdentifyAccount;

use sc_chain_spec::ChainSpecExtension;
use sp_std::{collections::btree_map::BTreeMap, str::FromStr};

use serde::{Deserialize, Serialize};

use hex_literal::hex;
use sp_core::{bytes::from_hex, crypto::UncheckedInto};

use bitg_primitives::{AccountPublic, Balance, Nonce};

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.bitgreen.org/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<bitg_primitives::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<bitg_primitives::Block>,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn get_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys {
		babe,
		grandpa,
		im_online,
		authority_discovery,
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an authority keys.
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![get_authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(bitg_properties()),
		// Extensions
		Default::default(),
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					get_authority_keys_from_seed("Alice"),
					get_authority_keys_from_seed("Bob"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		// TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		None,
		// Protocol ID
		Some("bitg_local_testnet"),
		// Properties
		Some(bitg_properties()),
		// Extensions
		Default::default(),
	))
}

pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Bitgreen Testnet",
		// ID
		"bitg_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial authorities keys:
				// stash
				// controller
				// grandpa
				// babe
				// im-online
				// authority-discovery
				vec![
					(
						// Boot Node/Validator 1: ip address 95.217.2.2
						hex!["0a8e3c33501c0001bb08efc7511ca0b81f700eb8010ec91dc2d9324b3e0d2860"]
							.into(),
						hex!["fabe0e7010a320055f706192258876f3e97d237f836c8e1b753175df17d97b40"]
							.into(),
						hex!["a538f6fbd05fdcd42d91600eca9c7e6b7240fa82fc23b569deea91af6c97a5f1"]
							.unchecked_into(),
						hex!["a66bdf2c219a5fc728188edfeed3eb1ef17612d65764a71effcffd7f16a72834"]
							.unchecked_into(),
						hex!["50341e1134ad1a2244caaba520c4fff3a44bcca58c68d2fb9fdf924180c6b67f"]
							.unchecked_into(),
						hex!["7e4e3fd0a36ae866460c5e857682671a3c9f6a511d589084d400f3fe55672439"]
							.unchecked_into(),
					),
					(
						// Boot Node/Validator 2: ip addres 95.217.4.158
						hex!["c866ae8a5b961062016bda70138e00b84a2975c22b6f61d44d2004a4ef9c6116"]
							.into(),
						hex!["ca514d67c95446d8ee62020f22c0b70616b4d6112dfc2c6e5dcafd67e9c27d09"]
							.into(),
						hex!["7ea36efa84a2e3e9416d94f4979aa9a6423d6a6dd2abb475e496cb571ee99dff"]
							.unchecked_into(),
						hex!["44c24fd0fc8cda7ba0c2613be44ef78782ee600eea05d4eb3af0a64f12056e6e"]
							.unchecked_into(),
						hex!["768d7df7f1f8dfbef05c34fa11358a52106a71ada232fa2bd35cb15df86f1b58"]
							.unchecked_into(),
						hex!["a2c666674fbf9a66df9ad15f1fa3bcea315990f7515c3dd2c26dcccc9d4bc27a"]
							.unchecked_into(),
					),
					(
						// Boot Node/Validator 3: ip addres 95.217.1.25
						hex!["42547f143cb05a47a8de112e9b8362dec1d586fa241f29f0e591e17815f1d71a"]
							.into(),
						hex!["8e4ddd28c76a5963b7e81f73495afd663d9ed26ec66dfe8b8b7e3a8b4f01b330"]
							.into(),
						hex!["b034fb357e174f6c7bf7588dba0e191a24ae4f0b63f4ee41e9fcd9b00fcc5ed0"]
							.unchecked_into(),
						hex!["a401838c6a5043cc3353517e7c4396090277b568100806fed7769fadc8baa35b"]
							.unchecked_into(),
						hex!["6acdf720031ddf5d4c7cb06ee7876eef77688956575f5c0480df40b059804a38"]
							.unchecked_into(),
						hex!["bc2efdfdccb4c371eebd9fc1d9d86a6a2eaf516878a5989b0ddcd4f09101f606"]
							.unchecked_into(),
					),
				],
				// Sudo
				hex!["caca29e30cce36df2263139cb15dd0e3ecbeb053d37aeac7ac8d2291ff7afa5c"].into(),
				// Endowed accounts
				vec![
					hex!["caca29e30cce36df2263139cb15dd0e3ecbeb053d37aeac7ac8d2291ff7afa5c"].into(),
					hex!["4e5758a750dad72ea2e777a16222765b5b713c9eeda2eb8cd3f3b989203c5008"].into(),
					hex!["ccf2f08394dde73dd5bd1946552e2042d1f502cb50eb7c29061873f5831caf42"].into(),
				],
			)
		},
		// Bootnodes
		vec![
			"/ip4/95.217.2.2/tcp/30333/p2p/12D3KooWQrG8VAfYs8nXv95XJDu9Yo1iKKQ2KZRJkBVtfxpLvoYe"
				.parse()
				.unwrap(),
			"/ip4/95.217.4.158/tcp/30333/p2p/12D3KooWCAdsB5VvsQ7eCWPZZ4eCdYbriFVwEJnvE5B9YAmRNTuY"
				.parse()
				.unwrap(),
			"/ip4/95.217.1.25/tcp/30333/p2p/12D3KooWCRjP5nofPVvE5d7yj7wktKBPKwWxzc4oNd78z63nNvJC"
				.parse()
				.unwrap(),
		],
		// Telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// Protocol ID
		Some("bitg_testnet"),
		// Properties
		Some(bitg_properties()),
		// Extensions
		Default::default(),
	))
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Bitgreen Mainnet",
		// ID
		"bitg_mainnet",
		ChainType::Live,
		move || mainnet_genesis(
			wasm_binary,
			// Initial authorities keys:
			// stash
			// controller
			// grandpa
			// babe
			// im-online
			// authority-discovery
			vec![
				(
					hex!["6c08c1f8e0cf1e200b24b43fca4c4e407b963b6b1e459d1aeff80c566a1da469"].into(),
					hex!["864eff3160ff8609c030316867630850a9d6e35c47d3efe54de44264fef7665e"].into(),
					hex!["dc41d9325da71d90806d727b826d125cd523da28eb39ab048ab983d7bb74fb32"].unchecked_into(),
					hex!["8a688a748fd39bedaa507c942600c40478c2082dee17b8263613fc3c086b0c53"].unchecked_into(),
					hex!["3a4e80c48718f72326b49c4ae80199d35285643751e75a743f30b7561b538676"].unchecked_into(),
					hex!["68d39d0d386ed4e9dd7e280d62e7dc9cf61dc508ef25efb74b6d740fa4dde463"].unchecked_into(),
				),
				(
					hex!["5c22097b5c8b5912ce28b72ba4de52c3da8aca9379c748c1356a6642107d4c4a"].into(),
					hex!["543fd4fd9a284c0f955bb083ae6e0fe7a584eb6f6e72b386071a250b94f99a59"].into(),
					hex!["f15a651be0ea0afcfe691a118ee7acfa114d11a27cf10991ee91ea97942d2135"].unchecked_into(),
					hex!["70e74bed02b733e47bc044da80418fd287bb2b7a0c032bd211d7956c68c9561b"].unchecked_into(),
					hex!["724cefffeaa10a44935a973511b9427a8c3c4fb08582afc4af8bf110fe4aac4b"].unchecked_into(),
					hex!["a068435c438ddc61b1b656e3f61c876e109706383cf4e27309cc1e308f88b86f"].unchecked_into(),
				),
				(
					hex!["a67f388c1b8d68287fb3288b5aa36f069875c15ebcb9b1e4e62678aad6b24b44"].into(),
					hex!["ec912201d98911842b1a8e82983f71f2116dd8b898798ece4e1d210590de7d60"].into(),
					hex!["347f5342875b9847ec089ca723c1c09cc532e53dca4b940a6138040025d94eb9"].unchecked_into(),
					hex!["64841d2d124e1b1dd5485a58908ab244b296b184ae645a0c103adcbcc565f070"].unchecked_into(),
					hex!["50a3452ca93800a8b660d624521c240e5cb20a47a33d23174bb7681811950646"].unchecked_into(),
					hex!["7a0caeb50fbcd657b8388adfaeca41a2ae3e85b8916a2ce92761ce1a4db89035"].unchecked_into(),
				),
			],
			// Sudo
			hex!["9c48c0498bdf1d716f4544fc21f050963409f2db8154ba21e5233001202cbf08"].into(),
			// Endowed accounts
			vec![
				// Investors
				(hex!["3c483acc759b79f8b12fa177e4bdfa0448a6ea03c389cf4db2b4325f0fc8f84a"].into(), 4_340_893_656 as u128),
				// Liquidity bridge reserves
				(hex!["5adebb35eb317412b58672db0434e4b112fcd27abaf28039f07c0db155b26650"].into(), 2_000_000_000 as u128),
				// Lockup & core nominators
				(hex!["746db342d3981b230804d1a187245e565f8eb3a2897f83d0d841cc52282e324c"].into(), 500_000_000 as u128),
				(hex!["da512d1335a62ad6f79baecfe87578c5d829113dc85dbb984d90a83f50680145"].into(), 500_000_000 as u128),
				(hex!["b493eacad9ca9d7d8dc21b940966b4db65dfbe01084f73c1eee2793b1b0a1504"].into(), 500_000_000 as u128),
				(hex!["849cf6f8a093c28fd0f699b47383767b0618f06aad9df61c4a9aff4af5809841"].into(), 250_000_000 as u128),
				(hex!["863bd6a38c7beb526be033068ac625536cd5d8a83cd51c1577a1779fab41655c"].into(), 250_000_000 as u128),
				(hex!["c2d2d7784e9272ef1785f92630dbce167a280149b22f2ae3b0262435e478884d"].into(), 250_000_000 as u128),
				// Sudo
				(hex!["9c48c0498bdf1d716f4544fc21f050963409f2db8154ba21e5233001202cbf08"].into(), 100_000_000 as u128),
				// Developer pool & faucet
				(hex!["1acc4a5c6361770eac4da9be1c37ac37ea91a55f57121c03240ceabf0b7c1c5e"].into(), 10_000_000 as u128),
			],
		),
		// Bootnodes
		vec![
			"/dns/bootnode-1.bitgreen.org/tcp/30333/p2p/12D3KooWFHSc9cUcyNtavUkLg4VBAeBnYNgy713BnovUa9WNY5pp".parse().unwrap(),
			"/dns/bootnode-2.bitgreen.org/tcp/30333/p2p/12D3KooWAQqcXvcvt4eVEgogpDLAdGWgR5bY1drew44We6FfJAYq".parse().unwrap(),
			"/dns/bootnode-3.bitgreen.org/tcp/30333/p2p/12D3KooWCT7rnUmEK7anTp7svwr4GTs6k3XXnSjmgTcNvdzWzgWU".parse().unwrap(),
		],
		// Telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// Protocol ID
		Some("bitg_mainnet"),
		// Properties
		Some(bitg_properties()),
		// Extensions
		Default::default(),
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	let evm_genesis_accounts = evm_genesis();

	const INITIAL_BALANCE: u128 = 100_000_000 * BBB;
	const INITIAL_STAKING: u128 = 1_000_000 * BBB;
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();
	let enable_println = true;
	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), INITIAL_STAKING))
		.chain(
			endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, INITIAL_BALANCE)),
		)
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.collect::<Vec<(AccountId, Balance)>>();

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(), // stash
						x.0.clone(), // stash
						get_session_keys(
							x.2.clone(), // grandpa
							x.3.clone(), // babe
							x.4.clone(), // im-online
							x.5.clone(), // authority-discovery
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						INITIAL_STAKING,
						StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: sp_runtime::Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Default::default(),
		orml_tokens: Some(TokensConfig {
			endowed_accounts: endowed_accounts
				.iter()
				.flat_map(|x| {
					vec![(
						x.clone(),
						CurrencyId::Token(TokenSymbol::USDG),
						INITIAL_BALANCE,
					)]
				})
				.collect(),
		}),
		module_evm: Some(EvmConfig {
			accounts: evm_genesis_accounts,
		}),
		pallet_sudo: Some(SudoConfig { key: root_key }),
		pallet_collective_Instance1: Some(Default::default()),
		// Nft pallet
		orml_nft: Default::default(),
		// Smart contracts !Ink Language
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println,
				..Default::default()
			},
		}),
		pallet_bridge: Some(BridgeConfig {
			lockdown_status: false,
		}),
	}
}

fn mainnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
) -> GenesisConfig {
	let evm_genesis_accounts = evm_genesis();

	const INITIAL_STAKING: u128 = 1_000_000 * BBB;
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();
	let enable_println = true;
	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), INITIAL_STAKING * 2))
		.chain(
			endowed_accounts
				.iter()
				.cloned()
				.map(|x| (x.0.clone(), x.1 * BBB)),
		)
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.collect::<Vec<(AccountId, Balance)>>();

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(), // stash
						x.0.clone(), // stash
						get_session_keys(
							x.2.clone(), // grandpa
							x.3.clone(), // babe
							x.4.clone(), // im-online
							x.5.clone(), // authority-discovery
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						INITIAL_STAKING,
						StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: sp_runtime::Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Default::default(),
		orml_tokens: Some(TokensConfig {
			endowed_accounts: vec![],
		}),
		module_evm: Some(EvmConfig {
			accounts: evm_genesis_accounts,
		}),
		pallet_sudo: Some(SudoConfig { key: root_key }),
		pallet_collective_Instance1: Some(Default::default()),
		// Nft pallet
		orml_nft: Default::default(),
		// Smart contracts !Ink Language
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println,
				..Default::default()
			},
		}),
		pallet_bridge: Some(BridgeConfig {
			lockdown_status: false,
		}),
	}
}

/// Token
pub fn bitg_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("ss58format".into(), 42.into());
	p.insert("tokenDecimals".into(), 18.into());
	p.insert("tokenSymbol".into(), "BBB".into());
	p
}

/// Predeployed contract addresses
pub fn evm_genesis() -> BTreeMap<H160, module_evm::GenesisAccount<Balance, Nonce>> {
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();
	let contracts_json = &include_bytes!("../../assets/bytecodes.json")[..];
	let contracts: Vec<(String, String, String)> = serde_json::from_slice(contracts_json).unwrap();
	let mut accounts = BTreeMap::new();
	for (_, address, code_string) in contracts {
		let account = module_evm::GenesisAccount {
			nonce: 0,
			balance: existential_deposit,
			storage: Default::default(),
			code: Bytes::from_str(&code_string).unwrap().0,
		};
		let addr = H160::from_slice(
			from_hex(address.as_str())
				.expect("predeploy-contracts must specify address")
				.as_slice(),
		);
		accounts.insert(addr, account);
	}
	accounts
}
