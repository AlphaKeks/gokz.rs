/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("records/replay/list")
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
/// All possible parameters for this route
pub struct ReplayListParams {
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for ReplayListParams {
	fn default() -> Self {
		ReplayListParams { offset: None, limit: Some(1) }
	}
}

impl super::super::super::IsParams for ReplayListParams {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct Replay {
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

impl super::super::super::IsResponse for Replay {}
impl super::super::super::IsResponse for Vec<Replay> {}
