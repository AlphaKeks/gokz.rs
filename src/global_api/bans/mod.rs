#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/bans` route.
pub struct Params {
	pub ban_types: Option<String>,
	pub ban_types_list: Option<Vec<String>>,
	pub is_expired: Option<bool>,
	pub ip: Option<String>,
	pub steamid64: Option<String>,
	pub steam_id: Option<String>,
	pub notes_contains: Option<String>,
	pub stats_contains: Option<String>,
	pub server_id: Option<u32>,
	pub created_since: Option<String>,
	pub updated_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Params {
			ban_types: None,
			ban_types_list: None,
			is_expired: None,
			ip: None,
			steamid64: None,
			steam_id: None,
			notes_contains: None,
			stats_contains: None,
			server_id: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
	pub id: u32,
	pub ban_type: String,
	pub expires_on: String,
	pub steamid64: String,
	pub player_name: String,
	pub steam_id: String,
	pub notes: String,
	pub stats: String,
	pub server_id: u16,
	pub updated_by_id: String,
	pub created_on: String,
	pub updated_on: String,
}

impl super::IsResponse for Response {}
impl super::IsResponse for Vec<Response> {}
