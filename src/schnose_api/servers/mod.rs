use {
	crate::{http, Error, Result, ServerIdentifier, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub owner_name: String,
	pub owner_steam_id: SteamID,
	pub approver_name: String,
	pub approver_steam_id: SteamID,
}

/// `/maps` route
pub mod index;
impl TryFrom<index::Response> for Server {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		let owner_steam_id = if let Ok(steam_id) = value.owned_by.steam_id.parse() {
			steam_id
		} else {
			value.owned_by.steam_id64.parse()?
		};

		let approver_steam_id = if let Ok(steam_id) = value.approved_by.steam_id.parse() {
			steam_id
		} else {
			value.approved_by.steam_id64.parse()?
		};

		Ok(Self {
			id: value.id,
			name: value.name,
			owner_name: value.owned_by.name,
			owner_steam_id,
			approver_name: value.approved_by.name,
			approver_steam_id,
		})
	}
}

/// Fetches servers with the given `params`.
pub async fn get_servers(params: index::Params, client: &crate::Client) -> Result<Vec<Server>> {
	let response: super::Response<Vec<index::Response>> =
		http::get_with_params(&format!("{}/servers", super::BASE_URL), params, client).await?;

	if response.result.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.result
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a single server.
pub async fn get_server(
	server_identifier: &ServerIdentifier,
	client: &crate::Client,
) -> Result<Server> {
	http::get::<super::Response<index::Response>>(
		&format!("{}/servers/{}", super::BASE_URL, server_identifier),
		client,
	)
	.await?
	.result
	.try_into()
}
