use {
	crate::{schnose_api::maps::index::Mapper, PlayerIdentifier, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub name: Option<String>,
	pub owned_by: Option<PlayerIdentifier>,
	pub limit: Option<u16>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			owned_by: None,
			limit: Some(1),
		}
	}
}

#[allow(missing_docs)]
pub type ServerOwner = Mapper;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Server {
	pub id: u16,
	pub name: String,
	pub owned_by: Option<ServerOwner>,
	pub approved_by: Option<SteamID>,
}
