//! `/servers` endpoints
//!
//! Covered:
//! - `/servers`
//! - `/servers/:server_identifier`

use {
	super::API_URL,
	crate::{http, yeet, PlayerIdentifier, Result, ServerIdentifier, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub owner_id: SteamID,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub owner: Option<PlayerIdentifier>,
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
	let servers = http::get! {
		url = format!("{API_URL}/servers");
		params = params;
		deserialize = Vec<Server>;
		client = client;
	}?;

	if servers.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(servers)
}

/// `/servers` route
///
/// Fetches servers owned by the given `server_owner`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_servers_owned_by(
	server_owner: impl Into<PlayerIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Vec<Server>> {
	let params = Params { owner: Some(server_owner.into()), ..Default::default() };

	get_servers_with(&params, client).await
}

/// `/servers/:server_identifier` route
///
/// Fetches a single server by its name or ID.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_server(
	server: impl Into<ServerIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<Server> {
	let server = http::get! {
		url = format!("{API_URL}/servers/{}", server.into());
		deserialize = Server;
		client = client;
	}?;

	Ok(server)
}
