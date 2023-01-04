use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/players/steamid/{steamid}/ip/{ip}`
/// - Lets you fetch player information
/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a String)
/// - `ip`: since none of these routes are well documented, I can only guess that this is
/// supposed to be an IPv4 address as a String.
pub async fn get(
	steam_id: &SteamID,
	ip: &str,
	client: &crate::Client,
) -> Result<super::Player, Error> {
	let route = format!("/players/steam_id/{}/ip/{}", steam_id, ip);
	GlobalAPI::get(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
