//! `/maps` endpoints
//!
//! Covered:
//! - `/maps`
//! - `/maps/name/:map_name`
//!
//! NOTE: `/maps/:map_id` seems to be broken.

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
	pub filesize: u64,
	pub validated: bool,
	pub difficulty: Tier,

	#[serde(rename = "approved_by_steamid64", serialize_with = "SteamID::serialize_opt_as_u64")]
	pub approved_by: Option<SteamID>,

	pub workshop_url: Option<String>,
	pub download_url: Option<String>,

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

impl Map {
	/// Checks whether the map is global or not.
	pub const fn is_global(&self) -> bool {
		self.validated
	}

	/// Returns a URL to an image of the map.
	pub fn image_url(&self) -> String {
		format!(
			"https://raw.githubusercontent.com/KZGlobalTeam/map-images/master/images/{}.jpg",
			self.name
		)
	}

	/// Returns a link to download the map as a `.bsp` file.
	pub fn download_url(&self) -> String {
		format!("https://maps.global-api.com/bsps/{}.bsp", self.name)
	}

	/// Returns the map's Steam Workshop ID if it has a known one.
	pub fn workshop_id(&self) -> Option<u32> {
		self.workshop_url
			.as_ref()
			.and_then(|url| url.rsplit_once('=').map(|(_, id)| id))
			.and_then(|id| id.parse::<u32>().ok())
	}

	/// Returns a link to fetch this map from the GlobalAPI.
	pub fn api(&self) -> String {
		format!("{API_URL}/maps/name/{}", self.name)
	}

	/// Returns a link to the map's KZ:GO page.
	pub fn kzgo(&self) -> String {
		format!("https://kzgo.eu/maps/{}", self.name)
	}
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub id: Option<u16>,
	pub name: Option<String>,
	pub larger_than_filesize: Option<u64>,
	pub smaller_than_filesize: Option<u64>,
	pub is_validated: Option<bool>,
	pub difficulty: Option<Tier>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date_opt",
		deserialize_with = "crate::serde::chrono::deserialize_date_opt"
	)]
	pub created_since: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_since: Option<String>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date_opt",
		deserialize_with = "crate::serde::chrono::deserialize_date_opt"
	)]
	pub updated_since: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub updated_since: Option<String>,

	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/maps` route
///
/// Fetches maps with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maps_with(params: &Params, client: &http::Client) -> Result<Vec<Map>> {
	let maps = http::get! {
		url = format!("{API_URL}/maps");
		params = params;
		deserialize = Vec<Map>;
		client = client;
	}?;

	if maps.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(maps)
}

/// `/maps` route
///
/// Fetches maps with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maps(global: bool, client: &http::Client) -> Result<Vec<Map>> {
	let params = Params { is_validated: Some(global), limit: Some(9999), ..Default::default() };

	get_maps_with(&params, client).await
}

/// `/maps/name/:map_name` route
///
/// Fetches a single map with the given `name`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_map(name: &str, client: &http::Client) -> Result<Map> {
	let map = http::get! {
		url = format!("{API_URL}/maps/{name}");
		deserialize = Map;
		client = client;
	}?;

	Ok(map)
}
