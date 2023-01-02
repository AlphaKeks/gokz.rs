pub(crate) mod alts;
pub(crate) mod ip;
pub(crate) mod steam_id;

use {
	super::{GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/players`
/// - Lets you fetch player information
pub(crate) async fn get(params: Params, client: &crate::Client) -> Result<Vec<Response>, Error> {
	match GlobalAPI::get::<Vec<Response>, Params>("/players?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("Vec<Player>") },
					msg: String::from("No players found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
	pub steamid64: String,
	pub steam_id: String,
	pub is_banned: bool,
	pub total_records: u32,
	pub name: String,
}

impl GlobalAPIResponse for Response {}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub name: Option<String>,
	pub steam_id: Option<String>,
	pub is_banned: Option<bool>,
	pub total_records: Option<u32>,
	pub ip: Option<String>,
	pub steamid64_list: Option<u64>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl GlobalAPIParams for Params {}
