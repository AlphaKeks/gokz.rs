use std::fmt::Display;

/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResult {
	pub status: u16,
	pub hostname: String,
	pub duration: u32,
	pub condition_results: [ConditionResult; 2],
	pub success: bool,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response for a [Health Check](https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1)
pub struct HealthResponse {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
}

impl Display for HealthResponse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"
			Response {{
				name: {},
				key: {},
				results: {:?},
				events: {:?},
			}}
        	",
			self.name, self.key, self.results, self.events
		)
	}
}

impl super::IsResponse for HealthResponse {}
impl super::IsResponse for Vec<HealthResponse> {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FancyHealthReport {
	pub successful_responses: u8,
	pub fast_responses: u8,
}

impl Display for FancyHealthReport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.successful_responses, self.fast_responses)
	}
}
