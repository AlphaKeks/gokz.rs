//! Types and functions to interact with the [Schnose API](https://schnose.xyz/health).

use {
	crate::{http, MapIdentifier, PlayerIdentifier, Result, ServerIdentifier},
	log::trace,
};

/// Base URL for SchnoseAPI requests.
pub const BASE_URL: &str = "https://schnose-api.shuttleapp.rs/api";

/// The `/maps` route.
pub mod maps;
pub use maps::Map;

/// Fetches all maps.
pub async fn get_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_maps {params:#?}");

	http::get::<Vec<maps::index::Map>>(&format!("{}/maps", BASE_URL), client).await
}

/// Fetches all global/validated maps.
pub async fn get_global_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		global: Some(true),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_global_maps {params:#?}");

	http::get_with_params::<_, Vec<maps::index::Map>>(&format!("{}/maps", BASE_URL), params, client)
		.await
}

/// Fetches all non-global/non-validated maps.
pub async fn get_nonglobal_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		global: Some(false),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_nonglobal_maps {params:#?}");

	http::get_with_params::<_, Vec<maps::index::Map>>(&format!("{}/maps", BASE_URL), params, client)
		.await
}

/// Fetches a single map.
pub async fn get_map(map_identifier: &MapIdentifier, client: &crate::Client) -> Result<Map> {
	trace!("> get_map {{ map_identifier: {map_identifier} }}");
	http::get::<maps::index::Map>(&format!("{}/maps/{}", BASE_URL, map_identifier), client).await
}

/// The `/players` route.
pub mod players;
pub use players::{FancyPlayer, Player, RawFancyPlayer};

/// Fetches players.
pub async fn get_players(
	offset: i64,
	limit: u16,
	client: &crate::Client,
) -> Result<Vec<players::index::Player>> {
	let params = players::index::Params {
		offset: Some(offset),
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_players {params:#?}");

	players::get_players(params, client).await
}

/// Fetches a single player.
pub async fn get_player(
	player_identifier: PlayerIdentifier,
	client: &crate::Client,
) -> Result<FancyPlayer> {
	trace!("> get_player {{ player_identifier: {player_identifier:#?} }}");

	players::get_player(player_identifier, client).await
}

/// The `/servers` route.
pub mod servers;
pub use servers::index::Server;

/// Fetches all servers.
pub async fn get_servers(client: &crate::Client) -> Result<Vec<Server>> {
	let params = servers::index::Params {
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_servers {params:#?}");

	servers::get_servers(params, client).await
}

/// Fetches a single server.
pub async fn get_server(
	server_identifier: &ServerIdentifier,
	client: &crate::Client,
) -> Result<Server> {
	trace!("> get_server {{ server_identifier: {server_identifier} }}");
	servers::get_server(server_identifier, client).await
}

/// Fetches all servers with common names (e.g. "House of Climb").
pub async fn get_server_group(
	server_identifier: ServerIdentifier,
	client: &crate::Client,
) -> Result<Vec<Server>> {
	let params = servers::index::Params {
		name: Some(server_identifier.to_string()),
		..Default::default()
	};
	trace!("> get_server_group {params:#?}");
	servers::get_servers(params, client).await
}

/// Fetches all servers owned by a specific player.
pub async fn get_servers_owned_by(
	server_owner: PlayerIdentifier,
	client: &crate::Client,
) -> Result<Vec<Server>> {
	let params = servers::index::Params {
		owned_by: Some(server_owner),
		..Default::default()
	};
	trace!("> get_servers_owned_by {params:#?}");
	servers::get_servers(params, client).await
}

/// The `/records` route (and subroutes).
pub mod records;
pub use records::{get_record, Record};
