use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
	prelude::*,
};

/// Route: `/players/steamid/{steamid}/ip/{ip}`
/// - Lets you fetch player information
/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a String)
/// - `ip`: since none of these routes are well documented, I can only guess that this is
/// supposed to be an IPv4 address as a String.
pub(crate) async fn get(
	steam_id: &SteamID,
	ip: &str,
	client: &crate::Client,
) -> Result<super::Response, Error> {
	let route = format!("/players/steam_id/{}/ip/{}", steam_id, ip);
	GlobalAPI::get::<super::Response, Params>(&route, Params::default(), client).await
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
impl GlobalAPIParams for Params {}
