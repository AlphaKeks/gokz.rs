use serde::{Deserialize, Serialize};

/// The URL for getting health stats fro the GlobalAPI
pub const URL: &str = "https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HealthReport {
	pub successful_responses: u8,
	pub fast_responses: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
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
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}
