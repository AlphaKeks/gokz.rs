use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/servers/{id}`
/// - Lets you fetch information about global servers
/// - `id`: `id` property on a [Server](super::Response)
pub async fn get(server_id: i32, client: &crate::Client) -> Result<super::Server, Error> {
	let route = format!("/servers/{}", server_id);
	GlobalAPI::get(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
