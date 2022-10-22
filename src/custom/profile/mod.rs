use crate::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Response {
	pub name: Option<String>,
	pub steam_id: Option<String>,
	pub steam_id64: Option<String>,
	pub is_banned: Option<bool>,
	pub rank: Option<Rank>,
	pub points: (u32, u32),
	pub records: (u32, u32),
	pub completion: [(u32, u32); 8],
	pub completion_percentage: [(f32, f32); 8],
}

impl Default for Response {
	fn default() -> Self {
		Response {
			name: None,
			steam_id: None,
			steam_id64: None,
			is_banned: None,
			rank: None,
			points: (0, 0),
			records: (0, 0),
			completion: [(0, 0); 8],
			completion_percentage: [(0.0, 0.0); 8],
		}
	}
}
