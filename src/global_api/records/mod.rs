pub mod place;
pub mod recent;
pub mod top;

use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/records/{id}`
/// - Lets you fetch a record stored in the GlobalAPI
/// - `id`: `record_id` property on a [Map](crate::global_api::maps::Response)
pub async fn get(record_id: u32, client: &crate::Client) -> Result<Record, Error> {
	let route = format!("/records/{}", record_id);
	GlobalAPI::get(&route, Params::default(), client).await
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Record {
	pub id: u32,
	pub steamid64: String,
	pub player_name: Option<String>,
	pub steam_id: Option<String>,
	pub server_id: u32,
	pub map_id: u32,
	pub stage: u8,
	pub mode: String,
	pub tickrate: u8,
	pub time: f32,
	pub teleports: u32,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by: u64,
	pub record_filter_id: i32,
	pub server_name: Option<String>,
	pub map_name: String,
	pub points: u32,
	pub replay_id: u32,
}

api_response!(Record);

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
