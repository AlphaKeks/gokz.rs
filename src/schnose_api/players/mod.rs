use {
	super::Response,
	crate::{http, Error, PlayerIdentifier, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct RawPlayer {
	pub id: u32,
	pub name: String,
	pub steam_id: String,
	pub steam_id64: String,
	pub is_banned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

impl TryFrom<RawPlayer> for Player {
	type Error = Error;

	fn try_from(value: RawPlayer) -> Result<Self> {
		let steam_id = if let Ok(steam_id) = value.steam_id.parse() {
			steam_id
		} else if let Ok(steam_id) = value.steam_id64.parse() {
			steam_id
		} else {
			let steam_id3 = format!("U:1:{}", value.id);
			SteamID::new(steam_id3)?
		};

		Ok(Self {
			name: value.name,
			steam_id,
			is_banned: value.is_banned,
		})
	}
}

/// The `/players` route.
pub mod index;

/// Route: `/players`
///
/// Fetch players from the API.
pub async fn get_players(
	params: index::Params,
	client: &crate::Client,
) -> Result<Response<Vec<index::Player>>> {
	http::get_with_params(&format!("{}/players", super::BASE_URL), params, client).await
}

/// The `/players/:ident` route.
pub mod ident;
pub use ident::{FancyPlayer, RawFancyPlayer, RecordCount, RecordSummary};

/// Route: `/players/:ident`
///
/// Fetch a single player from the API by an identifier.
pub async fn get_player(player: PlayerIdentifier, client: &crate::Client) -> Result<FancyPlayer> {
	let url = format!("{}/players/{}", super::BASE_URL, player);
	http::get::<Response<RawFancyPlayer>>(&url, client)
		.await?
		.result
		.try_into()
}
