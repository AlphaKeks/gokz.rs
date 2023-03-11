use {
	crate::schnose_api::{maps::Course, players::RawPlayer},
	serde::Deserialize,
};

#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub id: u32,
	pub map_name: String,
	pub course: Course,
	pub mode: String,
	pub player: RawPlayer,
	pub server_name: String,
	pub time: f64,
	pub teleports: u32,
	pub created_on: String,
}
