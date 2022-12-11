#![allow(dead_code)]
use {crate::prelude::*, futures::future::join_all, reqwest::StatusCode};

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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct EmptyParams;
impl IsParams for EmptyParams {}

/// Makes an HTTPS GET request using a [`reqwest::Client`] and parses the response into a struct.
async fn api_request<'a, T, P>(
	route: &'a str,
	params: P,
	client: &reqwest::Client,
) -> Result<T, Error>
where
	T: std::fmt::Debug + serde::de::DeserializeOwned + IsResponse,
	P: std::fmt::Debug + serde::Serialize + IsParams,
{
	log::trace!("Making a request to the GlobalAPI.");
	log::trace!("{{ route: {}, params: {:?} }}", route, &params);

	match client.get(get_url() + route).query(&params).send().await {
		Ok(response) => {
			log::trace!("Successful GlobalAPI request");
			log::trace!("Response: {:?}", &response);
			match response.status() {
				StatusCode::OK => match response.json::<T>().await {
					Ok(parsed_response) => {
						log::trace!("Successfully parsed GlobalAPI response.");
						log::trace!("Parsed Response: {:?}", &parsed_response);
						return Ok(parsed_response);
					},
					Err(why) => {
						log::warn!("Failed parsing GlobalAPI response.");
						return Err(Error {
							kind: ErrorKind::Parsing,
							origin: String::from("gokz_rs::global_api::api_request"),
							tldr: String::from("Failed to parse JSON."),
							raw: Some(why.to_string()),
						});
					},
				},
				StatusCode::INTERNAL_SERVER_ERROR => {
					log::warn!("GlobalAPI Internal Server Error");
					return Err(Error {
						kind: ErrorKind::GlobalAPI,
						origin: String::from("gokz_rs::global_api::api_request"),
						tldr: String::from("GlobalAPI returned an internal server error. You can check it's health via `/apistatus`."),
						raw: None
					});
				},
				StatusCode::TOO_MANY_REQUESTS => {
					log::warn!("We're getting rate limited");
					return Err(Error {
						kind: ErrorKind::GlobalAPI,
						origin: String::from("gokz_rs::global_api::api_request"),
						tldr: String::from("Currently too many requests are being made. Please wait a bit before using the next command."),
						raw: None
					});
				},
				code => {
					log::warn!("Got a response from the GlobalAPI, but not an `OK` Code.");
					log::warn!("Code: {}", &code);
					return Err(Error {
						kind: ErrorKind::GlobalAPI,
						origin: String::from("gokz_rs::global_api::api_request"),
						tldr: String::from("GlobalAPI request failed."),
						raw: Some(code.to_string()),
					});
				},
			}
		},
		Err(why) => {
			log::warn!("Failed GlobalAPI request");
			log::warn!("Error: {}", why);
			return Err(Error {
				kind: ErrorKind::GlobalAPI,
				origin: String::from("gokz_rs::global_api::api_request"),
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			});
		},
	}
}

