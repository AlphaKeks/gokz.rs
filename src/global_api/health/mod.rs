pub fn get_url() -> String {
	String::from("https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/bans` route
pub struct Params;

impl Default for Params {
	fn default() -> Self {
		Params
	}
}

impl super::IsParams for Params {}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusResult {
	pub status: u16,
	pub hostname: String,
	pub duration: u32,
	pub condition_results: [ConditionResult; 2],
	pub success: bool,
	pub timestamp: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response for a [Health Check](https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1)
pub struct Response {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
}

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Fancy {
	pub successful_responses: u8,
	pub fast_responses: u8,
}
