#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		http::get_json,
		kzgo_api::BASE_URL,
		traits::Mode as _,
		types::{Mode, Runtype, SteamID},
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldRecord {
	pub map_id: u16,
	pub map_name: String,
	pub player_name: String,
	pub steam_id: SteamID,
	pub time: f64,
	#[serde(rename = "diff")]
	pub time_difference: f64,
	#[serde(rename = "tps")]
	pub teleports: u32,
	pub server_id: u16,
	pub server_name: String,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub created_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,
}

/// # /wrs/:mode
///
/// Fetches all world records for a given mode
#[tracing::instrument(
	name = "KZ:GO request to `/wrs/:mode`",
	level = "TRACE",
	skip(client),
	err(Debug)
)]
pub async fn get_wrs(
	mode: Mode,
	runtype: Option<Runtype>,
	client: &crate::Client,
) -> Result<Vec<WorldRecord>> {
	let mut url = format!("{BASE_URL}/wrs/{mode}/", mode = mode.api());

	match runtype.unwrap_or_default() {
		Runtype::TP => url += "tp",
		Runtype::Pro => url += "pro",
	};

	let records: Vec<_> = get_json(&url, &EmptyParams, client).await?;

	if records.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(records)
}

/// The `/wrs/leaderboards` routes
pub mod leaderboards {
	use super::*;

	#[allow(missing_docs)]
	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Player {
		pub name: String,
		pub steam_id: SteamID,
		pub count: u16,
	}

	/// # /wrs/leaderboards/:mode/:runtype
	///
	/// Fetches a leaderboard of players with the most world records in a given mode
	#[tracing::instrument(
		name = "KZ:GO request to `/wrs/leaderboards/:mode/:runtype`",
		level = "DEBUG",
		skip(client),
		err(Debug)
	)]
	pub async fn get_wr_leaderboard(
		mode: Mode,
		runtype: Runtype,
		client: &crate::Client,
	) -> Result<Vec<Player>> {
		let url = format!(
			"{BASE_URL}/wrs/{mode}/{runtype}",
			mode = mode.api(),
			runtype = match runtype {
				Runtype::TP => "tp",
				Runtype::Pro => "pro",
			}
		);

		let players: Vec<_> = get_json(&url, &EmptyParams, client).await?;

		if players.is_empty() {
			return Err(Error::EmptyResponse);
		}

		Ok(players)
	}
}
