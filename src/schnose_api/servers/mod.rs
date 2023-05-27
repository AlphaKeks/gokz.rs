use crate::{http, Error, Result, ServerIdentifier};

/// `/maps` route
pub mod index;

/// Fetches servers with the given `params`.
pub async fn get_servers(
	params: index::Params,
	client: &crate::Client,
) -> Result<Vec<index::Server>> {
	let response: Vec<index::Server> =
		http::get_with_params(&format!("{}/servers", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// Fetches a single server.
pub async fn get_server(
	server_identifier: &ServerIdentifier,
	client: &crate::Client,
) -> Result<index::Server> {
	http::get::<index::Server>(
		&format!("{}/servers/{}", super::BASE_URL, server_identifier),
		client,
	)
	.await
}
