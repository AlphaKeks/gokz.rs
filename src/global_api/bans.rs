//! `/bans` endpoint

use {
	super::API_URL,
	crate::{http, yeet, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ban {
	pub id: u32,
	pub ban_type: BanType,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date",
		deserialize_with = "super::serde::chrono::deserialize_date"
	)]
	pub expires_on: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub expires_on: String,

	pub player_name: String,
	pub steam_id: SteamID,
	pub notes: String,
	pub stats: String,
	pub server_id: u16,
	pub updated_by_id: String,

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

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BanType {
	BhopHack,
	BhopMacro,
	StrafeHack,
	BanEvasion,
	Other,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	#[serde(
		serialize_with = "super::serde::serialize_ban_types",
		deserialize_with = "super::serde::deserialize_ban_types"
	)]
	pub ban_types: Option<Vec<BanType>>,

	pub is_expired: Option<bool>,
	pub ip: Option<String>,
	pub steam_id: Option<SteamID>,
	pub notes_contains: Option<String>,
	pub stats_contains: Option<String>,
	pub server_id: Option<u16>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date_opt",
		deserialize_with = "super::serde::chrono::deserialize_date_opt"
	)]
	pub created_since: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_since: Option<String>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "super::serde::chrono::serialize_date_opt",
		deserialize_with = "super::serde::chrono::deserialize_date_opt"
	)]
	pub updated_since: Option<chrono::DateTime<chrono::Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub updated_since: Option<String>,

	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/bans` route
///
/// Fetches the ban data for the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_bans_with(params: &Params, client: &http::Client) -> Result<Vec<Ban>> {
	let bans = http::get! {
		url = format!("{API_URL}/bans");
		params = params;
		deserialize = Vec<Ban>;
		client = client;
	}?;

	if bans.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(bans)
}
