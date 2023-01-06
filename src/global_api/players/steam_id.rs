use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/players/steamid/{steamid}`
/// - Lets you fetch player information
/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a string)
pub async fn get(steam_id: &SteamID, client: &crate::Client) -> Result<super::Player, Error> {
	let route = format!("/players/steamid/{}", steam_id);
	let mut players =
		GlobalAPI::get::<Vec<super::Player>, Params>(&route, Params::default(), client).await?;
	Ok(players.remove(0))
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
