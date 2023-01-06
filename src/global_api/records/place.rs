use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/records/place/{id}`
/// - Lets you fetch the leaderboard spot of a given record
/// - `id`: `record_id` property on a [Map](crate::global_api::maps::Response)
pub async fn get(record_id: u32, client: &crate::Client) -> Result<Response, Error> {
	let route = format!("/records/place/{}", record_id);
	GlobalAPI::get(&route, Params::default(), client).await
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Response(pub u32);
api_response!(Response);

impl From<Response> for u32 {
	fn from(value: Response) -> Self {
		value.0
	}
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
