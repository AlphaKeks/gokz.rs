use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/records/top/recent`
/// - Lets you fetch the most recently created records
/// - Some notes:
///   - `mode` is required because if you don't specify one, the API will return an `internal
///      server error`.
///   - will only yield personal bests
///   - endpoint is pretty slow; it will take a while until a record appears here
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<RecentRecord>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/records/top/recent?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("Vec<Record>") },
					msg: String::from("No records found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub steam_id: Option<String>,
	pub steamid64: Option<u64>,
	pub map_id: Option<i32>,
	pub map_name: Option<String>,
	pub has_teleports: Option<bool>,
	pub tickrate: Option<i32>,
	pub stage: Option<i32>,
	pub modes_list_string: Option<String>,
	pub modes_list: Option<String>,
	pub place_top_at_least: Option<i32>,
	pub place_top_overall_at_least: Option<i32>,
	pub created_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);

impl Default for Params {
	fn default() -> Self {
		Self {
			steam_id: None,
			steamid64: None,
			map_id: None,
			map_name: None,
			has_teleports: None,
			tickrate: Some(128),
			stage: None,
			modes_list_string: Some(String::from("kz_timer")),
			modes_list: None,
			place_top_at_least: None,
			place_top_overall_at_least: None,
			created_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecentRecord {
	pub id: i32,
	pub steamid64: String,
	pub player_name: Option<String>,
	pub steam_id: Option<String>,
	pub server_id: i32,
	pub map_id: i32,
	pub stage: i32,
	pub mode: String,
	pub tickrate: i32,
	pub time: f64,
	pub teleports: i32,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by: i64,
	pub place: i32,
	pub top_100: i32,
	pub top_100_overall: i32,
	pub server_name: Option<String>,
	pub map_name: String,
	pub points: i32,
	pub record_filter_id: i32,
	pub replay_id: i32,
}

api_response!(RecentRecord);
