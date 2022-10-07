pub mod global_api;
pub mod util;

use std::collections::HashMap;

use chrono::NaiveDateTime;
use futures::future::join_all;

use crate::global_api::*;
use crate::util::*;

async fn api_request<T>(path: String, params: Vec<(&str, String)>, client: &reqwest::Client) -> Result<T, GOKZError>
where
	T: serde::de::DeserializeOwned,
{
	let url = format!("https://kztimerglobal.com/api/v2/{path}");
	let url = match reqwest::Url::parse_with_params(&url, params) {
		Ok(url) => url,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Parsing,
				tldr: String::from("Invalid params."),
				raw: Some(why.to_string()),
			})
		}
	};

	let request = match client.get(url).send().await {
		Ok(data) => data,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::GlobalAPI,
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			})
		}
	};

	match request.json::<T>().await {
		Ok(json) => Ok(json),
		Err(why) => Err(GOKZError {
			r#type: GOKZErrorType::Parsing,
			tldr: String::from("Failed to parse to JSON."),
			raw: Some(why.to_string()),
		}),
	}
}

pub async fn check_api(client: &reqwest::Client) -> Result<GlobalAPIStatus, GOKZError> {
	let url = String::from("https://status.global-api.com/api/v2/summary.json");

	let request = match client.get(url).send().await {
		Ok(data) => data,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::GlobalAPI,
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			})
		}
	};

	match request.json::<GlobalAPIStatus>().await {
		Ok(json) => Ok(json),
		Err(why) => Err(GOKZError {
			r#type: GOKZErrorType::Parsing,
			tldr: String::from("Failed to parse to JSON."),
			raw: Some(why.to_string()),
		}),
	}
}

pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<GOKZMap>, GOKZError> {
	let params = vec![("is_validated", true.to_string()), ("limit", 999.to_string())];

	api_request::<Vec<GOKZMap>>(String::from("maps?"), params, &client).await
}

pub async fn get_map(identifier: GOKZMapIdentifier, client: &reqwest::Client) -> Result<GOKZMap, GOKZError> {
	let mut params = vec![("is_validated", true.to_string()), ("limit", 1.to_string())];

	let map = match identifier {
		GOKZMapIdentifier::Name(name) => ("name", name),
		GOKZMapIdentifier::Id(id) => ("id", id.to_string()),
	};

	params.push((map.0, map.1));

	match api_request::<Vec<GOKZMap>>(String::from("maps?"), params, &client).await {
		Ok(mut maps) => {
			if maps.len() > 0 {
				Ok(maps.remove(0))
			} else {
				Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("Map not found."),
					raw: None,
				})
			}
		}
		Err(why) => Err(why),
	}
}

pub async fn validate_map(identifier: GOKZMapIdentifier, map_list: Vec<GOKZMap>) -> Result<GOKZMap, GOKZError> {
	let not_global = GOKZError {
		r#type: GOKZErrorType::Other,
		tldr: String::from("The provided map is not global."),
		raw: None,
	};

	match identifier {
		GOKZMapIdentifier::Name(name) => {
			for map in map_list {
				if map.name.contains(&name.to_lowercase()) {
					return Ok(map);
				}
			}

			Err(not_global)
		}
		GOKZMapIdentifier::Id(id) => {
			for map in map_list {
				if map.id == id {
					return Ok(map);
				}
			}

			Err(not_global)
		}
	}
}

pub async fn get_modes(client: &reqwest::Client) -> Result<Vec<GOKZMode>, GOKZError> {
	api_request::<Vec<GOKZMode>>(String::from("modes?"), vec![], &client).await
}

pub async fn get_mode(identifier: GOKZModeIdentifier, client: &reqwest::Client) -> Result<GOKZMode, GOKZError> {
	let mut path = String::from("modes/");

	match identifier {
		GOKZModeIdentifier::Name(name) => path.push_str(format!("name/{}", name.as_str()).as_str()),
		GOKZModeIdentifier::Id(id) => path.push_str(format!("id/{id}").as_str()),
	}

	api_request(path, vec![], &client).await
}

