#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use {
	crate::types::{MapIdentifier, SteamID, Tier},
	serde::{Deserialize, Serialize},
	serde_json::Value as JsonValue,
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub bonuses: u8,
	#[serde(deserialize_with = "deserialize_mapper_names")]
	pub mapper_names: Vec<String>,
	#[serde(deserialize_with = "deserialize_mapper_ids")]
	pub mapper_ids: Vec<SteamID>,
	#[serde(rename = "sp")]
	pub skz: bool,
	#[serde(rename = "vp")]
	pub vnl: bool,
	pub workshop_id: String,

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

impl crate::traits::MapIdentifier for Map {
	#[inline]
	fn image_url(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).image_url() }

	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String { MapIdentifier::Name(self.name.clone()).global_api() }

	#[inline]
	fn kzgo(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).kzgo() }

	#[inline]
	fn kzgo_api(&self) -> Option<String> { MapIdentifier::Name(self.name.clone()).kzgo_api() }

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String { MapIdentifier::Id(self.id).schnose_api() }
}

fn deserialize_mapper_ids<'de, D>(deserializer: D) -> Result<Vec<SteamID>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	Ok(Vec::<String>::deserialize(deserializer)?
		.into_iter()
		.flat_map(|steam_id| steam_id.parse())
		.collect())
}

fn deserialize_mapper_names<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	Ok(Vec::<JsonValue>::deserialize(deserializer)?
		.into_iter()
		.filter_map(|name| if let JsonValue::String(name) = name { Some(name) } else { None })
		.collect())
}
