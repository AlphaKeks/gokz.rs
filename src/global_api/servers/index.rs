use {
	super::ServerID,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub id: Option<ServerID>,
	pub port: Option<u16>,
	pub ip: Option<String>,
	pub name: Option<String>,
	pub owner_steamid64: Option<u64>,
	pub approval_status: Option<i32>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			id: None,
			port: None,
			ip: None,
			name: None,
			owner_steamid64: None,
			approval_status: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: i32,
	pub port: i32,
	pub ip: String,
	pub name: String,
	pub owner_steamid64: String,
}
