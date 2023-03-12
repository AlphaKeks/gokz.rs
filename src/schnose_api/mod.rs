//! Types and functions to interact with the [Schnose API](https://schnose.xyz/api).

use {
	crate::{http, MapIdentifier, Mode, PlayerIdentifier, Result, ServerIdentifier, Tier},
	log::trace,
	serde::Deserialize,
};

/// Base URL for SchnoseAPI requests.
pub const BASE_URL: &str = "https://schnose.xyz/api";

/// This gets returned from all API calls.
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
	result: T,
	#[allow(unused)]
	took: u128,
}

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

	Ok(
		http::get::<Response<Vec<maps::index::Response>>>(&format!("{}/maps", BASE_URL), client)
			.await
			.map(|response| response.result)?
			.into_iter()
			.filter_map(|map| map.try_into().ok())
			.collect(),
	)
}

/// Fetches all global/validated maps.
pub async fn get_global_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		validated: Some(true),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_global_maps {params:#?}");

	Ok(
		http::get_with_params::<_, Response<Vec<maps::index::Response>>>(
			&format!("{}/maps", BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|map| map.try_into().ok())
		.collect(),
	)
}

/// Fetches all non-global/non-validated maps.
pub async fn get_nonglobal_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		validated: Some(false),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_nonglobal_maps {params:#?}");

	Ok(
		http::get_with_params::<_, Response<Vec<maps::index::Response>>>(
			&format!("{}/maps", BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|map| map.try_into().ok())
		.collect(),
	)
}

/// Fetches a single map.
pub async fn get_map(map_identifier: &MapIdentifier, client: &crate::Client) -> Result<Map> {
	trace!("> get_map {{ map_identifier: {map_identifier} }}");
	http::get::<Response<maps::index::Response>>(
		&format!("{}/maps/{}", BASE_URL, map_identifier),
		client,
	)
	.await
	.map(|response| response.result)?
	.try_into()
}

/// Fetches all maps with a specific tier.
pub async fn get_maps_by_tier(tier: Tier, client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		tier: Some(tier as u8),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_map_by_tier {params:#?}");
	Ok(
		http::get_with_params::<_, Response<Vec<maps::index::Response>>>(
			&format!("{}/maps", BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|map| map.try_into().ok())
		.collect(),
	)
}

/// Fetches all maps made by a specific person.
pub async fn get_maps_by_mapper(
	mapper: PlayerIdentifier,
	client: &crate::Client,
) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		created_by: Some(mapper.to_string()),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_map_by_mapper {params:#?}");
	Ok(
		http::get_with_params::<_, Response<Vec<maps::index::Response>>>(
			&format!("{}/maps", BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|map| map.try_into().ok())
		.collect(),
	)
}

/// Fetches all maps approved by a specific person.
pub async fn get_maps_by_approver(
	approver: PlayerIdentifier,
	client: &crate::Client,
) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		approved_by: Some(approver.to_string()),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_map_by_approver {params:#?}");
	Ok(
		http::get_with_params::<_, Response<Vec<maps::index::Response>>>(
			&format!("{}/maps", BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|map| map.try_into().ok())
		.collect(),
	)
}

/// The `/players` route.
pub mod players;
pub use players::{FancyPlayer, Player, RawFancyPlayer};

/// Fetches players.
pub async fn get_players(
	offset: i32,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<players::index::Player>> {
	let params = players::index::Params {
		offset: Some(offset),
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_players {params:#?}");

	players::get_players(params, client)
		.await
		.map(|response| response.result)
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
pub use servers::Server;

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

/// Fetches `limit` records. These are sorted by date by the API automatically.
pub async fn get_records(limit: u32, client: &crate::Client) -> Result<Vec<Record>> {
	let params = records::index::Params {
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_records {params:#?}");

	records::get_records(params, client).await
}

/// Fetches `limit` records for a player.
pub async fn get_player_records(
	player_identifier: PlayerIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let params = records::index::Params {
		player: Some(player_identifier),
		mode: Some(mode),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_player_records {params:#?}");

	records::get_records(params, client).await
}

/// Fetches the world record on a given map.
pub async fn get_wr(
	map_identifier: MapIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	client: &crate::Client,
) -> Result<Record> {
	let params = records::top::MapParams {
		mode: Some(mode),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(1),
		..Default::default()
	};
	trace!("> get_wr {params:#?}");
	Ok(records::get_top_map(map_identifier, params, client)
		.await?
		.remove(0))
}

/// Fetches the top 100 records on a given map.
pub async fn get_maptop(
	map_identifier: MapIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let params = records::top::MapParams {
		mode: Some(mode),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(100),
		..Default::default()
	};
	trace!("> get_maptop {params:#?}");
	records::get_top_map(map_identifier, params, client).await
}

/// Fetches a player's personal best on a given map.
pub async fn get_pb(
	player_identifier: PlayerIdentifier,
	map_identifier: MapIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	client: &crate::Client,
) -> Result<Record> {
	let params = records::top::PlayerParams {
		map: Some(map_identifier),
		mode: Some(mode),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(1),
		..Default::default()
	};
	trace!("> get_maptop {params:#?}");
	Ok(records::get_top_player(player_identifier, params, client)
		.await?
		.remove(0))
}

/// Fetches a player's most recent record.
pub async fn get_recent(
	player_identifier: PlayerIdentifier,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let params = records::index::Params {
		player: Some(player_identifier),
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_recent {params:#?}");
	Ok(records::get_records(params, client)
		.await?
		.into_iter()
		.collect())
}

/// Fetches a player's most recent personal best.
pub async fn get_recent_pb(
	player_identifier: PlayerIdentifier,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let params = records::top::PlayerParams {
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_recent_pb {params:#?}");
	Ok(records::get_top_player(player_identifier, params, client)
		.await?
		.into_iter()
		.collect())
}

/// Fetches the most recent record on a map.
pub async fn get_recent_on_map(
	map_identifier: MapIdentifier,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let params = records::index::Params {
		map: Some(map_identifier),
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_recent_on_map {params:#?}");
	Ok(records::get_records(params, client)
		.await?
		.into_iter()
		.collect())
}
