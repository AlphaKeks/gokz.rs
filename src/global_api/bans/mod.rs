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

/// The different kinds of bans a player could receive.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BanType {
	Other,
	BhopHack,
	BhopMacro,
	StrafeHack,
	BanEvasion,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ban {
	pub id: u32,
	pub ban_type: BanType,
	pub player_name: String,
	pub steam_id: SteamID,
	pub server_id: u16,
	pub stats: String,
	pub notes: String,
	pub updated_by_id: SteamID,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub expires_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	pub expires_on: String,

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

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub ban_types: Option<BanType>,
	/// A `,` separated list of [`BanType`]s
	pub ban_types_list: Option<String>,
	pub is_expired: Option<bool>,
	pub ip: Option<String>,
	pub steamid64: Option<u64>,
	pub steam_id: Option<SteamID>,
	pub notes_contains: Option<String>,
	pub stats_contains: Option<String>,
	pub server_id: Option<u16>,

	#[cfg(feature = "chrono")]
	pub created_since: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_since: Option<String>,

	#[cfg(feature = "chrono")]
	pub updated_since: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub updated_since: Option<String>,

	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			ban_types: None,
			ban_types_list: None,
			is_expired: None,
			ip: None,
			steamid64: None,
			steam_id: None,
			notes_contains: None,
			stats_contains: None,
			server_id: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /bans
///
/// Fetches bans
#[tracing::instrument(name = "GlobalAPI request to `/bans`", level = "TRACE", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Ban>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/bans"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}
