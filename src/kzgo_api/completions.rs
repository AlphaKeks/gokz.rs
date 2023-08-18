//! `/completions/:mode` endpoint

use {
	super::API_URL,
	crate::{http, Mode, Result},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionStats {
	pub mode: Mode,
	pub tp: CompletionCount,
	pub pro: CompletionCount,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionCount {
	#[serde(rename(deserialize = "1"))]
	pub one: u16,
	#[serde(rename(deserialize = "2"))]
	pub two: u16,
	#[serde(rename(deserialize = "3"))]
	pub three: u16,
	#[serde(rename(deserialize = "4"))]
	pub four: u16,
	#[serde(rename(deserialize = "5"))]
	pub five: u16,
	#[serde(rename(deserialize = "6"))]
	pub six: u16,
	#[serde(rename(deserialize = "7"))]
	pub seven: u16,
	pub total: u16,
}

/// `/completions/:mode` route
///
/// Fetches the total amount of completions for the given `mode`.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_completions(
	mode: impl Into<Mode> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<CompletionStats> {
	http::get! {
		url = format!("{API_URL}/completions/{}", mode.into().api());
		deserialize = CompletionStats;
		client = client;
	}
}
