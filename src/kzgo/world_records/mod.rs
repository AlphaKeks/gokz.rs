use {
	crate::{
		chrono::{parse_date, ser_date},
		Error, Result, SteamID,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct WorldRecord {
	pub map_id: u16,
	pub map_name: String,
	pub player_name: String,
	pub steam_id: SteamID,
	pub time: f64,
	pub time_difference: f64,
	pub teleports: u32,
	pub server_id: u16,
	pub server_name: String,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
}

/// `/wrs/:mode`
pub mod mode;
impl TryFrom<mode::Response> for WorldRecord {
	type Error = Error;

	fn try_from(value: mode::Response) -> Result<Self> {
		let steam_id = if let Ok(steam_id) = value.steamId.parse() {
			steam_id
		} else {
			SteamID::new(&value.steamId64)?
		};

		Ok(Self {
			map_id: value.mapId,
			map_name: value.mapName,
			player_name: value.playerName,
			steam_id,
			time: value.time,
			time_difference: value.diff,
			teleports: value.tps,
			server_id: value.serverId,
			server_name: value.serverName,
			created_on: parse_date!(value.createdOn),
		})
	}
}

/// `/wrs/leaderboards/:mode/:runtype`
pub mod leaderboard;

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct LeaderboardEntry {
	pub name: String,
	pub steam_id: SteamID,
	pub count: u16,
}

impl TryFrom<leaderboard::Response> for LeaderboardEntry {
	type Error = Error;

	fn try_from(value: leaderboard::Response) -> Result<Self> {
		Ok(Self {
			name: value.playerName,
			steam_id: value._id.parse()?,
			count: value.count,
		})
	}
}
