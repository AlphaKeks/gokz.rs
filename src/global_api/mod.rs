//! Types and functions to interact with the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

use {
	crate::{
		http, Error, MapIdentifier, Mode, PlayerIdentifier, Result, ServerIdentifier, SteamID, Tier,
	},
	chrono::NaiveDateTime,
	futures::future::join_all,
	log::trace,
	std::collections::HashSet,
};

/// Base URL for GlobalAPI requests.
pub const BASE_URL: &str = "https://kztimerglobal.com/api/v2";

/// The `/bans` route.
pub mod bans;
pub use bans::{Ban, BanType};

/// Fetches `limit` bans.
pub async fn get_bans(limit: u32, client: &crate::Client) -> Result<Vec<Ban>> {
	let params = bans::index::Params {
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_bans {params:#?} ");

	bans::get_bans(params, client).await
}

/// Fetches all bans for a given [`SteamID`].
pub async fn get_bans_for_player(steam_id: &SteamID, client: &crate::Client) -> Result<Vec<Ban>> {
	let params = bans::index::Params {
		steamid64: Some(steam_id.as_id64()),
		..Default::default()
	};
	trace!("> get_bans_for_player {params:#?}");

	bans::get_bans(params, client).await
}

/// Fetches all bans that happened since a specific date.
pub async fn get_bans_since(since: NaiveDateTime, client: &crate::Client) -> Result<Vec<Ban>> {
	let params = bans::index::Params {
		created_since: Some(since),
		..Default::default()
	};
	trace!("> get_bans_since {params:#?}");

	bans::get_bans(params, client).await
}

/// API health checks
pub mod health;
pub use health::{checkhealth, HealthReport};

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

	maps::get_maps(params, client).await
}

/// Fetches all global/validated maps.
pub async fn get_global_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		is_validated: Some(true),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_global_maps {params:#?}");

	maps::get_maps(params, client).await
}

/// Fetches all non-global/non-validated maps.
pub async fn get_nonglobal_maps(client: &crate::Client) -> Result<Vec<Map>> {
	let params = maps::index::Params {
		is_validated: Some(false),
		limit: Some(9999),
		..Default::default()
	};
	trace!("> get_nonglobal_maps {params:#?}");

	maps::get_maps(params, client).await
}

/// Fetches a list of all global map names.
pub async fn get_mapcycle(tier: Option<Tier>, client: &crate::Client) -> Result<Vec<String>> {
	let url = format!(
		"https://maps.global-api.com/mapcycles/{}",
		match tier {
			Some(tier) => format!("tier{}.txt", tier as u8),
			None => String::from("gokz.txt"),
		}
	);
	trace!("> get_mapcycle {{ tier: {tier:?}, url: {url} }}");

	Ok(http::get_text(&url, client)
		.await?
		.lines()
		.map(String::from)
		.collect())
}

/// Fetches a single map.
pub async fn get_map(map_identifier: &MapIdentifier, client: &crate::Client) -> Result<Map> {
	trace!("> get_map {{ map_identifier: {map_identifier} }}");
	match map_identifier {
		MapIdentifier::Name(map_name) => maps::get_map_by_name(map_name, client).await,
		MapIdentifier::ID(map_id) => maps::get_map_by_id(*map_id, client).await,
	}
}

/// Fetches all global maps and checks if a map exists that matches the given [`MapIdentifier`].
pub async fn is_global(map_ident: &MapIdentifier, client: &crate::Client) -> Result<Option<Map>> {
	trace!("> is_global {{ map_ident: {map_ident} }}");
	Ok(get_global_maps(client)
		.await?
		.into_iter()
		.find(|map| match map_ident {
			MapIdentifier::Name(map_name) => map
				.name
				.contains(&map_name.to_lowercase()),
			MapIdentifier::ID(map_id) => map.id == *map_id,
		}))
}

/// The `/players` route.
pub mod players;
pub use players::Player;

/// Fetches players.
pub async fn get_players(offset: i32, limit: u32, client: &crate::Client) -> Result<Vec<Player>> {
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
) -> Result<Player> {
	let mut params = players::index::Params::default();
	match player_identifier {
		PlayerIdentifier::Name(player_name) => params.name = Some(player_name),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
	};
	trace!("> get_player {params:#?}");

	Ok(players::get_players(params, client)
		.await?
		.remove(0))
}

/// The `/record_filters` route.
pub mod record_filters;
pub use record_filters::RecordFilter;

/// Fetches all filters for a given map.
pub async fn get_filters(map_id: u16, client: &crate::Client) -> Result<Vec<RecordFilter>> {
	let params = record_filters::index::Params {
		map_ids: Some(map_id),
		..Default::default()
	};
	trace!("> get_filters {params:#?}");

	record_filters::get_filters(params, client).await
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
	match server_identifier {
		ServerIdentifier::Name(server_name) => {
			servers::get_server_by_name(server_name, client).await
		}
		ServerIdentifier::ID(server_id) => servers::get_server_by_id(*server_id, client).await,
	}
}

/// The `/records` route (and subroutes).
pub mod records;
pub use records::{get_place, get_record, get_wr_top, Record};

/// Fetches `limit` records. Note that this only includes personal bests, not all records.
pub async fn get_records(limit: u32, client: &crate::Client) -> Result<Vec<Record>> {
	let params = records::top::Params {
		limit: Some(limit),
		..Default::default()
	};
	trace!("> get_records {params:#?}");

	records::get_top(params, client).await
}

/// Fetches `limit` records for a player. Note that this only includes personal bests, not all
/// records.
pub async fn get_player_records(
	player_identifier: PlayerIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let mut params = records::top::Params {
		tickrate: Some(128),
		modes_list_string: Some(mode.api()),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(limit),
		..Default::default()
	};
	match player_identifier {
		PlayerIdentifier::Name(player_name) => params.player_name = Some(player_name),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
	};
	trace!("> get_player_records {params:#?}");

	records::get_top(params, client).await
}

async fn get_records_on_map(
	map_identifier: MapIdentifier,
	player_identifier: Option<PlayerIdentifier>,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.api()),
		has_teleports: Some(has_teleports),
		stage: Some(course),
		limit: Some(limit),
		..Default::default()
	};
	if let Some(player_identifier) = player_identifier {
		match player_identifier {
			PlayerIdentifier::Name(player_name) => params.player_name = Some(player_name),
			PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
		};
	}
	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id),
	};
	trace!("> get_records_on_map {params:#?}");

	records::get_top(params, client).await
}

