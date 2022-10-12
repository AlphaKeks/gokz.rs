#![allow(dead_code)]

pub mod global_api;
pub mod kzgo;
pub mod prelude;

pub mod functions {
	use std::collections::HashMap;

	use chrono::NaiveDateTime;
	use futures::future::join_all;
	use serde::de::DeserializeOwned;
	use serde::Serialize;

	use crate::global_api::{
		maps, modes, players, profile, record_filters, records, status, ParamData, ResponseData, BASE_URL,
	};
	use crate::kzgo;
	use crate::prelude::{Error, ErrorKind, MapIdentifier, Mode, PlayerIdentifier, Rank, SteamId};

	async fn api_request<T, P>(route: String, params: P, client: &reqwest::Client) -> Result<T, Error>
	where
		T: DeserializeOwned + ResponseData,
		P: Serialize + ParamData,
	{
		match client.get(format!("{}{}", BASE_URL, route)).query(&params).send().await {
			Ok(data) => match data.json::<T>().await {
				Ok(json) => Ok(json),
				Err(why) => Err(Error {
					kind: ErrorKind::Parsing,
					tldr: String::from("Failed to parse JSON."),
					raw: match why.status() {
						Some(status) => Some(status.to_string()),
						None => Some(why.to_string()),
					},
				}),
			},
			Err(why) => Err(Error {
				kind: ErrorKind::GlobalAPI,
				tldr: String::from("GlobalAPI Request failed."),
				raw: Some(why.to_string()),
			}),
		}
	}

	pub async fn check_api(client: &reqwest::Client) -> Result<status::APIStatusShort, Error> {
		match client
			.get("https://status.global-api.com/api/v2/summary.json")
			.send()
			.await
		{
			Ok(data) => match data.json::<status::APIStatus>().await {
				Ok(mut json) => Ok(status::APIStatusShort {
					status: json.status.description,
					frontend: json.components.remove(0).status,
					backend: json.components.remove(0).status,
				}),
				Err(why) => Err(Error {
					kind: ErrorKind::Parsing,
					tldr: String::from("Failed to parse JSON."),
					raw: Some(why.to_string()),
				}),
			},
			Err(why) => Err(Error {
				kind: ErrorKind::GlobalAPI,
				tldr: String::from("GlobalAPI Request failed."),
				raw: Some(why.to_string()),
			}),
		}
	}

	pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::Response>, Error> {
		let mut params = maps::Params::default();
		params.limit = Some(999);
		params.is_validated = Some(true);

		match api_request::<Vec<maps::Response>, maps::Params>(String::from("maps?"), params, &client).await {
			Ok(maps) => {
				if maps.len() > 0 {
					Ok(maps)
				} else {
					Err(Error {
						kind: ErrorKind::GlobalAPI,
						tldr: String::from("seems like gc deleted all the maps lololol"),
						raw: None,
					})
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_map(map: &MapIdentifier, client: &reqwest::Client) -> Result<maps::Response, Error> {
		let params = match map {
			MapIdentifier::Id(_) => {
				return Err(Error {
					kind: ErrorKind::InvalidInput,
					tldr: String::from("Please do not use an ID for this function."),
					raw: None,
				})
			}
			MapIdentifier::Name(name) => {
				let mut temp = maps::Params::default();
				temp.name = Some(name.to_owned());
				temp
			}
		};

		match api_request::<Vec<maps::Response>, maps::Params>(String::from("maps?"), params, client).await {
			Ok(mut data) => {
				if data.len() < 1 {
					Err(Error {
						kind: ErrorKind::GlobalAPI,
						tldr: String::from("This map does not exist."),
						raw: None,
					})
				} else {
					Ok(data.remove(0))
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn is_global(map: &MapIdentifier, map_list: &Vec<maps::Response>) -> Result<maps::Response, Error> {
		match map {
			MapIdentifier::Name(name) => {
				for map in map_list {
					if map.name.contains(&name.to_lowercase()) {
						return Ok(map.to_owned());
					}
				}
			}
			MapIdentifier::Id(id) => {
				for map in map_list {
					if &map.id == id {
						return Ok(map.to_owned());
					}
				}
			}
		}

		Err(Error {
			kind: ErrorKind::InvalidInput,
			tldr: String::from("This map is not global."),
			raw: None,
		})
	}

	pub async fn get_modes(client: &reqwest::Client) -> Result<Vec<modes::Response>, Error> {
		api_request::<Vec<modes::Response>, modes::name::Params>(String::from("modes?"), modes::name::Params {}, client)
			.await
	}

	pub async fn get_mode(mode: &Mode, client: &reqwest::Client) -> Result<modes::Response, Error> {
		api_request::<modes::Response, modes::name::Params>(mode.as_route(), modes::name::Params {}, &client).await
	}

	pub async fn get_player(
		player_identifier: &PlayerIdentifier,
		client: &reqwest::Client,
	) -> Result<players::Response, Error> {
		let mut params = players::Params::default();

		match player_identifier {
			PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.to_string()),
		}

		match api_request::<Vec<players::Response>, players::Params>(String::from("players?"), params, client).await {
			Ok(mut players) => {
				if players.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: String::from("No players found."),
						raw: None,
					})
				} else {
					Ok(players.remove(0))
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_wr(
		map: &MapIdentifier,
		mode: &Mode,
		course: u8,
		runtype: bool,
		client: &reqwest::Client,
	) -> Result<records::top::Response, Error> {
		let mut params = records::top::Params::default();

		params.tickrate = Some(128);
		params.stage = Some(course);
		params.has_teleports = Some(runtype);
		params.limit = Some(1);
		params.modes_list_string = Some(mode.as_str().to_owned());

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name.to_owned()),
			MapIdentifier::Id(id) => params.map_id = Some(id.to_owned()),
		}

		match api_request::<Vec<records::top::Response>, records::top::Params>(
			String::from("records/top?"),
			params,
			client,
		)
		.await
		{
			Ok(mut records) => {
				if records.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: String::from("No records found."),
						raw: None,
					})
				} else {
					Ok(records.remove(0))
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_maptop(
		map: &MapIdentifier,
		mode: &Mode,
		course: u8,
		runtype: bool,
		client: &reqwest::Client,
	) -> Result<Vec<records::top::Response>, Error> {
		let mut params = records::top::Params::default();

		params.tickrate = Some(128);
		params.stage = Some(course);
		params.has_teleports = Some(runtype);
		params.limit = Some(100);
		params.modes_list_string = Some(mode.as_str().to_owned());

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name.to_owned()),
			MapIdentifier::Id(id) => params.map_id = Some(id.to_owned()),
		}

		match api_request::<Vec<records::top::Response>, records::top::Params>(
			String::from("records/top?"),
			params,
			client,
		)
		.await
		{
			Ok(records) => {
				if records.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: String::from("No records found."),
						raw: None,
					})
				} else {
					Ok(records)
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_pb(
		player_identifier: &PlayerIdentifier,
		map: &MapIdentifier,
		mode: &Mode,
		course: u8,
		runtype: bool,
		client: &reqwest::Client,
	) -> Result<records::top::Response, Error> {
		let mut params = records::top::Params::default();

		params.tickrate = Some(128);
		params.stage = Some(course);
		params.has_teleports = Some(runtype);
		params.modes_list_string = Some(mode.as_str().to_owned());
		params.limit = Some(1);

		match player_identifier {
			PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.to_string()),
		}

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name.to_owned()),
			MapIdentifier::Id(id) => params.map_id = Some(id.to_owned()),
		}

		match api_request::<Vec<records::top::Response>, records::top::Params>(
			String::from("records/top?"),
			params,
			client,
		)
		.await
		{
			Ok(mut records) => {
				if records.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: String::from("No records found."),
						raw: None,
					})
				} else {
					Ok(records.remove(0))
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_times(
		player_identifier: &PlayerIdentifier,
		mode: &Mode,
		course: u8,
		runtype: bool,
		client: &reqwest::Client,
	) -> Result<Vec<records::top::Response>, Error> {
		let mut params = records::top::Params::default();

		params.tickrate = Some(128);
		params.modes_list_string = Some(mode.as_str().to_owned());
		params.stage = Some(course);
		params.has_teleports = Some(runtype);
		params.limit = Some(9999);

		match player_identifier {
			PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.to_string()),
		}

		let global_maps: Vec<String> = (get_maps(client).await?).into_iter().map(|m| m.name).collect();

		match api_request::<Vec<records::top::Response>, records::top::Params>(
			String::from("records/top?"),
			params,
			client,
		)
		.await
		{
			Ok(records) => {
				if records.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: String::from("No records found."),
						raw: None,
					})
				} else {
					let records = records
						.into_iter()
						.filter(|r| global_maps.contains(&r.map_name))
						.collect();

					Ok(records)
				}
			}
			Err(why) => Err(why),
		}
	}

