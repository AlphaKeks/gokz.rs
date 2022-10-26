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

fn get_url() -> String {
	String::from("https://kztimerglobal.com/api/v2/")
}

trait IsResponse {}
trait IsParams {}

/// The base function that everything else relies on. Every function in this
/// module will at some point call this base function to call the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
///
/// [`api_request`] will try to make an HTTPS request to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and
/// parse the response into a struct.
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
		Err(why) => Err(Error {
			kind: ErrorKind::GlobalAPI,
			origin: String::from("gokz_rs::global_api::api_request"),
			tldr: String::from("GlobalAPI request failed."),
			raw: Some(why.to_string()),
		}),
		Ok(response) => match response.json::<T>().await {
			Err(why) => Err(Error {
				kind: ErrorKind::Parsing,
				origin: String::from("gokz_rs::global_api::api_request"),
				tldr: String::from("Failed to parse JSON."),
				raw: Some(why.to_string()),
			}),
			Ok(parsed_response) => Ok(parsed_response),
		},
	}
}

/// This function will request [all of a player's bans](`crate::global_api::bans::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no bans the function will return an [`Error`].
pub async fn get_bans(
	steam_id: SteamID,
	client: &reqwest::Client,
) -> Result<Vec<bans::Response>, Error> {
	let params = bans::Params {
		steam_id: Some(steam_id.0),
		..Default::default()
	};

	match api_request::<Vec<bans::Response>, bans::Params>(&bans::get_url(), params, client).await {
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_bans"),
			..why
		}),
		Ok(response) => {
			if response.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_ban"),
					tldr: String::from("No bans found."),
					raw: None,
				})
			} else {
				Ok(response)
			}
		}
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

/// This function will request [all maps](`crate::global_api::maps::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) which are marked as `validated` and return them.
/// If there are no maps the function will return an [`Error`]. (very unlikely)
pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::Response>, Error> {
	let params = maps::Params {
		is_validated: Some(true),
		..Default::default()
	};

	match api_request::<Vec<maps::Response>, maps::Params>(&maps::get_url(), params, client).await {
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_maps"),
			..why
		}),
		Ok(maps) => {
			if maps.len() < 1 {
				Err(Error {
					kind: ErrorKind::GlobalAPI,
					origin: String::from("gokz_rs::global_api::get_maps"),
					tldr: String::from("No maps found."),
					raw: None,
				})
			} else {
				Ok(maps)
			}
		}
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

/// This function will request [a specific map](`crate::global_api::maps::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) marked as `validated` and return it.
/// If there is no map the function will return an [`Error`].
pub async fn get_map(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<maps::Response, Error> {
	let mut params = maps::Params {
		is_validated: Some(true),
		limit: Some(1),
		..Default::default()
	};

	match map_identifier {
		MapIdentifier::ID(map_id) => params.id = Some(*map_id),
		MapIdentifier::Name(map_name) => params.name = Some(map_name.to_owned()),
	}

	match api_request::<Vec<maps::Response>, maps::Params>(&maps::get_url(), params, client).await {
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_map"),
			..why
		}),
		Ok(mut maps) => {
			if maps.len() < 1 {
				Err(Error {
					kind: ErrorKind::GlobalAPI,
					origin: String::from("gokz_rs::global_api::get_map"),
					tldr: String::from("This map is not global."),
					raw: None,
				})
			} else {
				Ok(maps.remove(0))
			}
		}
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

/// This function will request [all modes](`crate::global_api::modes::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no modes the function will return an [`Error`]. (very unlikely)
pub async fn get_modes(client: &reqwest::Client) -> Result<Vec<modes::Response>, Error> {
	match api_request::<Vec<modes::Response>, modes::Params>(
		&modes::get_url(),
		modes::Params::default(),
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_modes"),
			..why
		}),
		Ok(modes) => {
			if modes.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_modes"),
					tldr: String::from("No modes found."),
					raw: None,
				})
			} else {
				Ok(modes)
			}
		}
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

