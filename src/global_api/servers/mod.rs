use {
	crate::{http, Error, Result, SteamID},
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub owner_steamid: SteamID,
	pub ip: String,
	pub port: u16,
}

/// `/maps` route
pub mod index;
impl TryFrom<index::Response> for Server {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			name: value.name,
			owner_steamid: value.owner_steamid64.parse()?,
			ip: value.ip,
			port: value.port.try_into()?,
		})
	}
}

/// Fetches servers with the given `params`.
pub async fn get_servers(params: index::Params, client: &crate::Client) -> Result<Vec<Server>> {
	let response: Vec<index::Response> =
		http::get_with_params(&format!("{}/servers", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a server by its name.
pub async fn get_server_by_name(server_name: &str, client: &crate::Client) -> Result<Server> {
	http::get::<index::Response>(
		&format!("{}/servers/name/{}", super::BASE_URL, server_name),
		client,
	)
	.await?
	.try_into()
}

/// Fetches a server by its ID.
pub async fn get_server_by_id(server_id: u16, client: &crate::Client) -> Result<Server> {
	let mut servers = http::get::<Vec<index::Response>>(
		&format!("{}/servers?id={}", super::BASE_URL, server_id),
		client,
	)
	.await?;

	if servers.is_empty() {
		return Err(Error::EmptyResponse);
	}

	servers.remove(0).try_into()
}
