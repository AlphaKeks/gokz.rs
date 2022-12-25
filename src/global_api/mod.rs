pub(crate) mod bans;

use {
	self::bans::Response as Ban,
	crate::prelude::*,
	log::{info, trace, warn},
};

pub struct GlobalAPI;

pub(crate) trait GlobalAPIResponse {}
pub(crate) trait GlobalAPIParams {}

impl GlobalAPI {
	pub const BASE_URL: &str = "https://kztimerglobal.com/api/v2";

	/// Base request to the GlobalAPI
	///
	/// - `route`: **NOT** the full URL. Only the relevant part. (e.g. `/records/top?`)
	/// - `params`: the parameters for the request
	pub async fn get_raw<Response, Params>(
		route: &str,
		params: Params,
		client: &crate::Client,
	) -> Result<Response, Error>
	where
		Response: std::fmt::Debug + serde::de::DeserializeOwned,
		Params: std::fmt::Debug + Clone + serde::Serialize,
	{
		info!("[GlobalAPI::request] START");
		trace!("[GlobalAPI::request] `route`: {:?}", route);
		trace!("[GlobalAPI::request] `params`: {:?}", &params);

		// construct full URL
		// e.g. `https://kztimerglobal.com/api/v2/records/top?
		let full_route = format!("{}{}", Self::BASE_URL, route);
		let url = match reqwest::Url::parse(&full_route) {
			Err(why) => {
				warn!("[GlobalAPI::request] Failed to parse URL: {:?}", &why);

				return Err(Error {
					kind: ErrorKind::Parsing {
						input: Some(format!("{{ route: `{}`, params: `{:?}` }}", route, params)),
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
				} else {
					warn!("[GlobalAPI::request] Request failed with no status code.");
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: None,
							raw_message: Some(why.to_string()),
						},
						msg: String::from(
							"GlobalAPI request failed. No status code has been returned.",
						),
					});
				}
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
				warn!(
					"[GlobalAPI::request] Failed to parse response as `{}`.",
					stringify!(Response)
				);
				warn!("[GlobalAPI::request] {:?}", why);
				return Err(Error {
					kind: ErrorKind::Parsing { input: None },
					msg: String::from("Failed to parse GlobalAPI response."),
				});
			},
			Ok(parsed_response) => {
				trace!(
					"[GlobalAPI::request] Successfully parsed response as `{}`.",
					stringify!(Response)
				);
				parsed_response
			},
		};

		info!("[GlobalAPI::request] Successfully completed.");
		info!("[GlobalAPI::request] Response: {:?}", &parsed_response);

		// return the `Response`
		Ok(parsed_response)
	}

	pub async fn get_bans(steam_id: SteamID, client: &crate::Client) -> Result<Ban, Error> {
		match self::bans::get(steam_id, client).await {
			Err(why) => Err(why),
			Ok(response) => {
				info!("[GlobalAPI::get_bans] Successfully completed.");
				Ok(response)
			},
		}
	}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Route {
	/// Route: `/bans`
	/// - Lets you fetch ban entries of players
	Bans,

	/// Route: `/jumpstats`
	/// - Lets you fetch "global" jumpstats from legacy KZTimer servers
	Jumpstats,

	/// Route: `/jumpstats/{jump_type}/top`
	/// - `jump_type`: not documented anywhere.
	/// - Lets you fetch the top "global" jumpstats from legacy KZTimer servers
	JumpstatsTop,

	/// Route: `/jumpstats/{jump_type}/top30`
	/// - `jump_type`: not documented anywhere.
	/// - Note: The last time I tried using this route it didn't work.
	JumpstatsTop30,

	/// Route: `/maps`
	/// - Lets you fetch all maps stored in the GlobalAPI
	Maps,

	/// Route: `/maps/{id}`
	/// - `id`: `id` property on a [Map]()
	/// - Lets you fetch a map stored in the GlobalAPI
	MapsID,

	/// Route: `/maps/name/{map_name}`
	/// - `map_name`: any of [these](https://maps.global-api.com/mapcycles/gokz.txt)
	/// - Lets you fetch a map stored in the GlobalAPI
	MapsName,

	/// Route: `/modes`
	/// - Lets you fetch all modes stored in the GlobalAPI
	Modes,

	/// Route: `/modes/name/{mode_name}`
	/// - Lets you fetch a mode stored in the GlobalAPI
	/// - Available `mode_name`s:
	///   - `kz_timer`
	///   - `kz_simple`
	///   - `kz_vanilla`
	///
	/// All of these are accessible via [this method](crate::prelude::Mode::api).
	ModesName,

	/// Route: `/modes/id/{id}`
	/// - Lets you fetch a mode stored in the GlobalAPI
	/// - Available `id`s:
	///   - `200` (KZTimer)
	///   - `201` (SimpleKZ)
	///   - `202` (Vanilla)
	///
	/// All of these are accessible by casting a [Mode](crate::prelude::Mode) to an integer using
	/// the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.
	ModesId,

	/// Route: `/player_ranks`
	/// - The purpose of this route isn't documented but it seems to fetch leaderboard entries.
	PlayerRanks,

	/// Route: `/players`
	/// - Lets you fetch player information
	Players,

	/// Route: `/players/steamid/{steamid}`
	/// - Lets you fetch player information
	/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a string)
	PlayersSteamID,

	/// Route: `/players/steamid/{steamid}/ip/{ip}`
	/// - Lets you fetch player information
	/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a String)
	/// - `ip`: since none of these routes are well documented, I can only guess that this is
	/// supposed to be an IPv4 address as a String.
	PlayersSteamIDByIP,

	/// Route: `/players/steamid/{steamid}/alts`
	/// - Lets you fetch alternate accounts of a player
	/// - `steamid`: any valid [SteamID](crate::prelude::SteamID) (as a string)
	PlayersSteamIDAlts,

	/// Route: `/players/get_banned_players/steamid`
	/// - Note: since none of these routes are well documented, I can only guess that this used to
	/// serve the same purpose as the [Bans](Route::Bans) route. You probably shouldn't use it.
	PlayersGetBannedPlayers,

	/// Route: `/record_filters`
	/// - Lets you fetch record filters for individual courses
	RecordFilters,

	/// Route: `/record_filters/distributions`
	/// - I have no idea what this does, but it returns some information.
	RecordFiltersDistributions,

	/// Route: `/records/place/{id}`
	/// - Lets you fetch the leaderboard spot of a given record
	/// - `id`: `record_id` property on a [Map]()
	RecordsPlaceID,

	/// Route: `/records/{id}`
	/// - Lets you fetch a record stored in the GlobalAPI
	/// - `id`: `record_id` property on a [Map]()
	RecordsID,

	/// Route: `/records/top`
	/// - Lets you fetch records stored in the GlobalAPI
	RecordsTop,

	/// Route: `/records/top/world_records`
	/// - Lets you fetch world records stored in the GlobalAPI
	RecordsTopWorldRecords,

	/// Route: `/records/top/recent`
	/// - Lets you fetch the most recently created records
	/// - Some notes:
	///   - only personal bests will be queryable
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
}
