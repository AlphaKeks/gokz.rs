#![allow(dead_code)]
use futures::future::join_all;

use crate::prelude::*;

pub mod bans;
pub mod health;
pub mod maps;
pub mod modes;
pub mod player_ranks;
pub mod players;
pub mod record_filters;
pub mod records;

/// Constructs the base API route for the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
fn get_url() -> String {
	String::from("https://kztimerglobal.com/api/v2/")
}

trait IsResponse {}
trait IsParams {}

/// Makes an HTTPS GET request using a [`reqwest::Client`] and parses the response into a struct.
async fn api_request<'a, T, P>(
	route: &'a str,
	params: P,
	client: &reqwest::Client,
) -> Result<T, Error>
where
	T: serde::de::DeserializeOwned + IsResponse,
	P: serde::Serialize + IsParams,
{
	match client.get(get_url() + route).query(&params).send().await {
		Ok(response) => match response.json::<T>().await {
			Ok(parsed_response) => return Ok(parsed_response),
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::api_request"),
					tldr: String::from("Failed to parse JSON."),
					raw: Some(why.to_string()),
				})
			},
		},
		Err(why) => {
			return Err(Error {
				kind: ErrorKind::GlobalAPI,
				origin: String::from("gokz_rs::global_api::api_request"),
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			})
		},
	}
}

