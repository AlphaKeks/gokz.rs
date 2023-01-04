use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// - Available `mode_name`s:
///   - `kz_timer`
///   - `kz_simple`
///   - `kz_vanilla`
///
/// All of these are accessible via [this method](crate::prelude::Mode::api).
#[allow(dead_code)]
pub async fn get(mode_name: &str, client: &crate::Client) -> Result<super::APIMode, Error> {
	let route = format!("/modes/name/{}", mode_name);
	GlobalAPI::get(&route, Params::default(), client).await
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
