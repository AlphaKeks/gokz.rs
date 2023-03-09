use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
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
		Self {
			ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: None,
			has_teleports: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: i32,
	pub map_id: i32,
	pub stage: i32,
	pub mode_id: i32,
	pub tickrate: i32,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_by_id: String,
}
