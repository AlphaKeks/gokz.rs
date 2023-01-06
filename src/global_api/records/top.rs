use {
	super::{api_params, GlobalAPI, GlobalAPIParams},
	crate::prelude::*,
};

/// Route: `/records/top`
/// - Lets you fetch records stored in the GlobalAPI
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<super::Record>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/records/top?", params, client).await {
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
	pub server_id: Option<u32>,
	pub steamid64: Option<u64>,
	pub map_id: Option<u32>,
	pub map_name: Option<String>,
	pub tickrate: Option<u8>,
	pub overall: Option<bool>,
	pub stage: Option<u8>,
	pub modes_list_string: Option<String>,
	pub modes_list: Option<String>,
	pub has_teleports: Option<bool>,
	pub player_name: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);

impl Default for Params {
	fn default() -> Self {
		Self {
			steam_id: None,
			server_id: None,
			steamid64: None,
			map_id: None,
			map_name: None,
			tickrate: Some(128),
			overall: None,
			stage: None,
			modes_list_string: None,
			modes_list: None,
			has_teleports: None,
			player_name: None,
			offset: None,
			limit: Some(1),
		}
	}
}
