/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("records/top/world_records?")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// All possible parameters for this route
pub struct WorldRecordParams {
	pub ids: Option<u32>,
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

impl Default for WorldRecordParams {
	fn default() -> Self {
		WorldRecordParams {
			ids: None,
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

impl super::super::super::IsParams for WorldRecordParams {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
// I can't think of a better name for this, lol
pub struct WorldRecordHolder {
	steamid64: String,
	steam_id: Option<String>,
	count: u32,
	player_name: Option<String>,
}

impl super::super::super::IsResponse for WorldRecordHolder {}
impl super::super::super::IsResponse for Vec<WorldRecordHolder> {}
