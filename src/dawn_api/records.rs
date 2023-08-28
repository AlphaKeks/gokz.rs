//! `/records` endpoints
//!
//! Covered:
//! - `/records`
//! - `/records/:record_id`
//! - `/records/top`
//! - `/records/progression/:player/:mode`

use {
	super::API_URL,
	crate::{
		http, yeet, MapIdentifier, Mode, PlayerIdentifier, Result, Runtype, ServerIdentifier,
		SteamID, Tier,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
	pub id: u32,
	pub course_id: u32,
	pub map_id: u16,
	pub map_name: String,
	pub map_stage: u8,
	pub stage_tier: Option<Tier>,
	pub steam_id: SteamID,
	pub player_name: String,
	pub mode: Mode,
	pub server_id: u16,
	pub server_name: String,
	pub time: f64,
	pub teleports: u32,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date",
		deserialize_with = "super::serde::chrono::deserialize_date"
	)]
	pub created_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,
}

impl Record {
	/// Returns the [`Runtype`] for this record.
	pub const fn runtype(&self) -> Runtype {
		match self.teleports > 0 {
			true => Runtype::TP,
			false => Runtype::Pro,
		}
	}

	/// Returns a URL to an image of the map the record was done on.
	pub fn map_thumbnail(&self) -> String {
		format!(
			"https://raw.githubusercontent.com/KZGlobalTeam/map-images/master/images/{}.jpg",
			self.map_name
		)
	}

	/// Returns a link to download the map the record was done on as a `.bsp` file.
	pub fn map_download(&self) -> String {
		format!("https://maps.global-api.com/bsps/{}.bsp", self.map_name)
	}

	/// Returns a link to fetch the map the record was done on from DawnAPI.
	pub fn api(&self) -> String {
		format!("{API_URL}/maps/name/{}", self.map_name)
	}

	/// Returns a link to fetch the map the record was done on from the GlobalAPI.
	pub fn global_api(&self) -> String {
		format!("https://kztimerglobal.com/api/v2/maps/name/{}", self.map_name)
	}

	/// Returns a link to the KZ:GO page of the map this record was done on.
	pub fn kzgo(&self) -> String {
		format!("https://kzgo.eu/maps/{}?{}=", self.map_name, self.mode.short().to_lowercase())
	}
}

/// `/records/:record_id` route
///
/// Fetches a specific record by id.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_record(record_id: u32, client: &crate::http::Client) -> Result<Record> {
	http::get! {
		url = format!("{API_URL}/records/{record_id}");
		deserialize = Record;
		client = client;
	}
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub map: Option<MapIdentifier>,
	pub stage: Option<u8>,
	pub player: Option<PlayerIdentifier>,
	pub mode: Option<Mode>,
	pub server: Option<ServerIdentifier>,
	pub runtype: Option<Runtype>,
	pub sort_by: Option<SortRecordsBy>,
	pub allow_banned: Option<bool>,
	pub limit: Option<u64>,
	pub offset: Option<i64>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortRecordsBy {
	Newest,
	Oldest,
	Fastest,
	Slowest,
}

/// `/records` route
///
/// Fetches records with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_records_with(
	params: &Params,
	client: &crate::http::Client,
) -> Result<Vec<Record>> {
	let records = http::get! {
		url = format!("{API_URL}/records");
		params = params;
		deserialize = Vec<Record>;
		client = client;
	}?;

	if records.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(records)
}

/// `/records/top` route
///
/// Fetches the world record for the given parameters.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_wr(
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	stage: u8,
	mode: impl Into<Mode> + std::fmt::Debug,
	runtype: impl Into<Runtype> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<Record> {
	get_maptop(map, stage, mode, runtype, 1, client)
		.await
		.map(|mut records| records.remove(0))
}

/// `/records/top` route
///
/// Fetches the top `n` records for the given parameters.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maptop(
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	stage: u8,
	mode: impl Into<Mode> + std::fmt::Debug,
	runtype: impl Into<Runtype> + std::fmt::Debug,
	n: usize,
	client: &crate::http::Client,
) -> Result<Vec<Record>> {
	let params = Params {
		map: Some(map.into()),
		stage: Some(stage),
		mode: Some(mode.into()),
		runtype: Some(runtype.into()),
		limit: Some(n as u64),
		..Default::default()
	};

	let records = http::get! {
		url = format!("{API_URL}/records/top");
		params = &params;
		deserialize = Vec<Record>;
		client = client;
	}?;

	if records.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(records)
}

/// `/records/top` route
///
/// Fetches a player's personal best on the given map.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_pb(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	course: u8,
	mode: impl Into<Mode> + std::fmt::Debug,
	runtype: impl Into<Runtype> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<Record> {
	let params = Params {
		map: Some(map.into()),
		stage: Some(course),
		player: Some(player.into()),
		mode: Some(mode.into()),
		runtype: Some(runtype.into()),
		limit: Some(1),
		..Default::default()
	};

	let mut records = http::get! {
		url = format!("{API_URL}/records/top");
		params = &params;
		deserialize = Vec<Record>;
		client = client;
	}?;

	if records.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(records.remove(0))
}

/// `/records/top` route
///
/// Fetches all of a player's personal bests.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_pbs(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	course: u8,
	client: &crate::http::Client,
) -> Result<Vec<Record>> {
	let params = Params {
		stage: Some(course),
		player: Some(player.into()),
		limit: Some(100000),
		..Default::default()
	};

	let records = http::get! {
		url = format!("{API_URL}/records/top");
		params = &params;
		deserialize = Vec<Record>;
		client = client;
	}?;

	if records.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(records)
}
