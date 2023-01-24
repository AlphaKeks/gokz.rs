use {
	crate::prelude::*,
	futures::future::join_all,
	log::{debug, info, trace, warn},
};

pub mod bans;
pub mod health;
pub mod jumpstats;
pub mod maps;
pub mod modes;
pub mod players;
pub mod record_filters;
pub mod records;
pub mod servers;

/// Marker trait for possible API response bodies. Feel free to implement this for your own types
/// but keep in mind that this trait doesn't actually guarantee anything.
pub trait GlobalAPIResponse: std::fmt::Debug + serde::de::DeserializeOwned {}
macro_rules! api_response {
	($type_name:ident) => {
		impl GlobalAPIResponse for $type_name {}
		impl GlobalAPIResponse for Vec<$type_name> {}
	};
}
pub(crate) use api_response;

/// Marker trait for possible API request parameters. Feel free to implement this for your own
/// types but keep in mind that this trait doesn't actually guarantee anything.
pub trait GlobalAPIParams: std::fmt::Debug + serde::Serialize {}
macro_rules! api_params {
	($type_name:ident) => {
		impl GlobalAPIParams for $type_name {}
		impl GlobalAPIParams for Vec<$type_name> {}
	};
}
pub(crate) use api_params;

/// Methods for the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)
pub struct GlobalAPI;
impl GlobalAPI {
	pub const BASE_URL: &str = "https://kztimerglobal.com/api/v2";

