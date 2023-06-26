/// The `/bans` route
pub mod bans;

pub use bans::{Ban, BanType};

/// The `/maps` routes
pub mod maps;
pub use maps::Map;

/// The `/servers` routes
pub mod servers;
pub use servers::Server;

/// The `/modes` routes
pub mod modes;
pub use modes::Mode;

/// The `/players` routes
pub mod players;
pub use players::Player;

/// The `/record_filters` routes
pub mod record_filters;
pub use record_filters::RecordFilter;

/// The `/records/*` routes
pub mod records;
pub use records::Record;

#[rustfmt::skip]
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::utils::EmptyParams;
#[rustfmt::skip]
use crate::{error::{Error, Result}, prelude};
use std::collections::HashSet;

/// The base URL for all API requests.
pub const BASE_URL: &str = "https://kztimerglobal.com/api/v2";

/// Get the last `limit` bans
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_bans(limit: u32, client: &crate::Client) -> Result<Vec<bans::Ban>> {
	let params = bans::Params {
		limit: Some(limit),
		..Default::default()
	};

	bans::root(&params, client).await
}

/// Get all bans for a given player
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_player_bans<S>(steam_id: S, client: &crate::Client) -> Result<Vec<bans::Ban>>
where
	S: Into<prelude::SteamID> + std::fmt::Debug,
{
	let params = bans::Params {
		steam_id: Some(steam_id.into()),
		limit: None,
		..Default::default()
	};

	bans::root(&params, client).await
}

/// Get all bans since a given date
#[cfg(feature = "chrono")]
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_bans_since<D>(since: D, client: &crate::Client) -> Result<Vec<bans::Ban>>
where
	D: Into<DateTime<Utc>> + std::fmt::Debug,
{
	let params = bans::Params {
		created_since: Some(since.into()),
		limit: None,
		..Default::default()
	};

	bans::root(&params, client).await
}

/// Get all bans since a given date
#[cfg(not(feature = "chrono"))]
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_bans_since(since: String, client: &crate::Client) -> Result<Vec<bans::Ban>> {
	let params = bans::Params {
		created_since: Some(since),
		limit: None,
		..Default::default()
	};

	bans::root(&params, client).await
}

/// Get `limit` or less maps
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_maps(limit: u32, client: &crate::Client) -> Result<Vec<maps::Map>> {
	let params = maps::Params {
		limit: Some(limit),
		..Default::default()
	};

	maps::root(&params, client).await
}

/// Get global (`validated`) maps
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_global_maps(limit: u32, client: &crate::Client) -> Result<Vec<maps::Map>> {
	let params = maps::Params {
		is_validated: Some(true),
		limit: Some(limit),
		..Default::default()
	};

	maps::root(&params, client).await
}

/// Get non-global (`validated`) maps
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_non_global_maps(limit: u32, client: &crate::Client) -> Result<Vec<maps::Map>> {
	let params = maps::Params {
		is_validated: Some(false),
		limit: Some(limit),
		..Default::default()
	};

	maps::root(&params, client).await
}

/// Get a single map
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_map<M>(map_identifier: M, client: &crate::Client) -> Result<maps::Map>
where
	M: Into<prelude::MapIdentifier> + std::fmt::Debug,
{
	match map_identifier.into() {
		// For some reason the `/maps/:map_id` endpoint seems to be broken.
		prelude::MapIdentifier::Id(map_id) => {
			let params = maps::Params {
				id: Some(map_id),
				..Default::default()
			};

			let response = maps::root(&params, client).await?;
			let map = response.into_iter().next().expect("The response should not be empty.");

			Ok(map)
		}
		prelude::MapIdentifier::Name(map_name) => maps::name(&map_name, client).await,
	}
}

/// Get a list of all global maps
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_mapcycle<T>(tier_filter: T, client: &crate::Client) -> Result<String>
where
	T: Into<Option<prelude::Tier>> + std::fmt::Debug,
{
	let url = format!(
		"https://maps.global-api.com/mapcycles/{tier}",
		tier = match tier_filter.into() {
			None => String::from("gokz.txt"),
			Some(tier) => format!("tier{}.txt", tier as u8),
		}
	);

	crate::http::get_text(&url, &EmptyParams, client).await
}

