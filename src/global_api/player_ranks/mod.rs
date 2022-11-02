/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("player_ranks?")
}

#[derive(Debug, Clone, serde::Serialize)]
/// All possible parameters for this route
pub struct Params {
	pub points_greater_than: Option<u32>,
	pub average_greater_than: Option<u32>,
	pub rating_greater_than: Option<u32>,
	pub finishes_greater_than: Option<u32>,
	pub steamid64s: Option<u64>,
	pub record_filter_ids: Option<u32>,
	pub map_ids: Option<u16>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
	#[serde(rename = "camelCase")]
	pub map_tag: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Params {
			points_greater_than: None,
			average_greater_than: None,
			rating_greater_than: None,
			finishes_greater_than: None,
			steamid64s: None,
			record_filter_ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: Some(128),
			has_teleports: None,
			map_tag: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::IsParams for Params {}

#[derive(Debug, Clone, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct Response {
	pub points: u32,
	pub average: u32,
	pub rating: u32,
	pub finishes: u32,
	pub steamid64: String,
	pub steamid: String,
	pub player_name: String,
}

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}
