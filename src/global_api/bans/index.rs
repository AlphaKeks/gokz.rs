use {
	crate::SteamID,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Params {
	pub ban_types: Option<String>,
	pub ban_types_list: Option<String>,
	pub is_expired: Option<bool>,
	pub ip: Option<String>,
	pub steamid64: Option<u64>,
	pub steam_id: Option<SteamID>,
	pub notes_contains: Option<String>,
	pub stats_contains: Option<String>,
	pub server_id: Option<u16>,
	pub created_since: Option<String>,
	pub updated_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: i32,
	pub ban_type: String,
	pub expires_on: String,
	pub steamid64: String,
	pub player_name: String,
	pub steam_id: String,
	pub notes: String,
	pub stats: String,
	pub server_id: i32,
	pub updated_by_id: String,
	pub created_on: String,
	pub updated_on: String,
}
