use {
	crate::{chrono::ser_opt_date, MapIdentifier, Mode, PlayerIdentifier},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub mode: Option<Mode>,
	pub stage: Option<u8>,
	pub map: Option<MapIdentifier>,
	pub player: Option<PlayerIdentifier>,
	pub has_teleports: Option<bool>,
	#[serde(serialize_with = "ser_opt_date")]
	pub created_after: Option<NaiveDateTime>,
	#[serde(serialize_with = "ser_opt_date")]
	pub created_before: Option<NaiveDateTime>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			mode: None,
			stage: None,
			map: None,
			player: None,
			has_teleports: None,
			created_after: None,
			created_before: None,
			limit: Some(1),
		}
	}
}
