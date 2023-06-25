use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::SteamID,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub steam_id: Option<SteamID>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub steamid64_list: Option<u64>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			steam_id: None,
			is_banned: None,
			total_records: None,
			ip: None,
			steamid64_list: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /players
///
/// Fetches players
#[tracing::instrument(
	name = "GlobalAPI request to `/players`",
	level = "trace",
	skip(client),
	err(Debug)
)]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Player>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/players"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /players/steam_id/:steam_id
///
/// Fetches a player by [`SteamID`]
#[tracing::instrument(
	name = "GlobalAPI request to `/players`",
	level = "TRACE",
	skip(client),
	err(Debug)
)]
pub async fn steam_id(steam_id: SteamID, client: &crate::Client) -> Result<Player> {
	let response =
		get_json(&format!("{BASE_URL}/players/steam_id/{steam_id}"), &[()], client).await?;

	Ok(response)
}
