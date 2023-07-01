#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	super::Course,
	crate::{
		error::{Error, Result},
		http::get_json,
		prelude,
		schnose_api::BASE_URL,
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
	pub id: u32,
	pub course: Course,
	pub map_name: String,
	pub mode: prelude::Mode,
	pub steam_id: prelude::SteamID,
	pub player_name: String,
	pub server_id: u16,
	pub server_name: String,
	pub time: f64,
	pub teleports: u16,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::utils::serialize_date_opt",
		deserialize_with = "crate::utils::deserialize_date_opt"
	)]
	pub created_on: Option<DateTime<Utc>>,

	#[cfg(not(feature = "chrono"))]
	pub created_on: Option<String>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
	pub player: Option<prelude::PlayerIdentifier>,
	pub mode: Option<prelude::Mode>,
	pub runtype: Option<prelude::Runtype>,
	pub map: Option<prelude::MapIdentifier>,
	pub stage: Option<u8>,
	pub server: Option<prelude::ServerIdentifier>,

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
			player: None,
			mode: None,
			runtype: None,
			map: None,
			stage: None,
			server: None,
			created_after: None,
			created_before: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /records
///
/// Fetches records
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Record>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/records"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /records/:id
///
/// Fetches a single record by id
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn ident(record_id: u32, client: &crate::Client) -> Result<Record> {
	get_json(&format!("{BASE_URL}/records/{record_id}"), &EmptyParams, client).await
}
