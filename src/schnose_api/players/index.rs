use {
	crate::SteamID,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(missing_docs)]
pub struct Params {
	pub is_banned: Option<bool>,
	pub limit: Option<u16>,
	pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}
