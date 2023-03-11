use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct Response {
	pub _id: Option<String>,
	pub mapId: u16,
	pub pro: bool,
	pub createdOn: String,
	pub diff: f64,
	pub mapName: String,
	pub playerName: String,
	pub serverId: u16,
	pub serverName: String,
	pub steamId: String,
	pub steamId64: String,
	pub time: f64,
	pub tps: u32,
}
