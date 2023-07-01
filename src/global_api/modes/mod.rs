#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		prelude,
		types::SteamID,
		utils::EmptyParams,
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

impl crate::traits::Mode for Mode {
	#[inline]
	fn api(&self) -> String {
		prelude::Mode::try_from(self.id)
			.expect("Mode returned by the GlobalAPI should be a valid `Mode` type.")
			.api()
	}

	#[inline]
	fn short(&self) -> String {
		prelude::Mode::try_from(self.id)
			.expect("Mode returned by the GlobalAPI should be a valid `Mode` type.")
			.short()
	}
}

/// # /modes
///
/// Fetches all modes
#[tracing::instrument(
	name = "GlobalAPI request to `/modes`",
	level = "TRACE",
	skip(client),
	err(Debug)
)]
pub async fn root(client: &crate::Client) -> Result<Vec<Mode>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/modes"), &EmptyParams, client).await?;

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
	level = "TRACE",
	skip(client),
	err(Debug)
)]
pub async fn id(mode_id: u8, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{mode_id}"), &EmptyParams, client).await
}

/// # /modes/name/:mode_name
///
/// Fetches a single mode by name
#[tracing::instrument(
	name = "GlobalAPI request to `/modes/id/:name`",
	level = "TRACE",
	skip(client),
	err(Debug)
)]
pub async fn name(mode_name: &str, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{mode_name}"), &EmptyParams, client).await
}
