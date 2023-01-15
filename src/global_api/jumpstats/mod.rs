pub mod top;
pub mod top30;

use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

#[allow(dead_code)]
/// Route: `/jumpstats`
/// - Lets you fetch "global" jumpstats from legacy KZTimer servers
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<Response>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/jumpstats", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("jumpstats or something") },
					msg: String::from("No jumpstats found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub id: Option<i32>,
	pub server_id: Option<i32>,
	pub steamid64: Option<u64>,
	pub steam_id: Option<String>,
	pub jump_type: Option<i8>,
	pub steamid64_list: Option<u64>,
	pub jumptype_list: Option<i8>,
	pub greater_than_distance: Option<f64>,
	pub less_than_distance: Option<f64>,
	pub json_jump_info: Option<String>,
	pub is_msl: Option<bool>,
	pub is_crouch_bind: Option<bool>,
	pub is_forward_bind: Option<bool>,
	pub is_crouch_boost: Option<bool>,
	pub updated_by_id: Option<i64>,
	pub created_since: Option<String>,
	pub updated_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);

impl Default for Params {
	fn default() -> Self {
		Self {
			id: None,
			server_id: None,
			steamid64: None,
			steam_id: None,
			jump_type: None,
			steamid64_list: None,
			jumptype_list: None,
			greater_than_distance: None,
			less_than_distance: None,
			json_jump_info: None,
			is_msl: None,
			is_crouch_bind: None,
			is_forward_bind: None,
			is_crouch_boost: None,
			updated_by_id: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
	pub id: i32,
	pub server_id: i32,
	pub steamid64: i64,
	pub player_name: String,
	pub steam_id: String,
	pub jump_type: i32,
	pub distance: f64,
	pub tickrate: i32,
	pub msl_count: i32,
	pub strafe_count: i32,
	pub is_crouch_bind: i32,
	pub is_forward_bind: i32,
	pub is_crouch_boost: i32,
	pub updated_by_id: i64,
	pub created_on: String,
	pub updated_on: String,
}

api_response!(Response);
