pub(crate) mod place;
pub(crate) mod top;

use {
	super::{GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/records/{id}`
/// - Lets you fetch a record stored in the GlobalAPI
/// - `id`: `record_id` property on a [Map](crate::global_api::maps::Response)
pub(crate) async fn get(record_id: u32, client: &crate::Client) -> Result<Response, Error> {
	let route = format!("/records/{}", record_id);
	GlobalAPI::get::<Response, Params>(&route, Params::default(), client).await
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
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

impl GlobalAPIResponse for Response {}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Params;

impl GlobalAPIParams for Params {}
