use {
	crate::{schnose_api::players::RawPlayer, PlayerIdentifier},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub name: Option<String>,
	pub owned_by: Option<PlayerIdentifier>,
	pub approved_by: Option<PlayerIdentifier>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			owned_by: None,
			approved_by: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: u16,
	pub name: String,
	pub owned_by: RawPlayer,
	pub approved_by: RawPlayer,
}