/// Check if a map is global by fetching all global maps and checking if it's in the list
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn is_global<M>(
	map_identifier: M,
	client: &crate::Client,
) -> Result<(Option<Map>, Vec<Map>)>
where
	M: Into<prelude::MapIdentifier> + std::fmt::Debug,
{
	let mut map_identifier = map_identifier.into();
	let global_maps = get_global_maps(9999, client).await?;

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

/// Get information about all global modes
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_modes(client: &crate::Client) -> Result<Vec<modes::Mode>> {
	modes::root(client).await
}

/// Get information about a specific mode
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_mode<M>(mode: M, client: &crate::Client) -> Result<modes::Mode>
where
	M: Into<prelude::Mode> + std::fmt::Debug,
{
	modes::id(mode.into() as u8, client).await
}

/// Get `limit` or less players
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_players(limit: u32, client: &crate::Client) -> Result<Vec<players::Player>> {
	let params = players::Params {
		limit: Some(limit),
		..Default::default()
	};

	players::root(&params, client).await
}

/// Get a single player
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_player<P>(player_identifier: P, client: &crate::Client) -> Result<players::Player>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
{
	match player_identifier.into() {
		prelude::PlayerIdentifier::SteamID(steam_id) => players::steam_id(steam_id, client).await,

		prelude::PlayerIdentifier::Name(name) => {
			let params = players::Params {
				name: Some(name),
				..Default::default()
			};

			let response = players::root(&params, client).await?;
			let player = response.into_iter().next().expect("The response should not be empty.");

			Ok(player)
		}
	}
}

/// Get the filters for a given map
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_filters(
	map_id: u16,
	client: &crate::Client,
) -> Result<Vec<record_filters::RecordFilter>> {
	let params = record_filters::Params {
		map_ids: Some(map_id),
		limit: Some(999),
		..Default::default()
	};

	record_filters::root(&params, client).await
}

/// Get a record by id
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_record(record_id: u32, client: &crate::Client) -> Result<records::Record> {
	records::root(record_id, client).await
}

/// Get the leaderboard spot of a single record
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_place(record_id: u32, client: &crate::Client) -> Result<u32> {
	records::place(record_id, client).await
}

/// Get `limit` personal bests of a player
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_player_records<P, MI, M, R>(
	player_identifier: P,
	map_identifier: MI,
	mode: M,
	runtype: R,
	course: u8,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<records::Record>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
	MI: Into<Option<prelude::MapIdentifier>> + std::fmt::Debug,
	M: Into<prelude::Mode> + std::fmt::Debug,
	R: Into<prelude::Runtype> + std::fmt::Debug,
{
	let mut params = records::top::Params {
		modes_list_string: Some(mode.into().api()),
		runtype: Some(runtype.into()),
		stage: Some(course),
		limit: Some(limit),
		..Default::default()
	};

	match player_identifier.into() {
		prelude::PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
		prelude::PlayerIdentifier::Name(name) => params.player_name = Some(name),
	};

	if let Some(map_identifier) = map_identifier.into() {
		match map_identifier {
			prelude::MapIdentifier::Id(map_id) => params.map_id = Some(map_id),
			prelude::MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
		};
	}

	records::top::root(&params, client).await
}

/// Get the world record on a map
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_wr<MI, M, R>(
	map_identifier: MI,
	mode: M,
	runtype: R,
	course: u8,
	client: &crate::Client,
) -> Result<records::Record>
where
	MI: Into<prelude::MapIdentifier> + std::fmt::Debug,
	M: Into<prelude::Mode> + std::fmt::Debug,
	R: Into<prelude::Runtype> + std::fmt::Debug,
{
	let mut params = records::top::Params {
		modes_list_string: Some(mode.into().api()),
		runtype: Some(runtype.into()),
		stage: Some(course),
		..Default::default()
	};

	match map_identifier.into() {
		prelude::MapIdentifier::Id(map_id) => params.map_id = Some(map_id),
		prelude::MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	records::top::root(&params, client)
		.await
		.map(|records| records.into_iter().next().expect("The response should not be empty."))
}

/// Get a player's personal best on a map
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_pb<P, MI, M, R>(
	player_identifier: P,
	map_identifier: MI,
	mode: M,
	runtype: R,
	course: u8,
	client: &crate::Client,
) -> Result<records::Record>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
	MI: Into<prelude::MapIdentifier> + std::fmt::Debug,
	M: Into<prelude::Mode> + std::fmt::Debug,
	R: Into<prelude::Runtype> + std::fmt::Debug,
{
	let mut params = records::top::Params {
		modes_list_string: Some(mode.into().api()),
		runtype: Some(runtype.into()),
		stage: Some(course),
		..Default::default()
	};

	match player_identifier.into() {
		prelude::PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id),
		prelude::PlayerIdentifier::Name(name) => params.player_name = Some(name),
	};

	match map_identifier.into() {
		prelude::MapIdentifier::Id(map_id) => params.map_id = Some(map_id),
		prelude::MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	records::top::root(&params, client)
		.await
		.map(|records| records.into_iter().next().expect("The response should not be empty."))
}

/// Get the top 100 records on a map
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_maptop<MI, M, R>(
	map_identifier: MI,
	mode: M,
	runtype: R,
	course: u8,
	client: &crate::Client,
) -> Result<Vec<records::Record>>
where
	MI: Into<prelude::MapIdentifier> + std::fmt::Debug,
	M: Into<prelude::Mode> + std::fmt::Debug,
	R: Into<prelude::Runtype> + std::fmt::Debug,
{
	let mut params = records::top::Params {
		modes_list_string: Some(mode.into().api()),
		runtype: Some(runtype.into()),
		stage: Some(course),
		limit: Some(100),
		..Default::default()
	};

	match map_identifier.into() {
		prelude::MapIdentifier::Id(map_id) => params.map_id = Some(map_id),
		prelude::MapIdentifier::Name(map_name) => params.map_name = Some(map_name),
	};

	records::top::root(&params, client).await
}

/// Get a player's most recent `limit` personal best(s)
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_recent<P>(
	player_identifier: P,
	limit: u32,
	client: &crate::Client,
) -> Result<Vec<records::Record>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
{
	let player_identifier = player_identifier.into();

	let requests = [
		(prelude::Mode::KZTimer, prelude::Runtype::TP),
		(prelude::Mode::KZTimer, prelude::Runtype::Pro),
		(prelude::Mode::SimpleKZ, prelude::Runtype::TP),
		(prelude::Mode::SimpleKZ, prelude::Runtype::Pro),
		(prelude::Mode::Vanilla, prelude::Runtype::TP),
		(prelude::Mode::Vanilla, prelude::Runtype::Pro),
	]
	.into_iter()
	.map(|(mode, runtype)| {
		get_player_records(player_identifier.clone(), None, mode, runtype, 0, 99999, client)
	});

	let mut records = Vec::new();

	for batch in futures::future::join_all(requests).await {
		match batch {
			Ok(batch) => records.extend(batch),
			Err(Error::EmptyResponse) => {}
			Err(Error::Http {
				code,
				message,
			}) if code == 429 => {
				return Err(Error::Http {
					code,
					message,
				});
			}
			Err(err) => return Err(err),
		};
	}

	match records.is_empty() {
		true => Err(Error::EmptyResponse),
		false => Ok(records),
	}
}

/// Get a list of maps a player hasn't finished yet
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_unfinished<P, M, R, T>(
	player_identifier: P,
	mode: M,
	runtype: R,
	tier: T,
	limit: u32,
	client: &crate::Client,
) -> Result<Option<Vec<maps::Map>>>
where
	P: Into<prelude::PlayerIdentifier> + std::fmt::Debug,
	M: Into<prelude::Mode> + std::fmt::Debug,
	R: Into<prelude::Runtype> + std::fmt::Debug,
	T: Into<Option<prelude::Tier>> + std::fmt::Debug,
{
	let player_identifier = player_identifier.into();
	let mode = mode.into();
	let runtype = runtype.into();
	let tier = tier.into();

	let completed = get_player_records(player_identifier, None, mode, runtype, 0, limit, client)
		.await?
		.into_iter()
		.map(|record| record.map_id)
		.collect::<HashSet<_>>();

	let filter_params = record_filters::Params {
		stages: Some(0),
		mode_ids: Some(mode as u8),
		tickrates: Some(128),
		runtype: Some(runtype),
		limit: Some(99999),
		..Default::default()
	};

	let (filters, global_maps) = futures::future::join(
		record_filters::root(&filter_params, client),
		get_global_maps(99999, client),
	)
	.await;

	let filters = filters?.into_iter().map(|filter| filter.map_id).collect::<HashSet<_>>();

	let uncompleted = filters.difference(&completed).collect::<HashSet<_>>();

	let unfinished = global_maps?
		.into_iter()
		.filter_map(|map| {
			let tier_matches = tier.map_or(true, |tier| tier == map.difficulty);
			let runtype_matches = *runtype && !map.name.starts_with("kzpro_");
			(uncompleted.contains(&map.id) && tier_matches && runtype_matches).then_some(map)
		})
		.collect::<Vec<_>>();

	match unfinished.is_empty() {
		true => Ok(None),
		false => Ok(Some(unfinished)),
	}
}

/// Get a specific server
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_server<S>(server_identifier: S, client: &crate::Client) -> Result<Server>
where
	S: Into<prelude::ServerIdentifier> + std::fmt::Debug,
{
	match server_identifier.into() {
		prelude::ServerIdentifier::Id(server_id) => servers::id(server_id, client).await,
		prelude::ServerIdentifier::Name(server_name) => servers::name(&server_name, client).await,
	}
}

/// Get a list of servers
#[tracing::instrument(level = "INFO", skip(client), err(Debug))]
pub async fn get_servers(limit: u32, client: &crate::Client) -> Result<Vec<Server>> {
	let params = servers::Params {
		limit: Some(limit),
		..Default::default()
	};

	servers::root(&params, client).await
}