	pub async fn get_recent(
		player_identifier: &PlayerIdentifier,
		client: &reqwest::Client,
	) -> Result<records::top::Response, Error> {
		let modes = [Mode::KZTimer, Mode::SimpleKZ, Mode::Vanilla];

		let mut records: Vec<records::top::Response> = vec![];

		(join_all([
			get_times(&player_identifier, &modes[0], 0, true, client),
			get_times(&player_identifier, &modes[0], 0, false, client),
			get_times(&player_identifier, &modes[1], 0, true, client),
			get_times(&player_identifier, &modes[1], 0, false, client),
			get_times(&player_identifier, &modes[2], 0, true, client),
			get_times(&player_identifier, &modes[2], 0, false, client),
		])
		.await)
			// TODO: make this less cringe
			.into_iter()
			.for_each(|req| {
				req.into_iter()
					.for_each(|r| r.into_iter().for_each(|r| records.push(r)))
			});

		if records.len() < 1 {
			return Err(Error {
				kind: ErrorKind::InvalidInput,
				tldr: String::from("No records found."),
				raw: None,
			});
		}

		let mut recent = (0, &records[0]);

		for i in 1..records.len() {
			let date = match NaiveDateTime::parse_from_str(&records[i].created_on, "%Y-%m-%dT%H:%M:%S") {
				Ok(date) => date,
				Err(why) => {
					return Err(Error {
						kind: ErrorKind::Parsing,
						tldr: String::from("Failed to parse date."),
						raw: Some(why.to_string()),
					})
				}
			};

			if date.timestamp() > recent.0 {
				recent = (date.timestamp(), &records[i]);
			}
		}

		Ok(recent.1.to_owned())
	}

	pub async fn get_place(
		record: &records::top::Response,
		client: &reqwest::Client,
	) -> Result<records::place::Response, Error> {
		api_request::<records::place::Response, records::place::Params>(
			format!("records/place/{}", record.id),
			records::place::Params {},
			client,
		)
		.await
	}

	pub async fn get_filter_dist(
		mode: &Mode,
		runtype: bool,
		client: &reqwest::Client,
	) -> Result<Vec<record_filters::base::Response>, Error> {
		let mut params = record_filters::base::Params::default();
		params.mode_ids = Some(mode.as_id());
		params.has_teleports = Some(runtype);
		params.stages = Some(0);
		params.tickrates = Some(128);
		params.limit = Some(9999);

		api_request::<Vec<record_filters::base::Response>, record_filters::base::Params>(
			String::from("record_filters?"),
			params,
			client,
		)
		.await
	}

