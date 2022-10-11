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
	use crate::prelude::{Error, ErrorKind, MapIdentifier, Mode, PlayerIdentifier, Rank};

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
					tldr: "Failed to parse JSON.",
					raw: Some(why.to_string()),
				}),
			},
			Err(why) => Err(Error {
				kind: ErrorKind::GlobalAPI,
				tldr: "GlobalAPI Request failed.",
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
					tldr: "Failed to parse JSON.",
					raw: Some(why.to_string()),
				}),
			},
			Err(why) => Err(Error {
				kind: ErrorKind::GlobalAPI,
				tldr: "GlobalAPI Request failed.",
				raw: Some(why.to_string()),
			}),
		}
	}

	pub async fn is_global(map: MapIdentifier, map_list: &Vec<maps::Response>) -> Result<maps::Response, Error> {
		match map {
			MapIdentifier::Name(name) => {
				for map in map_list {
					if map.name.contains(&name) {
						return Ok(map.to_owned());
					}
				}
			}
			MapIdentifier::Id(id) => {
				for map in map_list {
					if map.id == id {
						return Ok(map.to_owned());
					}
				}
			}
		}

		Err(Error {
			kind: ErrorKind::InvalidInput,
			tldr: "This map is not global.",
			raw: None,
		})
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
						tldr: "seems like gc deleted all the maps lololol",
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
					tldr: "Please do not use an ID for this function.",
					raw: None,
				})
			}
			MapIdentifier::Name(name) => {
				let mut temp = maps::Params::default();
				temp.name = Some(name);
				temp
			}
		};

		match api_request::<Vec<maps::Response>, maps::Params>(String::from("maps?"), params, client).await {
			Ok(mut data) => {
				if data.len() < 1 {
					Err(Error {
						kind: ErrorKind::GlobalAPI,
						tldr: "This map does not exist.",
						raw: None,
					})
				} else {
					Ok(data.remove(0))
				}
			}
			Err(why) => Err(why),
		}
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
			PlayerIdentifier::Name(name) => params.name = Some(name),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.val),
		}

		match api_request::<Vec<players::Response>, players::Params>(String::from("players?"), params, client).await {
			Ok(mut players) => {
				if players.len() < 1 {
					Err(Error {
						kind: ErrorKind::InvalidInput,
						tldr: "No players found.",
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
		params.modes_list_string = Some(mode.as_str());

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name),
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
						tldr: "No records found.",
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
		params.limit = Some(1);
		params.modes_list_string = Some(mode.as_str());

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name),
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
						tldr: "No records found.",
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
		params.modes_list_string = Some(mode.as_str());
		params.limit = Some(1);

		match player_identifier {
			PlayerIdentifier::Name(name) => params.player_name = Some(name),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.val),
		}

		match map {
			MapIdentifier::Name(name) => params.map_name = Some(name),
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
						tldr: "No records found.",
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
		params.modes_list_string = Some(mode.as_str());
		params.stage = Some(course);
		params.has_teleports = Some(runtype);
		params.limit = Some(9999);

		match player_identifier {
			PlayerIdentifier::Name(name) => params.player_name = Some(name),
			PlayerIdentifier::SteamId(steam_id) => params.steam_id = Some(steam_id.val),
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
						tldr: "No records found.",
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
				tldr: "No records found.",
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
						tldr: "Failed to parse date.",
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
	) -> Result<Vec<record_filters::distributions::Response>, Error> {
		let mut params = record_filters::distributions::Params::default();
		params.mode_ids = Some(vec![mode.as_id()]);
		params.has_teleports = Some(runtype);
		params.stages = Some(vec![0]);
		params.tickrates = Some(128);
		params.limit = Some(9999);

		api_request::<Vec<record_filters::distributions::Response>, record_filters::distributions::Params>(
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

		if let Ok(res) = get_player(player_identifier, &client).await {
			player.name = Some(res.name);
			player.steam_id = Some(res.steam_id);
			player.steam_id64 = Some(res.steamid64);
			player.is_banned = Some(res.is_banned);
		}

		let mut global_maps = vec![HashMap::new(), HashMap::new()];

		if let Ok(maps) = get_maps(&client).await {
			for map in maps {
				global_maps[0].insert(map.name.clone(), map.difficulty);
				global_maps[1].insert(map.name, map.difficulty);
			}
		}

		let (tp, pro) = (
			get_times(player_identifier, mode, 0, true, &client).await?,
			get_times(player_identifier, mode, 0, false, &client).await?,
		);

		if tp.len() == 0 && pro.len() == 0 {
			return Err(Error {
				kind: ErrorKind::InvalidInput,
				tldr: "This player has no records.",
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
	use crate::{
		functions::*,
		prelude::{MapIdentifier, Mode},
	};

	#[tokio::test]
	async fn check_api_test() {
		let client = reqwest::Client::new();

		match check_api(&client).await {
			Ok(res) => println!("Success: {:#?}", res),
			Err(err) => println!("Fail: {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_maps_test() {
		let client = reqwest::Client::new();

		match get_maps(&client).await {
			Ok(res) => println!("Success: Got {} maps.", res.len()),
			Err(err) => println!("Fail: {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_map_test() {
		let client = reqwest::Client::new();

		let lionharder_name1 = MapIdentifier::Name("kz_lionharder");
		let lionharder_name2 = MapIdentifier::Name("lionHard");
		let lionharder_id = MapIdentifier::Id(992);
		let erratum_name = MapIdentifier::Name("kz_erratum_v2");

		run(lionharder_name1, &client).await;
		run(lionharder_name2, &client).await;
		run(lionharder_id, &client).await;
		run(erratum_name, &client).await;

		async fn run(map: MapIdentifier, client: &reqwest::Client) {
			match get_map(&map, client).await {
				Ok(res) => println!("Success: {:#?}", res),
				Err(err) => println!("Fail: {:#?}", err),
			}
		}
	}

	#[tokio::test]
	async fn is_global_test() {
		let client = reqwest::Client::new();

		let maps = get_maps(&client).await.unwrap();

		match is_global(MapIdentifier::Name("kz_lionharder"), &maps).await {
			Ok(map) => println!("Success: {:#?}", map),
			Err(err) => println!("Fail: {:#?}", err),
		}

		match is_global(MapIdentifier::Id(992), &maps).await {
			Ok(map) => println!("Success: {:#?}", map),
			Err(err) => println!("Fail: {:#?}", err),
		}

		match is_global(MapIdentifier::Name("kz_penisman"), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(42069), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(0), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(1), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_modes_test() {
		let client = reqwest::Client::new();

		match get_modes(&client).await {
			Ok(modes) => println!("Success: {:#?}", modes),
			Err(why) => println!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_mode_test() {
		let client = reqwest::Client::new();

		match get_mode(&Mode::KZTimer, &client).await {
			Ok(mode) => {
				assert_eq!(200, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}

		match get_mode(&Mode::SimpleKZ, &client).await {
			Ok(mode) => {
				assert_eq!(201, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}

		match get_mode(&Mode::Vanilla, &client).await {
			Ok(mode) => {
				assert_eq!(202, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_player_test() {
		let client = reqwest::Client::new();

		// match get_player(crate::prelude::PlayerIdentifier::Name("AlphaKeks"), &client).await {
		// 	Ok(player) => println!("Success: {:#?}", player),
		// 	Err(why) => println!("Fail: {:#?}", why),
		// }

		let result = match get_player(&crate::prelude::PlayerIdentifier::Name("AlphaKeks"), &client).await {
			Ok(player) => player,
			Err(why) => return println!("Fail: {:#?}", why),
		};

		assert_eq!("76561198282622073", result.steamid64);
	}

	#[tokio::test]
	async fn get_wr_test() {
		let _client = reqwest::Client::new();

		todo!("write this");
	}

	#[tokio::test]
	async fn get_times_test() {
		let client = reqwest::Client::new();

		let result = get_times(
			&crate::prelude::PlayerIdentifier::Name("AlphaKeks"),
			&Mode::Vanilla,
			0,
			true,
			&client,
		)
		.await
		.unwrap();

		println!("records: {}", result.len());
	}

	#[tokio::test]
	async fn get_profile_test() {
		let client = reqwest::Client::new();

		let result = get_profile(
			&crate::prelude::PlayerIdentifier::Name("AlphaKeks"),
			&Mode::SimpleKZ,
			&client,
		)
		.await
		.unwrap();

		println!("{:#?}", result);
	}
}
