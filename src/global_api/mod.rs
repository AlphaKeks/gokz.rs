//! Types and functions to interact with the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

use {
	crate::{MapIdentifier, Result, SteamID},
	chrono::NaiveDateTime,
};

/// Base URL for GlobalAPI requests.
pub const BASE_URL: &str = "https://kztimerglobal.com/api/v2";

/// The `/bans` route.
pub mod bans;

/// Fetches `limit` bans.
pub async fn get_bans_limit(limit: u32, client: &reqwest::Client) -> Result<Vec<bans::Ban>> {
	bans::get_bans(
		bans::index::Params {
			limit: Some(limit),
			..Default::default()
		},
		client,
	)
	.await
}

/// Fetches all bans for a given [`SteamID`].
pub async fn get_bans_for_player(
	steam_id: &SteamID,
	client: &reqwest::Client,
) -> Result<Vec<bans::Ban>> {
	bans::get_bans(
		bans::index::Params {
			steamid64: Some(steam_id.as_id64()),
			..Default::default()
		},
		client,
	)
	.await
}

/// Fetches all bans that happened since a specific date.
pub async fn get_bans_since(
	since: NaiveDateTime,
	client: &reqwest::Client,
) -> Result<Vec<bans::Ban>> {
	bans::get_bans(
		bans::index::Params {
			created_since: Some(since),
			..Default::default()
		},
		client,
	)
	.await
}

/// API health checks
pub mod health;
pub use health::checkhealth;

/// The `/maps` route.
pub mod maps;

/// Fetches all maps.
pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::Map>> {
	maps::get_maps(
		maps::index::Params {
			limit: Some(9999),
			..Default::default()
		},
		client,
	)
	.await
}

/// Fetches all global/validated maps.
pub async fn get_global_maps(client: &reqwest::Client) -> Result<Vec<maps::Map>> {
	maps::get_maps(
		maps::index::Params {
			is_validated: Some(true),
			limit: Some(9999),
			..Default::default()
		},
		client,
	)
	.await
}

/// Fetches all non-global/non-validated maps.
pub async fn get_nonglobal_maps(client: &reqwest::Client) -> Result<Vec<maps::Map>> {
	maps::get_maps(
		maps::index::Params {
			is_validated: Some(false),
			limit: Some(9999),
			..Default::default()
		},
		client,
	)
	.await
}

/// Fetches a single map.
pub async fn get_map(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<maps::Map> {
	match map_identifier {
		MapIdentifier::Name(map_name) => maps::get_map_by_name(map_name, client).await,
		MapIdentifier::ID(map_id) => maps::get_map_by_id(*map_id, client).await,
	}
}

pub mod players;