	/// Base request to the GlobalAPI
	///
	/// - `route`: **NOT** the full URL. Only the relevant part. (e.g. `/records/top?`)
	/// - `params`: the parameters for the request
	pub async fn get<Response, Params>(
		route: &str,
		params: Params,
		client: &crate::Client,
	) -> Result<Response, Error>
	where
		Params: GlobalAPIParams,
		Response: GlobalAPIResponse,
	{
		info!("[GlobalAPI::get] starting...");
		debug!("[GlobalAPI::get] `route`: {:?}", route);
		debug!("[GlobalAPI::get] `params`: {:?}", &params);

		// construct full URL
		// e.g. `https://kztimerglobal.com/api/v2/records/top?`
		let full_route = format!("{}{}", Self::BASE_URL, route);
		let url = match reqwest::Url::parse(&full_route) {
			Err(why) => {
				warn!("[GlobalAPI::get] Failed to parse URL: {:?}", why);
				return Err(Error {
					kind: ErrorKind::Parsing {
						expected: String::from("valid URL"),
						got: Some(format!("route: {:?}\nparams: {:?}", route, params)),
					},
					msg: String::from("Failed to parse URL."),
				});
			},
			Ok(url) => {
				debug!("[GlobalAPI::get] Successfully constructed URL `{}`.", &url);
				url
			},
		};

		// make a GET request to the GlobalAPI
		let response = match client
			.get(url)
			.query(&params)
			.send()
			.await
		{
			Err(why) => {
				warn!("[GlobalAPI::get] HTTPS Request failed.");
				if let Some(code) = why.status() {
					warn!("[GlobalAPI::get] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("GlobalAPI request failed with code `{}`.", code),
					});
				}

				warn!("[GlobalAPI::get] Request failed with no status code.");
				return Err(Error {
					kind: ErrorKind::GlobalAPI {
						status_code: None,
						raw_message: Some(why.to_string()),
					},
					msg: String::from(
						"GlobalAPI request failed, but no status code has been returned.",
					),
				});
			},
			Ok(response) => match response.error_for_status() {
				Err(why) => {
					let Some(code) = why.status() else {
						warn!("[GlobalAPI::get] Request failed with no status code.");
						return Err(Error {
							kind: ErrorKind::GlobalAPI {
								status_code: None,
								raw_message: Some(why.to_string()),
							},
							msg: String::from(
								"GlobalAPI request failed, but no status code has been returned.",
							),
						});
					};

					warn!("[GlobalAPI::get] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("GlobalAPI request failed with code `{}`.", code),
					});
				},
				Ok(response) => {
					trace!(
						"[GlobalAPI::get] GlobalAPI responded successfully with code `{}`.",
						response.status()
					);
					response
				},
			},
		};

		// parse the response into the desired `Response` format
		let parsed_response = match response.json::<Response>().await {
			Err(why) => {
				warn!("[GlobalAPI::get] Failed to parse response.");
				warn!("[GlobalAPI::get] {:?}", why);

				return Err(Error {
					kind: ErrorKind::Parsing { expected: String::from("JSON"), got: None },
					msg: String::from("Failed to parse GlobalAPI response."),
				});
			},
			Ok(parsed_response) => {
				trace!("[GlobalAPI::get] Successfully parsed response.");
				parsed_response
			},
		};

		info!("[GlobalAPI::get] completed successfully.");
		debug!("[GlobalAPI::get] Response: {:?}", &parsed_response);

		// return the `Response`
		Ok(parsed_response)
	}

	/// Route: `/bans`
	/// - Lets you fetch ban entries of players
	pub async fn get_bans(
		steam_id: &SteamID,
		limit: u32,
		client: &crate::Client,
	) -> Result<Vec<bans::Ban>, Error> {
		info!("[GlobalAPI::get_bans] starting...");

		let params = bans::Params {
			steam_id: Some(steam_id.to_string()),
			limit: Some(limit),
			..Default::default()
		};

		let response = bans::get(params, client).await?;
		info!("[GlobalAPI::get_bans] completed successfully.");

		Ok(response)
	}

	/// Route: `/maps`
	/// - Lets you fetch all maps stored in the GlobalAPI
	pub async fn get_maps(
		validated_only: bool,
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<maps::Map>, Error> {
		info!("[GlobalAPI::get_maps] starting...");

		let params = maps::Params {
			is_validated: Some(validated_only),
			limit: if let Some(limit) = limit { Some(limit) } else { Some(99999) },
			..Default::default()
		};

		let response = maps::get(params, client).await?;
		info!("[GlobalAPI::get_maps] completed successfully.");

		Ok(response)
	}

	/// Route: `/maps/{map_id}` _OR_ `/maps/name/{map_name}`
	/// - Lets you fetch a map stored in the GlobalAPI
	pub async fn get_map(
		map_identifier: &MapIdentifier,
		client: &crate::Client,
	) -> Result<maps::Map, Error> {
		info!("[GlobalAPI::get_map] starting...");

		let response = match map_identifier {
			MapIdentifier::ID(id) => maps::id::get(*id, client).await?,
			MapIdentifier::Name(name) => maps::name::get(name, client).await?,
		};
		info!("[GlobalAPI::get_map] completed successfully.");

		Ok(response)
	}

	pub async fn get_mapcycle(
		tier: Option<Tier>,
		client: &crate::Client,
	) -> Result<Vec<String>, Error> {
		info!("[GlobalAPI::is_global] completed successfully.");

		let url = format!(
			"https://maps.global-api.com/mapcycles/{}",
			match tier {
				Some(tier) => format!("tier{}.txt", tier as u8),
				None => String::from("gokz.txt"),
			}
		);

		let response = match client.get(url).send().await {
			Err(why) => {
				warn!("[GlobalAPI::get_mapcycle] HTTPS Request failed.");
				if let Some(code) = why.status() {
					warn!("[GlobalAPI::get_mapcycle] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("GlobalAPI request failed with code `{}`.", code),
					});
				}

				warn!("[GlobalAPI::get_mapcycle] Request failed with no status code.");
				return Err(Error {
					kind: ErrorKind::GlobalAPI {
						status_code: None,
						raw_message: Some(why.to_string()),
					},
					msg: String::from(
						"GlobalAPI request failed, but no status code has been returned.",
					),
				});
			},
			Ok(response) => match response.error_for_status() {
				Err(why) => {
					let Some(code) = why.status() else {
						warn!("[GlobalAPI::get_mapcycle] Request failed with no status code.");
						return Err(Error {
							kind: ErrorKind::GlobalAPI {
								status_code: None,
								raw_message: Some(why.to_string()),
							},
							msg: String::from(
								"GlobalAPI request failed, but no status code has been returned.",
							),
						});
					};

					warn!("[GlobalAPI::get_mapcycle] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("GlobalAPI request failed with code `{}`.", code),
					});
				},
				Ok(response) => {
					trace!(
						"[GlobalAPI::get_mapcycle] GlobalAPI responded successfully with code `{}`.",
						response.status()
					);
					response
				},
			},
		};

		let parsed_response = match response.text().await {
			Err(why) => {
				warn!("[GlobalAPI::get_mapcycle] Failed to parse response.");
				warn!("[GlobalAPI::get_mapcycle] {:?}", why);

				return Err(Error {
					kind: ErrorKind::Parsing { expected: String::from("Text"), got: None },
					msg: String::from("Failed to parse GlobalAPI response."),
				});
			},
			Ok(parsed_response) => {
				trace!("[GlobalAPI::get_mapcycle] Successfully parsed response.");
				parsed_response
					.lines()
					.map(String::from)
					.collect::<Vec<String>>()
			},
		};

		info!("[GlobalAPI::get_mapcycle] completed successfully.");
		debug!("[GlobalAPI::get_mapcycle] Response: {:?}", &parsed_response);

		// return the `Response`
		Ok(parsed_response)
	}

	/// Will fetch all global map names and try to find one that matches the given `map_identifier`.
	pub async fn is_global(map_name: &str, client: &crate::Client) -> Option<String> {
		info!("[GlobalAPI::is_global] starting...");

		let result = Self::get_mapcycle(None, client)
			.await
			.ok()?
			.into_iter()
			.find(|name| name.contains(&map_name.to_lowercase()))?;
		info!("[GlobalAPI::is_global] completed successfully.");

		Some(result)
	}

	/// Route: `/modes`
	/// - Lets you fetch all modes stored in the GlobalAPI
	pub async fn get_modes(client: &crate::Client) -> Result<Vec<modes::APIMode>, Error> {
		info!("[GlobalAPI::get_modes] starting...");

		let response = modes::get(client).await?;
		info!("[GlobalAPI::get_modes] completed successfully.");

		Ok(response)
	}

	/// - Lets you fetch a mode stored in the GlobalAPI
	///
	/// Routes:
	///
	/// - `/modes/id/{id}`:
	///   - `200` ([KZTimer](crate::prelude::Mode::KZTimer))
	///   - `201` ([SimpleKZ](crate::prelude::Mode::SimpleKZ))
	///   - `202` ([Vanilla](crate::prelude::Mode::Vanilla))
	///
	///   -> All of these are accessible by casting a [Mode](crate::prelude::Mode) to an integer
	///      using the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.
	///
	/// - `/modes/name/{mode_name}`
	///   - `kz_timer` ([KZTimer](crate::prelude::Mode::KZTimer))
	///   - `kz_simple` ([SimpleKZ](crate::prelude::Mode::SimpleKZ))
	///   - `kz_vanilla` ([Vanilla](crate::prelude::Mode::Vanilla))
	///
	///   -> All of these are accessible via [this method](crate::prelude::Mode::api).
	///
	/// This function uses `/modes/id/{id}` because I wanted to.
	pub async fn get_mode(mode: Mode, client: &crate::Client) -> Result<modes::APIMode, Error> {
		info!("[GlobalAPI::get_mode] starting...");

		let response = modes::id::get(mode as u8, client).await?;
		info!("[GlobalAPI::get_mode] completed successfully.");

		Ok(response)
	}

	/// Route: `/players`
	/// - Lets you fetch player information
	///
	/// NOTE: if you want access to more parameters than just `limit`, consider directly using the
	/// [players](crate::global_api::players) module.
	pub async fn get_players(
		offset: Option<i32>,
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<players::Player>, Error> {
		info!("[GlobalAPI::get_players] starting...");

		// not quite sure what to put here yet
		let params = players::Params { offset, limit, ..Default::default() };

		let response = players::get(params, client).await?;
		info!("[GlobalAPI::get_players] completed successfully.");

		Ok(response)
	}

	/// Route: `/players/steam_id/{steam_id}` if `player_identifier` is a [`SteamID`], otherwise
	/// it's `/players` with the according parameters
	/// - Lets you fetch player information
	pub async fn get_player(
		player_identifier: &PlayerIdentifier,
		client: &crate::Client,
	) -> Result<players::Player, Error> {
		info!("[GlobalAPI::get_player] starting...");

		// This is usually faster, so we prioritize it.
		if let PlayerIdentifier::SteamID(steam_id) = player_identifier {
			let response = players::steam_id::get(steam_id, client).await?;
			info!("[GlobalAPI::get_player] completed successfully.");

			return Ok(response);
		}

		let mut params = players::Params::default();

		match player_identifier {
			PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
			PlayerIdentifier::SteamID64(steam_id64) => params.steamid64_list = Some(*steam_id64),
			PlayerIdentifier::SteamID(_) => unreachable!("This case is already covered earlier."),
		}

		let mut response = players::get(params, client).await?;
		info!("[GlobalAPI::get_player] completed successfully.");

		// this is safe to do since `players::get` would've returned early already if the response
		// was empty
		Ok(response.remove(0))
	}

	/// Route: `/players/steamid/{steamid}/ip/{ip}`
	/// - Lets you fetch player information
	/// - `steam_id`: any valid [SteamID](crate::prelude::SteamID) (as a String)
	/// - `ip`: since none of these routes are well documented, I can only guess that this is
	/// supposed to be an IPv4 address.
	pub async fn get_player_by_ip(
		steam_id: &SteamID,
		ip: String,
		client: &crate::Client,
	) -> Result<players::Player, Error> {
		info!("[GlobalAPI::get_player_by_ip] starting...");

		let response = players::ip::get(steam_id, &ip, client).await?;
		info!("[GlobalAPI::get_player_by_ip] completed successfully.");

		Ok(response)
	}

	/// Route: `/players/steamid/{steamid}/alts`
	/// - Lets you fetch alternate accounts of a player
	/// - `steam_id`: any valid [SteamID](crate::prelude::SteamID) (as a string)
	pub async fn get_player_alts(
		steam_id: &SteamID,
		client: &crate::Client,
	) -> Result<Vec<players::Player>, Error> {
		info!("[GlobalAPI::get_player_alts] starting...");

		let response = players::alts::get(steam_id, client).await?;
		info!("[GlobalAPI::get_player_alts] completed successfully.");

		Ok(response)
	}

	/// Route: `/record_filters`
	/// - Lets you fetch record filters for individual courses
	pub async fn get_filters(
		map_id: i32,
		client: &crate::Client,
	) -> Result<Vec<record_filters::RecordFilter>, Error> {
		info!("[GlobalAPI::get_filters] starting...");
		let params = record_filters::Params { map_ids: Some(map_id), ..Default::default() };

		let response = record_filters::get(params, client).await?;
		info!("[GlobalAPI::get_filters] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/place/{id}`
	/// - Lets you fetch the leaderboard spot of a given record
	/// - `id`: `record_id` field on a [Map](maps::Response)
	pub async fn get_place(record_id: i32, client: &crate::Client) -> Result<i32, Error> {
		info!("[GlobalAPI::get_place] starting...");

		let response = records::place::get(record_id, client).await?;
		info!("[GlobalAPI::get_place] completed successfully.");

		Ok(response.0)
	}

	/// Route: `/records/{id}`
	/// - Lets you fetch a record stored in the GlobalAPI
	/// - `id`: `record_id` field on a [Map](maps::Response)
	pub async fn get_record(
		record_id: i32,
		client: &crate::Client,
	) -> Result<records::Record, Error> {
		info!("[GlobalAPI::get_record] starting...");

		let response = records::get(record_id, client).await?;
		info!("[GlobalAPI::get_record] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/top`
	/// - Lets you fetch records stored in the GlobalAPI
	///
	/// NOTE: if you want access to more parameters than just `limit`, consider directly using the
	/// [records::top](crate::global_api::records::top) module.
	pub async fn get_records(
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<records::Record>, Error> {
		info!("[GlobalAPI::get_records] starting...");

		// not quite sure what to put here yet
		let params = records::top::Params { limit, ..Default::default() };

		let response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_records] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/top`
	/// - Lets you fetch all records of a player
	///
	/// NOTE: if you want access to more parameters, consider directly using the
	/// [records::top](crate::global_api::records::top) module.
	pub async fn get_player_records(
		player_identifier: &PlayerIdentifier,
		mode: Mode,
		has_teleports: bool,
		course: u8,
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<records::Record>, Error> {
		info!("[GlobalAPI::get_player_records] starting...");

		let mut params = records::top::Params {
			modes_list_string: Some(mode.api()),
			has_teleports: Some(has_teleports),
			stage: Some(course as i32),
			limit,
			..Default::default()
		};

		match player_identifier {
			PlayerIdentifier::Name(name) => params.player_name = Some(name.to_owned()),
			PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
			PlayerIdentifier::SteamID64(steam_id64) => params.steamid64 = Some(*steam_id64),
		};

		let response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_player_records] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/top/recent`
	/// - Lets you fetch the most recently created records
	/// - Some notes:
	///   - endpoint is pretty slow; it will take a while until a record appears here
	///   - will only yield personal bests
	///   - `mode` is required because if you don't specify one, the API will return an `internal
	///      server error`.
	///
	/// Comparison to [get_recent](Self::get_recent):
	/// - less reliable, because records take a while until they show up here
	/// - not player-specific
	///   - if you want more control over the parameters, consider directly using the
	///     [records::recent](crate::global_api::records::recent) module.
	pub async fn get_recent_lossy(
		mode: Mode,
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<records::recent::RecentRecord>, Error> {
		info!("[GlobalAPI::get_recent_lossy] starting...");

		let params = records::recent::Params {
			modes_list_string: Some(mode.api()),
			limit,
			..Default::default()
		};

		let response = records::recent::get(params, client).await?;
		info!("[GlobalAPI::get_lossy] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/top`
	/// - Lets you fetch a player's most recently set PB(s)
	/// - Will yield as many records as specified, or less.
	/// - Some notes:
	///   - will only yield personal bests
	///   - only cares about main course, not bonuses
	///   - 6 requests are necessary here, as the API returns unreliable results if you're not
	///     specific enough.
	///
	/// Comparison to [get_recent_lossy](Self::get_recent_lossy):
	/// - more reliable, because _all_ of a player's records get fetched, no matter how old they
	///   are
	/// - more expensive, because 6 requests
	/// - player-specific
	pub async fn get_recent(
		player_identifier: &PlayerIdentifier,
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<records::Record>, Error> {
		info!("[GlobalAPI::get_recent] starting...");

		// this is necessary because if we're not specific enough, the API might not return all the
		// data we want
		// if the API returns an error that is not `NoData` in any of the requests, we want to
		// return that error later. these errors include but are not limited to:
		//   - no status code
		//   - internal server error
		let mut potential_err = None;

		// make requests
		let (kzt, skz, vnl) = (Mode::KZTimer, Mode::SimpleKZ, Mode::Vanilla);
		let mut records = join_all([
			Self::get_player_records(player_identifier, kzt, true, 0, Some(9999), client),
			Self::get_player_records(player_identifier, kzt, false, 0, Some(9999), client),
			Self::get_player_records(player_identifier, skz, true, 0, Some(9999), client),
			Self::get_player_records(player_identifier, skz, false, 0, Some(9999), client),
			Self::get_player_records(player_identifier, vnl, true, 0, Some(9999), client),
			Self::get_player_records(player_identifier, vnl, false, 0, Some(9999), client),
		])
		.await
		.into_iter()
		.filter_map(|res| {
			// We want to filter out `NoData` records
			match res {
				Err(why) => {
					// `NoData` is fine; a player might just not have any times in a certain mode
					if let ErrorKind::NoData { expected: _ } = why.kind {
						return None;
					}
					potential_err = Some(why);
					None
				},
				Ok(records) => Some(records.into_iter().filter_map(|rec| {
					let timestamp = match chrono::NaiveDateTime::parse_from_str(
						&rec.created_on,
						"%Y-%m-%dT%H:%M:%S",
					) {
						Ok(timestamp) => timestamp.timestamp(),
						_ => return None,
					};

					Some((timestamp, rec))
				})),
			}
		})
		.flatten()
		.collect::<Vec<_>>();

		// return if we had a "bad" error
		if let Some(why) = potential_err {
			return Err(why);
		}

		// check that there are records
		if records.is_empty() {
			warn!("[GlobalAPI::get_recent] empty response from the API");
			return Err(Error {
				kind: ErrorKind::NoData { expected: String::from("Vec<Record>") },
				msg: String::from("No recent PB found."),
			});
		}

		// sort records by date
		records.sort_by(|(a_timestamp, _), (b_timestamp, _)| b_timestamp.cmp(a_timestamp));

		let mut records = records
			.into_iter()
			.map(|(_, rec)| rec)
			.collect::<Vec<_>>();

		info!("[GlobalAPI::get_recent] completed successfully.");

		if let Some(limit) = limit {
			// We only want to call `.drain()` if there are enough records. Otherwise we just return
			// everything.
			if records.len() >= limit as usize {
				return Ok(records
					.drain(..limit as usize)
					.collect());
			}
		}

		Ok(records)
	}

	/// Route: `/records/top`
	/// - Lets you fetch the World Record on a map
	pub async fn get_wr(
		map_identifier: &MapIdentifier,
		mode: Mode,
		runtype: bool,
		course: u8,
		client: &crate::Client,
	) -> Result<records::Record, Error> {
		info!("[GlobalAPI::get_wr] starting...");

		let mut params = records::top::Params {
			tickrate: Some(128),
			stage: Some(course as i32),
			modes_list_string: Some(mode.api()),
			has_teleports: Some(runtype),
			limit: Some(1),
			..Default::default()
		};

		match map_identifier {
			MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
			MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		};

		let mut response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_wr] completed successfully.");

		Ok(response.remove(0))
	}

	/// Route: `/records/top`
	/// - Lets you fetch a player's personal best on a map
	pub async fn get_pb(
		player_identifier: &PlayerIdentifier,
		map_identifier: &MapIdentifier,
		mode: Mode,
		runtype: bool,
		course: u8,
		client: &crate::Client,
	) -> Result<records::Record, Error> {
		info!("[GlobalAPI::get_pb] starting...");

		let mut params = records::top::Params {
			tickrate: Some(128),
			stage: Some(course as i32),
			modes_list_string: Some(mode.api()),
			has_teleports: Some(runtype),
			limit: Some(1),
			..Default::default()
		};

		match player_identifier {
			PlayerIdentifier::Name(player_name) => {
				params.player_name = Some(player_name.to_owned())
			},
			PlayerIdentifier::SteamID(steam_id) => params.steam_id = Some(steam_id.to_string()),
			PlayerIdentifier::SteamID64(steam_id64) => params.steamid64 = Some(*steam_id64),
		};

		match map_identifier {
			MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
			MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		};

		let mut response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_pb] completed successfully.");

		Ok(response.remove(0))
	}

	/// Route: `/records/top`
	/// - Lets you fetch the top 100 records on a map
	pub async fn get_maptop(
		map_identifier: &MapIdentifier,
		mode: Mode,
		runtype: bool,
		course: u8,
		client: &crate::Client,
	) -> Result<Vec<records::Record>, Error> {
		info!("[GlobalAPI::get_maptop] starting...");

		let mut params = records::top::Params {
			tickrate: Some(128),
			stage: Some(course as i32),
			modes_list_string: Some(mode.api()),
			has_teleports: Some(runtype),
			limit: Some(100),
			..Default::default()
		};

		match map_identifier {
			MapIdentifier::ID(map_id) => params.map_id = Some(*map_id),
			MapIdentifier::Name(map_name) => params.map_name = Some(map_name.to_owned()),
		};

		let response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_maptop] completed successfully.");

		Ok(response)
	}

	/// - Returns a download link to a replay
	/// Route: `/records/replay/{replay_id}`
	/// - `replay_id`: `replay_id` field on a [Record](records::Response)
	/// - Some notes:
	///   - only works for records created on servers with GOKZ version 3.0.0 or higher
	///   - not all of those records made it to the API; expect some missing ones
	pub fn get_replay_by_id(replay_id: i32) -> String {
		info!("[GlobalAPI::get_replay_by_id] completed successfully.");
		format!("{}/records/replay/{}", Self::BASE_URL, replay_id)
	}

	/// Route: `/records/{record_id}/replay`
	/// - Returns a download link to the replay file of a record.
	/// - `record_id`: `id` field on a [Record](records::Response)
	/// - Some notes:
	///   - only works for records created on servers with GOKZ version 3.0.0 or higher
	///   - not all of those records made it to the API; expect some missing ones
	pub fn get_replay_by_record_id(record_id: i32) -> String {
		info!("[GlobalAPI::get_replay_by_record_id] completed successfully.");
		format!("{}/records/{}/replay", Self::BASE_URL, record_id)
	}

	/// Route: `/servers`
	/// - Lets you fetch information about global servers
	pub async fn get_servers(
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<servers::Server>, Error> {
		info!("[GlobalAPI::get_servers] starting...");

		// not quite sure what to put here yet
		let params = servers::Params { limit, ..Default::default() };

		let response = servers::get(params, client).await?;
		info!("[GlobalAPI::get_servers] completed successfully.");

		Ok(response)
	}

	/// Route: `/servers/{id}`
	/// - Lets you fetch information about global servers
	/// - `id`: `id` field on a [Server](servers::Response)
	pub async fn get_server_by_id(
		server_id: i32,
		client: &crate::Client,
	) -> Result<servers::Server, Error> {
		info!("[GlobalAPI::get_server_by_id] starting...");

		let response = servers::id::get(server_id, client).await?;
		info!("[GlobalAPI::get_server_by_id] completed successfully.");

		Ok(response)
	}

	/// Route: `/servers/name/{server_name}`
	/// - Lets you fetch information about global servers
	/// - `server_name`: `server_name` field on a [Server](servers::Response)
	pub async fn get_server_by_name(
		server_name: &str,
		client: &crate::Client,
	) -> Result<servers::Server, Error> {
		info!("[GlobalAPI::get_server_by_name] starting...");

		let response = servers::name::get(server_name, client).await?;
		info!("[GlobalAPI::get_server_by_name] completed successfully.");

		Ok(response)
	}

	/// GlobalAPI Health Report
	///
	/// NOTE: [source](https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1)
	pub async fn checkhealth(client: &crate::Client) -> Result<health::HealthReport, Error> {
		info!("[GlobalAPI::checkhealth] starting...");

		let response = health::get(client).await?;
		info!("[GlobalAPI::checkhealth] completed successfully.");

		Ok(response)
	}
}

/* TODO: some routes are omitted, mainly the ones with non-obvious purposes

/// Route: `/records/top/world_records`
/// - Lets you fetch world records stored in the GlobalAPI
RecordsTopWorldRecords,

/// Route: `/records/replay/list`
/// - Seems to return some information about replays, no idea.
/// - You can not filter this by anything, so the endpoint is pretty useless.
RecordsReplayList,

*/
