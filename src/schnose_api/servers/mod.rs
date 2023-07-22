use {
	super::Player,
	crate::{
		error::{Error, Result},
		http::get_json,
		prelude,
		schnose_api::BASE_URL,
		types::ServerIdentifier,
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub owned_by: Player,
}

impl crate::traits::ServerIdentifier for Server {
	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String { ServerIdentifier::Name(self.name.clone()).global_api() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { ServerIdentifier::Id(self.id).schnose_api() }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub owned_by: Option<prelude::PlayerIdentifier>,
	pub offset: Option<i64>,
	pub limit: Option<u64>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			owned_by: None,
			offset: None,
			limit: Some(1),
		}
	}
}

/// # /servers
///
/// Fetches servers
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn root(params: &Params, client: &crate::Client) -> Result<Vec<Server>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/servers"), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /servers/:ident
///
/// Fetches a single server
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn ident(
	server_identifier: prelude::ServerIdentifier,
	client: &crate::Client,
) -> Result<Server> {
	get_json(&format!("{BASE_URL}/servers/{server_identifier}"), &EmptyParams, client).await
}
