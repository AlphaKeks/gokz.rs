#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	super::Player,
	crate::{
		error::{Error, Result},
		http::get_json,
		prelude,
		schnose_api::BASE_URL,
		types::MapIdentifier,
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub global: bool,
	pub courses: Vec<Course>,
	pub workshop_id: Option<u32>,
	pub filesize: Option<u64>,
	pub mappers: Option<Vec<Player>>,
	pub created_on: DateTime<Utc>,
	pub updated_on: DateTime<Utc>,
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
	fn schnose_api(&self) -> String { MapIdentifier::Id(self.id).schnose_api() }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
	pub id: u32,
	pub map_id: u16,
	pub stage: u8,
	pub tier: Option<prelude::Tier>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub mapper: Option<prelude::PlayerIdentifier>,
	pub global: Option<bool>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date_opt",
		deserialize_with = "crate::utils::deserialize_date_opt"
	)]
	pub created_after: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_after: Option<String>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date_opt",
		deserialize_with = "crate::utils::deserialize_date_opt"
	)]
	pub created_before: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_before: Option<String>,

	pub offset: Option<i64>,
	pub limit: Option<u64>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			mapper: None,
			global: None,
			created_after: None,
			created_before: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /maps
///
/// Fetches maps
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Map>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/maps"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /maps/:ident
///
/// Fetches a single map
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn ident(map_identifier: prelude::MapIdentifier, client: &crate::Client) -> Result<Map> {
	get_json(&format!("{BASE_URL}/maps/{map_identifier}"), &EmptyParams, client).await
}
