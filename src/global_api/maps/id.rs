use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/maps/{id}`
/// - `id`: `id` property on a [Map]()
/// - Lets you fetch a map stored in the GlobalAPI
pub(super) async fn get(map_id: u16, client: &crate::Client) -> Result<super::Response, Error> {
	let route = format!("/maps/id/{map_id}");
	GlobalAPI::get_raw::<super::Response, Params>(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
impl GlobalAPIParams for Params {}