pub async fn get_player(identifier: GOKZPlayerIdentifier, client: &reqwest::Client) -> Result<GOKZPlayer, GOKZError> {
	let mut params = vec![("limit", 1.to_string()), ("", String::new())];

	match identifier {
		GOKZPlayerIdentifier::Name(name) => {
			params[1].0 = "name";
			params[1].1.push_str(name.as_ref());
		}
		GOKZPlayerIdentifier::SteamID(steam_id) => {
			params[1].0 = "steam_id";
			params[1].1.push_str(steam_id.as_ref());
		}
	}

	match api_request::<Vec<GOKZPlayer>>(String::from("players?"), params, &client).await {
		Ok(mut players) => {
			if players.len() > 0 {
				Ok(players.remove(0))
			} else {
				Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("Player not found."),
					raw: None,
				})
			}
		}
		Err(why) => Err(why),
	}
}

pub async fn get_wr(
	map: GOKZMapIdentifier,
	course: u8,
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<GOKZRecord, GOKZError> {
	let mut params = vec![
		("tickrate", 128.to_string()),
		("stage", course.to_string()),
		("has_teleports", runtype.to_string()),
		("limit", 1.to_string()),
		("", String::new()),
		("", String::new()),
	];

	match map {
		GOKZMapIdentifier::Name(name) => {
			params[4].0 = "map_name";
			params[4].1 = name;
		}
		GOKZMapIdentifier::Id(id) => {
			params[4].0 = "map_id";
			params[4].1 = id.to_string();
		}
	}

	match mode {
		GOKZModeIdentifier::Name(mode_name) => {
			params[5].0 = "modes_list_string";
			params[5].1 = String::from(mode_name.as_str());
		}
		GOKZModeIdentifier::Id(_) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: None,
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params, &client).await {
		Ok(mut records) => {
			if records.len() > 0 {
				Ok(records.remove(0))
			} else {
				Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("No records found."),
					raw: None,
				})
			}
		}
		Err(why) => Err(why),
	}
}

pub async fn get_maptop(
	map: GOKZMapIdentifier,
	course: u8,
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<GOKZRecord>, GOKZError> {
	let mut params = vec![
		("tickrate", 128.to_string()),
		("stage", course.to_string()),
		("has_teleports", runtype.to_string()),
		("limit", 100.to_string()),
		("", String::new()),
		("", String::new()),
	];

	match map {
		GOKZMapIdentifier::Name(name) => {
			params[4].0 = "map_name";
			params[4].1 = name;
		}
		GOKZMapIdentifier::Id(id) => {
			params[4].0 = "map_id";
			params[4].1 = id.to_string();
		}
	}

	match mode {
		GOKZModeIdentifier::Name(mode_name) => {
			params[5].0 = "modes_list_string";
			params[5].1 = String::from(mode_name.as_str());
		}
		GOKZModeIdentifier::Id(_) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: None,
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params, &client).await {
		Ok(records) => {
			if records.len() > 0 {
				return Ok(records);
			} else {
				return Err(GOKZError {
					r#type: GOKZErrorType::Other,
					tldr: String::from("This map has 0 completions."),
					raw: None,
				});
			}
		}
		Err(why) => return Err(why),
	}
}

pub async fn get_pb(
	player: GOKZPlayerIdentifier,
	map: GOKZMapIdentifier,
	course: u8,
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<GOKZRecord, GOKZError> {
	let mut params = vec![
		("tickrate", 128.to_string()),
		("stage", course.to_string()),
		("has_teleports", runtype.to_string()),
		("limit", 1.to_string()),
		("", String::new()),
		("", String::new()),
		("", String::new()),
	];

	match player {
		GOKZPlayerIdentifier::Name(name) => {
			params[4].0 = "player_name";
			params[4].1 = name;
		}
		GOKZPlayerIdentifier::SteamID(steam_id) => {
			params[4].0 = "steam_id";
			params[4].1 = steam_id;
		}
	}

	match map {
		GOKZMapIdentifier::Name(name) => {
			params[5].0 = "map_name";
			params[5].1 = name;
		}
		GOKZMapIdentifier::Id(id) => {
			params[5].0 = "map_id";
			params[5].1 = id.to_string();
		}
	}

	match mode {
		GOKZModeIdentifier::Name(mode_name) => {
			params[6].0 = "modes_list_string";
			params[6].1 = String::from(mode_name.as_str());
		}
		GOKZModeIdentifier::Id(_) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: None,
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params, &client).await {
		Ok(mut records) => {
			if records.len() > 0 {
				Ok(records.remove(0))
			} else {
				Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("No records found."),
					raw: None,
				})
			}
		}
		Err(why) => Err(why),
	}
}

