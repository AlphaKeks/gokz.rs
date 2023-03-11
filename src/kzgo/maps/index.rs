use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct Response {
	pub _id: Option<String>,
	pub name: String,
	pub id: u16,
	pub tier: u8,
	pub workshopId: String,
	pub bonuses: u8,
	pub sp: bool,
	pub vp: bool,
	pub mapperNames: Vec<String>,
	pub mapperIds: Vec<String>,
	pub date: String,
}
