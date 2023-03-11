use {
	crate::{chrono::ser_opt_date, Tier},
	chrono::NaiveDateTime,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub id: Option<u16>,
	pub name: Option<String>,
	pub larger_than_filesize: Option<u64>,
	pub smaller_than_filesize: Option<u64>,
	pub is_validated: Option<bool>,
	pub difficulty: Option<Tier>,
	#[serde(serialize_with = "ser_opt_date")]
	pub created_since: Option<NaiveDateTime>,
	#[serde(serialize_with = "ser_opt_date")]
	pub updated_since: Option<NaiveDateTime>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			id: None,
			name: None,
			larger_than_filesize: None,
			smaller_than_filesize: None,
			is_validated: None,
			difficulty: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: i32,
	pub name: String,
	pub filesize: i64,
	pub validated: bool,
	pub difficulty: i32,
	pub created_on: String,
	pub updated_on: String,
	pub approved_by_steamid64: String,
	pub workshop_url: String,
	pub download_url: Option<String>,
}
