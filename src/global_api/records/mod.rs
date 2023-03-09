use {
	crate::{
		chrono::{parse_date, ser_date},
		global_api::ServerID,
		http, Error, MapID, MapName, Mode, Result, SteamID,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

pub type RecordID = u32;
pub type Place = u32;

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Record {
	pub id: RecordID,
	pub player_name: String,
	pub steam_id: SteamID,
	pub map_id: MapID,
	pub map_name: MapName,
	pub stage: u8,
	pub mode: Mode,
	pub server_id: ServerID,
	pub server_name: String,
	pub time: f64,
	pub teleports: u32,
	pub points: u32,
	pub replay_id: u32,
	pub tickrate: u8,
	pub record_filter_id: u32,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	pub updated_on: NaiveDateTime,
}

impl Record {
	/// Returns a link to download a global replay by its ID.
	pub fn replay_download_link(&self) -> String {
		format!("{}/records/{}/replay", super::BASE_URL, self.id)
	}

	/// Returns a link to watch a global replay using
	/// [GameChaos' GlobalReplays Project](https://github.com/GameChaos/GlobalReplays).
	pub fn replay_view_link(&self) -> Option<String> {
		if self.replay_id == 0 {
			return None;
		}

		Some(format!(
			"http://gokzmaptest.site.nfoservers.com/GlobalReplays/?replay={}",
			self.replay_id
		))
	}
}

/// `/records` route
pub mod id;
impl TryFrom<id::Response> for Record {
	type Error = Error;

	fn try_from(value: id::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			player_name: value
				.player_name
				.unwrap_or_else(|| String::from("unknown")),
			steam_id: value
				.steam_id
				.ok_or(Error::Custom("Missing SteamID."))?
				.parse()?,
			map_id: value.map_id.try_into()?,
			map_name: value
				.map_name
				.unwrap_or_else(|| String::from("unknown")),
			stage: value.stage.try_into()?,
			mode: value.mode.parse()?,
			server_id: value.server_id.try_into()?,
			server_name: value
				.server_name
				.unwrap_or_else(|| String::from("unknown")),
			time: value.time,
			teleports: value.teleports.try_into()?,
			points: value.points.try_into()?,
			replay_id: value.replay_id.try_into()?,
			tickrate: value.tickrate.try_into()?,
			record_filter_id: value.record_filter_id.try_into()?,
			created_on: parse_date!(value.created_on),
			updated_on: parse_date!(value.updated_on),
		})
	}
}

/// The `/records/top` route.
pub mod top;

/// Fetches a single record by its ID.
pub async fn get_record(record_id: RecordID, client: &crate::Client) -> Result<Record> {
	http::get::<id::Response>(
		&format!("{}/records/{}", super::BASE_URL, record_id),
		client,
	)
	.await?
	.try_into()
}

/// Fetches the place of a record on its leaderboard.
pub async fn get_place(record_id: RecordID, client: &crate::Client) -> Result<Place> {
	http::get::<Place>(
		&format!("{}/records/place/{}", super::BASE_URL, record_id),
		client,
	)
	.await
}

/// The `/records/top` route.
pub async fn get_top(params: top::Params, client: &crate::Client) -> Result<Vec<Record>> {
	let response: Vec<id::Response> =
		http::get_with_params(&format!("{}/top", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|record| record.try_into().ok())
		.collect())
}
