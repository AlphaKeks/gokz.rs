//! `/maps` endpoints
//!
//! Covered:
//! - `/maps`
//! - `/maps/:map_identifier`

use {
	super::API_URL,
	crate::{http, yeet, MapIdentifier, PlayerIdentifier, Result, SteamID, Tier},
	serde::{Deserialize, Serialize},
	std::ops::Deref,
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub global: bool,
	pub courses: Vec<Course>,
	pub mappers: Vec<Mapper>,
	pub workshop_id: Option<u32>,
	pub filesize: u64,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date",
		deserialize_with = "super::serde::chrono::deserialize_date"
	)]
	pub created_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: String,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date",
		deserialize_with = "super::serde::chrono::deserialize_date"
	)]
	pub updated_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub updated_on: String,
}

impl Map {
	/// Checks whether the map is global or not.
	pub const fn is_global(&self) -> bool {
		self.global
	}

	/// Extracts the [`Tier`] for the main course on this [`Map`].
	///
	/// DawnAPI has tiers per course instead of per map, and not all courses have a [`Tier`],
	/// the main course however does.
	///
	/// # Panics
	///
	/// This code assumes invariants about DawnAPI:
	///
	/// 1. `self.courses` is non-empty.
	/// 2. `self.courses[0]` is the main course of the map.
	/// 3. `self.courses[0].tier` is [`Some`] because every main course is tiered.
	///
	/// If you construct a [`Map`] yourself, it needs to adhere to these invariants, otherwise
	/// this method will panic.
	pub fn tier(&self) -> Tier {
		self.courses[0]
			.tier
			.expect("DawnAPI maps always have at least 1 course with a `Tier`.")
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

	/// Returns a link to the map's Steam Workshop page if it has a known one.
	pub fn workshop_url(&self) -> Option<String> {
		self.workshop_id
			.map(|id| format!("https://steamcommunity.com/sharedfiles/filedetails/?id={id}"))
	}

	/// Returns a link to fetch this map from DawnAPI.
	pub fn api(&self) -> String {
		format!("{API_URL}/maps/{}", self.id)
	}

	/// Returns a link to fetch this map from the GlobalAPI.
	pub fn global_api(&self) -> String {
		format!("{API_URL}/maps/name/{}", self.name)
	}

	/// Returns a link to the map's KZ:GO page.
	pub fn kzgo(&self) -> String {
		format!("https://kzgo.eu/maps/{}", self.name)
	}
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Course {
	pub id: u32,
	pub stage: u8,
	pub tier: Option<Tier>,

	/// The course has a filter for KZT.
	pub kzt: bool,

	/// The course has a filter for SKZ.
	pub skz: bool,

	/// The course has a filter for VNL.
	pub vnl: bool,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Mapper {
	pub id: SteamID,
	pub name: String,
}

impl Deref for Mapper {
	type Target = SteamID;

	fn deref(&self) -> &Self::Target {
		&self.id
	}
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub global: Option<bool>,
	pub tier: Option<Tier>,
	pub mapper: Option<PlayerIdentifier>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date_opt",
		deserialize_with = "crate::serde::chrono::deserialize_date_opt"
	)]
	pub created_after: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_after: Option<String>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date_opt",
		deserialize_with = "crate::serde::chrono::deserialize_date_opt"
	)]
	pub created_before: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_before: Option<String>,

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
	let params = Params { global: Some(global), limit: Some(9999), ..Default::default() };

	get_maps_with(&params, client).await
}

/// `/maps/:map_identifier` route
///
/// Fetches a single map either by its name or ID.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_map(
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Map> {
	let map = http::get! {
		url = format!("{API_URL}/maps/{}", map.into());
		deserialize = Map;
		client = client;
	}?;

	Ok(map)
}

/// `/maps` route
///
/// Fetches all maps made by a specific player.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_maps_by(
	mapper: impl Into<PlayerIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Vec<Map>> {
	let params = Params { mapper: Some(mapper.into()), ..Default::default() };

	get_maps_with(&params, client).await
}
