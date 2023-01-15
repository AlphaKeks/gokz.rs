use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/maps/{id}`
/// - `id`: `id` property on a [Map](super::Response)
/// - Lets you fetch a map stored in the GlobalAPI
pub async fn get(map_id: i32, client: &crate::Client) -> Result<super::Map, Error> {
	let route = format!("/maps/{}", map_id);
	GlobalAPI::get(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
