//! `/servers` endpoints
//!
//! Covered:
//! - `/servers`
//! - `/servers/name/:server_name`
//!
//! NOTE: `/servers/:server_id` seems to be broken.

use {
	super::API_URL,
	crate::{http, http::append_pairs, yeet, Result, SteamID},
	reqwest::Url,
	serde::{Deserialize, Serialize},
	std::net::{IpAddr, SocketAddr},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub ip: String,
	pub port: u16,

	#[serde(rename = "owner_steamid64", serialize_with = "SteamID::serialize_as_u64")]
	pub owned_by: SteamID,
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

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	#[serde(skip)]
	pub ids: Option<Vec<u16>>,

	pub name: Option<String>,
	pub ip: Option<String>,
	pub port: Option<u16>,

	#[serde(rename = "owner_steamid64", serialize_with = "SteamID::serialize_opt_as_u64")]
	pub owned_by: Option<SteamID>,

	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/servers` route
///
/// Fetches servers with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_servers_with(params: &Params, client: &http::Client) -> Result<Vec<Server>> {
	let mut url = Url::parse(&format!("{API_URL}/servers")).expect("This is a valid URL.");

	append_pairs!(&mut url, &params.ids, "id");

	let servers = http::get! {
		url = url;
		params = params;
		deserialize = Vec<Server>;
		client = client;
	}?;

	if servers.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(servers)
}

/// `/maps` route
///
/// Fetches maps with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_servers_owned_by(
	server_owner: impl Into<SteamID> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Vec<Server>> {
	let params = Params { owned_by: Some(server_owner.into()), ..Default::default() };

	get_servers_with(&params, client).await
}

/// `/servers/name/:server_name` route
///
/// Fetches a single server with the given `name`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_server(name: &str, client: &http::Client) -> Result<Server> {
	let server = http::get! {
		url = format!("{API_URL}/servers/{name}");
		deserialize = Server;
		client = client;
	}?;

	Ok(server)
}
