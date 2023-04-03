use {
	crate::{http, Result},
	log::trace,
	serde::{Deserialize, Serialize},
};

/// Check the health of the GlobalAPI.
pub async fn checkhealth(client: &crate::Client) -> Result<HealthReport> {
	trace!("> checkhealth");
	http::get::<RawHealthReport>(
		"https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1",
		client,
	)
	.await
	.map(|report| HealthReport {
		successful_responses: report.results[0..10]
			.iter()
			.filter(|result| result.condition_results[0].success)
			.count() as u8,
		fast_responses: report.results[0..10]
			.iter()
			.filter(|result| result.condition_results[1].success)
			.count() as u8,
	})
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
pub struct StatusResult {
	pub status: u64,
	pub hostname: String,
	pub duration: u64,
	pub condition_results: [ConditionResult; 2],
	pub success: bool,
	pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RawHealthReport {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HealthReport {
	pub successful_responses: u8,
	pub fast_responses: u8,
}
