use super::Server;

use {
	super::{maps::Course, players::Player},
	crate::{
		chrono::{deser_date, ser_date},
		http, Mode, Result,
	},
	chrono::NaiveDateTime,
	log::trace,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Record {
	pub id: u32,
	pub map_id: u16,
	pub map_name: String,
	pub course: Course,
	pub mode: Mode,
	pub player: Player,
	pub server: Server,
	pub time: f64,
	pub teleports: u16,

	#[serde(serialize_with = "ser_date", deserialize_with = "deser_date")]
	pub created_on: NaiveDateTime,
}

/// Fetches a single record by its ID.
pub async fn get_record(client: &crate::Client) -> Result<Record> {
	trace!("> get_record");
	http::get::<Record>(&format!("{}/records", super::BASE_URL), client).await
}
