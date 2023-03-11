use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(missing_docs)]
pub struct Params {
	pub is_banned: Option<bool>,
	pub limit: Option<u32>,
	pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Player {
	pub id: u32,
	pub name: String,
	pub is_banned: bool,
}
