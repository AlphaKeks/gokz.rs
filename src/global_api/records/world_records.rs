use {
	crate::{Error, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs, non_snake_case)]
pub struct Params {
	pub ids: Option<u32>,
	pub map_ids: Option<u16>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
	pub mapTag: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: None,
			has_teleports: None,
			mapTag: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub steamid64: String,
	pub steam_id: String,
	pub count: u32,
	pub player_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct WRStats {
	pub player_name: String,
	pub steam_id: SteamID,
	pub count: u32,
}

impl TryFrom<Response> for WRStats {
	type Error = Error;

	fn try_from(value: Response) -> Result<Self, Self::Error> {
		let steam_id = 'ret: {
			if let Ok(steam_id) = value.steam_id.parse() {
				break 'ret steam_id;
			}

			value.steamid64.parse()?
		};

		Ok(Self {
			player_name: value.player_name,
			steam_id,
			count: value.count,
		})
	}
}
