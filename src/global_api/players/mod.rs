use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::{PlayerIdentifier, SteamID},
		utils::EmptyParams,
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

impl crate::traits::PlayerIdentifier for Player {
	#[inline]
	fn steam_profile(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).steam_profile() }

	#[inline]
	fn global_api(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).global_api() }

	#[inline]
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).kzgo() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).schnose_api() }
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
#[tracing::instrument(name = "GlobalAPI request to `/players`", level = "trace", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Player>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/players"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /players/steamid/:steam_id
///
/// Fetches a player by [`SteamID`]
#[tracing::instrument(name = "GlobalAPI request to `/players`", level = "TRACE", skip(client))]
pub async fn steam_id(steam_id: SteamID, client: &crate::Client) -> Result<Player> {
	let mut response: Vec<_> =
		get_json(&format!("{BASE_URL}/players/steamid/{steam_id}"), &EmptyParams, client).await?;

	response.pop().ok_or(Error::EmptyResponse)
}
