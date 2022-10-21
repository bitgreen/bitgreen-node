//! BitgreenRococo chain spec
use super::*;

pub type RococoChainSpec =
	sc_service::GenericChainSpec<bitgreen_rococo_runtime::GenesisConfig, Extensions>;

pub fn rococo_config() -> RococoChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "BBB".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 2106.into());

	RococoChainSpec::from_genesis(
		// Name
		"BitgreenRococo",
		// ID
		"bitgreen_rococo",
		ChainType::Live,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
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
		Extensions {
			relay_chain: "rococo".into(),
			para_id: ROCOCO_PARA_ID,
		},
	)
}
