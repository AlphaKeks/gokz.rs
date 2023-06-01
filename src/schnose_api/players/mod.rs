use {
	crate::{http, PlayerIdentifier, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

/// The `/players` route.
pub mod index;

/// Route: `/players`
///
/// Fetch players from the API.
pub async fn get_players(
	params: index::Params,
	client: &crate::Client,
) -> Result<Vec<index::Player>> {
	http::get_with_params(&format!("{}/players", super::BASE_URL), params, client).await
}

/// Route: `/players/:ident`
///
/// Fetch a single player from the API by an identifier.
pub async fn get_player(player: PlayerIdentifier, client: &crate::Client) -> Result<Player> {
	let url = format!("{}/players/{}", super::BASE_URL, player);
	http::get::<Player>(&url, client).await
}
