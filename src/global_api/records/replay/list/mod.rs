pub fn get_url() -> String {
	String::from("records/replay/list")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `records/replay/list` route
pub struct Params {
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Params { offset: None, limit: Some(1) }
	}
}

impl super::super::super::IsParams for Params {}

#[derive(Debug, serde::Deserialize, Clone)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/records/replay/list` route
pub struct Response {
	pub id: u32,
	pub steamid64: String,
	pub server_id: u16,
	pub record_filter_id: i32,
	pub time: f32,
	pub teleports: u32,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by: u64,
	pub points: u16,
	pub replay_id: u32,
}

impl super::super::super::IsResponse for Response {}
impl super::super::super::IsResponse for Vec<Response> {}
