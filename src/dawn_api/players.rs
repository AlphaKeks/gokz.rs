//! `/players` endpoints
//!
//! Covered:
//! - `/players`
//! - `/players/:identifier`
//! - `/players/:identifier/completion`

use {
	super::{Record, API_URL},
	crate::{
		http::{self, append_pairs},
		yeet, MapIdentifier, Mode, PlayerIdentifier, Result, Runtype, SteamID, Tier,
	},
	reqwest::Url,
	serde::{Deserialize, Serialize},
	std::ops::{Deref, DerefMut},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerWithCompletion {
	pub name: String,
	pub steam_id: SteamID,
	pub is_banned: bool,
	pub completions: Completion,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Completion {
	pub kzt: CompletionCount,
	pub skz: CompletionCount,
	pub vnl: CompletionCount,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompletionCount {
	pub tp: u32,
	pub pro: u32,
}

macro_rules! impl_player {
	($type:ty) => {
		impl $type {
			/// Returns a link to the player's Steam profile.
			#[inline]
			pub fn steam_profile(&self) -> String {
				format!("https://steamcommunity.com/profiles/{}", self.steam_id.as_id64())
			}

			/// Returns a link to fetch this player from the GlobalAPI.
			#[inline]
			pub fn api(&self) -> String {
				format!("{API_URL}/players/steam_id/{}", self.steam_id)
			}

			/// Returns a link to the player's KZ:GO profile.
			#[inline]
			pub fn kzgo_profile(&self) -> String {
				format!("https://kzgo.eu/players/{}", self.steam_id)
			}
		}
	};
}

impl_player!(Player);
impl_player!(PlayerWithCompletion);

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Params {
	pub is_banned: Option<bool>,
	pub offset: Option<u32>,
	pub limit: Option<u32>,
}

/// `/players` route
///
/// Fetches players with the given `params`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_players_with(params: &Params, client: &http::Client) -> Result<Vec<Player>> {
	let players = http::get! {
		url = format!("{API_URL}/players");
		params = params;
		deserialize = Vec<Player>;
		client = client;
	}?;

	if players.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(players)
}

/// `/players/:player_identifier` route.
///
/// Fetches a single player by their name or [`SteamID`].
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_player(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	client: &http::Client,
) -> Result<PlayerWithCompletion> {
	let player = http::get! {
		url = format!("{API_URL}/players/{}", player.into());
		deserialize = PlayerWithCompletion;
		client = client;
	}?;

	Ok(player)
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompletionParams {
	#[serde(skip)]
	pub stages: Option<Vec<u8>>,
	pub tier: Option<Tier>,
	pub mode: Option<Mode>,
	pub runtype: Option<Runtype>,
	pub not_completed: Option<bool>,
	pub limit: Option<u64>,
	pub offset: Option<i64>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompletedCourse {
	pub course_id: u32,
	pub map_id: u16,
	pub map_name: String,
	pub map_stage: u8,
	pub stage_tier: Option<Tier>,
	pub mode: Mode,
	pub has_teleports: Runtype,
	pub steam_id: SteamID,
	pub player_name: String,
}

/// `/players/:player_identifier/completion` route.
///
/// Fetches all courses (not) completed by a given `player`.
///
/// If the API response is empty, this function will return an [`Error`](crate::Error).
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_completed(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	params: &CompletionParams,
	client: &http::Client,
) -> Result<Vec<CompletedCourse>> {
	let mut url = Url::parse(&format!("{API_URL}/players/{}/completion", player.into()))
		.expect("This is a valid URL.");

	append_pairs!(&mut url, &params.stages, "stages");

	let completed = http::get! {
		url = url;
		params = params;
		deserialize = Vec<CompletedCourse>;
		client = client;
	}?;

	if completed.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(completed)
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgressionParams {
	pub map: Option<MapIdentifier>,
	pub stage: Option<u8>,
	pub runtype: Option<Runtype>,
	pub limit: Option<u64>,
	pub offset: Option<i64>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressionRecord {
	#[serde(flatten)]
	pub record: Record,
	pub time_improvement: Option<f64>,
	pub attempts: u32,
}

impl Deref for ProgressionRecord {
	type Target = Record;

	fn deref(&self) -> &Self::Target {
		&self.record
	}
}

impl DerefMut for ProgressionRecord {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.record
	}
}

/// `/records/progression/:player/:mode` route
///
/// Fetches all of a player's personal bests in chronological order for the given mode.
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_pb_progresion(
	player: impl Into<PlayerIdentifier> + std::fmt::Debug,
	map: impl Into<MapIdentifier> + std::fmt::Debug,
	mode: impl Into<Mode> + std::fmt::Debug,
	params: &ProgressionParams,
	client: &crate::http::Client,
) -> Result<Vec<ProgressionRecord>> {
	let records = http::get! {
		url = format!("{API_URL}/players/{}/progression/{}/{}", player.into(), map.into(), mode.into().api());
		params = &params;
		deserialize = Vec<ProgressionRecord>;
		client = client;
	}?;

	if records.is_empty() {
		yeet!(EmptyResponse);
	}

	Ok(records)
}
