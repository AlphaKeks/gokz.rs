use {
	crate::{
		chrono::{parse_date, ser_date},
		Error, Result, SteamID, Tier,
	},
	chrono::NaiveDateTime,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Params {
	pub name: Option<String>,
	pub tier: Option<u8>,
	pub stages: Option<u8>,
	pub validated: Option<bool>,
	pub created_by: Option<String>,
	pub approved_by: Option<String>,
	pub limit: Option<u32>,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			name: None,
			tier: None,
			stages: None,
			validated: None,
			created_by: None,
			approved_by: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Course {
	pub id: u32,
	pub stage: u8,
	pub kzt: bool,
	pub kzt_difficulty: u8,
	pub skz: bool,
	pub skz_difficulty: u8,
	pub vnl: bool,
	pub vnl_difficulty: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: u16,
	pub name: String,
	pub tier: u8,
	pub courses: Vec<Course>,
	pub validated: bool,
	pub mapper_name: String,
	pub mapper_steam_id64: String,
	pub approver_name: String,
	pub approver_steam_id64: String,
	pub filesize: String,
	pub created_on: String,
	pub updated_on: String,
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub courses: Vec<Course>,
	pub validated: bool,
	pub mapper_name: String,
	pub mapper_steam_id: Option<SteamID>,
	pub approver_name: String,
	pub approver_steam_id: Option<SteamID>,
	pub filesize: u64,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	pub updated_on: NaiveDateTime,
}

impl TryFrom<Response> for Map {
	type Error = Error;

	fn try_from(value: Response) -> Result<Self> {
		Ok(Self {
			id: value.id,
			name: value.name,
			tier: value.tier.try_into()?,
			courses: value.courses,
			validated: value.validated,
			mapper_name: value.mapper_name,
			mapper_steam_id: value.mapper_steam_id64.parse().ok(),
			approver_name: value.approver_name,
			approver_steam_id: value.approver_steam_id64.parse().ok(),
			filesize: value.filesize.parse()?,
			created_on: parse_date!(value.created_on),
			updated_on: parse_date!(value.updated_on),
		})
	}
}
