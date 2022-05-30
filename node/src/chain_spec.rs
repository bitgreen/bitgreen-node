use bitg_node_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    SystemConfig, WASM_BINARY,
};
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.bitgreen.org/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            generate_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
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
        None,
        // Properties
        Some(bitg_properties()),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            generate_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
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
        None,
        // Protocol ID
        None,
        // Properties
        None,
        Some(bitg_properties()),
        // Extensions
        None,
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
            generate_genesis(
                wasm_binary,
                // Initial authorities keys:
                // Aura
                // Grandpa
                vec![
                    (
                        // Boot Node/Validator 1: ip address 95.217.2.2
                        hex!["a538f6fbd05fdcd42d91600eca9c7e6b7240fa82fc23b569deea91af6c97a5f1"]
                            .unchecked_into(),
                        hex!["a66bdf2c219a5fc728188edfeed3eb1ef17612d65764a71effcffd7f16a72834"]
                            .unchecked_into(),
                    ),
                    (
                        // Boot Node/Validator 2: ip addres 95.217.4.158
                        hex!["7ea36efa84a2e3e9416d94f4979aa9a6423d6a6dd2abb475e496cb571ee99dff"]
                            .unchecked_into(),
                        hex!["44c24fd0fc8cda7ba0c2613be44ef78782ee600eea05d4eb3af0a64f12056e6e"]
                            .unchecked_into(),
                    ),
                    (
                        // Boot Node/Validator 3: ip addres 95.217.1.25
                        hex!["b034fb357e174f6c7bf7588dba0e191a24ae4f0b63f4ee41e9fcd9b00fcc5ed0"]
                            .unchecked_into(),
                        hex!["a401838c6a5043cc3353517e7c4396090277b568100806fed7769fadc8baa35b"]
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
        None,
        // Properties
        Some(bitg_properties()),
        // Extensions
        Default::default(),
    ))
}

// pub fn mainnet_config() -> Result<ChainSpec, String> {
// 	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
// 	Ok(ChainSpec::from_genesis(
// 		// Name
// 		"Bitgreen Mainnet",
// 		// ID
// 		"bitg_mainnet",
// 		ChainType::Live,
// 		move || generate_genesis(
// 			wasm_binary,
// 			// Initial authorities keys:
//             // Aura
//             // Grandpa
// 			vec![
// 				(
// 					hex!["dc41d9325da71d90806d727b826d125cd523da28eb39ab048ab983d7bb74fb32"].unchecked_into(),
// 					hex!["8a688a748fd39bedaa507c942600c40478c2082dee17b8263613fc3c086b0c53"].unchecked_into(),
// 				),
// 				(
// 					hex!["f15a651be0ea0afcfe691a118ee7acfa114d11a27cf10991ee91ea97942d2135"].unchecked_into(),
// 					hex!["70e74bed02b733e47bc044da80418fd287bb2b7a0c032bd211d7956c68c9561b"].unchecked_into(),
// 				),
// 				(
// 					hex!["347f5342875b9847ec089ca723c1c09cc532e53dca4b940a6138040025d94eb9"].unchecked_into(),
// 					hex!["64841d2d124e1b1dd5485a58908ab244b296b184ae645a0c103adcbcc565f070"].unchecked_into(),
// 				),
// 			],
// 			// Sudo
// 			hex!["9c48c0498bdf1d716f4544fc21f050963409f2db8154ba21e5233001202cbf08"].into(),
// 			// Endowed accounts
// 			vec![
// 				// Investors
// 				(hex!["3c483acc759b79f8b12fa177e4bdfa0448a6ea03c389cf4db2b4325f0fc8f84a"].into(), 4_340_893_656 as u128),
// 				// Liquidity bridge reserves
// 				(hex!["5adebb35eb317412b58672db0434e4b112fcd27abaf28039f07c0db155b26650"].into(), 2_000_000_000 as u128),
// 				// Lockup & core nominators
// 				(hex!["746db342d3981b230804d1a187245e565f8eb3a2897f83d0d841cc52282e324c"].into(), 500_000_000 as u128),
// 				(hex!["da512d1335a62ad6f79baecfe87578c5d829113dc85dbb984d90a83f50680145"].into(), 500_000_000 as u128),
// 				(hex!["b493eacad9ca9d7d8dc21b940966b4db65dfbe01084f73c1eee2793b1b0a1504"].into(), 500_000_000 as u128),
// 				(hex!["849cf6f8a093c28fd0f699b47383767b0618f06aad9df61c4a9aff4af5809841"].into(), 250_000_000 as u128),
// 				(hex!["863bd6a38c7beb526be033068ac625536cd5d8a83cd51c1577a1779fab41655c"].into(), 250_000_000 as u128),
// 				(hex!["c2d2d7784e9272ef1785f92630dbce167a280149b22f2ae3b0262435e478884d"].into(), 250_000_000 as u128),
// 				// Sudo
// 				(hex!["9c48c0498bdf1d716f4544fc21f050963409f2db8154ba21e5233001202cbf08"].into(), 100_000_000 as u128),
// 				// Developer pool & faucet
// 				(hex!["1acc4a5c6361770eac4da9be1c37ac37ea91a55f57121c03240ceabf0b7c1c5e"].into(), 10_000_000 as u128),
// 			],
// 		),
// 		// Bootnodes
// 		vec![
// 			"/dns/bootnode-1.bitgreen.org/tcp/30333/p2p/12D3KooWFHSc9cUcyNtavUkLg4VBAeBnYNgy713BnovUa9WNY5pp".parse().unwrap(),
// 			"/dns/bootnode-2.bitgreen.org/tcp/30333/p2p/12D3KooWAQqcXvcvt4eVEgogpDLAdGWgR5bY1drew44We6FfJAYq".parse().unwrap(),
// 			"/dns/bootnode-3.bitgreen.org/tcp/30333/p2p/12D3KooWCT7rnUmEK7anTp7svwr4GTs6k3XXnSjmgTcNvdzWzgWU".parse().unwrap(),
// 		],
// 		// Telemetry
// 		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
// 		// Protocol ID
// 		Some("bitg_mainnet"),
//         None,
// 		// Properties
// 		Some(bitg_properties()),
// 		// Extensions
// 		Default::default(),
// 	))
// }

/// Configure initial storage state for FRAME modules.
fn generate_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
        bridge: bitg_node_runtime::BridgeConfig {
            lockdown_status: false,
        },
        tokens: bitg_node_runtime::TokensConfig {
            balances: [].to_vec(),
        },
        nft: bitg_node_runtime::NftConfig { tokens: vec![] },
        vcu: bitg_node_runtime::VCUConfig {
            next_asset_id: 1000_u32.into(),
        },
    }
}

pub fn bitg_properties() -> Properties {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "BBB".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    properties
}
