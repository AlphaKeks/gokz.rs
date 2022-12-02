/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("players?")
}

#[derive(Debug, Clone, serde::Serialize)]
/// All possible parameters for this route
pub struct PlayerParams {
	pub name: Option<String>,
	pub steam_id: Option<String>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub steamid64_list: Option<u64>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for PlayerParams {
	fn default() -> Self {
		PlayerParams {
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

impl super::IsParams for PlayerParams {}

#[derive(Debug, Clone, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct APIPlayer {
	pub steamid64: String,
	pub steam_id: String,
	pub is_banned: bool,
	pub total_records: u32,
	pub name: String,
}

impl super::IsResponse for APIPlayer {}
impl super::IsResponse for Vec<APIPlayer> {}
