use {
	super::{GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/records/place/{id}`
/// - Lets you fetch the leaderboard spot of a given record
/// - `id`: `record_id` property on a [Map](crate::global_api::maps::Response)
pub(crate) async fn get(record_id: u32, client: &crate::Client) -> Result<Response, Error> {
	let route = format!("/records/place/{}", record_id);
	let response = GlobalAPI::get::<u32, Params>(&route, Params::default(), client).await?;
	Ok(Response(response))
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Response(pub u32);

impl GlobalAPIResponse for Response {}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;

impl GlobalAPIParams for Params {}
