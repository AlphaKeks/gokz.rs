use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct FancyPlayer {
	pub id: u32,
	pub name: String,
	pub steam_id: String,
	pub steam_id64: String,
	pub is_banned: bool,
	pub records: RecordSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RecordSummary {
	pub total: u32,
	pub kzt: RecordCount,
	pub skz: RecordCount,
	pub vnl: RecordCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RecordCount {
	pub tp: u32,
	pub pro: u32,
}