/// This function will request [a specific mode](`crate::global_api::modes::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there is no mode the function will return an [`Error`]. (very unlikely)
///
/// Although it doesn't really matter whether an ID or a Name is used here, I
/// think in the very unlikely case of modes changing their names, the modes
/// will keep their IDs.
pub async fn get_mode(mode: &Mode, client: &reqwest::Client) -> Result<modes::Response, Error> {
	match api_request::<modes::Response, modes::Params>(
		&modes::id::get_url(mode),
		modes::Params::default(),
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_mode"),
			..why
		}),
		Ok(mode) => Ok(mode),
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

/* TODO: figure out what this is for and implement it correctly

/// This function will request [data](`crate::global_api::player_ranks::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there is no data the function will return an [`Error`].
pub async fn get_player_ranks(
	mode: &Mode,
	limit: u32,
	client: &reqwest::Client,
) -> Result<player_ranks::Response, Error> {
	let params = player_ranks::Params {
		mode_ids: Some(vec![mode.as_id()]),
		limit: Some(limit),
		..Default::default()
	};

	api_request::<player_ranks::Response, player_ranks::Params>(player_ranks::ROUTE, params, client)
		.await
}

*/

/// This function will request [a player](`crate::global_api::players::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there is no mode the function will return an [`Error`].
pub async fn get_player(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<players::Response, Error> {
	let mut params = players::Params::default();

	match player {
		PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
	}

	match api_request::<Vec<players::Response>, players::Params>(
		&players::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_player"),
			..why
		}),
		Ok(mut players) => {
			if players.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_player"),
					tldr: String::from("No player found."),
					raw: None,
				})
			} else {
				Ok(players.remove(0))
			}
		}
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

/// This function will request [record filters](`crate::global_api::record_filters::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no filters the function will return an [`Error`].
pub async fn get_filters(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<Vec<record_filters::Response>, Error> {
	let mut params = record_filters::Params::default();

	match map_identifier {
		&MapIdentifier::Name(_) => {
			return Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::global_api::get_filters"),
				tldr: String::from("Please only use IDs for this function."),
				raw: None,
			})
		}
		&MapIdentifier::ID(map_id) => params.map_ids = Some(map_id),
	}

	match api_request::<Vec<record_filters::Response>, record_filters::Params>(
		&record_filters::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_filters"),
			..why
		}),
		Ok(filters) => Ok(filters),
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_filters_test() {
	let client = reqwest::Client::new();

	match get_filters(&MapIdentifier::Name(String::from("kz_lionharder")), &client).await {
		Err(why) => println!("Test failed successfully: {:#?}", why),
		Ok(filters) => panic!("Why did this work wtf: {:#?}", filters),
	}

	match get_filters(&MapIdentifier::ID(992), &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(filters) => println!("Test successfuly: {:#?}", filters),
	}
}

/// This function will request [record filters](`crate::global_api::record_filters::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no filters the function will return an [`Error`].
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
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_filter_dist"),
			..why
		}),
		Ok(filters) => Ok(filters),
	}
}

/// This function will request [all maps](`crate::global_api::record_filters::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) that haven't been finished by a given [player](`crate::global_api::players::Response`) and return them.
pub async fn get_unfinished(
	player_identifier: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	tier: Option<u8>,
	client: &reqwest::Client,
) -> Result<Vec<String>, Error> {
	let doable = get_filter_dist(mode, runtype, &client).await?;
	let completed: Vec<u16> = (get_times(player_identifier, mode, runtype, 0, client).await?)
		.into_iter()
		.map(|rec| rec.map_id)
		.collect();
	let mut uncomp_ids = vec![];

	for filter in doable {
		if !completed.contains(&filter.map_id) {
			uncomp_ids.push(filter.map_id);
		}
	}

	let global_maps = get_maps(&client).await?;
	let mut uncompleted = vec![];

	for map in global_maps {
		if uncomp_ids.contains(&map.id)
			&& (match tier {
				Some(x) => &map.difficulty == &x,
				None => true,
			}) && (if runtype {
			!&map.name.starts_with("kzpro_")
		} else {
			true
		}) {
			uncompleted.push(map.name);
		}
	}

	Ok(uncompleted)
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
		Ok(maps) => println!(
			"Test successful: {} maps left (alphakeks, skz, tp, t7)",
			maps.len()
		),
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
		Ok(maps) => println!(
			"Test successful: {} maps left (jucci, kzt, pro)",
			maps.len()
		),
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
		Ok(maps) => println!(
			"Test successful: {} maps left (charlie, skz, tp, t7)",
			maps.len()
		),
	}
}

