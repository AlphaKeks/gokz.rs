//! `/maps` endpoints
//!
//! Covered:
//! - `/maps`
//! - `/maps/name/:map_name`

use {
	super::API_URL,
	crate::{http, yeet, Result, SteamID, Tier},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub bonuses: u8,

	#[serde(
		rename = "workshopId",
		deserialize_with = "crate::serde::kzgo::deserialize_workshop_id"
	)]
	pub workshop_id: Option<u32>,

	/// Is the map possible in SKZ?
	#[serde(rename = "sp")]
	pub skz: bool,

	/// Is the map possible in VNL?
	#[serde(rename = "vp")]
	pub vnl: bool,

	#[serde(
		rename = "mapperNames",
		deserialize_with = "crate::serde::kzgo::deserialize_mapper_names"
	)]
	pub mapper_names: Vec<String>,

	#[serde(rename = "mapperIds", deserialize_with = "crate::serde::kzgo::deserialize_mapper_ids")]
	pub mapper_ids: Vec<SteamID>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date",
		deserialize_with = "crate::serde::chrono::deserialize_date"
	)]
	pub date: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub date: String,
}

/// `/maps/:map_name` route
///
/// Fetches a single map by name.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_map(map_name: &str, client: &crate::http::Client) -> Result<Map> {
	http::get! {
		url = format!("{API_URL}/maps/{map_name}");
		deserialize = Map;
		client = client;
	}
}

/// `/maps` route
///
/// Fetches all maps.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maps(client: &crate::http::Client) -> Result<Vec<Map>> {
	let maps = http::get! {
		url = format!("{API_URL}/maps");
		deserialize = Vec<Map>;
		client = client;
	}?;

	if maps.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(maps)
}
