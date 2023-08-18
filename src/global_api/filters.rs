//! `/record_filters` endpoint
//!
//! NOTE: `/record_filters/distributions` is not supported because I have no idea what it does.

use {
	super::API_URL,
	crate::{http, http::append_pairs, yeet, Mode, Result, Runtype, SteamID},
	reqwest::Url,
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordFilter {
	pub id: u16,
	pub map_id: u16,
	pub stage: u8,
	#[serde(rename = "mode_id")]
	pub mode: Mode,
	pub tickrate: u8,
	#[serde(rename = "has_teleports")]
	pub runtype: Runtype,
	#[serde(rename = "updated_by_id")]
	pub updated_by: SteamID,

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

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	#[serde(skip)]
	pub ids: Option<Vec<u16>>,

	#[serde(skip)]
	pub map_ids: Option<Vec<u16>>,

	#[serde(skip)]
	pub stages: Option<Vec<u8>>,

	#[serde(skip)]
	pub modes: Option<Vec<Mode>>,

	#[serde(rename = "tickrates")]
	pub tickrate: Option<u8>,

	#[serde(rename = "has_teleports")]
	pub runtype: Option<Runtype>,

	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/record_filters` route
///
/// Fetches the record filters for the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_filters_with(params: &Params, client: &http::Client) -> Result<Vec<RecordFilter>> {
	let mut url = Url::parse(&format!("{API_URL}/record_filters")).expect("This is a valid URL.");

	append_pairs!(&mut url, &params.ids, "ids");
	append_pairs!(&mut url, &params.map_ids, "map_ids");
	append_pairs!(&mut url, &params.stages, "stages");
	append_pairs!(&mut url, &params.modes, "modes");

	let bans = http::get! {
		url = url;
		params = params;
		deserialize = Vec<RecordFilter>;
		client = client;
	}?;

	if bans.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(bans)
}