pub async fn get_times(
	player: GOKZPlayerIdentifier,
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<GOKZRecord>, GOKZError> {
	let mut params = vec![
		("tickrate", 128.to_string()),
		("stage", 0.to_string()),
		("limit", 999.to_string()),
		("has_teleports", runtype.to_string()),
		("", String::new()),
		("", String::new()),
	];

	match player {
		GOKZPlayerIdentifier::Name(name) => {
			params[4].0 = "player_name";
			params[4].1 = name;
		}
		GOKZPlayerIdentifier::SteamID(steam_id) => {
			params[4].0 = "steam_id";
			params[4].1 = steam_id;
		}
	}

	match mode {
		GOKZModeIdentifier::Name(name) => {
			params[5].0 = "modes_list_string";
			params[5].1 = String::from(name.as_str());
		}
		GOKZModeIdentifier::Id(_) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: None,
			})
		}
	}

	let mut filtered_times: Vec<GOKZRecord> = vec![];
	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params, &client).await {
		Ok(records) => {
			let mut global_maps: HashMap<u16, GOKZMap> = HashMap::new();
			match get_maps(client).await {
				Ok(maps) => {
					for map in maps {
						global_maps.insert(map.id, map);
					}

					for rec in records {
						if global_maps.contains_key(&rec.map_id) {
							filtered_times.push(rec);
						}
					}
				}
				Err(why) => return Err(why),
			};

			Ok(filtered_times)
		}
		Err(why) => Err(why),
	}
}

