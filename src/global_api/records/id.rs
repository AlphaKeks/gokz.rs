use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: i32,
	pub steamid64: String,
	pub player_name: Option<String>,
	pub steam_id: Option<String>,
	pub server_id: i32,
	pub map_id: i32,
	pub stage: i32,
	pub mode: String,
	pub tickrate: i32,
	pub time: f64,
	pub teleports: i32,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by: i64,
	pub record_filter_id: i32,
	pub server_name: Option<String>,
	pub map_name: Option<String>,
	pub points: i32,
	pub replay_id: i32,
}
