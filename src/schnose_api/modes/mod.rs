use {
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mode {
	pub id: u8,
	pub name: String,
}

/// # /modes
///
/// Fetches all modes
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn root(client: &crate::Client) -> Result<Vec<Mode>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/modes"), &EmptyParams, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /modes/:ident
///
/// Fetches a single mode
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn ident(mode: prelude::Mode, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{}", mode as u8), &EmptyParams, client).await
}
