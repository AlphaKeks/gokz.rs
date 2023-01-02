use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/maps/name/{map_name}`
/// - `map_name`: any of [these](https://maps.global-api.com/mapcycles/gokz.txt)
/// - Lets you fetch a map stored in the GlobalAPI
pub(crate) async fn get(map_name: &str, client: &crate::Client) -> Result<super::Response, Error> {
	let route = format!("/maps/name/{}", map_name);
	GlobalAPI::get::<super::Response, Params>(&route, Params::default(), client).await
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
impl GlobalAPIParams for Params {}
