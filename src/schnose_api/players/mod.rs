use {
	crate::{
		error::{Error, Result},
		http::get_json,
		prelude,
		schnose_api::BASE_URL,
		types::PlayerIdentifier,
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
	pub steam_id: prelude::SteamID,
	pub name: String,
	pub is_banned: bool,
}

impl crate::traits::PlayerIdentifier for Player {
	#[inline]
	fn steam_profile(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).steam_profile() }

	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).global_api() }

	#[inline]
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).kzgo() }

	#[inline]
	fn schnose_api(&self) -> String { PlayerIdentifier::SteamID(self.steam_id).schnose_api() }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub is_banned: Option<bool>,
	pub offset: Option<i64>,
	pub limit: Option<u64>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			is_banned: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /players
///
/// Fetches `limit` or less players (max. 1000).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Player>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/players"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /players/:ident
///
/// Fetches a single player
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn ident(
	player_identifier: prelude::PlayerIdentifier,
	client: &crate::Client,
) -> Result<Player> {
	get_json(&format!("{BASE_URL}/players/{player_identifier}"), &EmptyParams, client).await
}
