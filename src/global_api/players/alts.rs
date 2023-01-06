use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/players/steamid/{steamid}/alts`
/// - Lets you fetch alternate accounts of a player
/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a string)
pub async fn get(
	steam_id: &SteamID,
	client: &crate::Client,
) -> Result<Vec<super::Player>, Error> {
	let route = format!("/players/steamid/{}/alts", steam_id);
	GlobalAPI::get::<Vec<_>, _>(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
