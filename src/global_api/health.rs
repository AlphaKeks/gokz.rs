//! GlobalAPI Health Dashboard: <https://health.global-api.com/endpoints/_globalapi>
//!
//! This module gives programmatic access to this endpoint.

use {
	crate::{http, Result},
	serde::{Deserialize, Serialize},
};

/// The URL for the API's health endpoint.
pub const URL: &str = "https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1";

/// A summary of the last 10 health check requests to the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Health {
	/// The amount of successful healthchecks (out of 10)
	pub successful: u8,

	/// The amount of fast responses (out of 10)
	pub fast: u8,
}

/// Fetches a summary for the latest 10 health checks.
pub async fn healthcheck(client: &http::Client) -> Result<Health> {
	let response = http::get! {
		url = URL;
		deserialize = Response;
		client = client;
	}?;

	let (successful, fast) =
		response.results[..10]
			.iter()
			.fold((0, 0), |(success, fast), result| {
				let success = result.condition_results[0].success as u8 + success;
				let fast = result.condition_results[1].success as u8 + fast;
				(success, fast)
			});

	Ok(Health { successful, fast })
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
	pub results: Vec<Status>,

	#[serde(flatten)]
	pub body: serde_json::Value,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
	#[serde(rename = "conditionResults")]
	pub condition_results: [ConditionResult; 2],

	#[serde(flatten)]
	pub body: serde_json::Value,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionResult {
	pub success: bool,

	#[serde(flatten)]
	pub body: serde_json::Value,
}