	pub async fn get_unfinished(
		player_identifier: &PlayerIdentifier,
		mode: &Mode,
		runtype: bool,
		tier: Option<u8>,
		client: &reqwest::Client,
	) -> Result<Vec<String>, Error> {
		let doable = get_filter_dist(mode, runtype, &client).await?;
		let completed: Vec<u16> = (get_times(player_identifier, mode, 0, runtype, &client).await?)
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

	pub async fn get_profile(
		player_identifier: &PlayerIdentifier,
		mode: &Mode,
		client: &reqwest::Client,
	) -> Result<profile::Response, Error> {
		let mut player = profile::Response::default();

		let player_api = get_player(player_identifier, &client).await?;

		let player_identifier = PlayerIdentifier::SteamId(SteamId(player_api.steam_id.clone()));
		player.name = Some(player_api.name);
		player.steam_id = Some(player_api.steam_id);
		player.steam_id64 = Some(player_api.steamid64);
		player.is_banned = Some(player_api.is_banned);

		let mut global_maps = vec![HashMap::new(), HashMap::new()];

		if let Ok(maps) = get_maps(&client).await {
			for map in maps {
				global_maps[0].insert(map.name.clone(), map.difficulty);
				global_maps[1].insert(map.name, map.difficulty);
			}
		}

		let (tp, pro) = (
			get_times(&player_identifier, mode, 0, true, &client)
				.await
				.unwrap_or(vec![]),
			get_times(&player_identifier, mode, 0, false, &client)
				.await
				.unwrap_or(vec![]),
		);

		if tp.len() == 0 && pro.len() == 0 {
			return Err(Error {
				kind: ErrorKind::InvalidInput,
				tldr: String::from("This player has no records."),
				raw: None,
			});
		}

		let x = if tp.len() > pro.len() { tp.len() } else { pro.len() };

		for i in 0..x {
			if tp.len() > i {
				if global_maps[0].contains_key(&tp[i].map_name) {
					player.points.0 += tp[i].points as u32;
					player.completion[7].0 += 1;

					if let Some(tier) = global_maps[0].get(&tp[i].map_name) {
						player.completion[*tier as usize - 1].0 += 1;
					}

					if tp[i].points == 1000 {
						player.records.0 += 1;
					}

					global_maps[0].remove(&tp[i].map_name);
				}
			}

			if pro.len() > i {
				if global_maps[1].contains_key(&pro[i].map_name) {
					player.points.1 += pro[i].points as u32;
					player.completion[7].1 += 1;

					if let Some(tier) = global_maps[1].get(&pro[i].map_name) {
						player.completion[*tier as usize - 1].1 += 1;
					}

					if pro[i].points == 1000 {
						player.records.1 += 1;
					}

					global_maps[1].remove(&pro[i].map_name);
				}
			}
		}

		let total_points = &player.points.0 + &player.points.1;
		player.rank = Some(Rank::from_points(total_points, mode));

		let doable_count = kzgo::completion::get_completion_count(mode, &client).await?;

		let doable = [
			[
				doable_count.tp.one,
				doable_count.tp.two,
				doable_count.tp.three,
				doable_count.tp.four,
				doable_count.tp.five,
				doable_count.tp.six,
				doable_count.tp.seven,
				doable_count.tp.total,
			],
			[
				doable_count.pro.one,
				doable_count.pro.two,
				doable_count.pro.three,
				doable_count.pro.four,
				doable_count.pro.five,
				doable_count.pro.six,
				doable_count.pro.seven,
				doable_count.pro.total,
			],
		];

		for i in 0..8 {
			if player.completion[i].0 > 0 {
				player.completion_percentage[i].0 = (player.completion[i].0 as f32 / doable[0][i] as f32) * 100.0;
			}

			if player.completion[i].1 > 0 {
				player.completion_percentage[i].1 = (player.completion[i].1 as f32 / doable[1][i] as f32) * 100.0;
			}
		}

		Ok(player)
	}
}

#[cfg(test)]
mod function_tests {
	use reqwest::Client;

	use crate::{
		functions::{
			check_api, get_filter_dist, get_map, get_maps, get_maptop, get_mode, get_modes, get_pb, get_place,
			get_player, get_profile, get_recent, get_times, get_unfinished, get_wr, is_global,
		},
		prelude::{MapIdentifier, Mode, PlayerIdentifier, SteamId},
	};

	// #[tokio::test]
	// async fn _test() {
	// 	let client = Client::new();
	//
	// }

