// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot chain configurations.

use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use babe_primitives::AuthorityId as BabeId;
use grandpa::AuthorityId as GrandpaId;
use hex_literal::hex;
use kusama::constants::currency::DOTS as KSM;
use kusama_runtime as kusama;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use polkadot::constants::currency::DOTS;
use polkadot_primitives::v1::{AccountId, AccountPublic, ValidatorId};
use polkadot_runtime as polkadot;
use rococo_runtime as rococo;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;
use westend::constants::currency::DOTS as WND;
use westend_runtime as westend;

const POLKADOT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const KUSAMA_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const WESTEND_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ROCOCO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::v1::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v1::Block>,
}

/// The `ChainSpec` parametrised for the polkadot runtime.
pub type PolkadotChainSpec = service::GenericChainSpec<polkadot::GenesisConfig, Extensions>;

/// The `ChainSpec` parametrised for the kusama runtime.
pub type KusamaChainSpec = service::GenericChainSpec<kusama::GenesisConfig, Extensions>;

/// The `ChainSpec` parametrised for the westend runtime.
pub type WestendChainSpec = service::GenericChainSpec<westend::GenesisConfig, Extensions>;

/// The `ChainSpec` parametrized for the rococo runtime.
pub type RococoChainSpec = service::GenericChainSpec<rococo::GenesisConfig, Extensions>;

pub fn polkadot_config() -> Result<PolkadotChainSpec, String> {
	PolkadotChainSpec::from_json_bytes(&include_bytes!("../res/polkadot.json")[..])
}

pub fn kusama_config() -> Result<KusamaChainSpec, String> {
	KusamaChainSpec::from_json_bytes(&include_bytes!("../res/kusama.json")[..])
}

pub fn westend_config() -> Result<PolkadotChainSpec, String> {
	PolkadotChainSpec::from_json_bytes(&include_bytes!("../res/westend.json")[..])
}

pub fn rococo_config() -> Result<PolkadotChainSpec, String> {
	PolkadotChainSpec::from_json_bytes(&include_bytes!("../res/rococo.json")[..])
}

fn polkadot_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	parachain_validator: ValidatorId,
	authority_discovery: AuthorityDiscoveryId,
) -> polkadot::SessionKeys {
	polkadot::SessionKeys {
		babe,
		grandpa,
		im_online,
		parachain_validator,
		authority_discovery,
	}
}

fn kusama_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	parachain_validator: ValidatorId,
	authority_discovery: AuthorityDiscoveryId,
) -> kusama::SessionKeys {
	kusama::SessionKeys {
		babe,
		grandpa,
		im_online,
		parachain_validator,
		authority_discovery,
	}
}

fn westend_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	parachain_validator: ValidatorId,
	authority_discovery: AuthorityDiscoveryId,
) -> westend::SessionKeys {
	westend::SessionKeys {
		babe,
		grandpa,
		im_online,
		parachain_validator,
		authority_discovery,
	}
}

fn rococo_session_keys(
	babe: BabeId,
	_grandpa: GrandpaId,
	im_online: ImOnlineId,
	parachain_validator: ValidatorId,
	_authority_discovery: AuthorityDiscoveryId,
) -> rococo::SessionKeys {
	rococo::SessionKeys {
		babe,
		im_online,
		parachain_validator,
	}
}

