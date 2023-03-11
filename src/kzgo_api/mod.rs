//! Types and functions to interact with the
//! [KZ:GO API](https://kzgo.eu/).

use crate::{http, Error, Mode, Result, SteamID};

/// Base URL for KZ:GO API requests.
pub const BASE_URL: &str = "https://kzgo.eu/api";

/// The `/completions` route.
pub mod completions;
pub use completions::CompletionStats;

/// Fetches the amount of completable maps for a given mode.
pub async fn get_completions(mode: Mode, client: &crate::Client) -> Result<CompletionStats> {
	http::get::<completions::mode::Response>(
		&format!("{}/completions/{}", BASE_URL, mode.api()),
		client,
	)
	.await?
	.try_into()
}

/// The `/maps` route.
pub mod maps;
pub use maps::Map;

/// Fetches all maps.
pub async fn get_maps(client: &crate::Client) -> Result<Vec<Map>> {
	Ok(
		http::get::<Vec<maps::index::Response>>(&format!("{}/maps", BASE_URL), client)
			.await?
			.into_iter()
			.filter_map(|map| map.try_into().ok())
			.collect(),
	)
}

/// Fetches a single map.
pub async fn get_map(map_name: &str, client: &crate::Client) -> Result<Map> {
	http::get::<maps::index::Response>(&format!("{}/maps/{}", BASE_URL, map_name), client)
		.await?
		.try_into()
}

/// The `/steam` route.
pub mod steam;
pub use steam::User;

/// Fetches a user's Steam avatar URL.
pub async fn get_avatar(user_id: SteamID, client: &crate::Client) -> Result<User> {
	Ok(http::get::<steam::id64::Response>(
		&format!("{}/steam/{}", BASE_URL, user_id.as_id64()),
		client,
	)
	.await?
	.into())
}

/// The `/servers` route.
pub mod servers;
pub use servers::Server;

/// Fetches all servers from the "servers" tab on kzgo.eu
pub async fn get_servers(client: &crate::Client) -> Result<Vec<Server>> {
	Ok(
		http::get::<servers::index::Response>(&format!("{}/servers", BASE_URL), client)
			.await?
			.into(),
	)
}

/// The `/wrs` route.
pub mod world_records;
pub use world_records::{LeaderboardEntry, WorldRecord};

/// Fetch all world records for a specific mode.
pub async fn get_wrs(
	mode: Mode,
	has_teleports: Option<bool>,
	client: &crate::Client,
) -> Result<Vec<WorldRecord>> {
	use std::fmt::Write;

	let mut url = format!("{}/wrs/{}", BASE_URL, mode.api());
	if let Some(has_teleports) = has_teleports {
		write!(&mut url, "/{}", if has_teleports { "tp" } else { "pro" })
			.map_err(|_| Error::Custom("Failed to append to string."))?;
	}

	Ok(
		http::get::<Vec<world_records::mode::Response>>(&url, client)
			.await?
			.into_iter()
			.filter_map(|entry| entry.try_into().ok())
			.collect(),
	)
}

/// Fetch the leaderboard of world record holders for a specific mode and runtype.
pub async fn get_wr_leaderboard(
	mode: Mode,
	has_teleports: bool,
	client: &crate::Client,
) -> Result<Vec<LeaderboardEntry>> {
	Ok(http::get::<Vec<world_records::leaderboard::Response>>(
		&format!(
			"{}/wrs/leaderboards/{}/{}",
			BASE_URL,
			mode.api(),
			if has_teleports { "tp" } else { "pro" }
		),
		client,
	)
	.await?
	.into_iter()
	.filter_map(|entry| entry.try_into().ok())
	.collect())
}
