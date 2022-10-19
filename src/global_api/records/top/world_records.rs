pub fn get_url() -> String {
	String::from("records/top/world_records?")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/records/top/world_records` route
pub struct Params {
	pub ids: Option<Vec<u32>>,
	pub map_ids: Option<Vec<u16>>,
	pub stages: Option<Vec<u8>>,
	pub mode_ids: Option<Vec<u8>>,
	pub tickrates: Option<Vec<u8>>,
	pub has_teleports: Option<bool>,
	#[serde(rename = "camelCase")]
	pub map_tag: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Params {
			ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: None,
			has_teleports: None,
			map_tag: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::super::super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/records/top/world_records` route
pub struct Response {
	steamid64: String,
	steam_id: Option<String>,
	count: u32,
	player_name: Option<String>,
}

impl super::super::super::IsResponse for Response {}
impl super::super::super::IsResponse for Vec<Response> {}