	#[tokio::test]
	async fn check_api_test() {
		let client = Client::new();

		match check_api(&client).await {
			Ok(status) => println!("Success: {:#?}", status),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_maps_test() {
		let client = Client::new();

		match get_maps(&client).await {
			Ok(maps) => println!("Success: {} maps", maps.len()),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_map_test() {
		let client = Client::new();

		let lionheart = MapIdentifier::Id(992);
		let lionharder = MapIdentifier::Name(String::from("kz_lionharder"));

		match get_map(&lionheart, &client).await {
			Ok(map) => println!("Unexpected success: {}", map.name),
			Err(why) => println!("Error (expected): {}", why.tldr),
		}

		match get_map(&lionharder, &client).await {
			Ok(map) => assert_eq!(map.name, String::from("kz_lionharder")),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn is_global_test() {
		let client = Client::new();

		let global_maps = get_maps(&client).await.unwrap();

		let lionheart = MapIdentifier::Name(String::from("kz_lionheart"));
		let lionharder = MapIdentifier::Id(992);
		let fake_map = MapIdentifier::Name(String::from("kz_fjerwiotgj3r9og"));
		let fake_map2 = MapIdentifier::Id(42069);

		match is_global(&lionheart, &global_maps).await {
			Ok(map) => println!("Success: {} is global.", map.name),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match is_global(&lionharder, &global_maps).await {
			Ok(map) => println!("Success: {} is global.", map.name),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match is_global(&fake_map, &global_maps).await {
			Ok(map) => println!("Unexpected Success: {}", map.name),
			Err(why) => println!("Error (expected): {}", why.tldr),
		}

		match is_global(&fake_map2, &global_maps).await {
			Ok(map) => println!("Unexpected Success: {}", map.name),
			Err(why) => println!("Error (expected): {}", why.tldr),
		}
	}

	#[tokio::test]
	async fn get_modes_test() {
		let client = Client::new();

		match get_modes(&client).await {
			Ok(modes) => assert_eq!(3, modes.len()),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_mode_test() {
		let client = Client::new();

		let kzt = Mode::KZTimer;
		let skz = Mode::SimpleKZ;
		let vnl = Mode::Vanilla;

		match get_mode(&kzt, &client).await {
			Ok(mode) => assert_eq!(200, mode.id),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_mode(&skz, &client).await {
			Ok(mode) => assert_eq!(201, mode.id),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_mode(&vnl, &client).await {
			Ok(mode) => assert_eq!(202, mode.id),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_player_test() {
		let client = Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:161178172"))),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:152337044"))),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:165881949"))),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:46898346"))),
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:96787045"))),
		];

		for player in players {
			match get_player(&player, &client).await {
				Ok(result) => println!("Got player: {}", result.name),
				Err(why) => panic!("Fail: {:#?}", why),
			}
		}
	}

	#[tokio::test]
	async fn get_wr_test() {
		let client = Client::new();

		let lionharder = MapIdentifier::Name(String::from("kz_lionharder"));
		let kiwiterror = MapIdentifier::Name(String::from("kz_kiwiterror"));

		if let Err(why) = get_wr(&lionharder, &Mode::KZTimer, 0, true, &client).await {
			panic!("Fail: {:#?}", why);
		}

		if let Err(why) = get_wr(&lionharder, &Mode::SimpleKZ, 0, true, &client).await {
			panic!("Fail: {:#?}", why);
		}

		if let Ok(wtf) = get_wr(&lionharder, &Mode::Vanilla, 0, true, &client).await {
			panic!("the hell: {:#?}", wtf);
		}

		if let Err(why) = get_wr(&lionharder, &Mode::KZTimer, 1, true, &client).await {
			panic!("Fail: {:#?}", why);
		}

		if let Err(why) = get_wr(&lionharder, &Mode::SimpleKZ, 1, true, &client).await {
			panic!("Fail: {:#?}", why);
		}

		if let Ok(wtf) = get_wr(&lionharder, &Mode::Vanilla, 1, true, &client).await {
			panic!("the hell: {:#?}", wtf);
		}

		if let Ok(wtf) = get_wr(&kiwiterror, &Mode::Vanilla, 0, false, &client).await {
			panic!("the hell: {:#?}", wtf);
		}
	}

	#[tokio::test]
	async fn get_maptop_test() {
		let client = Client::new();

		let maps = [
			MapIdentifier::Name(String::from("kz_kiwiterror")),
			MapIdentifier::Name(String::from("kz_lionharder")),
			MapIdentifier::Name(String::from("kz_erratum_v2")),
			MapIdentifier::Name(String::from("kz_beginnerblock_go")),
			MapIdentifier::Name(String::from("kz_hitech")),
		];

		for map in maps {
			match get_maptop(&map, &Mode::KZTimer, 0, true, &client).await {
				Ok(recs) => println!("Got {} records.", recs.len()),
				Err(why) => panic!("Fail: {:#?}", why),
			}
		}
	}

	#[tokio::test]
	async fn get_pb_test() {
		let client = Client::new();

		let player = PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:161178172")));
		let mode = Mode::SimpleKZ;

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_lionharder")),
			&mode,
			0,
			true,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_lionharder")),
			&mode,
			0,
			false,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_erratum_v2")),
			&mode,
			0,
			true,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_spacemario_h")),
			&mode,
			0,
			false,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_lionheart")),
			&mode,
			0,
			true,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}

		match get_pb(
			&player,
			&MapIdentifier::Name(String::from("kz_hitech")),
			&mode,
			0,
			false,
			&client,
		)
		.await
		{
			Ok(rec) => println!("Success: {:#?}", rec.time),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_times_test() {
		let client = Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:161178172"))),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:152337044"))),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:165881949"))),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:46898346"))),
			// TODO: fob gives 2 different results for some reason
			// (and they're both wrong lol)
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:96787045"))),
		];

		for player in players {
			match get_times(&player, &Mode::SimpleKZ, 0, true, &client).await {
				Ok(recs) => println!(
					"{:?} ({:?}) has {} SKZ TP records.",
					&recs[0].player_name,
					&recs[0].steam_id,
					recs.len()
				),
				Err(why) => panic!("Fail: {:#?}", why),
			}
		}
	}

	#[tokio::test]
	async fn get_recent_test() {
		let client = Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:161178172"))),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:152337044"))),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:165881949"))),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:46898346"))),
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:96787045"))),
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

	#[tokio::test]
	async fn get_place_test() {
		let client = Client::new();

		let lionharder_pb = get_pb(
			&PlayerIdentifier::Name(String::from("AlphaKeks")),
			&MapIdentifier::Name(String::from("kz_lionharder")),
			&Mode::SimpleKZ,
			0,
			true,
			&client,
		)
		.await
		.unwrap();

		match get_place(&lionharder_pb, &client).await {
			Ok(place) => println!("Success: {:#?}", place),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_filter_dist_test() {
		let client = Client::new();

		match get_filter_dist(&Mode::SimpleKZ, true, &client).await {
			Ok(result) => println!("Success: {:#?}", result),
			Err(why) => panic!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_unfinished_test() {
		let client = Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:161178172"))),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:152337044"))),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:165881949"))),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:0:46898346"))),
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:96787045"))),
		];

		for player in players {
			match get_unfinished(&player, &Mode::SimpleKZ, true, Some(4), &client).await {
				Ok(maps) => println!(
					"{:?} still needs to finish {} tier 4 maps in skz tp: {:?}",
					&player,
					maps.len(),
					maps
				),
				Err(why) => panic!("Fail: {:#?}", why),
			}
		}
	}

	#[tokio::test]
	async fn get_profile_test() {
		let client = Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamId(SteamId(String::from(String::from("STEAM_1:1:161178172")))),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamId(SteamId(String::from(String::from("STEAM_1:1:152337044")))),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamId(SteamId(String::from(String::from("STEAM_1:0:165881949")))),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamId(SteamId(String::from(String::from("STEAM_1:0:46898346")))),
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamId(SteamId(String::from("STEAM_1:1:96787045"))),
		];

		for player in players {
			match get_profile(&player, &Mode::KZTimer, &client).await {
				Ok(profile) => println!("Success ({:?}): {:?}", player, profile.completion),
				Err(why) => panic!("Fail: {:#?} ({:#?})", why, player),
			}
		}
	}
}
