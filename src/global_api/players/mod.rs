pub fn get_url() -> String {
	String::from("players?")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/players` route
pub struct Params {
	pub name: Option<String>,
	pub steam_id: Option<String>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub steamid64_list: Option<Vec<u64>>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Params {
			name: None,
			steam_id: None,
			is_banned: None,
			total_records: None,
			ip: None,
			steamid64_list: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/players` route
pub struct Response {
	pub steamid64: String,
	pub steam_id: String,
	pub is_banned: bool,
	pub total_records: u32,
	pub name: String,
}

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}
