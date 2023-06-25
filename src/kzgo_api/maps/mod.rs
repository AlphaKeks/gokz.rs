#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::types::{SteamID, Tier},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub bonuses: u8,
	pub mapper_names: Vec<String>,
	pub mapper_ids: Vec<SteamID>,
	#[serde(rename = "sp")]
	pub skz: bool,
	#[serde(rename = "vp")]
	pub vnl: bool,
	pub workshop_id: u32,

	#[cfg(feature = "chrono")]
	#[serde(
		rename = "date",
		serialize_with = "crate::utils::serialize_date",
		deserialize_with = "crate::utils::deserialize_date"
	)]
	pub created_on: DateTime<Utc>,

	#[cfg(not(feature = "chrono"))]
	#[serde(rename = "date")]
	pub created_on: String,
}
