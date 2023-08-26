//! `/records` endpoints
//!
//! Covered:
//! - `/records/:record_id`
//! - `/records/top`
//! - `/records/top/world_records`
//! - `/records/place/:record_id`

use {
	super::API_URL,
	crate::{http, yeet, MapIdentifier, Mode, PlayerIdentifier, Result, Runtype, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
	pub id: u32,
	pub steam_id: SteamID,
	pub player_name: String,
	pub map_id: u16,
	pub map_name: String,
	pub stage: u8,
	pub mode: Mode,
	pub time: f64,
	pub teleports: u32,
	pub server_id: u16,
	pub server_name: String,
	pub points: u32,
	pub replay_id: u32,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date",
		deserialize_with = "crate::serde::chrono::deserialize_date"
	)]
	pub created_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date",
		deserialize_with = "crate::serde::chrono::deserialize_date"
	)]
	pub updated_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub updated_on: String,
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

	/// Returns a link to fetch the map the record was done on from the GlobalAPI.
	pub fn api(&self) -> String {
		format!("{API_URL}/maps/name/{}", self.map_name)
	}

	/// Returns a link to the KZ:GO page of the map this record was done on.
	pub fn kzgo(&self) -> String {
		format!("https://kzgo.eu/maps/{}?{}=", self.map_name, self.mode.short().to_lowercase())
	}

	/// Returns a download link for the replay of this record.
	pub fn replay_download(&self) -> Option<String> {
		if self.replay_id == 0 {
			return None;
		}

		Some(format!("{API_URL}/records/replay/{}", self.replay_id))
	}

	/// Returns a link to watch the replay of this record online.
	pub fn replay_view(&self) -> Option<String> {
		if self.replay_id == 0 {
			return None;
		}

		Some(format!(
			"http://gokzmaptest.site.nfoservers.com/GlobalReplays/?replay={}",
			self.replay_id
		))
	}

	/// Combination of [`Self::replay_download`] and [`Self::replay_view`] as both of them
	/// return [`None`] / [`Some`] on the same condition.
	pub fn replay_links(&self) -> Option<(String, String)> {
		if let (Some(download), Some(view)) = (self.replay_download(), self.replay_view()) {
			Some((download, view))
		} else {
			None
		}
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

/// `/records/place/:record_id` route
///
/// Fetches the leaderboard place for a specific record.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_place(record_id: u32, client: &crate::http::Client) -> Result<u32> {
	#[derive(Deserialize)]
	struct Place(u32);

	let Place(place) = http::get! {
		url = format!("{API_URL}/records/place/{record_id}");
		deserialize = Place;
		client = client;
	}?;

	Ok(place)
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub steam_id: Option<SteamID>,
	pub player_name: Option<String>,
	pub map_id: Option<u16>,
	pub map_name: Option<String>,
	pub stage: Option<u8>,

	#[serde(rename = "modes_list_string")]
	pub mode: Option<Mode>,

	#[serde(rename = "has_teleports")]
	pub runtype: Option<Runtype>,

	#[serde(rename = "overall")]
	pub nub: Option<bool>,

	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/records/top` route
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
		url = format!("{API_URL}/records/top");
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
	course: u8,
	mode: impl Into<Mode> + std::fmt::Debug,
	runtype: impl Into<Runtype> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<Record> {
	let mut params = Params {
		stage: Some(course),
		mode: Some(mode.into()),
		runtype: Some(runtype.into()),
		limit: Some(1),
		..Default::default()
	};

	match map.into() {
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id),
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	Ok(get_records_with(&params, client).await?.remove(0))
}

/// `/records/top` route
///
/// Fetches the top `n` records for the given parameters.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maptop(
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	course: u8,
	mode: impl Into<Mode> + std::fmt::Debug,
	runtype: impl Into<Runtype> + std::fmt::Debug,
	n: usize,
	client: &crate::http::Client,
) -> Result<Vec<Record>> {
	let mut params = Params {
		stage: Some(course),
		mode: Some(mode.into()),
		runtype: Some(runtype.into()),
		limit: Some(n as u32),
		..Default::default()
	};

	match map.into() {
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id),
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	get_records_with(&params, client).await
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
	let mut params = Params {
		stage: Some(course),
		mode: Some(mode.into()),
		runtype: Some(runtype.into()),
		limit: Some(1),
		..Default::default()
	};

	match player.into() {
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
		PlayerIdentifier::Name(name) => params.player_name = Some(name),
	};

	match map.into() {
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id),
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	Ok(get_records_with(&params, client).await?.remove(0))
}

/// `/records/world_records` endpoint
pub mod world_records {
	use {super::*, crate::http::append_pairs, reqwest::Url, std::ops::RangeInclusive};

	#[allow(missing_docs)]
	#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
	pub struct Params {
		#[serde(skip)]
		pub ids: Option<Vec<u32>>,

		#[serde(skip)]
		pub modes: Option<Vec<Mode>>,

		#[serde(skip)]
		pub map_ids: Option<Vec<u16>>,

		#[serde(skip)]
		pub stages: Option<Vec<u8>>,

		#[serde(skip)]
		pub tickrates: Option<Vec<u8>>,

		#[serde(rename = "has_teleports")]
		pub runtype: Option<Runtype>,

		#[serde(rename = "mapTag")]
		pub map_tag: Option<String>,

		pub offset: Option<u32>,
		pub limit: Option<u32>,
	}

	#[allow(missing_docs)]
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
	pub struct RecordHolder {
		pub count: u32,
		pub steam_id: SteamID,
		pub player_name: String,
	}

	/// `/records/top/world_records` route
	///
	/// Fetches a leaderboard of `n` world record holders.
	#[tracing::instrument(level = "TRACE", skip(client))]
	pub async fn get_wr_leaderboard(
		stages: impl Into<RangeInclusive<u8>> + std::fmt::Debug,
		mode: impl Into<Mode> + std::fmt::Debug,
		runtype: impl Into<Runtype> + std::fmt::Debug,
		n: usize,
		client: &crate::http::Client,
	) -> Result<Vec<RecordHolder>> {
		let params = Params {
			tickrates: Some(vec![128]),
			runtype: Some(runtype.into()),
			limit: Some(n as u32),
			..Default::default()
		};

		let mut url = Url::parse(&format!("{API_URL}/records/top/world_records"))
			.expect("This is a valid URL.");

		append_pairs!(&mut url, Some(stages.into().collect::<Vec<_>>()), "stages");
		append_pairs!(&mut url, Some(vec![mode.into() as u8]), "mode_ids");

		let leaderboard = http::get! {
			url = url;
			params = &params;
			deserialize = Vec<RecordHolder>;
			client = client;
		}?;

		if leaderboard.is_empty() {
			yeet!(EmptyResponse);
		}

		Ok(leaderboard)
	}
}
