use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/players/steamid/{steamid}`
/// - Lets you fetch player information
/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a string)
pub(crate) async fn get(
	steam_id: &SteamID,
	client: &crate::Client,
) -> Result<super::Response, Error> {
	let route = format!("/players/steam_id/{}", steam_id);
	GlobalAPI::get::<super::Response, Params>(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
impl GlobalAPIParams for Params {}
