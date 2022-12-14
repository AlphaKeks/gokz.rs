use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/servers/name/{server_name}`
/// - Lets you fetch information about global servers
/// - `server_name`: `server_name` property on a [Server](super::Response)
pub async fn get(server_name: &str, client: &crate::Client) -> Result<super::Server, Error> {
	let route = format!("/servers/name/{}", server_name);
	GlobalAPI::get(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
