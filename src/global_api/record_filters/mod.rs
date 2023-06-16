#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::{Mode, Runtype, SteamID},
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordFilter {
	pub id: u32,
	pub map_id: u16,
	pub stage: u8,
	#[serde(rename = "mode_id")]
	pub mode: Mode,
	#[serde(rename = "has_teleports")]
	pub runtype: Runtype,
	pub tickrate: u8,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub created_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,

	#[serde(rename = "updated_by_id")]
	pub updated_by: Option<SteamID>,
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
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /record_filters
///
/// Fetches record filters
#[tracing::instrument(
	name = "GlobalAPI request to `/record_filters`",
	level = "trace"
	skip(client),
	err(Debug),
)]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<RecordFilter>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/record_filters"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}