/// This function will request [a world record](`crate::global_api::records::top::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there are no records the function will return an [`Error`].
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
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id.to_owned()),
	}

	match api_request::<Vec<records::top::Response>, records::top::Params>(
		&records::top::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_wr"),
			..why
		}),
		Ok(mut records) => {
			if records.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No WR found."),
					raw: None,
				})
			} else {
				Ok(records.remove(0))
			}
		}
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

/// This function will request [a player's personal best](`crate::global_api::records::top::Response`) on some map from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there are no records the function will return an [`Error`].
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
	}

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id.to_owned()),
	}

	match api_request::<Vec<records::top::Response>, records::top::Params>(
		&records::top::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_pb"),
			..why
		}),
		Ok(mut records) => {
			if records.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No PB found."),
					raw: None,
				})
			} else {
				Ok(records.remove(0))
			}
		}
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

/// This function will request [the top 100 records](`crate::global_api::records::top::Response`) on some map from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no records the function will return an [`Error`].
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
		MapIdentifier::ID(map_id) => params.map_id = Some(map_id.to_owned()),
	}

	match api_request::<Vec<records::top::Response>, records::top::Params>(
		&records::top::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_maptop"),
			..why
		}),
		Ok(records) => {
			if records.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_wr"),
					tldr: String::from("No PB found."),
					raw: None,
				})
			} else {
				Ok(records)
			}
		}
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
		Ok(maptop) => println!(
			"Test successful: {} records (lionharder, skz, pro)",
			maptop.len()
		),
	}

	match get_maptop(&MapIdentifier::ID(992), &Mode::KZTimer, true, 0, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maptop) => println!(
			"Test successful: {} records (lionharder, kzt, tp)",
			maptop.len()
		),
	}
}

/// This function will request [all times of a player](`crate::global_api::records::top::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no records the function will return an [`Error`].
pub async fn get_times(
	player: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<Vec<records::top::Response>, Error> {
	let mut params = records::top::Params {
		modes_list_string: Some(mode.to_string()),
		has_teleports: Some(runtype),
		stage: Some(course),
		limit: Some(9999),
		..Default::default()
	};

	match player {
		PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
	}

	match api_request::<Vec<records::top::Response>, records::top::Params>(
		&records::top::get_url(),
		params,
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_times"),
			..why
		}),
		Ok(records) => Ok(records),
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_times_test() {
	let client = reqwest::Client::new();

	match get_times(
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

/// This function will request [a player's most recent personal best](`crate::global_api::records::top::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If there are no records the function will return an [`Error`].
pub async fn get_recent(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<records::top::Response, Error> {
	let mut records = {
		let modes = [Mode::KZTimer, Mode::SimpleKZ, Mode::Vanilla];
		let mut records = vec![];

		(join_all([
			get_times(player, &modes[0], true, 0, client),
			get_times(player, &modes[0], false, 0, client),
			get_times(player, &modes[1], true, 0, client),
			get_times(player, &modes[1], false, 0, client),
			get_times(player, &modes[2], true, 0, client),
			get_times(player, &modes[2], false, 0, client),
		]) // get all records from a player
		.await)
			.into_iter()
			.for_each(|res| {
				// loop over the 6 requests
				if let Ok(recs) = res {
					for rec in recs {
						// loop over all records in each request
						records.push(rec);
					}
				}
			});

		records // resulting vec with ALL the records
	};

	if records.len() < 1 {
		return Err(Error {
			kind: ErrorKind::NoData,
			origin: String::from("gokz_rs::global_api::get_recent"),
			tldr: String::from("No recent PB found."),
			raw: None,
		});
	}

	let mut recent = (0, 0); // store the most recent pb as (unix_timestamp, index)

	for i in 1..records.len() {
		let date = match chrono::NaiveDateTime::parse_from_str(
			&records[i].updated_on,
			"%Y-%m-%dT%H:%M:%S",
		) {
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::get_recent"),
					tldr: String::from("Failed to convert date."),
					raw: Some(why.to_string()),
				})
			}
			Ok(date) => date,
		};

		// replace current `recent` if newer record was found
		if date.timestamp() > recent.0 {
			recent = (date.timestamp(), i);
		}
	}

	Ok(records.remove(recent.1)) // return most recent pb using index
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
			}
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}
}

