//! `/wrs/:mode` endpoint

use {
	super::API_URL,
	crate::{http, Mode, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldRecord {
	pub map_id: u16,
	pub map_name: String,
	pub pro: bool,
	pub steam_id: SteamID,
	pub time: f64,

	#[serde(rename = "tps")]
	pub teleports: u32,

	pub server_id: u16,
	pub server_name: String,
	pub diff: Option<f64>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date",
		deserialize_with = "crate::serde::chrono::deserialize_date"
	)]
	pub created_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,
}

/// `/wrs/:mode_name` route
///
/// Fetches all world records for the given mode.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_world_records(
	mode: impl Into<Mode> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<Vec<WorldRecord>> {
	http::get! {
		url = format!("{API_URL}/wrs/{}", mode.into().api());
		deserialize = Vec<WorldRecord>;
		client = client;
	}
}
