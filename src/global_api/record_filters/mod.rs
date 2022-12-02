/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("record_filters?")
}

#[derive(Debug, Clone, serde::Serialize)]
/// All possible parameters for this route
pub struct RecordFilterParams {
	pub ids: Option<u32>,
	pub map_ids: Option<i16>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for RecordFilterParams {
	fn default() -> Self {
		RecordFilterParams {
			ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: Some(128),
			has_teleports: None,
			offset: None,
			limit: None,
		}
	}
}

impl super::IsParams for RecordFilterParams {}

#[derive(Debug, Clone, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct RecordFilter {
	pub id: u32,
	pub map_id: i16,
	pub stage: u8,
	pub mode_id: u8,
	pub tickrate: u8,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_by_id: String,
}

impl super::IsResponse for RecordFilter {}
impl super::IsResponse for Vec<RecordFilter> {}