/// This function will request request the leaderboard position of a [record](`crate::global_api::records::top::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return it.
/// If the given record doesn't have a position on the leaderboard the function will return an [`Error`].
/// This occurs sometimes, although I don't know why.
pub async fn get_place(
	record_id: &u32,
	client: &reqwest::Client,
) -> Result<records::place::Response, Error> {
	match api_request::<records::place::Response, records::place::Params>(
		&records::place::get_url(record_id),
		records::place::Params::default(),
		client,
	)
	.await
	{
		Err(why) => Err(Error {
			origin: String::from("gokz_rs::global_api::get_place"),
			..why
		}),
		Ok(place) => Ok(place),
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
		Ok(place) => println!(
			"Test successful: AlphaKeks is #{} on kz_lionharder (SKZ PRO).",
			place.0
		),
		Err(why) => panic!("Test failed: {:#?}", why),
	}

	Ok(())
}

// --- //

/// This function will check the most recent 10 health checks and return a
/// [fancy](health::Fancy) response.
pub async fn health_check(client: &reqwest::Client) -> Result<health::Fancy, Error> {
	match client.get(health::get_url()).send().await {
		Err(why) => Err(Error {
			kind: ErrorKind::GlobalAPI,
			origin: String::from("gokz_rs::global_api::health_check"),
			tldr: String::from("GlobalAPI request failed."),
			raw: Some(why.to_string()),
		}),
		Ok(response) => match response.json::<health::Response>().await {
			Err(why) => Err(Error {
				kind: ErrorKind::Parsing,
				origin: String::from("gokz_rs::global_api::health_check"),
				tldr: String::from("Failed to parse JSON."),
				raw: Some(why.to_string()),
			}),
			Ok(parsed_response) => {
				let mut result = health::Fancy {
					successful_responses: 0,
					fast_responses: 0,
				};

				for res in &parsed_response.results[0..10] {
					if res.condition_results[0].success {
						result.successful_responses += 1;
					}

					if res.condition_results[1].success {
						result.fast_responses += 1;
					}
				}

				Ok(result)
			}
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

/// This function will loop over [a list of maps](`crate::global_api::maps::Response`) and check
/// if the input is part of that list. If it is, it will return the appropriate entry from the list.
///
/// This can for example be used to turn the string "lionHARDer" into a full struct.
/// If the given input is not part of the list the function will return an [`Error`].
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
		}
		MapIdentifier::ID(map_id) => {
			for map in map_list {
				if &map.id == map_id {
					return Ok(map.to_owned());
				}
			}
		}
	}

	return Err(Error {
		kind: ErrorKind::Input,
		origin: String::from("gokz_rs::global_api::is_global"),
		tldr: String::from("This map is not global."),
		raw: None,
	});
}

#[cfg(test)]
#[tokio::test]
async fn is_global_test() -> Result<(), Error> {
	let client = reqwest::Client::new();

	let global_maps = get_maps(&client).await?;

	match is_global(&MapIdentifier::ID(42069), &global_maps).await {
		Ok(what) => panic!("KZ really did come far, huh?\n{:#?}", what),
		Err(why) => println!("Test successfully failed: {:#?}", why),
	}

	match is_global(&MapIdentifier::ID(992), &global_maps).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	match is_global(
		&MapIdentifier::Name(String::from("kz_lionHARDer")),
		&global_maps,
	)
	.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	match is_global(&MapIdentifier::Name(String::from("penis")), &global_maps).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test successfull: {:#?}", map),
	}

	Ok(())
}