/// Will make an API request for all ban records of a given player, identified by their [`SteamID`].
pub async fn get_bans(
	steam_id: SteamID,
	client: &reqwest::Client,
) -> Result<Vec<bans::Ban>, Error> {
	let params = bans::BanParams { steam_id: Some(steam_id.to_string()), ..Default::default() };

	log::info!("[START] get_bans() => Params {:?}", &params);

	let result = match api_request::<Vec<bans::Ban>, _>(&bans::get_url(), params, client).await {
		Ok(response) => {
			if response.len() > 0 {
				Ok(response)
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_bans() => Result {:?}", &result);

	return result;
}

/// Will make an API request for all _validated_ global maps. Since the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) contains more maps than
/// actually valid / "global" maps, this function will ensure to only request maps marked as
/// `validated`.
pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::KZMap>, Error> {
	let params = maps::MapParams { is_validated: Some(true), ..Default::default() };

	log::info!("[START] get_maps() => Params {:?}", &params);

	let result = match api_request::<Vec<maps::KZMap>, _>(&maps::get_url(), params, client).await {
		Ok(maps) => {
			if maps.len() > 0 {
				Ok(maps)
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_maps() => Result {:?}", &result);

	return result;
}

/// Will make an API request for all global maps.
pub async fn get_all_maps(client: &reqwest::Client) -> Result<Vec<maps::KZMap>, Error> {
	log::info!("[START] get_maps()");

	let result = match api_request::<Vec<maps::KZMap>, _>(
		&maps::get_url(),
		maps::MapParams::default(),
		client,
	)
	.await
	{
		Ok(maps) => {
			if maps.len() > 0 {
				Ok(maps)
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_maps() => Result {:?}", &result);

	return result;
}

/// Will request info about a specified map from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_map(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<maps::KZMap, Error> {
	let mut params =
		maps::MapParams { is_validated: Some(true), limit: Some(1), ..Default::default() };

	match map_identifier {
		MapIdentifier::ID(map_id) => params.id = Some(*map_id),
		MapIdentifier::Name(map_name) => params.name = Some(map_name.to_owned()),
	}

	log::info!("[START] get_map() => Params {:?}", &params);

	let result = match api_request::<Vec<maps::KZMap>, _>(&maps::get_url(), params, client).await {
		Ok(mut maps) => {
			if maps.len() > 0 {
				Ok(maps.remove(0))
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_map() => Result {:?}", &result);

	return result;
}

/// Will request all 3 [Modes](`crate::prelude::Mode`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_modes(client: &reqwest::Client) -> Result<Vec<modes::APIMode>, Error> {
	log::info!("[START] get_modes()");

	let result = match api_request::<Vec<modes::APIMode>, _>(&modes::get_url(), EmptyParams, client)
		.await
	{
		Ok(modes) => {
			if modes.len() > 0 {
				Ok(modes)
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_modes() => Result {:?}", &result);

	return result;
}

/// Will request a single mode from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
///
/// Note: You could either use a name or an id for this, it technically does not matter. I chose to
/// use an id.
pub async fn get_mode(mode: &Mode, client: &reqwest::Client) -> Result<modes::APIMode, Error> {
	log::info!("[START] get_mode() => Params {:?}", mode);

	let result = match api_request::<modes::APIMode, _>(
		&modes::id::get_url(mode),
		EmptyParams,
		client,
	)
	.await
	{
		Ok(mode) => Ok(mode),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_mode", ..why })
		},
	};

	log::info!("[END] get_mode() => Result {:?}", &result);

	return result;
}

/// Will request info about a player from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
pub async fn get_player(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<players::APIPlayer, Error> {
	let mut params = players::PlayerParams::default();

	match player {
		PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
		PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
		PlayerIdentifier::SteamID64(steam_id64) => params.steamid64_list = Some(*steam_id64),
	}

	log::info!("[START] get_player() => Params {:?}", &params);

	let result = match api_request::<Vec<players::APIPlayer>, _>(
		&players::get_url(),
		params,
		client,
	)
	.await
	{
		Ok(mut players) => {
			if players.len() > 0 {
				Ok(players.remove(0))
			} else {
				log::warn!("Received an empty response from the GlobalAPI.");
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
	};

	log::info!("[END] get_player() => Result {:?}", &result);

	return result;
}

/// Will request all record filters for a map.
pub async fn get_filters(
	map_id: i16,
	client: &reqwest::Client,
) -> Result<Vec<record_filters::RecordFilter>, Error> {
	let params = record_filters::RecordFilterParams { map_ids: Some(map_id), ..Default::default() };

	log::info!("[START] get_filters() => Params {:?}", &params);

	let result = match api_request::<Vec<record_filters::RecordFilter>, _>(
		&record_filters::get_url(),
		params,
		client,
	)
	.await
	{
		Ok(filters) => Ok(filters),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_filters", ..why })
		},
	};

	log::info!("[END] get_filters() => Result {:?}", &result);

	return result;
}

/// Will request all filters for a given [`Mode`] and runtype (TP / PRO). This will result in the
/// distribution of record filters per [`Mode`].
pub async fn get_filter_dist(
	mode: &Mode,
	runtype: bool,
	client: &reqwest::Client,
) -> Result<Vec<record_filters::RecordFilter>, Error> {
	let params = record_filters::RecordFilterParams {
		mode_ids: Some((*mode).into()),
		has_teleports: Some(runtype),
		stages: Some(0),
		limit: Some(9999),
		..Default::default()
	};

	log::info!("[START] get_filter_dist() => Params {:?}", &params);

	let result = match api_request::<
		Vec<record_filters::RecordFilter>,
		record_filters::RecordFilterParams,
	>(&record_filters::get_url(), params, client)
	.await
	{
		Ok(filters) => Ok(filters),
		Err(why) => {
			return Err(Error {
				origin: why.origin + " > gokz_rs::global_api::get_filter_dist",
				..why
			})
		},
	};

	log::info!("[END] get_filter_dist() => Result {:?}", &result);

	return result;
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
	log::info!(
		"[START] get_unfinished() => Function Input {{ player_identifier: {}, mode: {}, runtype: {}, tier: {:?} }}",
		player_identifier,
		mode,
		runtype,
		&tier
	);

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

	let completed = completed.into_iter().map(|rec| rec.map_id).collect::<Vec<i16>>();
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

	log::info!("[END] get_unfinished() => Result {:?}", &uncompleted);

	return Ok(uncompleted);
}

/// Will request the #1 record on a given map.
pub async fn get_wr(
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<records::top::Record, Error> {
	let mut params = records::top::RecordParams {
		modes_list_string: Some(mode.as_str().to_owned()),
		has_teleports: Some(runtype),
		stage: Some(course),
		..Default::default()
	};

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
	}

	log::info!("[START] get_wr() => Params {:?}", &params);

	let result =
		match api_request::<Vec<records::top::Record>, _>(&records::top::get_url(), params, client)
			.await
		{
			Ok(mut records) => {
				if records.len() > 0 {
					Ok(records.remove(0))
				} else {
					log::warn!("Received an empty response from the GlobalAPI.");
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
		};

	log::info!("[END] get_wr() => Result {:?}", &result);

	return result;
}

/// Will request a player's personal best on a given map.
pub async fn get_pb(
	player: &PlayerIdentifier,
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<records::top::Record, Error> {
	let mut params = records::top::RecordParams {
		modes_list_string: Some(mode.as_str().to_owned()),
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

	log::info!("[START] get_pb() => Params {:?}", &params);

	let result =
		match api_request::<Vec<records::top::Record>, _>(&records::top::get_url(), params, client)
			.await
		{
			Ok(mut records) => {
				if records.len() > 0 {
					Ok(records.remove(0))
				} else {
					log::warn!("Received an empty response from the GlobalAPI.");
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
		};

	log::info!("[END] get_pb() => Result {:?}", &result);

	return result;
}

/// Will request the top 100 records on a given map.
pub async fn get_maptop(
	map_identifier: &MapIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<Vec<records::top::Record>, Error> {
	let mut params = records::top::RecordParams {
		modes_list_string: Some(mode.as_str().to_owned()),
		has_teleports: Some(runtype),
		stage: Some(course),
		limit: Some(100),
		..Default::default()
	};

	match map_identifier {
		MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
	}

	log::info!("[START] get_maptop() => Params {:?}", &params);

	let result =
		match api_request::<Vec<records::top::Record>, _>(&records::top::get_url(), params, client)
			.await
		{
			Ok(records) => {
				if records.len() > 0 {
					Ok(records)
				} else {
					log::warn!("Received an empty response from the GlobalAPI.");
					return Err(Error {
						kind: ErrorKind::NoData,
						origin: String::from("gokz_rs::global_api::get_wr"),
						tldr: String::from("No PB found."),
						raw: None,
					});
				}
			},
			Err(why) => {
				return Err(Error {
					origin: why.origin + " > gokz_rs::global_api::get_maptop",
					..why
				})
			},
		};

	log::info!("[END] get_maptop() => Result {:?}", &result);

	return result;
}

/// Will request all records of a player.
///
/// Note: the function needs to be this specific because the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) will return inconsistent
/// results if not enough arguments are provided.
///
/// Note: this function returns **ALL** records from the API, even records on non-global maps.
pub async fn get_records(
	player: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	course: u8,
	client: &reqwest::Client,
) -> Result<Vec<records::top::Record>, Error> {
	let mut params = records::top::RecordParams {
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

	log::info!("[START] get_records() => Params {:?}", &params);

	let result =
		match api_request::<Vec<records::top::Record>, _>(&records::top::get_url(), params, client)
			.await
		{
			Ok(records) => {
				if records.len() > 0 {
					Ok(records)
				} else {
					log::warn!("Received an empty response from the GlobalAPI.");
					return Err(Error {
						kind: ErrorKind::NoData,
						origin: String::from("gokz_rs::global_api::get_times"),
						tldr: String::from("This player has 0 records."),
						raw: None,
					});
				}
			},
			Err(why) => {
				return Err(Error {
					origin: why.origin + " > gokz_rs::global_api::get_times",
					..why
				})
			},
		};

	log::info!("[END] get_records() => Result {:?}", &result);

	return result;
}

/// Will request all of a player's records and filter them to find the most recently set one.
pub async fn get_recent(
	player: &PlayerIdentifier,
	client: &reqwest::Client,
) -> Result<records::top::Record, Error> {
	log::info!("[START] get_recent() => Function Input {{ player: {:?} }}", player);

	let mut potential_err = None;

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
		.filter_map(|result| {
			// filter out errors
			if let Err(why) = &result {
				potential_err = Some(why.clone());
			}
			result.ok()
		})
		.flatten() // flatten into single Vec
		.collect::<Vec<_>>(); // Vec<Response>

	if records.len() < 1 {
		log::warn!("Received an empty response from the GlobalAPI.");
		if let Some(err) = potential_err {
			return Err(err);
		} else {
			return Err(Error {
				kind: ErrorKind::NoData,
				origin: String::from("gokz_rs::global_api::get_recent"),
				tldr: String::from("No recent PB found."),
				raw: None,
			});
		};
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
				log::warn!("Failed converting date: {}", &records[i].created_on);
				log::warn!("Error: {:?}", why);
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::get_recent"),
					tldr: String::from("Failed to convert date."),
					raw: Some(why.to_string()),
				});
			},
		};

		// replace current `recent` if newer record was found
		if date.timestamp() > recent.0 {
			recent = (date.timestamp(), i);
		}
	}

	log::info!("[END] get_recent() => Result {:?}", &records[recent.1]);

	// return most recent pb using index
	Ok(records.remove(recent.1))
}

/// Will request the #placement of a given record.
pub async fn get_place(
	record_id: &u32,
	client: &reqwest::Client,
) -> Result<records::place::Place, Error> {
	log::info!("[START] get_place() => Function Input {{ record_id: {} }}", record_id);

	let result = match api_request::<records::place::Place, _>(
		&records::place::get_url(record_id),
		EmptyParams,
		client,
	)
	.await
	{
		Ok(place) => Ok(place),
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::global_api::get_place", ..why })
		},
	};

	log::info!("[END] get_place() => Result {:?}", &result);

	return result;
}

// --------------------------------------------------------------------------------------------- //

/// This function will check the most recent 10 health checks and return a
/// [fancy](health::Fancy) response.
pub async fn health_check(client: &reqwest::Client) -> Result<health::FancyHealthReport, Error> {
	log::info!("[START] health_check()");

	match client.get(health::get_url()).send().await {
		Ok(response) => match response.status() {
			StatusCode::OK => {
				log::trace!("Successful GlobalAPI health check");
				log::trace!("Response: {:?}", &response);
				match response.json::<health::HealthResponse>().await {
					Ok(parsed_response) => {
						log::info!("Successfully parsed GlobalAPI response.");
						log::info!("Parsed Response: {:?}", &parsed_response);

						let mut result = health::FancyHealthReport {
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

						log::info!("[END] health_check() => Result {:?}", &result);

						return Ok(result);
					},
					Err(why) => {
						log::warn!("Failed parsing GlobalAPI Health Check.");
						return Err(Error {
							kind: ErrorKind::Parsing,
							origin: String::from("gokz_rs::global_api::health_check"),
							tldr: String::from("Failed to parse JSON."),
							raw: Some(why.to_string()),
						});
					},
				}
			},
			code => {
				log::warn!("Got a response from the GlobalAPI, but not an `OK` Code.");
				log::warn!("Code: {}", &code);
				return Err(Error {
					kind: ErrorKind::GlobalAPI,
					origin: String::from("gokz_rs::global_api::health_check"),
					tldr: String::from("GlobalAPI request failed."),
					raw: Some(code.to_string()),
				});
			},
		},
		Err(why) => {
			log::warn!("Failed GlobalAPI request");
			log::warn!("Error: {}", why);
			return Err(Error {
				kind: ErrorKind::GlobalAPI,
				origin: String::from("gokz_rs::global_api::health_check"),
				tldr: String::from("GlobalAPI request failed."),
				raw: Some(why.to_string()),
			});
		},
	}
}

/// Will iterate over a list of [maps](maps::Response) and check if any of them match a given
/// [`MapIdentifier`].
///
/// Note: Technically you can pass in any list of [maps](maps::Response) but it is intended to be
/// used with [`get_maps`].
pub async fn is_global(
	map_identifier: &MapIdentifier,
	map_list: &Vec<maps::KZMap>,
) -> Result<maps::KZMap, Error> {
	log::info!(
		"[START] is_global() => Function Input {{ map_identifier: {:?}, map_list_len: {} }}",
		map_identifier,
		map_list.len()
	);

	match map_identifier {
		MapIdentifier::Name(map_name) => {
			for map in map_list {
				if map.name.contains(&map_name.to_lowercase()) {
					log::info!("[END] is_global() => Result {:?}", map);
					return Ok(map.to_owned());
				}
			}
		},
		MapIdentifier::ID(map_id) => {
			for map in map_list {
				if &map.id == map_id {
					log::info!("[END] is_global() => Result {:?}", map);
					return Ok(map.to_owned());
				}
			}
		},
	}

	log::info!("[END] is_global() => Provided map (`{}`) is not global.", map_identifier);
	return Err(Error {
		kind: ErrorKind::Input,
		origin: String::from("gokz_rs::global_api::is_global"),
		tldr: format!("`{}` is not a global map.", map_identifier),
		raw: Some(map_identifier.to_string()),
	});
}

/// Returns download link to the replay of a given replay_id or an [`Error`]
pub async fn get_replay(replay_id: u32) -> Result<String, Error> {
	log::info!("[START] get_replay() => Function Input {{ replay_id: {} }}", replay_id);

	let result = match replay_id {
		0 => {
			return Err(Error {
				kind: ErrorKind::NoData,
				origin: String::from("gokz_rs::global_api::get_replay"),
				tldr: String::from("`replay_id` is 0."),
				raw: Some(replay_id.to_string()),
			})
		},
		replay_id => {
			// https://kztimerglobal.com/api/v2/records/replay/{replay_id}
			Ok(crate::global_api::get_url() + &records::replay::replay_id::get_url(replay_id))
		},
	};

	log::info!("[END] get_replay() => Result {:?}", replay_id);

	return result;
}

/// Will fetch a record by its ID.
pub async fn get_record(
	record_id: &u32,
	client: &reqwest::Client,
) -> Result<records::top::Record, Error> {
	let params = records::top::RecordParams { tickrate: None, limit: None, ..Default::default() };

	log::info!("[START] get_record() => Params {:?}", &params);

	let result =
		api_request::<records::top::Record, _>(&format!("records/{record_id}"), params, client)
			.await;

	log::info!("[END] get_record() => Result {:?}", &result);

	return result;
}
/// Will return a Vec<String> of all global map names
pub async fn get_mapcycle(
	tier: Option<u8>,
	client: &reqwest::Client,
) -> Result<Vec<String>, Error> {
	let url = format!(
		"https://maps.cawkz.net/mapcycles/{}",
		match tier {
			Some(tier) => format!("tier{}.txt", tier),
			None => String::from("gokz.txt"),
		}
	);

	log::info!("[START] get_mapcycle() => Params {{ url: {} }}", &url);

	let result = match client.get(url).send().await {
		Ok(response) => match response.text().await {
			Ok(text) => Ok(text.split_terminator("\r\n").map(|s| s.to_owned()).collect()),
			Err(why) => {
				log::warn!("Failed to parse plain text.");
				log::warn!("Error: {:?}", why);
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::global_api::get_mapcycle"),
					tldr: String::from("Failed to parse text."),
					raw: Some(why.to_string()),
				});
			},
		},
		Err(why) => {
			log::warn!("Failed to GET mapcycle.");
			return Err(Error {
				kind: ErrorKind::GlobalAPI,
				origin: String::from("gokz_rs::global_api::get_mapcycle"),
				tldr: String::from("GET Request failed."),
				raw: Some(why.to_string()),
			});
		},
	};

	log::info!("[END] get_mapcycle() => Result {:?}", &result);

	return result;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_bans_test() {
		let client = reqwest::Client::new();

		let no_bans = SteamID::new("STEAM_1:0:165881949").unwrap();

		match get_bans(no_bans, &client).await {
			Err(why) => println!("Test successful: {:#?}", why),
			Ok(bans) => panic!("Test failed: {:#?}", bans),
		}

		let bans = SteamID::new("STEAM_1:1:161178172").unwrap();

		match get_bans(bans, &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(bans) => println!("Test successful: {:#?}", bans),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_maps_test() {
		let client = reqwest::Client::new();

		match get_maps(&client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(maps) => println!("Test successful: {} maps", maps.len()),
		}
	}

	#[tokio::test]
	// #[ignore = "expensive"]
	async fn get_all_maps_test() {
		let client = reqwest::Client::new();

		match get_all_maps(&client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(maps) => println!("Test successful: {} maps", maps.len()),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_modes_test() {
		let client = reqwest::Client::new();

		match get_modes(&client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(modes) => println!("Test successful: {:#?}\n({} modes)", modes, modes.len()),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_player_test() {
		let client = reqwest::Client::new();

		let alphakeks = PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172").unwrap());
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_filters_test() {
		let client = reqwest::Client::new();

		match get_filters(992, &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(filters) => println!("Test successfuly: {:#?}", filters),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
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
				"Test successful: {} maps left (alphakeks, skz, tp, t7)\n{:?}",
				maps.len(),
				maps
			),
		}

		match get_unfinished(
			&PlayerIdentifier::SteamID(SteamID::new("STEAM_1:0:135486492").unwrap()),
			&Mode::KZTimer,
			false,
			None,
			&client,
		)
		.await
		{
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(maps) => {
				println!("Test successful: {} maps left (jucci, kzt, pro)\n{:?}", maps.len(), maps)
			},
		}

		match get_unfinished(
			&PlayerIdentifier::SteamID(SteamID::new("STEAM_1:0:46898346").unwrap()),
			&Mode::SimpleKZ,
			true,
			Some(7),
			&client,
		)
		.await
		{
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(maps) => {
				println!(
					"Test successful: {} maps left (charlie, skz, tp, t7)\n{:?}",
					maps.len(),
					maps
				)
			},
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_pb_test() {
		let client = reqwest::Client::new();

		match get_pb(
			&PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172").unwrap()),
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

	#[tokio::test]
	#[ignore = "expensive"]
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
			Ok(maptop) => {
				println!("Test successful: {} records (lionharder, skz, pro)", maptop.len())
			},
		}

		match get_maptop(&MapIdentifier::ID(992), &Mode::KZTimer, true, 0, &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(maptop) => {
				println!("Test successful: {} records (lionharder, kzt, tp)", maptop.len())
			},
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_records_test() {
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
			Ok(records) => {
				println!("Test successful: {} records (AlphaKeks, skz, tp)", records.len())
			},
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_recent_test() {
		let client = reqwest::Client::new();

		let players = [
			PlayerIdentifier::Name(String::from("AlphaKeks")),
			PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172").unwrap()),
			PlayerIdentifier::Name(String::from("racist75")),
			PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:152337044").unwrap()),
			PlayerIdentifier::Name(String::from("𝘨𝘰𝘴ℎℎℎℎℎℎℎ")),
			PlayerIdentifier::SteamID(SteamID::new("STEAM_1:0:165881949").unwrap()),
			PlayerIdentifier::Name(String::from("charlieeilrahc")),
			PlayerIdentifier::SteamID(SteamID::new("STEAM_1:0:46898346").unwrap()),
			PlayerIdentifier::Name(String::from("Fob")),
			PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:96787045").unwrap()),
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

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn health_test() {
		let client = reqwest::Client::new();

		match health_check(&client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(data) => println!("Test successful: {:#?}", data),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
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

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_record_test() {
		let client = reqwest::Client::new();

		match get_record(&328472, &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(record) => println!("Test successful: {:#?}", record),
		}
	}

	#[tokio::test]
	#[ignore = "expensive"]
	async fn get_mapcycle_test() {
		let client = reqwest::Client::new();

		match get_mapcycle(Some(7), &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(map_names) => println!("Test successful (T7): {:#?}", map_names),
		}

		match get_mapcycle(None, &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(map_names) => println!("Test successful (all): {:#?}", map_names),
		}

		match get_mapcycle(Some(3), &client).await {
			Err(why) => panic!("Test failed: {:#?}", why),
			Ok(map_names) => println!("Test successful (T3): {:#?}", map_names),
		}
	}
}
