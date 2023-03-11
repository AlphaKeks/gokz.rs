use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct Response {
	pub serverStates: Vec<ServerState>,
	pub lastUpdate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct ServerState {
	pub r#type: String,
	pub ip: String,
	pub port: u16,
	pub label: String,
	pub name: String,
	pub map: Map,
	pub players: Vec<Player>,
	pub tags: Vec<String>,
	pub errBefore: bool,
	pub maxPlayers: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Map {
	pub name: String,
	pub tier: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Player {
	pub name: String,
	pub raw: PlayerStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlayerStats {
	pub score: u32,
	pub time: f64,
}
