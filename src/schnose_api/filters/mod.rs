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
pub struct Filter {
	pub map_id: u16,
	pub map_name: String,
	pub stage: u8,
	pub kzt: bool,
	pub skz: bool,
	pub vnl: bool,
}

/// # /filters/:map_identifier
///
/// Fetches filters for a given map
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn map(
	map_identifier: prelude::MapIdentifier,
	client: &crate::Client,
) -> Result<Vec<Filter>> {
	let response: Vec<_> =
		get_json(&format!("{BASE_URL}/filters/map/{map_identifier}"), &EmptyParams, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}
