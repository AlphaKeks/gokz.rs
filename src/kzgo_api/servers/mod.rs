use {
	crate::types::{ServerIdentifier, Tier},
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

impl crate::traits::ServerIdentifier for Server {
	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String { ServerIdentifier::Name(self.name.clone()).global_api() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { ServerIdentifier::Name(self.name.clone()).schnose_api() }
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
