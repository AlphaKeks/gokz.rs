use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/modes/id/{id}` _OR_ `/modes/name/{mode_name}`
/// - Lets you fetch a mode stored in the GlobalAPI
///
/// - Available `id`s:
///   - `200` (KZTimer)
///   - `201` (SimpleKZ)
///   - `202` (Vanilla)
///
/// All of these are accessible by casting a [Mode](crate::prelude::Mode) to an integer using
/// the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.
pub async fn get(mode_id: u8, client: &crate::Client) -> Result<super::APIMode, Error> {
	let route = format!("/modes/id/{}", mode_id);
	GlobalAPI::get(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
