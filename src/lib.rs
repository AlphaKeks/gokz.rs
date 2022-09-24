#![allow(unused_imports, unused_variables, dead_code)]

pub mod global_api;
pub mod kzgo;
pub mod util;

use std::collections::HashMap;

use chrono::NaiveDateTime;
use futures::future::join_all;

use crate::global_api::*;
use crate::util::*;

async fn api_request<T>(path: String, params: Vec<(&str, String)>) -> Result<T, GOKZError>
where
	T: serde::de::DeserializeOwned,
{
	let url = format!("https://kztimerglobal.com/api/v2/{path}");
	let url = match reqwest::Url::parse_with_params(&url, params) {
		Ok(url) => url,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Conversion,
				tldr: String::from("Invalid params."),
				raw: why.to_string(),
			})
		}
	};

	let client = reqwest::Client::new();
	let request = match client.get(url).send().await {
		Ok(data) => data,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::GlobalAPI,
				tldr: String::from("GlobalAPI request failed."),
				raw: why.to_string(),
			})
		}
	};

	match request.json::<T>().await {
		Ok(json) => Ok(json),
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Conversion,
				tldr: String::from("Failed to parse to JSON."),
				raw: why.to_string(),
			})
		}
	}
}

pub async fn get_maps() -> Result<Vec<GOKZMap>, GOKZError> {
	let params = vec![
		("is_validated", true.to_string()),
		("limit", 999.to_string()),
	];

	return api_request::<Vec<GOKZMap>>(String::from("maps?"), params).await;
}

pub async fn get_map(identifier: GOKZMapIdentifier) -> Result<GOKZMap, GOKZError> {
	let mut params = vec![("is_validated", true.to_string()), ("limit", 1.to_string())];

	let map = match identifier {
		GOKZMapIdentifier::Name(name) => ("name", name),
		GOKZMapIdentifier::Id(id) => ("id", id.to_string()),
	};

	params.push((map.0, map.1));

	match api_request::<Vec<GOKZMap>>(String::from("maps?"), params).await {
		Ok(mut maps) => Ok(maps.remove(0)),
		Err(why) => Err(why),
	}
}

pub async fn validate_map(
	identifier: GOKZMapIdentifier,
	map_list: Vec<GOKZMap>,
) -> Result<GOKZMap, GOKZError> {
	let not_global = GOKZError {
		r#type: GOKZErrorType::Other,
		tldr: String::from("The provided map is not global."),
		raw: String::new(),
	};

	match identifier {
		GOKZMapIdentifier::Name(name) => {
			for map in map_list {
				if map.name == name {
					return Ok(map);
				}
			}

			return Err(not_global);
		}
		GOKZMapIdentifier::Id(id) => {
			for map in map_list {
				if map.id == id {
					return Ok(map);
				}
			}

			return Err(not_global);
		}
	}
}

pub async fn get_modes() -> Result<Vec<GOKZMode>, GOKZError> {
	return api_request::<Vec<GOKZMode>>(String::from("modes?"), vec![]).await;
}

pub async fn get_mode(identifier: GOKZModeIdentifier) -> Result<GOKZMode, GOKZError> {
	let mut path = String::from("modes/");

	match identifier {
		GOKZModeIdentifier::Name(name) => path.push_str(format!("name/{}", name.as_str()).as_str()),
		GOKZModeIdentifier::Id(id) => path.push_str(format!("id/{id}").as_str()),
	}

	return api_request(path, vec![]).await;
}

pub async fn get_player(identifier: GOKZPlayerIdentifier) -> Result<GOKZPlayer, GOKZError> {
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

	match api_request::<Vec<GOKZPlayer>>(String::from("players?"), params).await {
		Ok(mut players) => return Ok(players.remove(0)),
		Err(why) => return Err(why),
	}
}

pub async fn get_wr(
	map: GOKZMapIdentifier,
	course: u8,
	mode: GOKZModeIdentifier,
	runtype: bool,
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
		GOKZModeIdentifier::Id(mode_id) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: String::new(),
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params).await {
		Ok(mut records) => return Ok(records.remove(0)),
		Err(why) => return Err(why),
	}
}
pub async fn get_maptop(
	map: GOKZMapIdentifier,
	course: u8,
	mode: GOKZModeIdentifier,
	runtype: bool,
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
		GOKZModeIdentifier::Id(mode_id) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: String::new(),
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params).await {
		Ok(records) => {
			if records.len() < 1 {
				return Err(GOKZError {
					r#type: GOKZErrorType::Other,
					tldr: String::from("This map has 0 completions."),
					raw: String::new(),
				});
			} else {
				return Ok(records);
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
		GOKZModeIdentifier::Id(mode_id) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: String::new(),
			})
		}
	}

	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params).await {
		Ok(mut records) => return Ok(records.remove(0)),
		Err(why) => return Err(why),
	}
}

pub async fn get_times(
	player: GOKZPlayerIdentifier,
	mode: GOKZModeIdentifier,
	runtype: bool,
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
		GOKZModeIdentifier::Id(mode_id) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Other,
				tldr: String::from("This function only takes mode names."),
				raw: String::new(),
			})
		}
	}

	let mut filtered_times: Vec<GOKZRecord> = vec![];
	match api_request::<Vec<GOKZRecord>>(String::from("records/top?"), params).await {
		Ok(records) => {
			let mut global_maps: HashMap<u16, GOKZMap> = HashMap::new();
			match get_maps().await {
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

			return Ok(filtered_times);
		}
		Err(why) => return Err(why),
	};
}

pub async fn get_recent(player: GOKZPlayerIdentifier) -> Result<GOKZRecord, GOKZError> {
	let mut players: Vec<GOKZPlayerIdentifier> = vec![];
	for _ in 0..5 {
		players.push(player.clone());
	}

	let part1 = vec![
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_timer),
			true,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_timer),
			false,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
			true,
		),
	];

	let part2 = vec![
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
			false,
		),
		get_times(
			players.remove(0),
			GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla),
			true,
		),
		get_times(
			player,
			GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla),
			false,
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
			raw: String::new(),
		});
	} else {
		let mut recent = (0, &records[0]);

		for i in 1..records.len() {
			let date =
				match NaiveDateTime::parse_from_str(&records[i].created_on, "%Y-%m-%dT%H:%M:%S") {
					Ok(date) => date,
					Err(why) => {
						return Err(GOKZError {
							r#type: GOKZErrorType::Conversion,
							tldr: String::from("Failed to parse date string."),
							raw: why.to_string(),
						})
					}
				};

			if date.timestamp() > recent.0 {
				recent = (date.timestamp(), &records[i]);
			}
		}

		return Ok(recent.1.to_owned());
	}
}