pub async fn get_recent(player: GOKZPlayerIdentifier, client: &reqwest::Client) -> Result<GOKZRecord, GOKZError> {
	let mut players: Vec<GOKZPlayerIdentifier> = vec![];
	for _ in 0..5 {
		players.push(player.clone());
	}

	let part1 = vec![
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_timer),
			true,
			client,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_timer),
			false,
			client,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
			true,
			client,
		),
	];

	let part2 = vec![
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
			false,
			client,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla),
			true,
			client,
		),
		get_times(
			player,
			GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla),
			false,
			client,
		),
	];

	let mut results = vec![];
	results.append(&mut join_all(part1).await);
	results.append(&mut join_all(part2).await);

	let mut records = vec![];
	for result in results {
		if let Ok(mut recs) = result {
			records.append(&mut recs);
		}
	}

	if records.len() < 1 {
		return Err(GOKZError {
			r#type: GOKZErrorType::Other,
			tldr: String::from("This player has no recent times."),
			raw: None,
		});
	} else {
		let mut recent = (0, &records[0]);

		for i in 1..records.len() {
			let date = match NaiveDateTime::parse_from_str(&records[i].created_on, "%Y-%m-%dT%H:%M:%S") {
				Ok(date) => date,
				Err(why) => {
					return Err(GOKZError {
						r#type: GOKZErrorType::Parsing,
						tldr: String::from("Failed to parse date string."),
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
}

pub async fn get_place(record: GOKZRecord, client: &reqwest::Client) -> Result<u16, GOKZError> {
	return api_request::<u16>(format!("records/place/{}", record.id), vec![], &client).await;
}

pub async fn get_filter_dist(
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<GOKZRecordFilter>, GOKZError> {
	let mut params = vec![
		("stages", 0.to_string()),
		("tickrates", 128.to_string()),
		("limit", 999.to_string()),
		("has_teleports", runtype.to_string()),
		("mode_ids", String::new()),
	];

	match mode {
		GOKZModeIdentifier::Name(name) => {
			params[4].1 = match name {
				GOKZModeName::kz_timer => String::from("200"),
				GOKZModeName::kz_simple => String::from("201"),
				GOKZModeName::kz_vanilla => String::from("202"),
			}
		}
		GOKZModeIdentifier::Id(id) => params[4].1 = id.to_string(),
	}

	return api_request::<Vec<GOKZRecordFilter>>(String::from("record_filters?"), params, &client).await;
}

pub async fn get_unfinished(
	player: GOKZPlayerIdentifier,
	tier: Option<u8>,
	mode: GOKZModeIdentifier,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<String>, GOKZError> {
	let doable = match get_filter_dist(mode.clone(), runtype, client).await {
		Ok(filters) => filters,
		Err(why) => return Err(why),
	};

	let completed = match get_times(player, mode, runtype, client).await {
		Ok(records) => records,
		Err(why) => return Err(why),
	};

	let mut comp_ids = vec![];
	let mut uncomp_ids = vec![];

	for record in completed {
		comp_ids.push(record.map_id);
	}

	for filter in doable {
		if !comp_ids.contains(&filter.map_id) {
			uncomp_ids.push(filter.map_id);
		}
	}

	let global_maps = match get_maps(client).await {
		Ok(maps) => maps,
		Err(why) => return Err(why),
	};

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

	return Ok(uncompleted);
}

pub async fn get_profile(
	input_player: GOKZPlayerIdentifier,
	mode: GOKZModeIdentifier,
	client: &reqwest::Client,
) -> Result<GOKZPlayerProfile, GOKZError> {
	let mut player = GOKZPlayerProfile {
		name: String::new(),
		steam_id: None,
		steam_id64: String::new(),
		is_banned: false,
		points: (0, 0),
		records: (0, 0),
		completion: [(0, 0); 8],
		completion_percentage: [(0.0, 0.0); 8],
		rank: GOKZRank::New("New".to_string()),
	};

	match input_player.clone() {
		GOKZPlayerIdentifier::Name(name) => {
			if let Ok(res) = get_player(GOKZPlayerIdentifier::Name(name), client).await {
				player.name = res.name;
				player.steam_id = Some(res.steam_id);
				player.steam_id64 = res.steamid64;
				player.is_banned = res.is_banned;
			} else {
				return Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("Failed to get API Player by name."),
					raw: None,
				});
			}
		}
		GOKZPlayerIdentifier::SteamID(steam_id) => {
			if let Ok(res) = get_player(GOKZPlayerIdentifier::SteamID(steam_id), client).await {
				player.name = res.name;
				player.steam_id = Some(res.steam_id);
				player.steam_id64 = res.steamid64;
				player.is_banned = res.is_banned;
			} else {
				return Err(GOKZError {
					r#type: GOKZErrorType::GlobalAPI,
					tldr: String::from("Failed to get API Player by name."),
					raw: None,
				});
			}
		}
	}

	let mut global_maps = vec![HashMap::new(), HashMap::new()];
	match get_maps(client).await {
		Ok(maps) => {
			for map in maps {
				global_maps[0].insert(map.name.clone(), map.difficulty);
				global_maps[1].insert(map.name, map.difficulty);
			}
		}
		Err(why) => return Err(why),
	}

	let tp_times = if let Ok(times) = get_times(
		match player.steam_id.clone() {
			Some(steam_id) => GOKZPlayerIdentifier::SteamID(steam_id),
			None => GOKZPlayerIdentifier::Name(player.name.clone()),
		},
		mode.clone(),
		true,
		client,
	)
	.await
	{
		times
	} else {
		vec![]
	};

	let pro_times = if let Ok(times) = get_times(input_player, mode.clone(), false, client).await {
		times
	} else {
		vec![]
	};

	if tp_times.len() == 0 && pro_times.len() == 0 {
		return Err(GOKZError {
			r#type: GOKZErrorType::GlobalAPI,
			tldr: String::from("This player has no API times."),
			raw: None,
		});
	}

	let x;
	if tp_times.len() > pro_times.len() {
		x = tp_times.len()
	} else {
		x = pro_times.len()
	}

	for i in 0..x {
		if tp_times.len() > i {
			if global_maps[0].contains_key(&tp_times[i].map_name) {
				player.points.0 += tp_times[i].points as u32;
				player.completion[7].0 += 1;

				match global_maps[0].get(&tp_times[i].map_name) {
					Some(h) => match h {
						1 => player.completion[0].0 += 1,
						2 => player.completion[1].0 += 1,
						3 => player.completion[2].0 += 1,
						4 => player.completion[3].0 += 1,
						5 => player.completion[4].0 += 1,
						6 => player.completion[5].0 += 1,
						7 => player.completion[6].0 += 1,
						_ => (),
					},
					_ => (),
				}

				if &tp_times[i].points == &1000 {
					player.records.0 += 1;
				}

				global_maps[0].remove(&tp_times[i].map_name);
			}
		}

		if pro_times.len() > i {
			if global_maps[1].contains_key(&pro_times[i].map_name) {
				player.points.1 += pro_times[i].points as u32;
				player.completion[7].1 += 1;

				match global_maps[1].get(&pro_times[i].map_name) {
					Some(h) => match h {
						1 => player.completion[0].1 += 1,
						2 => player.completion[1].1 += 1,
						3 => player.completion[2].1 += 1,
						4 => player.completion[3].1 += 1,
						5 => player.completion[4].1 += 1,
						6 => player.completion[5].1 += 1,
						7 => player.completion[6].1 += 1,
						_ => (),
					},
					_ => (),
				}

				if &pro_times[i].points == &1000 {
					player.records.1 += 1;
				}

				global_maps[1].remove(&pro_times[i].map_name);
			}
		}
	}

	let total_points = &player.points.0 + &player.points.1;
	let mode = match mode {
		GOKZModeIdentifier::Name(name) => name,
		GOKZModeIdentifier::Id(id) => match id {
			200 => GOKZModeName::kz_timer,
			201 => GOKZModeName::kz_simple,
			202 => GOKZModeName::kz_timer,
			_ => GOKZModeName::kz_timer,
		},
	};
	match mode {
		GOKZModeName::kz_timer => {
			if total_points >= 1_000_000 {
				player.rank = GOKZRank::Legend(String::from("Legend"))
			} else if total_points >= 800_000 {
				player.rank = GOKZRank::Master("Master".to_string());
			} else if total_points >= 600_000 {
				player.rank = GOKZRank::Pro("Pro".to_string());
			} else if total_points >= 400_000 {
				player.rank = GOKZRank::Semipro("Semipro".to_string());
			} else if total_points >= 250_000 {
				player.rank = GOKZRank::ExpertPlus("Expert+".to_string());
			} else if total_points >= 230_000 {
				player.rank = GOKZRank::Expert("Expert".to_string());
			} else if total_points >= 200_000 {
				player.rank = GOKZRank::ExpertMinus("Expert-".to_string());
			} else if total_points >= 150_000 {
				player.rank = GOKZRank::SkilledPlus("Skilled+".to_string());
			} else if total_points >= 120_000 {
				player.rank = GOKZRank::Skilled("Skilled".to_string());
			} else if total_points >= 100_000 {
				player.rank = GOKZRank::SkilledMinus("Skilled-".to_string());
			} else if total_points >= 80_000 {
				player.rank = GOKZRank::RegularPlus("Regular+".to_string());
			} else if total_points >= 70_000 {
				player.rank = GOKZRank::Regular("Regular".to_string());
			} else if total_points >= 60_000 {
				player.rank = GOKZRank::RegularMinus("Regular-".to_string());
			} else if total_points >= 40_000 {
				player.rank = GOKZRank::CasualPlus("Casual+".to_string());
			} else if total_points >= 30_000 {
				player.rank = GOKZRank::Casual("Casual".to_string());
			} else if total_points >= 20_000 {
				player.rank = GOKZRank::CasualMinus("Casual-".to_string());
			} else if total_points >= 10_000 {
				player.rank = GOKZRank::AmateurPlus("Amateur+".to_string());
			} else if total_points >= 5_000 {
				player.rank = GOKZRank::Amateur("Amateur".to_string());
			} else if total_points >= 2_000 {
				player.rank = GOKZRank::AmateurMinus("Amateur-".to_string());
			} else if total_points >= 1_000 {
				player.rank = GOKZRank::BeginnerPlus("Beginner+".to_string());
			} else if total_points >= 500 {
				player.rank = GOKZRank::Beginner("Beginner".to_string());
			} else if total_points > 0 {
				player.rank = GOKZRank::BeginnerMinus("Beginner-".to_string());
			} else {
				player.rank = GOKZRank::New("New".to_string());
			}
		}
		GOKZModeName::kz_simple => {
			if total_points >= 800_000 {
				player.rank = GOKZRank::Legend("Legend".to_string());
			} else if total_points >= 500_000 {
				player.rank = GOKZRank::Master("Master".to_string());
			} else if total_points >= 400_000 {
				player.rank = GOKZRank::Pro("Pro".to_string());
			} else if total_points >= 300_000 {
				player.rank = GOKZRank::Semipro("Semipro".to_string());
			} else if total_points >= 250_000 {
				player.rank = GOKZRank::ExpertPlus("Expert+".to_string());
			} else if total_points >= 230_000 {
				player.rank = GOKZRank::Expert("Expert".to_string());
			} else if total_points >= 200_000 {
				player.rank = GOKZRank::ExpertMinus("Expert-".to_string());
			} else if total_points >= 150_000 {
				player.rank = GOKZRank::SkilledPlus("Skilled+".to_string());
			} else if total_points >= 120_000 {
				player.rank = GOKZRank::Skilled("Skilled".to_string());
			} else if total_points >= 100_000 {
				player.rank = GOKZRank::SkilledMinus("Skilled-".to_string());
			} else if total_points >= 80_000 {
				player.rank = GOKZRank::RegularPlus("Regular+".to_string());
			} else if total_points >= 70_000 {
				player.rank = GOKZRank::Regular("Regular".to_string());
			} else if total_points >= 60_000 {
				player.rank = GOKZRank::RegularMinus("Regular-".to_string());
			} else if total_points >= 40_000 {
				player.rank = GOKZRank::CasualPlus("Casual+".to_string());
			} else if total_points >= 30_000 {
				player.rank = GOKZRank::Casual("Casual".to_string());
			} else if total_points >= 20_000 {
				player.rank = GOKZRank::CasualMinus("Casual-".to_string());
			} else if total_points >= 10_000 {
				player.rank = GOKZRank::AmateurPlus("Amateur+".to_string());
			} else if total_points >= 5_000 {
				player.rank = GOKZRank::Amateur("Amateur".to_string());
			} else if total_points >= 2_000 {
				player.rank = GOKZRank::AmateurMinus("Amateur-".to_string());
			} else if total_points >= 1_000 {
				player.rank = GOKZRank::BeginnerPlus("Beginner+".to_string());
			} else if total_points >= 500 {
				player.rank = GOKZRank::Beginner("Beginner".to_string());
			} else if total_points > 0 {
				player.rank = GOKZRank::BeginnerMinus("Beginner-".to_string());
			} else {
				player.rank = GOKZRank::New("New".to_string());
			}
		}
		GOKZModeName::kz_vanilla => {
			if total_points >= 600_000 {
				player.rank = GOKZRank::Legend("Legend".to_string());
			} else if total_points >= 400_000 {
				player.rank = GOKZRank::Master("Master".to_string());
			} else if total_points >= 300_000 {
				player.rank = GOKZRank::Pro("Pro".to_string());
			} else if total_points >= 250_000 {
				player.rank = GOKZRank::Semipro("Semipro".to_string());
			} else if total_points >= 200_000 {
				player.rank = GOKZRank::ExpertPlus("Expert+".to_string());
			} else if total_points >= 180_000 {
				player.rank = GOKZRank::Expert("Expert".to_string());
			} else if total_points >= 160_000 {
				player.rank = GOKZRank::ExpertMinus("Expert-".to_string());
			} else if total_points >= 140_000 {
				player.rank = GOKZRank::SkilledPlus("Skilled+".to_string());
			} else if total_points >= 120_000 {
				player.rank = GOKZRank::Skilled("Skilled".to_string());
			} else if total_points >= 100_000 {
				player.rank = GOKZRank::SkilledMinus("Skilled-".to_string());
			} else if total_points >= 80_000 {
				player.rank = GOKZRank::RegularPlus("Regular+".to_string());
			} else if total_points >= 70_000 {
				player.rank = GOKZRank::Regular("Regular".to_string());
			} else if total_points >= 60_000 {
				player.rank = GOKZRank::RegularMinus("Regular-".to_string());
			} else if total_points >= 40_000 {
				player.rank = GOKZRank::CasualPlus("Casual+".to_string());
			} else if total_points >= 30_000 {
				player.rank = GOKZRank::Casual("Casual".to_string());
			} else if total_points >= 20_000 {
				player.rank = GOKZRank::CasualMinus("Casual-".to_string());
			} else if total_points >= 10_000 {
				player.rank = GOKZRank::AmateurPlus("Amateur+".to_string());
			} else if total_points >= 5_000 {
				player.rank = GOKZRank::Amateur("Amateur".to_string());
			} else if total_points >= 2_000 {
				player.rank = GOKZRank::AmateurMinus("Amateur-".to_string());
			} else if total_points >= 1_000 {
				player.rank = GOKZRank::BeginnerPlus("Beginner+".to_string());
			} else if total_points >= 500 {
				player.rank = GOKZRank::Beginner("Beginner".to_string());
			} else if total_points > 0 {
				player.rank = GOKZRank::BeginnerMinus("Beginner-".to_string());
			} else {
				player.rank = GOKZRank::New("New".to_string());
			}
		}
	}

	let doable_request = match reqwest::Client::new()
		.get(format!("https://kzgo.eu/api/completions/{}", mode.as_str()))
		.send()
		.await
	{
		Ok(data) => match data.json::<KZGOCompletionStats>().await {
			Ok(stats) => stats,
			Err(why) => {
				return Err(GOKZError {
					r#type: GOKZErrorType::Parsing,
					tldr: String::from("Failed to parse KZ:GO JSON."),
					raw: Some(why.to_string()),
				})
			}
		},
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::KZGO,
				tldr: String::from("KZ:GO API Request failed."),
				raw: Some(why.to_string()),
			})
		}
	};

	let doable = [
		[
			doable_request.tp.one,
			doable_request.tp.two,
			doable_request.tp.three,
			doable_request.tp.four,
			doable_request.tp.five,
			doable_request.tp.six,
			doable_request.tp.seven,
			doable_request.tp.total,
		],
		[
			doable_request.pro.one,
			doable_request.pro.two,
			doable_request.pro.three,
			doable_request.pro.four,
			doable_request.pro.five,
			doable_request.pro.six,
			doable_request.pro.seven,
			doable_request.pro.total,
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
