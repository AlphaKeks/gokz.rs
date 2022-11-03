use std::fmt::Display;

/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1")
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Params;

impl Default for Params {
	fn default() -> Self {
		Params
	}
}

impl super::IsParams for Params {}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResult {
	pub status: u16,
	pub hostname: String,
	pub duration: u32,
	pub condition_results: [ConditionResult; 2],
	pub success: bool,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response for a [Health Check](https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1)
pub struct Response {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
}

impl Display for Response {
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

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Fancy {
	pub successful_responses: u8,
	pub fast_responses: u8,
}

impl Display for Fancy {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.successful_responses, self.fast_responses)
	}
}