/// Will make an API request for all ban records of a given player, identified by their [`SteamID`].
pub async fn get_bans(
	steam_id: SteamID,
	client: &reqwest::Client,
) -> Result<Vec<bans::Response>, Error> {
	let params = bans::Params { steam_id: Some(steam_id.0), ..Default::default() };
	match api_request::<Vec<bans::Response>, _>(&bans::get_url(), params, client).await {
		Ok(response) => {
			if response.len() > 0 {
				return Ok(response);
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_ban"),
					tldr: String::from("No bans found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_bans", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_bans_test() {
	let client = reqwest::Client::new();

	let no_bans = SteamID(String::from("STEAM_1:0:165881949"));

	match get_bans(no_bans, &client).await {
		Err(why) => println!("Test successful: {:#?}", why),
		Ok(bans) => panic!("Test failed: {:#?}", bans),
	}

	let bans = SteamID(String::from("STEAM_1:1:161178172"));

	match get_bans(bans, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(bans) => println!("Test successful: {:#?}", bans),
	}
}

/// Will make an API request for all global maps. Since the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) contains more maps than
/// actually valid / "global" maps, this function will ensure to only request maps marked as
/// `validated`.
pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::Response>, Error> {
	let params = maps::Params { is_validated: Some(true), ..Default::default() };
	match api_request::<Vec<maps::Response>, _>(&maps::get_url(), params, client).await {
		Ok(maps) => {
			if maps.len() > 0 {
				return Ok(maps);
			} else {
				return Err(Error {
					kind: ErrorKind::GlobalAPI,
					origin: String::from("gokz_rs::global_api::get_maps"),
					tldr: String::from("No maps found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_maps", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_maps_test() {
	let client = reqwest::Client::new();

	match get_maps(&client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maps) => println!("Test successful: {} maps", maps.len()),
	}
}

/// Will request info about a specified map from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_map(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<maps::Response, Error> {
	let mut params =
		maps::Params { is_validated: Some(true), limit: Some(1), ..Default::default() };

	match map_identifier {
		MapIdentifier::ID(map_id) => params.id = Some(*map_id),
		MapIdentifier::Name(map_name) => params.name = Some(map_name.to_owned()),
	}

	match api_request::<Vec<maps::Response>, _>(&maps::get_url(), params, client).await {
		Ok(mut maps) => {
			if maps.len() > 0 {
				return Ok(maps.remove(0));
			} else {
				return Err(Error {
					kind: ErrorKind::Input,
					origin: String::from("gokz_rs::global_api::get_map"),
					tldr: String::from("This map is not global."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_map", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_map_test() {
	let client = reqwest::Client::new();

	match get_map(&MapIdentifier::Name(String::from("kz_lionharder")), &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successful: {:#?}", map),
	}

	match get_map(&MapIdentifier::ID(992), &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successful: {:#?}", map),
	}
}

/// Will request all 3 [Modes](`crate::prelude::Mode`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_modes(client: &reqwest::Client) -> Result<Vec<modes::Response>, Error> {
	match api_request::<Vec<modes::Response>, _>(
		&modes::get_url(),
		modes::Params::default(),
		client,
	)
	.await
	{
		Ok(modes) => {
			if modes.len() > 0 {
				return Ok(modes);
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_modes"),
					tldr: String::from("No modes found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_modes", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_modes_test() {
	let client = reqwest::Client::new();

	match get_modes(&client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(modes) => println!("Test successful: {:#?}\n({} modes)", modes, modes.len()),
	}
}

/// Will request a single mode from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
///
/// Note: You could either use a name or an id for this, it technically does not matter. I chose to
/// use an id.
pub async fn get_mode(mode: &Mode, client: &reqwest::Client) -> Result<modes::Response, Error> {
	match api_request::<modes::Response, _>(
		&modes::id::get_url(mode),
		modes::Params::default(),
		client,
	)
	.await
	{
		Ok(mode) => return Ok(mode),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_mode", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_mode_test() {
	let client = reqwest::Client::new();

	match get_mode(&Mode::KZTimer, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(mode) => assert_eq!(200, mode.id),
	}

	match get_mode(&Mode::SimpleKZ, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(mode) => assert_eq!(201, mode.id),
	}

	match get_mode(&Mode::Vanilla, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(mode) => assert_eq!(202, mode.id),
	}
}

/* TODO: figure out what this is for and implement it correctly */
// pub async fn get_player_ranks(
// 	mode: &Mode,
// 	limit: u32,
// 	client: &reqwest::Client,
// ) -> Result<player_ranks::Response, Error> {
// 	let params = player_ranks::Params {
// 		mode_ids: Some(vec![mode.as_id()]),
// 		limit: Some(limit),
// 		..Default::default()
// 	};
//
// 	api_request::<player_ranks::Response, player_ranks::Params>(player_ranks::ROUTE, params, client)
// 		.await
// }

/// Will request info about a player from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_player(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<players::Response, Error> {
	let mut params = players::Params::default();

	match player {
		PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
		PlayerIdentifier::SteamID64(steam_id64) => params.steamid64_list = Some(*steam_id64),
	}

	match api_request::<Vec<players::Response>, _>(&players::get_url(), params, client).await {
		Ok(mut players) => {
			if players.len() > 0 {
				return Ok(players.remove(0));
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_player"),
					tldr: String::from("No player found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_player", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_player_test() {
	let client = reqwest::Client::new();

	let alphakeks = PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:1:161178172")));
	let charlie = PlayerIdentifier::Name(String::from("charlieeilrahc"));

	match get_player(&alphakeks, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(player) => println!("Test successful: {:#?}", player),
	}

	match get_player(&charlie, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(player) => println!("Test successful: {:#?}", player),
	}
}

/// Will request all record filters for a map.
pub async fn get_filters(
	map_id: i16,
	client: &reqwest::Client,
) -> Result<Vec<record_filters::Response>, Error> {
	let params = record_filters::Params { map_ids: Some(map_id), ..Default::default() };
	match api_request::<Vec<record_filters::Response>, _>(
		&record_filters::get_url(),
		params,
		client,
	)
	.await
	{
		Ok(filters) => return Ok(filters),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_filters", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_filters_test() {
	let client = reqwest::Client::new();

	match get_filters(992, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(filters) => println!("Test successfuly: {:#?}", filters),
	}
}

/// Will request all filters for a given [`Mode`] and runtype (TP / PRO). This will result in the
/// distribution of record filters per [`Mode`].
pub async fn get_filter_dist(
	mode: &Mode,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<record_filters::Response>, Error> {
	let params = record_filters::Params {
		mode_ids: Some(mode.as_id()),
		has_teleports: Some(runtype),
		stages: Some(0),
		limit: Some(9999),
		..Default::default()
	};

	match api_request::<Vec<record_filters::Response>, record_filters::Params>(
		&record_filters::get_url(),
		params,
		client,
	)
	.await
	{
		Ok(filters) => return Ok(filters),
		Err(why) => {
			return Err(Error {
				origin: why.origin + " > gokz_rs::global_api::get_filter_dist",
				..why
			})
		},
	}
}

/// Will gather a list of maps which have not yet been completed by a given player and return their
/// names.
///
/// Note: the function needs to be this specific because the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) will return inconsistent
/// results if not enough arguments are provided.
pub async fn get_unfinished(
	player_identifier: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	tier: Option<u8>,
	client: &reqwest::Client,
) -> Result<Vec<String>, Error> {
	let doable = match get_filter_dist(mode, runtype, &client).await {
		Ok(filters) => filters,
		Err(why) => {
			return Err(Error {
				origin: why.origin + " > gokz_rs::global_api::get_unfinished",
				..why
			})
		},
	};

	let completed = match get_records(player_identifier, mode, runtype, 0, client).await {
		Ok(records) => records,
		Err(why) => {
			return Err(Error {
				origin: why.origin + " > gokz_rs::global_api::get_unfinished",
				..why
			})
		},
	};

	let completed: Vec<i16> = completed.into_iter().map(|rec| rec.map_id).collect();
	let mut uncomp_ids = Vec::new();

	for filter in doable {
		if !completed.contains(&filter.map_id) {
			uncomp_ids.push(filter.map_id);
		}
	}

	let global_maps = match get_maps(&client).await {
		Ok(maps) => maps,
		Err(why) => {
			return Err(Error {
				origin: why.origin + " > gokz_rs::global_api::get_unfinished",
				..why
			})
		},
	};
	let mut uncompleted = Vec::new();

	for map in global_maps {
		let matches_tier = match tier {
			Some(x) => map.difficulty == x,
			None => true,
		};
		let matches_runtype = if runtype { !&map.name.starts_with("kzpro_") } else { true };

		if uncomp_ids.contains(&map.id) && matches_tier && matches_runtype {
			uncompleted.push(map.name);
		}
	}

	return Ok(uncompleted);
}

#[cfg(test)]
#[tokio::test]
async fn get_unfinished_test() {
	let client = reqwest::Client::new();

	match get_unfinished(
		&PlayerIdentifier::Name(String::from("AlphaKeks")),
		&Mode::SimpleKZ,
		true,
		Some(7),
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maps) => println!("Test successful: {} maps left (alphakeks, skz, tp, t7)", maps.len()),
	}

	match get_unfinished(
		&PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:0:135486492"))),
		&Mode::SimpleKZ,
		false,
		None,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maps) => println!("Test successful: {} maps left (jucci, kzt, pro)", maps.len()),
	}

	match get_unfinished(
		&PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:0:46898346"))),
		&Mode::SimpleKZ,
		true,
		Some(7),
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maps) => println!("Test successful: {} maps left (charlie, skz, tp, t7)", maps.len()),
	}
}

/// Will request the #1 record on a given map.
pub async fn get_wr(
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<records::top::Response, Error> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.to_string()),
		has_teleports: Some(runtype),
		stage: Some(course),
		..Default::default()
	};

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
	}

	match api_request::<Vec<records::top::Response>, _>(&records::top::get_url(), params, client)
		.await
	{
		Ok(mut records) => {
			if records.len() > 0 {
				return Ok(records.remove(0));
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No WR found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_wr", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_wr_test() {
	let client = reqwest::Client::new();

	match get_wr(
		&MapIdentifier::Name(String::from("kz_lionharder")),
		&Mode::SimpleKZ,
		false,
		0,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(wr) => println!("Test successful: {:#?}", wr),
	}

	match get_wr(&MapIdentifier::ID(992), &Mode::KZTimer, true, 0, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(wr) => println!("Test successful: {:#?}", wr),
	}
}

/// Will request a player's personal best on a given map.
pub async fn get_pb(
	player: &PlayerIdentifier,
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<records::top::Response, Error> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.to_string()),
		has_teleports: Some(runtype),
		stage: Some(course),
		..Default::default()
	};

	match player {
		PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
		PlayerIdentifier::SteamID64(steam_id64) => params.steamid64 = Some(*steam_id64),
	}

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
	}

	match api_request::<Vec<records::top::Response>, _>(&records::top::get_url(), params, client)
		.await
	{
		Ok(mut records) => {
			if records.len() > 0 {
				return Ok(records.remove(0));
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No PB found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_pb", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_pb_test() {
	let client = reqwest::Client::new();

	match get_pb(
		&PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:1:161178172"))),
		&MapIdentifier::Name(String::from("kz_lionharder")),
		&Mode::SimpleKZ,
		false,
		0,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(wr) => println!("Test successful: {:#?}", wr),
	}

	match get_pb(
		&PlayerIdentifier::Name(String::from("racist75")),
		&MapIdentifier::ID(992),
		&Mode::SimpleKZ,
		true,
		0,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(wr) => println!("Test successful: {:#?}", wr),
	}
}

/// Will request the top 100 records on a given map.
pub async fn get_maptop(
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<Vec<records::top::Response>, Error> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.to_string()),
		has_teleports: Some(runtype),
		stage: Some(course),
		limit: Some(100),
		..Default::default()
	};

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
	}

	match api_request::<Vec<records::top::Response>, _>(&records::top::get_url(), params, client)
		.await
	{
		Ok(records) => {
			if records.len() > 0 {
				return Ok(records);
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No PB found."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_maptop", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_maptop_test() {
	let client = reqwest::Client::new();

	match get_maptop(
		&MapIdentifier::Name(String::from("kz_lionharder")),
		&Mode::SimpleKZ,
		false,
		0,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maptop) => println!("Test successful: {} records (lionharder, skz, pro)", maptop.len()),
	}

	match get_maptop(&MapIdentifier::ID(992), &Mode::KZTimer, true, 0, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maptop) => println!("Test successful: {} records (lionharder, kzt, tp)", maptop.len()),
	}
}

/// Will request all records of a player.
///
/// Note: the function needs to be this specific because the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) will return inconsistent
/// results if not enough arguments are provided.
pub async fn get_records(
	player: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<Vec<records::top::Response>, Error> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.as_str().to_owned()),
		has_teleports: Some(runtype),
		stage: Some(course),
		limit: Some(9999),
		..Default::default()
	};

	match player {
		PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
		PlayerIdentifier::SteamID64(steam_id64) => params.steamid64 = Some(*steam_id64),
	}

	match api_request::<Vec<records::top::Response>, _>(&records::top::get_url(), params, client)
		.await
	{
		Ok(records) => {
			if records.len() > 0 {
				return Ok(records);
			} else {
				return Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_times"),
					tldr: String::from("This player has 0 records."),
					raw: None,
				});
			}
		},
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_times", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_times_test() {
	let client = reqwest::Client::new();

	match get_records(
		&PlayerIdentifier::Name(String::from("AlphaKeks")),
		&Mode::SimpleKZ,
		true,
		0,
		&client,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(records) => println!("Test successful: {} records", records.len()),
	}
}

/// Will request all of a player's records and filter them to find the most recently set one.
pub async fn get_recent(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<records::top::Response, Error> {
	// get all records from a player
	// this needs to be very specific or the GlobalAPI won't give accurate results
	let modes = [Mode::KZTimer, Mode::SimpleKZ, Mode::Vanilla];
	let mut records = (join_all([
		get_records(player, &modes[0], true, 0, client),
		get_records(player, &modes[0], false, 0, client),
		get_records(player, &modes[1], true, 0, client),
		get_records(player, &modes[1], false, 0, client),
		get_records(player, &modes[2], true, 0, client),
		get_records(player, &modes[2], false, 0, client),
	])
	.await)
		.into_iter() // Vec<Result<Vec<Response>, Error>>
		.filter_map(|result| result.ok()) // filter out errors
		.flatten() // flatten into single Vec
		.collect::<Vec<_>>(); // Vec<Response>

	if records.len() < 1 {
		return Err(Error {
			kind: ErrorKind::NoData,
			origin: String::from("gokz_rs::global_api::get_recent"),
			tldr: String::from("No recent PB found."),
			raw: None,
		});
	}

	// store the most recent pb as (unix_timestamp, index)
	let mut recent = (0, 0);

	for i in 0..records.len() {
		let date = match chrono::NaiveDateTime::parse_from_str(
			&records[i].created_on,
			"%Y-%m-%dT%H:%M:%S",
		) {
			Ok(date) => date,
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::get_recent"),
					tldr: String::from("Failed to convert date."),
					raw: Some(why.to_string()),
				})
			},
		};

		// replace current `recent` if newer record was found
		if date.timestamp() > recent.0 {
			recent = (date.timestamp(), i);
		}
	}

	// return most recent pb using index
	Ok(records.remove(recent.1))
}

#[cfg(test)]
#[tokio::test]
async fn get_recent_test() {
	let client = reqwest::Client::new();

	let players = [
		PlayerIdentifier::Name(String::from("AlphaKeks")),
		PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:1:161178172"))),
		PlayerIdentifier::Name(String::from("racist75")),
		PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:1:152337044"))),
		PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
		PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:0:165881949"))),
		PlayerIdentifier::Name(String::from("charlieeilrahc")),
		PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:0:46898346"))),
		PlayerIdentifier::Name(String::from("Fob")),
		PlayerIdentifier::SteamID(SteamID(String::from("STEAM_1:1:96787045"))),
	];

	for player in players {
		match get_recent(&player, &client).await {
			Ok(recent) => {
				println!(
					"{:?}'s recent: {} ({} {}) - {}",
					&player,
					&recent.map_name,
					&recent.mode,
					if &recent.teleports > &0 { "TP" } else { "PRO" },
					&recent.time
				);
			},
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}
}

/// Will request the #placement of a given record.
pub async fn get_place(
	record_id: &u32,
	client: &reqwest::Client,
) -> Result<records::place::Response, Error> {
	match api_request::<records::place::Response, _>(
		&records::place::get_url(record_id),
		records::place::Params::default(),
		client,
	)
	.await
	{
		Ok(place) => return Ok(place),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_place", ..why })
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_place_test() -> Result<(), Error> {
	let client = reqwest::Client::new();

	let lionharder_pb = get_pb(
		&PlayerIdentifier::Name(String::from("AlphaKeks")),
		&MapIdentifier::ID(992),
		&Mode::SimpleKZ,
		false,
		0,
		&client,
	)
	.await?;

	match get_place(&lionharder_pb.id, &client).await {
		Ok(place) => {
			println!("Test successful: AlphaKeks is #{} on kz_lionharder (SKZ PRO).", place.0)
		},
		Err(why) => panic!("Test failed: {:#?}", why),
	}

	Ok(())
}

// --------------------------------------------------------------------------------------------- //

/// This function will check the most recent 10 health checks and return a
/// [fancy](health::Fancy) response.
pub async fn health_check(client: &reqwest::Client) -> Result<health::Fancy, Error> {
	match client.get(health::get_url()).send().await {
		Ok(response) => match response.json::<health::Response>().await {
			Ok(parsed_response) => {
				let mut result = health::Fancy { successful_responses: 0, fast_responses: 0 };

				for res in &parsed_response.results[0..10] {
					if res.condition_results[0].success {
						result.successful_responses += 1;
					}

					if res.condition_results[1].success {
						result.fast_responses += 1;
					}
				}

				return Ok(result);
			},
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::health_check"),
					tldr: String::from("Failed to parse JSON."),
					raw: Some(why.to_string()),
				})
			},
		},
		Err(why) => {
			return Err(Error {
				kind: ErrorKind::GlobalAPI,
				origin: String::from("gokz_rs::global_api::health_check"),
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			})
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn health_test() {
	let client = reqwest::Client::new();

	match health_check(&client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(data) => println!("Test successful: {:#?}", data),
	}
}

/// Will iterate over a list of [maps](maps::Response) and check if any of them match a given
/// [`MapIdentifier`].
///
/// Note: Technically you can pass in any list of [maps](maps::Response) but it is intended to be
/// used with [`get_maps`].
pub async fn is_global(
	map_identifier: &MapIdentifier,
	map_list: &Vec<maps::Response>,
) -> Result<maps::Response, Error> {
	match map_identifier {
		MapIdentifier::Name(map_name) => {
			for map in map_list {
				if map.name.contains(&map_name.to_lowercase()) {
					return Ok(map.to_owned());
				}
			}
		},
		MapIdentifier::ID(map_id) => {
			for map in map_list {
				if &map.id == map_id {
					return Ok(map.to_owned());
				}
			}
		},
	}

	return Err(Error {
		kind: ErrorKind::Input,
		origin: String::from("gokz_rs::global_api::is_global"),
		tldr: format!("{} is not global.", map_identifier),
		raw: None,
	});
}

#[cfg(test)]
#[tokio::test]
async fn is_global_test() -> Result<(), Error> {
	let client = reqwest::Client::new();

	let global_maps = get_maps(&client).await?;

	match is_global(&MapIdentifier::ID(1337), &global_maps).await {
		Ok(what) => panic!("KZ really did come far, huh?\n{:#?}", what),
		Err(why) => println!("Test successfully failed: {:#?}", why),
	}

	match is_global(&MapIdentifier::ID(992), &global_maps).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	match is_global(&MapIdentifier::Name(String::from("kz_lionHARDer")), &global_maps).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	match is_global(&MapIdentifier::Name(String::from("penis")), &global_maps).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	Ok(())
}

/// Returns download link to the replay of a given replay_id or an [`Error`]
pub async fn get_replay(replay_id: u32) -> Result<String, Error> {
	match replay_id {
		0 => {
			return Err(Error {
				kind: ErrorKind::NoData,
				origin: String::from("gokz_rs::global_api::get_replay"),
				tldr: String::from("`replay_id` is 0."),
				raw: None,
			})
		},
		replay_id => {
			// https://kztimerglobal.com/api/v2/records/replay/{replay_id}
			return Ok(
				crate::global_api::get_url() + &records::replay::replay_id::get_url(replay_id)
			);
		},
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_replay_test() -> Result<(), Error> {
	let client = reqwest::Client::new();

	let eventide_pb = get_pb(
		&PlayerIdentifier::Name(String::from("AlphaKeks")),
		&MapIdentifier::Name(String::from("kz_eventide")),
		&Mode::SimpleKZ,
		false,
		0,
		&client,
	)
	.await?;

	match get_replay(eventide_pb.replay_id).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(link) => println!("Test successful: {}", link),
	};

	Ok(())
}
