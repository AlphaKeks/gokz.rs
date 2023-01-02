use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
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
pub(crate) async fn get(mode_id: u8, client: &crate::Client) -> Result<super::Response, Error> {
	let route = format!("/modes/id/{}", mode_id);
	GlobalAPI::get::<super::Response, Params>(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
impl GlobalAPIParams for Params {}
