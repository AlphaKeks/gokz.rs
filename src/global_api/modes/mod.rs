#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::SteamID,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mode {
	pub id: u8,
	pub name: String,
	pub description: String,
	pub latest_version: u32,
	pub latest_version_description: String,
	pub website: String,
	pub repo: String,
	pub contact_steamid64: SteamID,
	pub supported_tickrates: Option<serde_json::Value>,

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

	pub updated_by_id: SteamID,
}

/// # /modes
///
/// Fetches all modes
#[tracing::instrument(
	name = "GlobalAPI request to `/modes`",
	level = "trace"
	skip(client),
	err(Debug),
)]
pub async fn root(client: &crate::Client) -> Result<Vec<Mode>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/modes"), &[()], client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /modes/id/:mode_id
///
/// Fetches a single mode by id
#[tracing::instrument(
	name = "GlobalAPI request to `/modes/id/:mode_id`",
	level = "trace"
	skip(client),
	err(Debug),
)]
pub async fn id(mode_id: u8, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{mode_id}"), &[()], client).await
}

/// # /modes/name/:mode_name
///
/// Fetches a single mode by name
#[tracing::instrument(
	name = "GlobalAPI request to `/modes/id/:name`",
	level = "trace"
	skip(client),
	err(Debug),
)]
pub async fn name(mode_name: &str, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{mode_name}"), &[()], client).await
}
