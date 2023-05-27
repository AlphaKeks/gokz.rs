use {
	crate::{
		chrono::{deser_date, ser_date},
		SteamID, Tier,
	},
	chrono::NaiveDateTime,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub name: Option<String>,
	pub global: Option<bool>,
	pub limit: Option<u16>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			global: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Course {
	pub id: u32,
	pub stage: u8,
	pub tier: Tier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Mapper {
	pub name: String,
	pub steam_id: SteamID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub global: bool,
	pub filesize: u32,
	pub courses: Vec<Course>,
	pub mappers: Vec<Mapper>,
	pub approved_by: Option<SteamID>,

	#[serde(serialize_with = "ser_date", deserialize_with = "deser_date")]
	pub created_on: NaiveDateTime,

	#[serde(serialize_with = "ser_date", deserialize_with = "deser_date")]
	pub updated_on: NaiveDateTime,
}