/// Fetches the world record on a given map.
pub async fn get_wr(
	map_identifier: MapIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	client: &crate::Client,
) -> Result<Record> {
	trace!("> get_wr -> get_records_on_map");
	Ok(
		get_records_on_map(map_identifier, None, mode, has_teleports, course, 1, client)
			.await?
			.remove(0),
	)
}

/// Fetches the top 100 records on a given map.
pub async fn get_maptop(
	map_identifier: MapIdentifier,
	mode: Mode,
	has_teleports: bool,
	course: u8,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	trace!("> get_maptop -> get_records_on_map");
	get_records_on_map(
		map_identifier,
		None,
		mode,
		has_teleports,
		course,
		100,
		client,
	)
	.await
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
	trace!("> get_pb -> get_records_on_map");
	Ok(get_records_on_map(
		map_identifier,
		Some(player_identifier),
		mode,
		has_teleports,
		course,
		1,
		client,
	)
	.await?
	.remove(0))
}

/// Fetches a player's most recent personal best.
pub async fn get_recent(
	player_identifier: PlayerIdentifier,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	trace!("> get_recent {{ player_identifier: {player_identifier}, limit: {limit} }}");

	let mut records = Vec::new();

	for chunk in join_all([
		get_player_records(
			player_identifier.clone(),
			Mode::KZTimer,
			true,
			0,
			999999,
			client,
		),
		get_player_records(
			player_identifier.clone(),
			Mode::KZTimer,
			false,
			0,
			999999,
			client,
		),
		get_player_records(
			player_identifier.clone(),
			Mode::SimpleKZ,
			true,
			0,
			999999,
			client,
		),
		get_player_records(
			player_identifier.clone(),
			Mode::SimpleKZ,
			false,
			0,
			999999,
			client,
		),
		get_player_records(
			player_identifier.clone(),
			Mode::Vanilla,
			true,
			0,
			999999,
			client,
		),
		get_player_records(
			player_identifier.clone(),
			Mode::Vanilla,
			false,
			0,
			999999,
			client,
		),
	])
	.await
	.into_iter()
	{
		match chunk {
			Ok(recs) => records.extend(recs),
			Err(why) => {
				if let Error::Http { status_code } = &why {
					// If this is ever `true` we probably made too many requests and want to abort.
					// The GlobalAPI unfortunately sometimes returns `INTERNAL_SERVER_ERROR`
					// even if it _should_ return `TOO_MANY_REQUESTS`.
					if status_code.0 == reqwest::StatusCode::INTERNAL_SERVER_ERROR
						|| status_code.0 == reqwest::StatusCode::TOO_MANY_REQUESTS
					{
						return Err(why);
					}
				}
			}
		};
	}

	if records.is_empty() {
		return Err(Error::EmptyResponse);
	}

	records.sort_by(|a, b| b.created_on.cmp(&a.created_on));

	Ok(records
		.into_iter()
		.take(limit as usize)
		.collect())
}

