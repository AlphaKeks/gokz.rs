#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::{MapIdentifier, SteamID, Tier},
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub difficulty: Tier,
	pub validated: bool,
	pub filesize: u64,
	pub approved_by: Option<SteamID>,
	pub workshop_url: Option<String>,
	pub download_url: Option<String>,

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

impl crate::traits::MapIdentifier for Map {
	#[inline]
	fn image_url(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).image_url() }

	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String { MapIdentifier::Name(self.name.clone()).global_api() }

	#[inline]
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).kzgo() }

	#[inline]
	#[cfg(feature = "kzgo-api")]
	fn kzgo_api(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).kzgo_api() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { MapIdentifier::Id(self.id).schnose_api() }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub id: Option<u16>,
	pub name: Option<String>,
	pub larger_than_filesize: Option<u64>,
	pub smaller_than_filesize: Option<u64>,
	pub is_validated: Option<bool>,
	pub difficulty: Option<Tier>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date_opt",
		deserialize_with = "crate::utils::deserialize_date_opt"
	)]
	pub created_on: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: Option<String>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date_opt",
		deserialize_with = "crate::utils::deserialize_date_opt"
	)]
	pub updated_on: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub updated_on: Option<String>,

	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			id: None,
			name: None,
			larger_than_filesize: None,
			smaller_than_filesize: None,
			is_validated: None,
			difficulty: None,
			created_on: None,
			updated_on: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /maps
///
/// Fetches maps
#[tracing::instrument(name = "GlobalAPI request to `/maps`", level = "TRACE", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Map>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/maps"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /maps/id/:map_id
///
/// Fetches a single map by id
#[tracing::instrument(
	name = "GlobalAPI request to `/maps/id/:map_id`",
	level = "TRACE",
	skip(client)
)]
pub async fn id(map_id: u16, client: &crate::Client) -> Result<Map> {
	get_json(&format!("{BASE_URL}/maps/{map_id}"), &EmptyParams, client).await
}

/// # /maps/name/:map_name
///
/// Fetches a single map by name
#[tracing::instrument(
	name = "GlobalAPI request to `/maps/id/:name`",
	level = "TRACE",
	skip(client)
)]
pub async fn name(map_name: &str, client: &crate::Client) -> Result<Map> {
	get_json(&format!("{BASE_URL}/maps/{map_name}"), &EmptyParams, client).await
}
