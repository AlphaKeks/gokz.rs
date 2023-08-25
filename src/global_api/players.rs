//! `/players` endpoints
//!
//! Covered:
//! - `/players`
//! - `/players/steamid/:steam_id`
//!
//! NOTE: `/players/steamid/:steam_id/alts` seems to be broken.

use {
	super::API_URL,
	crate::{http, yeet, PlayerIdentifier, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

impl Player {
	/// Returns a link to the player's Steam profile.
	#[inline]
	pub fn steam_profile(&self) -> String {
		format!("https://steamcommunity.com/profiles/{}", self.steam_id.as_id64())
	}

	/// Returns a link to fetch this player from the GlobalAPI.
	#[inline]
	pub fn api(&self) -> String {
		format!("{API_URL}/players/steam_id/{}", self.steam_id)
	}

	/// Returns a link to the player's KZ:GO profile.
	#[inline]
	pub fn kzgo_profile(&self) -> String {
		format!("https://kzgo.eu/players/{}", self.steam_id)
	}
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub steam_id: Option<SteamID>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/players` route
///
/// Fetches players with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_players_with(params: &Params, client: &http::Client) -> Result<Vec<Player>> {
	let players = http::get! {
		url = format!("{API_URL}/players");
		params = params;
		deserialize = Vec<Player>;
		client = client;
	}?;

	if players.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(players)
}

/// `/players` or `/players/steamid/:steam_id` route depending on the exact input.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_player(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Player> {
	let (url, params) = match player.into() {
		PlayerIdentifier::Name(name) => (format!("{API_URL}/players"), Params {
			name: Some(name),
			limit: Some(1),
			..Default::default()
		}),
		PlayerIdentifier::SteamID(steam_id) => {
			(format!("{API_URL}/players/steamid/{steam_id}"), Params::default())
		}
	};

	let mut players = http::get! {
		url = url;
		params = &params;
		deserialize = Vec<Player>;
		client = client;
	}?;

	if players.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(players.remove(0))
}