/// Fetches all the maps a player hasn't finished yet.
pub async fn get_unfinished(
	player_identifier: PlayerIdentifier,
	mode: Mode,
	has_teleports: bool,
	tier: Option<Tier>,
	client: &crate::Client,
) -> Result<Option<Vec<Map>>> {
	trace!("> get_unfinished {{ player_identifier: {player_identifier:?}, mode: {mode:?}, has_teleports: {has_teleports:?}, tier: {tier:?} }}");

	let completed_map_ids =
		get_player_records(player_identifier, mode, has_teleports, 0, 99999, client)
			.await?
			.into_iter()
			.map(|record| record.map_id)
			.collect::<HashSet<_>>();

	let all_map_ids = record_filters::get_filters(
		record_filters::index::Params {
			stages: Some(0),
			mode_ids: Some(mode as u8),
			tickrates: Some(128),
			has_teleports: Some(has_teleports),
			limit: Some(99999),
			..Default::default()
		},
		client,
	)
	.await?
	.into_iter()
	.map(|record_filter| record_filter.map_id)
	.collect::<HashSet<_>>();

	let uncompleted_map_ids = all_map_ids
		.difference(&completed_map_ids)
		.collect::<HashSet<_>>();

	let uncompleted_maps = get_global_maps(client)
		.await?
		.into_iter()
		.filter_map(|map| {
			let tier_matches = tier.map_or(true, |tier| tier == map.difficulty);
			let runtype_matches = if has_teleports {
				!map.name.starts_with("kzpro_")
			} else {
				true
			};

			if uncompleted_map_ids.contains(&map.id) && tier_matches && runtype_matches {
				return Some(map);
			}

			None
		})
		.collect::<Vec<_>>();

	if uncompleted_maps.is_empty() {
		Ok(None)
	} else {
		Ok(Some(uncompleted_maps))
	}
}

/// Returns a link to download a global replay by its ID.
pub async fn get_replay_download_link(replay_id: u32) -> String {
	trace!("> get_replay_download_link {{ replay_id: {replay_id} }}");
	format!("{}/records/replay/{}", BASE_URL, replay_id)
}

/// Returns a link to watch a global replay using
/// [GameChaos' GlobalReplays Project](https://github.com/GameChaos/GlobalReplays).
pub async fn get_replay_view_link(replay_id: u32) -> String {
	trace!("> get_replay_view_link {{ replay_id: {replay_id} }}");
	format!("http://gokzmaptest.site.nfoservers.com/GlobalReplays/?replay={replay_id}")
}
