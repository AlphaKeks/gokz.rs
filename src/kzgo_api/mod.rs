use crate::{
	error::{Error, Result},
	http::get_json,
	prelude,
	traits::Mode as _,
	types::SteamID,
	utils::EmptyParams,
};

/// The base URL for all API requests.
pub const BASE_URL: &str = "https://kzgo.eu/api";

/// The `/completions` route
pub mod completions;
pub use completions::{CompletionCount, Completions};

/// The `/maps` route
pub mod maps;
pub use maps::Map;

/// The `/steam` route
pub mod steam;
pub use steam::User;

/// The `/servers` route
pub mod servers;
pub use servers::Server;

/// The `/wrs` routes
pub mod world_records;
pub use world_records::{get_wrs, leaderboards, WorldRecord};

/// Get a player's completions for a given mode
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_completions<M>(mode: M, client: &crate::Client) -> Result<Completions>
where
	M: Into<prelude::Mode> + std::fmt::Debug,
{
	get_json(&format!("{BASE_URL}/completions/{}", mode.into().api()), &EmptyParams, client).await
}

/// Fetches maps
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let maps: Vec<_> = get_json(&format!("{BASE_URL}/maps"), &EmptyParams, client).await?;

	if maps.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(maps)
}

/// Fetches a single map
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_map(map_name: &str, client: &crate::Client) -> Result<Map> {
	get_json(&format!("{BASE_URL}/maps/{map_name}"), &EmptyParams, client).await
}

/// Fetches information about a player
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_steam_user<S>(steam_id: S, client: &crate::Client) -> Result<User>
where
	S: Into<SteamID> + std::fmt::Debug,
{
	get_json(&format!("{BASE_URL}/steam/{}", steam_id.into().as_id64()), &EmptyParams, client).await
}

/// Fetches all servers from the "servers" tab on kzgo.eu
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_servers(client: &crate::Client) -> Result<Vec<Server>> {
	let servers: Vec<_> = get_json(&format!("{BASE_URL}/servers"), &EmptyParams, client).await?;

	if servers.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(servers)
}
