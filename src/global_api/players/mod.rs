use {
	crate::{http::get_with_params, Error, Result, SteamID},
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

/// `/players` route
pub mod index;
impl TryFrom<index::Response> for Player {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		let steam_id = 'ret: {
			if let Ok(steam_id) = value.steam_id.parse::<SteamID>() {
				break 'ret steam_id;
			}

			value.steamid64.parse::<SteamID>()?
		};

		Ok(Self {
			name: value.name,
			steam_id,
			is_banned: value.is_banned,
		})
	}
}

/// Fetches maps with the given `params`.
pub async fn get_players(params: index::Params, client: &reqwest::Client) -> Result<Vec<Player>> {
	let response: Vec<index::Response> =
		get_with_params(&format!("{}/players", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}