fn polkadot_staging_testnet_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)> = vec![];

	const ENDOWMENT: u128 = 1_000_000 * DOTS;
	const STASH: u128 = 100 * DOTS;

	polkadot::GenesisConfig {
		frame_system: Some(polkadot::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(polkadot::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(polkadot::IndicesConfig { indices: vec![] }),
		pallet_session: Some(polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(polkadot::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						polkadot::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_democracy: Some(Default::default()),
		pallet_collective_Instance1: Some(polkadot::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(polkadot::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(polkadot::AuthorityDiscoveryConfig { keys: vec![] }),
		claims: Some(polkadot::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		}),
		pallet_vesting: Some(polkadot::VestingConfig { vesting: vec![] }),
	}
}

fn westend_staging_testnet_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5ENpP27BrVdJTdUfY6djmcw3d3xEJ6NzSUU52CCPmGpMrdEY
		hex!["6648d7f3382690650c681aba1b993cd11e54deb4df21a3a18c3e2177de9f7342"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in parachains; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f
			hex!["9ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d"].into(),
			// 5ExdKyXFhtrjiFhexnyQPDyGSP8xU9qHc4KDwVrtWxaP2RP6
			hex!["8011fb3641f0641f5570ba8787a64a0ff7d9c9999481f333d7207c4abd7e981c"].into(),
			// 5Ef8qY8LRV6RFd4bThrwxBhhWfLjzqmd4rK8nX3Xs7zJqqp7
			hex!["72bae70a1398c0ba52f815cc5dfbc9ec5c013771e541ae28e05d1129243e3001"]
				.unchecked_into(),
			// 5FSscBiPfaPaEhFbAt2qRhcYjryKBKf714X76F5nFfwtdXLa
			hex!["959cebf18fecb305b96fd998c95f850145f52cbbb64b3ef937c0575cc7ebd652"]
				.unchecked_into(),
			// 5Ef8qY8LRV6RFd4bThrwxBhhWfLjzqmd4rK8nX3Xs7zJqqp7
			hex!["72bae70a1398c0ba52f815cc5dfbc9ec5c013771e541ae28e05d1129243e3001"]
				.unchecked_into(),
			// 5Ef8qY8LRV6RFd4bThrwxBhhWfLjzqmd4rK8nX3Xs7zJqqp7
			hex!["72bae70a1398c0ba52f815cc5dfbc9ec5c013771e541ae28e05d1129243e3001"]
				.unchecked_into(),
			// 5Ef8qY8LRV6RFd4bThrwxBhhWfLjzqmd4rK8nX3Xs7zJqqp7
			hex!["72bae70a1398c0ba52f815cc5dfbc9ec5c013771e541ae28e05d1129243e3001"]
				.unchecked_into(),
		),
		(
			// 5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ
			hex!["aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934"].into(),
			// 5GeoZ1Mzix6Xnj32X8Xpj7q89X1SQHU5XTK1cnUVNXKTvXdK
			hex!["caf27345aebc2fefeca85c9a67f4859eab3178d28ef92244714402290f3f415a"].into(),
			// 5Et8y49AyE7ncVKiSRgzN6zbqbYtMK6y7kKuUaS8YqvfLBD9
			hex!["7ca58770eb41c1a68ef77e92255e4635fc11f665cb89aee469e920511c48343a"]
				.unchecked_into(),
			// 5Hpn3HVViECsuxMDFtinWjRj2dNfpRp1kB24nZHvQCJsSUek
			hex!["feca0be2c87141f6074b221c919c0161a1c468d9173c5c1be59b68fab9a0ff93"]
				.unchecked_into(),
			// 5Et8y49AyE7ncVKiSRgzN6zbqbYtMK6y7kKuUaS8YqvfLBD9
			hex!["7ca58770eb41c1a68ef77e92255e4635fc11f665cb89aee469e920511c48343a"]
				.unchecked_into(),
			// 5Et8y49AyE7ncVKiSRgzN6zbqbYtMK6y7kKuUaS8YqvfLBD9
			hex!["7ca58770eb41c1a68ef77e92255e4635fc11f665cb89aee469e920511c48343a"]
				.unchecked_into(),
			// 5Et8y49AyE7ncVKiSRgzN6zbqbYtMK6y7kKuUaS8YqvfLBD9
			hex!["7ca58770eb41c1a68ef77e92255e4635fc11f665cb89aee469e920511c48343a"]
				.unchecked_into(),
		),
		(
			// 5HYYWyhyUQ7Ae11f8fCid58bhJ7ikLHM9bU8A6Ynwoc3dStR
			hex!["f268995cc38974ce0686df1364875f26f2c32b246ddc18835512c3f9969f5836"].into(),
			// 5DnUXT3xiQn6ZRttFT6eSCJbT9P2tiLdexr5WsvnbLG8igqW
			hex!["4c17a9bfdd19411f452fa32420fa7acab622e87e57351f4ba3248ae40ce75123"].into(),
			// 5EhnN1SumSv5KxwLAdwE8ugJaw1S8xARZb8V2BMYCKaD7ure
			hex!["74bfb70627416e6e6c4785e928ced384c6c06e5c8dd173a094bc3118da7b673e"]
				.unchecked_into(),
			// 5Hmvd2qjb1zatrJTkPwgFicxPfZuwaTwa2L7adSRmz6mVxfb
			hex!["fc9d33059580a69454179ffa41cbae6de2bc8d2bd2c3f1d018fe5484a5a91956"]
				.unchecked_into(),
			// 5EhnN1SumSv5KxwLAdwE8ugJaw1S8xARZb8V2BMYCKaD7ure
			hex!["74bfb70627416e6e6c4785e928ced384c6c06e5c8dd173a094bc3118da7b673e"]
				.unchecked_into(),
			// 5EhnN1SumSv5KxwLAdwE8ugJaw1S8xARZb8V2BMYCKaD7ure
			hex!["74bfb70627416e6e6c4785e928ced384c6c06e5c8dd173a094bc3118da7b673e"]
				.unchecked_into(),
			// 5EhnN1SumSv5KxwLAdwE8ugJaw1S8xARZb8V2BMYCKaD7ure
			hex!["74bfb70627416e6e6c4785e928ced384c6c06e5c8dd173a094bc3118da7b673e"]
				.unchecked_into(),
		),
		(
			// 5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh
			hex!["08264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d"].into(),
			// 5F6z64cYZFRAmyMUhp7rnge6jaZmbY6o7XfA9czJyuAUiaFD
			hex!["8671d451c3d4f6de8c16ea0bc61cf714914d6b2ffa2899872620525419327478"].into(),
			// 5Ft7o2uqDq5pXCK4g5wR94BctmtLEzCBy5MvPqRa8753ZemD
			hex!["a8ddd0891e14725841cd1b5581d23806a97f41c28a25436db6473c86e15dcd4f"]
				.unchecked_into(),
			// 5FgBijJLL6p7nDZgQed56L3BM7ovgwc4t4FYsv9apYtRGAGv
			hex!["9fc415cce1d0b2eed702c9e05f476217d23b46a8723fd56f08cddad650be7c2d"]
				.unchecked_into(),
			// 5Ft7o2uqDq5pXCK4g5wR94BctmtLEzCBy5MvPqRa8753ZemD
			hex!["a8ddd0891e14725841cd1b5581d23806a97f41c28a25436db6473c86e15dcd4f"]
				.unchecked_into(),
			// 5Ft7o2uqDq5pXCK4g5wR94BctmtLEzCBy5MvPqRa8753ZemD
			hex!["a8ddd0891e14725841cd1b5581d23806a97f41c28a25436db6473c86e15dcd4f"]
				.unchecked_into(),
			// 5Ft7o2uqDq5pXCK4g5wR94BctmtLEzCBy5MvPqRa8753ZemD
			hex!["a8ddd0891e14725841cd1b5581d23806a97f41c28a25436db6473c86e15dcd4f"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	westend::GenesisConfig {
		frame_system: Some(westend::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(westend::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(westend::IndicesConfig { indices: vec![] }),
		pallet_session: Some(westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(westend::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						westend::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(westend::AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_vesting: Some(westend::VestingConfig { vesting: vec![] }),
		pallet_sudo: Some(westend::SudoConfig {
			key: endowed_accounts[0].clone(),
		}),
	}
}

fn kusama_staging_testnet_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5CVFESwfkk7NmhQ6FwHCM9roBvr9BGa4vJHFYU8DnGQxrXvz
		hex!["12b782529c22032ed4694e0f6e7d486be7daa6d12088f6bc74d593b3900b8438"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in parachains; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
			hex!["32a5718e87d16071756d4b1370c411bbbb947eb62f0e6e0b937d5cbfc0ea633b"].into(),
			// 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
			hex!["bee39fe862c85c91aaf343e130d30b643c6ea0b4406a980206f1df8331f7093b"].into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
			hex!["76620f7c98bce8619979c2b58cf2b0aff71824126d2b039358729dad993223db"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
		),
		(
			// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
			hex!["b496c98a405ceab59b9e970e59ef61acd7765a19b704e02ab06c1cdfe171e40f"].into(),
			// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
			hex!["86d3a7571dd60139d297e55d8238d0c977b2e208c5af088f7f0136b565b0c103"].into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
			hex!["e2234d661bee4a04c38392c75d1566200aa9e6ae44dd98ee8765e4cc9af63cb7"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
		),
		(
			// 5FzwpgGvk2kk9agow6KsywLYcPzjYc8suKej2bne5G5b9YU3
			hex!["ae12f70078a22882bf5135d134468f77301927aa67c376e8c55b7ff127ace115"].into(),
			// 5EqoZhVC2BcsM4WjvZNidu2muKAbu5THQTBKe3EjvxXkdP7A
			hex!["7addb914ec8486bbc60643d2647685dcc06373401fa80e09813b630c5831d54b"].into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5E8ULLQrDAtWhfnVfZmX41Yux86zNAwVJYguWJZVWrJvdhBe
			hex!["5b57ed1443c8967f461db1f6eb2ada24794d163a668f1cf9d9ce3235dfad8799"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
		),
		(
			// 5CFj6Kg9rmVn1vrqpyjau2ztyBzKeVdRKwNPiA3tqhB5HPqq
			hex!["0867dbb49721126df589db100dda728dc3b475cbf414dad8f72a1d5e84897252"].into(),
			// 5CwQXP6nvWzigFqNhh2jvCaW9zWVzkdveCJY3tz2MhXMjTon
			hex!["26ab2b4b2eba2263b1e55ceb48f687bb0018130a88df0712fbdaf6a347d50e2a"].into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5HGLmrZsiTFTPp3QoS1W8w9NxByt8PVq79reqvdxNcQkByqK
			hex!["e60d23f49e93c1c1f2d7c115957df5bbd7faf5ebf138d1e9d02e8b39a1f63df0"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	kusama::GenesisConfig {
		frame_system: Some(kusama::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(kusama::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(kusama::IndicesConfig { indices: vec![] }),
		pallet_session: Some(kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(kusama::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						kusama::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_democracy: Some(Default::default()),
		pallet_collective_Instance1: Some(kusama::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(kusama::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(kusama::AuthorityDiscoveryConfig { keys: vec![] }),
		claims: Some(kusama::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		}),
		pallet_vesting: Some(kusama::VestingConfig { vesting: vec![] }),
	}
}

fn rococo_staging_testnet_config_genesis(wasm_binary: &[u8]) -> rococo::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5CVFESwfkk7NmhQ6FwHCM9roBvr9BGa4vJHFYU8DnGQxrXvz
		hex!["12b782529c22032ed4694e0f6e7d486be7daa6d12088f6bc74d593b3900b8438"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in parachains; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
			hex!["32a5718e87d16071756d4b1370c411bbbb947eb62f0e6e0b937d5cbfc0ea633b"].into(),
			// 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
			hex!["bee39fe862c85c91aaf343e130d30b643c6ea0b4406a980206f1df8331f7093b"].into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
			hex!["76620f7c98bce8619979c2b58cf2b0aff71824126d2b039358729dad993223db"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
		),
		(
			// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
			hex!["b496c98a405ceab59b9e970e59ef61acd7765a19b704e02ab06c1cdfe171e40f"].into(),
			// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
			hex!["86d3a7571dd60139d297e55d8238d0c977b2e208c5af088f7f0136b565b0c103"].into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
			hex!["e2234d661bee4a04c38392c75d1566200aa9e6ae44dd98ee8765e4cc9af63cb7"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
		),
		(
			// 5FzwpgGvk2kk9agow6KsywLYcPzjYc8suKej2bne5G5b9YU3
			hex!["ae12f70078a22882bf5135d134468f77301927aa67c376e8c55b7ff127ace115"].into(),
			// 5EqoZhVC2BcsM4WjvZNidu2muKAbu5THQTBKe3EjvxXkdP7A
			hex!["7addb914ec8486bbc60643d2647685dcc06373401fa80e09813b630c5831d54b"].into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5E8ULLQrDAtWhfnVfZmX41Yux86zNAwVJYguWJZVWrJvdhBe
			hex!["5b57ed1443c8967f461db1f6eb2ada24794d163a668f1cf9d9ce3235dfad8799"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
		),
		(
			// 5CFj6Kg9rmVn1vrqpyjau2ztyBzKeVdRKwNPiA3tqhB5HPqq
			hex!["0867dbb49721126df589db100dda728dc3b475cbf414dad8f72a1d5e84897252"].into(),
			// 5CwQXP6nvWzigFqNhh2jvCaW9zWVzkdveCJY3tz2MhXMjTon
			hex!["26ab2b4b2eba2263b1e55ceb48f687bb0018130a88df0712fbdaf6a347d50e2a"].into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5HGLmrZsiTFTPp3QoS1W8w9NxByt8PVq79reqvdxNcQkByqK
			hex!["e60d23f49e93c1c1f2d7c115957df5bbd7faf5ebf138d1e9d02e8b39a1f63df0"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	rococo::GenesisConfig {
		frame_system: Some(rococo::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(rococo::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(rococo::IndicesConfig { indices: vec![] }),
		pallet_session: Some(rococo::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(rococo::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						rococo::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(rococo::AuthorityDiscoveryConfig { keys: vec![] }),
	}
}

/// Polkadot staging testnet config.
pub fn polkadot_staging_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;
	let boot_nodes = vec![];

	Ok(PolkadotChainSpec::from_genesis(
		"Polkadot Staging Testnet",
		"polkadot_staging_testnet",
		ChainType::Live,
		move || polkadot_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Staging testnet config.
pub fn kusama_staging_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;
	let boot_nodes = vec![];

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Staging Testnet",
		"kusama_staging_testnet",
		ChainType::Live,
		move || kusama_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(KUSAMA_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Kusama Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Westend staging testnet config.
pub fn westend_staging_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;
	let boot_nodes = vec![];

	Ok(WestendChainSpec::from_genesis(
		"Westend Staging Testnet",
		"westend_staging_testnet",
		ChainType::Live,
		move || westend_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(WESTEND_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Rococo staging testnet config.
pub fn rococo_staging_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;
	let boot_nodes = vec![];

	Ok(RococoChainSpec::from_genesis(
		"Rococo Staging Testnet",
		"rococo_staging_testnet",
		ChainType::Live,
		move || rococo_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(ROCOCO_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Rococo Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
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

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
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
	]
}

/// Helper function to create polkadot GenesisConfig for testing
pub fn polkadot_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> polkadot::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOTS;
	const STASH: u128 = 100 * DOTS;

	polkadot::GenesisConfig {
		frame_system: Some(polkadot::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(polkadot::IndicesConfig { indices: vec![] }),
		pallet_balances: Some(polkadot::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		}),
		pallet_session: Some(polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(polkadot::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						polkadot::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_democracy: Some(polkadot::DemocracyConfig::default()),
		pallet_collective_Instance1: Some(polkadot::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(polkadot::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(polkadot::AuthorityDiscoveryConfig { keys: vec![] }),
		claims: Some(polkadot::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		}),
		pallet_vesting: Some(polkadot::VestingConfig { vesting: vec![] }),
	}
}

/// Helper function to create kusama GenesisConfig for testing
pub fn kusama_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> kusama::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	kusama::GenesisConfig {
		frame_system: Some(kusama::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(kusama::IndicesConfig { indices: vec![] }),
		pallet_balances: Some(kusama::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		}),
		pallet_session: Some(kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(kusama::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						kusama::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_democracy: Some(kusama::DemocracyConfig::default()),
		pallet_collective_Instance1: Some(kusama::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(kusama::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(kusama::AuthorityDiscoveryConfig { keys: vec![] }),
		claims: Some(kusama::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		}),
		pallet_vesting: Some(kusama::VestingConfig { vesting: vec![] }),
	}
}

/// Helper function to create westend GenesisConfig for testing
pub fn westend_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> westend::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOTS;
	const STASH: u128 = 100 * DOTS;

	westend::GenesisConfig {
		frame_system: Some(westend::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(westend::IndicesConfig { indices: vec![] }),
		pallet_balances: Some(westend::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		}),
		pallet_session: Some(westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(westend::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						westend::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(westend::AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_vesting: Some(westend::VestingConfig { vesting: vec![] }),
		pallet_sudo: Some(westend::SudoConfig { key: root_key }),
	}
}

/// Helper function to create rococo GenesisConfig for testing
pub fn rococo_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AuthorityDiscoveryId,
	)>,
	endowed_accounts: Option<Vec<AccountId>>,
) -> rococo::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOTS;
	const STASH: u128 = 100 * DOTS;

	rococo::GenesisConfig {
		frame_system: Some(rococo::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(rococo::IndicesConfig { indices: vec![] }),
		pallet_balances: Some(rococo::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		}),
		pallet_session: Some(rococo::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(rococo::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						rococo::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(rococo::AuthorityDiscoveryConfig { keys: vec![] }),
	}
}

fn polkadot_development_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

fn kusama_development_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

fn westend_development_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot development config (single validator Alice)
pub fn polkadot_development_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || polkadot_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Kusama development config (single validator Alice)
pub fn kusama_development_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Development",
		"kusama_dev",
		ChainType::Development,
		move || kusama_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Westend development config (single validator Alice)
pub fn westend_development_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Development",
		"westend_dev",
		ChainType::Development,
		move || westend_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn polkadot_local_testnet_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot local testnet config (multivalidator Alice + Bob)
pub fn polkadot_local_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || polkadot_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn kusama_local_testnet_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Kusama local testnet config (multivalidator Alice + Bob)
pub fn kusama_local_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Local Testnet",
		"kusama_local_testnet",
		ChainType::Local,
		move || kusama_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn westend_local_testnet_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Westend local testnet config (multivalidator Alice + Bob)
pub fn westend_local_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Westend Local Testnet",
		"westend_local_testnet",
		ChainType::Local,
		move || westend_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn rococo_local_testnet_genesis(wasm_binary: &[u8]) -> rococo::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		None,
	)
}

/// Rococo local testnet config (multivalidator Alice + Bob)
pub fn rococo_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Rococo Local Testnet",
		"rococo_local_testnet",
		ChainType::Local,
		move || rococo_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}
