//! BitgreenRococo chain spec
use super::*;
use bitgreen_rococo_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use hex_literal::hex;

pub type RococoChainSpec =
	sc_service::GenericChainSpec<bitgreen_rococo_runtime::GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> bitgreen_rococo_runtime::SessionKeys {
	bitgreen_rococo_runtime::SessionKeys { aura: keys }
}

// same as rococo config but for local testing
pub fn rococo_config_local() -> RococoChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "BBB".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 2106.into());

	RococoChainSpec::from_genesis(
		// Name
		"BitgreenTestnet",
		// ID
		"bitgreen_testnet",
		ChainType::Development,
		move || {
			rococo_genesis(
				// Rootkey
				hex!("dc31445d24993e946ebf9f444dd17a9698fe859eeb574b78910100baab083b75").into(),
				// initial collators.
				generate_collator_keys(&[hex!(
					"66a44eae61bbaa03111e4958c2ad47460de3945a8bc8e236ddc73706e34e8b31"
				)]),
				// initial endowed accounts
				vec![
					hex!("dc31445d24993e946ebf9f444dd17a9698fe859eeb574b78910100baab083b75").into()
				],
				2048.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("bitgreen-rococo-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions { relay_chain: "rococo-local".into(), para_id: 2048 },
	)
}

pub fn rococo_config() -> RococoChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "BBB".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), ROCOCO_PARA_ID.into());

	RococoChainSpec::from_genesis(
		// Name
		"BitgreenRococo",
		// ID
		"bitgreen_rococo",
		ChainType::Live,
		move || {
			rococo_genesis(
				// Rootkey
				hex!("dc31445d24993e946ebf9f444dd17a9698fe859eeb574b78910100baab083b75").into(),
				// initial collators.
				generate_collator_keys(&[
					hex!("66a44eae61bbaa03111e4958c2ad47460de3945a8bc8e236ddc73706e34e8b31"),
					hex!("b60ae0674d01cc80e9f60bdf2f4bfb07daa9bfc4f9958120ead06e6a79208a7b"),
				]),
				// initial endowed accounts
				vec![
					hex!("dc31445d24993e946ebf9f444dd17a9698fe859eeb574b78910100baab083b75").into()
				],
				ROCOCO_PARA_ID.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("bitgreen-rococo"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions { relay_chain: "rococo".into(), para_id: ROCOCO_PARA_ID },
	)
}

fn rococo_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> bitgreen_rococo_runtime::GenesisConfig {
	bitgreen_rococo_runtime::GenesisConfig {
		system: bitgreen_rococo_runtime::SystemConfig {
			code: bitgreen_rococo_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: bitgreen_rococo_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1_000_000_000 * 1000000000000000000))
				.collect(),
		},
		parachain_info: bitgreen_rococo_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		parachain_staking: bitgreen_rococo_runtime::ParachainStakingConfig {
			invulnerables: invulnerables
				.iter()
				.cloned()
				.map(|(acc, _)| pallet_parachain_staking::types::CandidateInfo {
					who: acc,
					deposit: Default::default(),
					delegators: Default::default(),
					total_stake: Default::default(),
				})
				.collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: bitgreen_rococo_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: bitgreen_rococo_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		kyc: bitgreen_rococo_runtime::KYCConfig {
			members: [].to_vec().try_into().unwrap(),
			phantom: Default::default(),
		},
		tokens: bitgreen_rococo_runtime::TokensConfig { balances: [].to_vec() },
		sudo: bitgreen_rococo_runtime::SudoConfig { key: Some(root_key) },
		treasury: Default::default(),
	}
}
