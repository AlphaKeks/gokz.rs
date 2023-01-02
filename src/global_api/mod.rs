use {
	crate::prelude::*,
	log::{info, trace, warn},
};

pub(crate) mod bans;
use bans::Response as BanResponse;

pub mod jumpstats;

pub mod maps;
use maps::Response as MapResponse;

pub mod modes;
use modes::Response as ModeResponse;

pub mod players;
use players::Response as PlayerResponse;

pub mod record_filters;
use record_filters::Response as RecordFilterResponse;

pub mod records;
use records::{place::Response as PlaceResponse, Response as RecordResponse};

pub(crate) trait GlobalAPIResponse {}

pub(crate) trait GlobalAPIParams {}

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
		Response: std::fmt::Debug + serde::de::DeserializeOwned,
		Params: std::fmt::Debug + Clone + serde::Serialize,
	{
		info!("[GlobalAPI::request] starting...");
		trace!("[GlobalAPI::request] `route`: {:?}", route);
		trace!("[GlobalAPI::request] `params`: {:?}", &params);

		// construct full URL
		// e.g. `https://kztimerglobal.com/api/v2/records/top?`
		let full_route = format!("{}{}", Self::BASE_URL, route);
		let url = match reqwest::Url::parse(&full_route) {
			Err(why) => {
				warn!("[GlobalAPI::request] Failed to parse URL: {:?}", why);

				return Err(Error {
					kind: ErrorKind::Parsing {
						expected: String::from("valid URL"),
						got: Some(format!("route: {:?}\nparams: {:?}", route, params)),
					},
					msg: String::from("Failed to parse URL."),
				});
			},
			Ok(url) => {
				trace!("[GlobalAPI::request] Successfully constructed URL `{}`.", &url);
				url
			},
		};

		// make a GET request to the GlobalAPI
		let response = match client.get(url).query(&params).send().await {
			Err(why) => {
				warn!("[GlobalAPI::request] HTTPS Request failed.");

				if let Some(code) = why.status() {
					warn!("[GlobalAPI::request] Request failed with status code `{}`.", &code);

					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("GlobalAPI request failed with code `{}`.", code),
					});
				}

				warn!("[GlobalAPI::request] Request failed with no status code.");

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
			Ok(response) => {
				trace!(
					"[GlobalAPI::request] GlobalAPI responded successfully with code `{}`.",
					response.status()
				);
				response
			},
		};

		// parse the response into the desired `Response` format
		let parsed_response = match response.json::<Response>().await {
			Err(why) => {
				warn!("[GlobalAPI::request] Failed to parse response.");
				warn!("[GlobalAPI::request] {:?}", why);

				return Err(Error {
					kind: ErrorKind::Parsing { expected: String::from("JSON"), got: None },
					msg: String::from("Failed to parse GlobalAPI response."),
				});
			},
			Ok(parsed_response) => {
				trace!("[GlobalAPI::request] Successfully parsed response.");
				parsed_response
			},
		};

		info!("[GlobalAPI::request] completed successfully.");
		info!("[GlobalAPI::request] Response: {:?}", &parsed_response);

		// return the `Response`
		Ok(parsed_response)
	}

	/// Route: `/bans`
	/// - Lets you fetch ban entries of players
	pub async fn get_bans(
		steam_id: SteamID,
		limit: u32,
		client: &crate::Client,
	) -> Result<Vec<BanResponse>, Error> {
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
	) -> Result<Vec<MapResponse>, Error> {
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

	/// Route: `/maps/id/{map_id}` _OR_ `/maps/name/{map_name}`
	/// - Lets you fetch a map stored in the GlobalAPI
	pub async fn get_map(
		map_identifier: &MapIdentifier,
		client: &crate::Client,
	) -> Result<MapResponse, Error> {
		info!("[GlobalAPI::get_map] starting...");

		let response = match map_identifier {
			MapIdentifier::ID(id) => maps::id::get(*id, client).await?,
			MapIdentifier::Name(name) => maps::name::get(name, client).await?,
		};
		info!("[GlobalAPI::get_map] completed successfully.");

		Ok(response)
	}

	/// Route: `/modes`
	/// - Lets you fetch all modes stored in the GlobalAPI
	pub async fn get_modes(client: &crate::Client) -> Result<Vec<ModeResponse>, Error> {
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
	///   - `200` (KZTimer)
	///   - `201` (SimpleKZ)
	///   - `202` (Vanilla)
	///
	/// All of these are accessible by casting a [Mode](crate::prelude::Mode) to an integer using
	/// the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.
	///
	/// - `/modes/name/{mode_name}`
	///   - `kz_timer`
	///   - `kz_simple`
	///   - `kz_vanilla`
	///
	/// All of these are accessible via [this method](crate::prelude::Mode::api).
	///
	/// This function uses `/modes/id/{id}` because I wanted to. There is not objective choice here.
	pub async fn get_mode(mode: &Mode, client: &crate::Client) -> Result<ModeResponse, Error> {
		info!("[GlobalAPI::get_mode] starting...");

		let response = modes::id::get(*mode as u8, client).await?;
		info!("[GlobalAPI::get_mode] completed successfully.");

		Ok(response)
	}

	/// Route: `/players`
	/// - Lets you fetch player information
	///
	/// NOTE: if you want access to more parameters than just `limit`, consider directly using
	/// [`players::get`] instead.
	pub async fn get_players(
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<PlayerResponse>, Error> {
		info!("[GlobalAPI::get_players] starting...");

		// not quite sure what to put here yet
		let params = players::Params { limit, ..Default::default() };

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
	) -> Result<PlayerResponse, Error> {
		info!("[GlobalAPI::get_player] starting...");

		// This is usually faster, so we prioritize it.
		if let PlayerIdentifier::SteamID(steam_id) = player_identifier {
			let response = players::steam_id::get(steam_id, client).await?;
			info!("[GlobalAPI::get_players] completed successfully.");

			return Ok(response);
		}

		let mut params = players::Params::default();

		match player_identifier {
			PlayerIdentifier::Name(name) => params.name = Some(name.to_owned()),
			PlayerIdentifier::SteamID64(steam_id64) => params.steamid64_list = Some(*steam_id64),
			PlayerIdentifier::SteamID(_) => unreachable!("This case is already covered earlier."),
		}

		let mut response = players::get(params, client).await?;
		info!("[GlobalAPI::get_players] completed successfully.");

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
	) -> Result<PlayerResponse, Error> {
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
	) -> Result<Vec<PlayerResponse>, Error> {
		info!("[GlobalAPI::get_player_alts] starting...");

		let response = players::alts::get(steam_id, client).await?;
		info!("[GlobalAPI::get_player_alts] completed successfully.");

		Ok(response)
	}

	/// Route: `/record_filters`
	/// - Lets you fetch record filters for individual courses
	pub async fn get_filters(
		map_id: u32,
		client: &crate::Client,
	) -> Result<Vec<RecordFilterResponse>, Error> {
		info!("[GlobalAPI::get_filters] starting...");
		let params = record_filters::Params { map_ids: Some(map_id), ..Default::default() };

		let response = record_filters::get(params, client).await?;
		info!("[GlobalAPI::get_filters] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/place/{id}`
	/// - Lets you fetch the leaderboard spot of a given record
	/// - `id`: `record_id` property on a [Map](maps::Response)
	pub async fn get_place(record_id: u32, client: &crate::Client) -> Result<PlaceResponse, Error> {
		info!("[GlobalAPI::get_place] starting...");

		let response = records::place::get(record_id, client).await?;
		info!("[GlobalAPI::get_place] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/{id}`
	/// - Lets you fetch a record stored in the GlobalAPI
	/// - `id`: `record_id` property on a [Map](maps::Response)
	pub async fn get_record(
		record_id: u32,
		client: &crate::Client,
	) -> Result<RecordResponse, Error> {
		info!("[GlobalAPI::get_record] starting...");

		let response = records::get(record_id, client).await?;
		info!("[GlobalAPI::get_record] completed successfully.");

		Ok(response)
	}

	/// Route: `/records/top`
	/// - Lets you fetch records stored in the GlobalAPI
	///
	/// NOTE: if you want access to more parameters than just `limit`, consider directly using
	/// [`records::top::get`] instead.
	pub async fn get_records(
		limit: Option<u32>,
		client: &crate::Client,
	) -> Result<Vec<RecordResponse>, Error> {
		info!("[GlobalAPI::get_records] starting...");

		// not quite sure what to put here yet
		let params = records::top::Params { limit, ..Default::default() };

		let response = records::top::get(params, client).await?;
		info!("[GlobalAPI::get_records] completed successfully.");

		Ok(response)
	}
}

/* TODO: some routes are omitted, mainly the ones with non-obvious purposes

/// Route: `/records/top/world_records`
/// - Lets you fetch world records stored in the GlobalAPI
RecordsTopWorldRecords,

/// Route: `/records/top/recent`
/// - Lets you fetch the most recently created records
/// - Some notes:
///   - only works for personal bests
///   - this endpoint is pretty slow, so expect to wait some time before a record actually
///     appears here
RecordsTopRecent,

/// Route: `/records/record_filter`
/// - I have no idea what this does, but it returns some information.
RecordsRecordFilter,

/// Route: `/records/{record_id}/replay`
/// - Returns a download link to the replay file of a record.
/// - Some notes:
///   - only works for records created on servers with GOKZ version 3.0.0 or higher
///   - not all of those records made it to the API; expect some missing ones
/// - `record_id`: `id` property on a [Record]()
RecordsIDReplay,

/// Route: `/records/replay/{replay_id}`
/// - Returns a download link to a replay
/// - Some notes:
///   - only works for records created on servers with GOKZ version 3.0.0 or higher
///   - not all of those records made it to the API; expect some missing ones
/// - `replay_id`: `replay_id` property on a [Record]()
RecordsReplayID,

/// Route: `/records/replay/list`
/// - Seems to return some information about replays, no idea.
/// - You can not filter this by anything, so the endpoint is pretty useless.
RecordsReplayList,

/// Route: `/servers`
/// - Lets you fetch information about global servers
Servers,

/// Route: `/servers/{id}`
/// - Lets you fetch information about a global server
/// - `id`: if you are a server owner, you can get your server's ID [here](https://portal.global-api.com/dashboard/servers/owned).
ServersID,

/// Route: `/servers/name/{server_name}`
/// - Lets you fetch information about a global server
/// - `server_name`: name of a server
ServersName,

*/
