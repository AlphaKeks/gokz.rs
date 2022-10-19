pub fn get_url() -> String {
	String::from("record_filters?")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/record_filters` route
pub struct Params {
	pub ids: Option<u32>,
	pub map_ids: Option<u16>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
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
			offset: None,
			limit: None,
		}
	}
}

impl super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/record_filters` route
pub struct Response {
	pub id: u32,
	pub map_id: u16,
	pub stage: u8,
	pub mode_id: u8,
	pub tickrate: u8,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_by_id: String,
}

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}
