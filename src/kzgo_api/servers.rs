//! `/servers` endpoint

use {
	super::API_URL,
	crate::{http, Result, Tier},
	serde::{Deserialize, Serialize},
	std::net::{IpAddr, SocketAddr},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStates {
	pub server_states: Vec<Server>,

	#[cfg(feature = "chrono")]
	#[serde(
		serialize_with = "crate::serde::chrono::serialize_date",
		deserialize_with = "crate::serde::chrono::deserialize_date"
	)]
	pub last_updated: chrono::DateTime<chrono::Utc>,

	#[cfg(not(feature = "chrono"))]
	pub last_updated: String,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
	pub r#type: String,
	pub ip: String,
	pub port: u16,
	pub label: String,
	pub name: String,
	pub map: Map,
	pub players: Vec<Player>,
	pub tags: Vec<String>,

	#[serde(rename = "errBefore")]
	pub err_before: bool,

	#[serde(rename = "maxPlayers")]
	pub max_players: u32,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
	pub name: String,
	pub tier: Tier,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub raw: Raw,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Raw {
	pub score: Option<u32>,
	pub time: Option<f64>,
}

impl Server {
	/// Returns the server's IP address as a [`SocketAddr`].
	pub fn ip_addr(&self) -> Option<SocketAddr> {
		self.ip
			.parse::<IpAddr>()
			.map(|ip| SocketAddr::new(ip, self.port))
			.ok()
	}
}

/// `/servers` route
///
/// Fetches all servers.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_servers(client: &crate::http::Client) -> Result<Vec<Server>> {
	http::get! {
		url = format!("{API_URL}/servers");
		deserialize = ServerStates;
		client = client;
	}
	.map(|result| result.server_states)
}
