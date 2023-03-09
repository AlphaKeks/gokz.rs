//! Types and functions to interact with the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

use {
	crate::{Result, SteamID},
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
	since: &NaiveDateTime,
	client: &reqwest::Client,
) -> Result<Vec<bans::Ban>> {
	bans::get_bans(
		bans::index::Params {
			created_since: Some(
				since
					.format("%Y-%m-%dT%H:%M:%S")
					.to_string(),
			),
			..Default::default()
		},
		client,
	)
	.await
}
