use {
	crate::SteamID,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub name: Option<String>,
	pub steam_id: Option<SteamID>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub steamid64_list: Option<u64>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub steamid64: String,
	pub steam_id: String,
	pub is_banned: bool,
	pub total_records: i32,
	pub name: String,
}
