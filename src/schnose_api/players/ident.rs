use {
	crate::{Error, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RawFancyPlayer {
	pub id: u32,
	pub name: String,
	pub steam_id: String,
	pub steam_id64: String,
	pub is_banned: bool,
	pub records: RecordSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct FancyPlayer {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
	pub records: RecordSummary,
}

impl TryFrom<RawFancyPlayer> for FancyPlayer {
	type Error = Error;

	fn try_from(value: RawFancyPlayer) -> Result<Self> {
		let steam_id = if let Ok(steam_id) = value.steam_id.parse() {
			steam_id
		} else {
			value.steam_id64.parse()?
		};

		Ok(Self {
			name: value.name,
			steam_id,
			is_banned: value.is_banned,
			records: value.records,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RecordSummary {
	pub total: u32,
	pub kzt: RecordCount,
	pub skz: RecordCount,
	pub vnl: RecordCount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RecordCount {
	pub tp: u32,
	pub pro: u32,
}
