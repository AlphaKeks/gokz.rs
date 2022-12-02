pub mod recent;
pub mod world_records;

/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("records/top?")
}

#[derive(Debug, Clone, serde::Serialize)]
/// All possible parameters for this route
pub struct RecordParams {
	pub steam_id: Option<String>,
	pub server_id: Option<u32>,
	pub steamid64: Option<u64>,
	pub map_id: Option<i16>,
	pub map_name: Option<String>,
	pub tickrate: Option<u8>,
	pub overall: Option<bool>,
	pub stage: Option<u8>,
	pub modes_list_string: Option<String>,
	pub modes_list: Option<Vec<String>>,
	pub has_teleports: Option<bool>,
	pub player_name: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for RecordParams {
	fn default() -> Self {
		RecordParams {
			steam_id: None,
			server_id: None,
			steamid64: None,
			map_id: None,
			map_name: None,
			tickrate: Some(128),
			overall: None,
			stage: None,
			modes_list_string: None,
			modes_list: None,
			has_teleports: None,
			player_name: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::super::IsParams for RecordParams {}

#[derive(Debug, Clone, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct Record {
	pub id: u32,
	pub steamid64: String,
	pub player_name: Option<String>,
	pub steam_id: Option<String>,
	pub server_id: u16,
	pub map_id: i16,
	pub stage: u8,
	pub mode: String,
	pub tickrate: u8,
	pub time: f32,
	pub teleports: u32,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by: u64,
	pub record_filter_id: i32,
	pub server_name: Option<String>,
	pub map_name: String,
	pub points: u16,
	pub replay_id: u32,
}

impl super::super::IsResponse for Record {}
impl super::super::IsResponse for Vec<Record> {}
