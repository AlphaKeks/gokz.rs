#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::{Mode, Runtype, SteamID},
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
	pub id: u32,
	pub map_id: u16,
	pub map_name: String,
	pub steam_id: SteamID,
	pub player_name: String,
	pub server_id: u16,
	pub server_name: Option<String>,
	pub stage: u8,
	pub mode: Mode,
	pub teleports: u32,
	pub time: f64,
	pub tickrate: u8,
	pub points: u16,
	pub replay_id: u32,
	pub record_filter_id: u32,
	pub updated_by: u64,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub created_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub updated_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	pub updated_on: String,
}

impl crate::traits::Record for Record {
	#[inline]
	fn replay_download_link(&self) -> Option<String> {
		match self.replay_id {
			0 => None,
			id => Some(format!("{BASE_URL}/records/replay/{id}")),
		}
	}

	#[inline]
	fn replay_view_link(&self) -> Option<String> {
		match self.replay_id {
			0 => None,
			id => {
				Some(format!("http://gokzmaptest.site.nfoservers.com/GlobalReplays/?replay={id}"))
			}
		}
	}
}

/// # /records/:record_id
///
/// Fetches a single record by id
#[tracing::instrument(
	name = "GlobalAPI request to `/records/:record_id`",
	level = "trace",
	skip(client)
)]
pub async fn root(record_id: u32, client: &crate::Client) -> Result<Record> {
	get_json(&format!("{BASE_URL}/records/{record_id}"), &EmptyParams, client).await
}

#[derive(Debug, Deserialize)]
struct Place(u32);

/// # /records/place/:record_id
///
/// Fetches the leaderboard placement for a single record
#[tracing::instrument(
	name = "GlobalAPI request to `/records/place/:record_id`",
	level = "trace",
	skip(client)
)]
pub async fn place(record_id: u32, client: &crate::Client) -> Result<u32> {
	let Place(place) =
		get_json(&format!("{BASE_URL}/records/place/{record_id}"), &EmptyParams, client).await?;

	Ok(place)
}

/// The `/records/top` route
pub mod top {
	use super::*;

	#[allow(missing_docs)]
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
	pub struct Params {
		pub steam_id: Option<SteamID>,
		pub server_id: Option<u16>,
		pub steamid64: Option<u64>,
		pub map_id: Option<u16>,
		pub map_name: Option<String>,
		pub tickrate: Option<u8>,
		pub overall: Option<bool>,
		pub stage: Option<u8>,
		pub modes_list_string: Option<String>,
		pub modes_list: Option<String>,
		#[serde(rename = "has_teleports")]
		pub runtype: Option<Runtype>,
		pub player_name: Option<String>,
		pub offset: Option<i32>,
		pub limit: Option<u32>,
	}

	impl Default for Params {
		fn default() -> Self {
			Self {
				steam_id: None,
				server_id: None,
				steamid64: None,
				map_id: None,
				map_name: None,
				tickrate: None,
				overall: None,
				stage: None,
				modes_list_string: None,
				modes_list: None,
				runtype: None,
				player_name: None,
				offset: None,
				limit: Some(1),
			}
		}
	}

	/// # /records/top
	///
	/// Fetches records (personal bests)
	#[tracing::instrument(
		name = "GlobalAPI request to `/records/top`",
		level = "trace",
		skip(client)
	)]
	pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Record>> {
		let records: Vec<_> = get_json(&format!("{BASE_URL}/records/top"), params, client).await?;

		if records.is_empty() {
			return Err(Error::EmptyResponse);
		}

		Ok(records)
	}

	/// The `/records/top/world_records` route
	pub mod world_records {
		use super::*;

		#[allow(missing_docs)]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		pub struct WorldRecordStats {
			pub player_name: String,
			pub steam_id: SteamID,
			pub count: u32,
		}

		#[allow(missing_docs)]
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
		pub struct Params {
			pub ids: Option<u32>,
			pub map_ids: Option<u16>,
			pub stages: Option<u8>,
			pub mode_ids: Option<u8>,
			pub tickrates: Option<u8>,
			#[serde(rename = "has_teleports")]
			pub runtype: Option<Runtype>,
			#[serde(rename = "camelCase")]
			pub map_tag: Option<String>,
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
					runtype: None,
					map_tag: None,
					offset: None,
					limit: Some(1),
				}
			}
		}

		/// # /records/world_records
		///
		/// Fetches a leaderboard of the players with the most world records
		#[tracing::instrument(
			name = "GlobalAPI request to `/records/world_records`",
			level = "trace",
			skip(client)
		)]
		pub async fn root(
			params: &Params,
			client: &crate::Client,
		) -> Result<Vec<WorldRecordStats>> {
			let records: Vec<_> =
				get_json(&format!("{BASE_URL}/records/world_records"), params, client).await?;

			if records.is_empty() {
				return Err(Error::EmptyResponse);
			}

			Ok(records)
		}
	}

	/// The `/records/recent` route
	pub mod recent {
		use super::*;

		#[allow(missing_docs)]
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
		pub struct Params {
			pub steam_id: Option<SteamID>,
			pub steamid64: Option<u64>,
			pub map_id: Option<u16>,
			pub map_name: Option<String>,
			#[serde(rename = "has_teleports")]
			pub runtype: Option<Runtype>,
			pub tickrate: Option<u8>,
			pub stage: Option<u8>,
			pub modes_list_string: Option<String>,
			pub modes_list: Option<String>,
			pub place_top_at_least: Option<u32>,
			pub place_top_overall_at_least: Option<u32>,

			#[cfg(feature = "chrono")]
			#[serde(
				serialize_with = "crate::utils::serialize_date_opt",
				deserialize_with = "crate::utils::deserialize_date_opt"
			)]
			pub created_since: Option<DateTime<Utc>>,

			#[cfg(not(feature = "chrono"))]
			pub created_since: Option<String>,

			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl Default for Params {
			fn default() -> Self {
				Self {
					steam_id: None,
					steamid64: None,
					map_id: None,
					map_name: None,
					runtype: None,
					tickrate: None,
					stage: None,
					modes_list_string: None,
					modes_list: None,
					place_top_at_least: None,
					place_top_overall_at_least: None,
					created_since: None,
					offset: None,
					limit: Some(1),
				}
			}
		}

		/// # /records/recent
		///
		/// Fetches recent personal bests
		#[tracing::instrument(
			name = "GlobalAPI request to `/records/recent`",
			level = "trace",
			skip(client)
		)]
		pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Record>> {
			let records: Vec<_> =
				get_json(&format!("{BASE_URL}/records/recent"), params, client).await?;

			if records.is_empty() {
				return Err(Error::EmptyResponse);
			}

			Ok(records)
		}
	}
}
