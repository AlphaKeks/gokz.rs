use {
	crate::types::Tier,
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
	#[serde(rename = "type")]
	pub kind: String,
	pub ip: String,
	pub port: u16,
	pub label: String,
	pub name: String,
	pub map: Map,
	pub players: Vec<Player>,
	pub tags: Vec<String>,
	pub err_before: bool,
	pub max_players: u8,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
	pub name: String,
	pub tier: Tier,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub raw: RawPlayer,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPlayer {
	pub score: u32,
	pub time: f64,
}
