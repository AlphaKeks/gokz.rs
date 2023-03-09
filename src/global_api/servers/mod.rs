use {
	crate::{
		http::{get, get_with_params},
		Error, Result, SteamID,
	},
	serde::Serialize,
};

pub type ServerID = u16;

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Server {
	pub id: ServerID,
	pub port: u16,
	pub ip: String,
	pub name: String,
	pub owner_steamid: SteamID,
}

/// `/maps` route
pub mod index;
impl TryFrom<index::Response> for Server {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			port: value.port.try_into()?,
			ip: value.ip,
			name: value.name,
			owner_steamid: value.owner_steamid64.parse()?,
		})
	}
}

/// Fetches servers with the given `params`.
pub async fn get_servers(params: index::Params, client: &reqwest::Client) -> Result<Vec<Server>> {
	let response: Vec<index::Response> =
		get_with_params(&format!("{}/servers", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a server by its name.
pub async fn get_server_by_name(server_name: &str, client: &reqwest::Client) -> Result<Server> {
	get::<index::Response>(
		&format!("{}/servers/name/{}", super::BASE_URL, server_name),
		client,
	)
	.await?
	.try_into()
}

/// Fetches a server by its ID.
pub async fn get_server_by_id(server_id: ServerID, client: &reqwest::Client) -> Result<Server> {
	get::<index::Response>(
		&format!("{}/servers/id/{}", super::BASE_URL, server_id),
		client,
	)
	.await?
	.try_into()
}
