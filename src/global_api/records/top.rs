use {crate::SteamID, serde::Serialize};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub steam_id: Option<SteamID>,
	pub server_id: Option<u16>,
	pub steamid64: Option<u64>,
	pub map_id: Option<u16>,
	pub map_name: Option<String>,
	pub tickrate: Option<u8>,
	pub overall: Option<bool>,
	pub stage: Option<u8>,
	pub modes_list_string: Option<String>,
	pub modes_list: Option<String>,
	pub has_teleports: Option<bool>,
	pub player_name: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			steam_id: None,
			server_id: None,
			steamid64: None,
			map_id: None,
			map_name: None,
			tickrate: None,
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
