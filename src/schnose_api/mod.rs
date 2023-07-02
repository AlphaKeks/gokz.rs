/// `/modes` routes
pub mod modes;
pub use modes::Mode;

/// `/players` routes
pub mod players;
pub use players::Player;

/// `/maps` routes
pub mod maps;
pub use maps::{Course, Map};

/// `/filters` routes
pub mod filters;
pub use filters::Filter;

/// `/servers` routes
pub mod servers;
pub use servers::Server;

/// `/records` routes
pub mod records;
pub use records::Record;

#[rustfmt::skip]
use crate::{
	error::Result,
	prelude,
	utils::EmptyParams,
};

/// The base URL for all API requests.
pub const BASE_URL: &str = "https://schnose.xyz/api";

/// The URL for the API's SwaggerUI page.
pub const SWAGGER_URL: &str = "https://schnose.xyz/api/docs/swagger";

/// Check if the API is up
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn health(client: &crate::Client) -> Result<String> {
	crate::http::get_json::<serde_json::Value, _>(
		&format!("{BASE_URL}/health"),
		&EmptyParams,
		client,
	)
	.await
	.map(|json| json.to_string())
}

/// Fetches all game modes
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_modes(client: &crate::Client) -> Result<Vec<modes::Mode>> {
	modes::root(client).await
}

/// Fetches a single game mode
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_mode<M>(mode: M, client: &crate::Client) -> Result<modes::Mode>
where
	M: Into<prelude::Mode> + std::fmt::Debug,
{
	modes::ident(mode.into(), client).await
}

/// Fetches `limit` or less players (1000 max.)
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_players<L>(limit: L, client: &crate::Client) -> Result<Vec<players::Player>>
where
	L: Into<u64> + std::fmt::Debug,
{
	let params = players::Params {
		limit: Some(limit.into()),
		..Default::default()
	};

	players::root(&params, client).await
}

/// Fetches a single player
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_player<P>(player: P, client: &crate::Client) -> Result<players::Player>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
{
	players::ident(player.into(), client).await
}

/// Fetches maps
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_maps(client: &crate::Client) -> Result<Vec<maps::Map>> {
	let params = maps::Params {
		limit: Some(9999),
		..Default::default()
	};

	maps::root(&params, client).await
}

/// Fetches maps made by a specific player
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_maps_by<P>(mapper: P, client: &crate::Client) -> Result<Vec<maps::Map>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
{
	let params = maps::Params {
		limit: Some(9999),
		mapper: Some(mapper.into()),
		..Default::default()
	};

	maps::root(&params, client).await
}

/// Fetches a single map
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_map<M>(map: M, client: &crate::Client) -> Result<maps::Map>
where
	M: Into<prelude::MapIdentifier> + std::fmt::Debug,
{
	maps::ident(map.into(), client).await
}

/// Check if a map is global either by fetching all global maps and checking if it's in the list,
/// or by checking if the provided `maps` contains it. Returns the list of maps it searched
/// through as well.
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn is_global<M>(
	map_identifier: M,
	maps: Option<Vec<maps::Map>>,
	client: &crate::Client,
) -> Result<(Option<Map>, Vec<maps::Map>)>
where
	M: Into<prelude::MapIdentifier> + std::fmt::Debug,
{
	let mut map_identifier = map_identifier.into();

	let params = maps::Params {
		global: Some(true),
		limit: Some(9999),
		..Default::default()
	};

	let global_maps = match maps {
		None => maps::root(&params, client).await?,
		Some(maps) => maps,
	};

	if let prelude::MapIdentifier::Name(ref mut map_name) = map_identifier {
		*map_name = map_name.to_lowercase();
	}

	let mut iter = global_maps.iter();
	let map = match map_identifier {
		prelude::MapIdentifier::Id(map_id) => iter.find(|map| map.id == map_id),
		prelude::MapIdentifier::Name(ref map_name) => iter.find(|map| map.name.contains(map_name)),
	}
	.map(ToOwned::to_owned);

	Ok((map, global_maps))
}

/// Fetches filters for a given map
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_filters<M>(map: M, client: &crate::Client) -> Result<Vec<filters::Filter>>
where
	M: Into<prelude::MapIdentifier> + std::fmt::Debug,
{
	filters::map(map.into(), client).await
}

/// Fetches servers
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_servers(client: &crate::Client) -> Result<Vec<servers::Server>> {
	let params = servers::Params {
		limit: Some(9999),
		..Default::default()
	};

	servers::root(&params, client).await
}

/// Fetches servers owned by a specific player
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_servers_owned_by<P>(
	owner: P,
	client: &crate::Client,
) -> Result<Vec<servers::Server>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
{
	let params = servers::Params {
		limit: Some(9999),
		owned_by: Some(owner.into()),
		..Default::default()
	};

	servers::root(&params, client).await
}

/// Fetches a single server
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_server<S>(server: S, client: &crate::Client) -> Result<servers::Server>
where
	S: Into<prelude::ServerIdentifier> + std::fmt::Debug,
{
	servers::ident(server.into(), client).await
}

/// Fetches the most recent `limit` records (max. 1000).
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_records<L>(limit: L, client: &crate::Client) -> Result<Vec<records::Record>>
where
	L: Into<u64> + std::fmt::Debug,
{
	let params = records::Params {
		limit: Some(limit.into()),
		..Default::default()
	};

	records::root(&params, client).await
}

/// Fetches the most recent `limit` records (max. 1000) for a given player.
#[tracing::instrument(level = "DEBUG", skip(client))]
pub async fn get_player_records<P, L>(
	player: P,
	limit: L,
	client: &crate::Client,
) -> Result<Vec<records::Record>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
	L: Into<u64> + std::fmt::Debug,
{
	let params = records::Params {
		player: Some(player.into()),
		limit: Some(limit.into()),
		..Default::default()
	};

	records::root(&params, client).await
}
