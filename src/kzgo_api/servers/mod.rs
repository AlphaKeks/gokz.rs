use {
	crate::{Error, Result, Tier},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Server {
	pub label: String,
	pub name: String,
	pub kind: String,
	pub map_name: String,
	pub map_tier: Tier,
	pub players: Vec<String>,
	pub max_players: u8,
	pub tags: Vec<String>,
}

/// `/servers`
pub mod index;
impl TryFrom<index::ServerState> for Server {
	type Error = Error;

	fn try_from(value: index::ServerState) -> Result<Self> {
		Ok(Self {
			label: value.label,
			name: value.name,
			kind: value.r#type,
			map_name: value.map.name,
			map_tier: value.map.tier.try_into()?,
			players: value
				.players
				.into_iter()
				.map(|player| player.name)
				.collect(),
			max_players: value.maxPlayers,
			tags: value.tags,
		})
	}
}

impl From<index::Response> for Vec<Server> {
	fn from(value: index::Response) -> Self {
		value
			.serverStates
			.into_iter()
			.filter_map(|server| server.try_into().ok())
			.collect()
	}
}
