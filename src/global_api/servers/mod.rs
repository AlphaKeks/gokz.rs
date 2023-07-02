use {
	crate::{
		error::{Error, Result},
		global_api::BASE_URL,
		http::get_json,
		types::{ServerIdentifier, SteamID},
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Server {
	pub id: u16,
	pub port: u16,
	pub ip: String,
	pub name: String,
	#[serde(rename = "owner_steamid64")]
	pub owner_id: SteamID,
}

impl crate::traits::ServerIdentifier for Server {
	#[inline]
	fn global_api(&self) -> String { ServerIdentifier::Name(self.name.clone()).global_api() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { ServerIdentifier::Id(self.id).schnose_api() }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub id: Option<u16>,
	pub port: Option<u16>,
	pub ip: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "owner_steamid64")]
	pub owner_id: Option<SteamID>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			id: None,
			port: None,
			ip: None,
			name: None,
			owner_id: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /servers
///
/// Fetches servers
#[tracing::instrument(name = "GlobalAPI request to `/servers`", level = "TRACE", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Server>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/servers"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /servers/:server_id
///
/// Fetches a single server by id
#[tracing::instrument(
	name = "GlobalAPI request to `/servers/:server_id`",
	level = "TRACE",
	skip(client)
)]
pub async fn id(server_id: u16, client: &crate::Client) -> Result<Server> {
	get_json(&format!("{BASE_URL}/servers/{server_id}"), &EmptyParams, client).await
}

/// # /servers/name/:server_name
///
/// Fetches a single server by name
#[tracing::instrument(
	name = "GlobalAPI request to `/servers/id/:name`",
	level = "TRACE",
	skip(client)
)]
pub async fn name(server_name: &str, client: &crate::Client) -> Result<Server> {
	get_json(&format!("{BASE_URL}/servers/{server_name}"), &EmptyParams, client).await
}
