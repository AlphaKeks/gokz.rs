use {
	crate::{
		chrono::{parse_date, ser_date},
		http, Error, Mode, Result, SteamID,
	},
	chrono::NaiveDateTime,
	log::{debug, trace},
	serde::Serialize,
	std::ops::Range,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Record {
	pub id: u32,
	pub player_name: String,
	pub steam_id: SteamID,
	pub map_id: u16,
	pub map_name: String,
	pub stage: u8,
	pub mode: Mode,
	pub server_id: u16,
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
		if self.replay_id != 0 {
			return format!("{}/records/replay/{}", super::BASE_URL, self.replay_id);
		}
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
pub async fn get_record(record_id: u32, client: &crate::Client) -> Result<Record> {
	trace!("> get_record {{ record_id: {record_id} }}");
	http::get::<id::Response>(
		&format!("{}/records/{}", super::BASE_URL, record_id),
		client,
	)
	.await?
	.try_into()
}

/// Fetches the place of a record on its leaderboard.
pub async fn get_place(record_id: u32, client: &crate::Client) -> Result<u32> {
	trace!("> get_place {{ record_id: {record_id} }}");
	http::get::<u32>(
		&format!("{}/records/place/{}", super::BASE_URL, record_id),
		client,
	)
	.await
}

/// The `/records/top` route.
pub async fn get_top(params: top::Params, client: &crate::Client) -> Result<Vec<Record>> {
	let response: Vec<id::Response> =
		http::get_with_params(&format!("{}/records/top", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|record| {
			let converted = record.try_into();
			if converted.is_err() {
				debug!("Converted: {converted:#?}");
			}
			converted.ok()
		})
		.collect())
}

/// The `/records/top/world_records` route.
pub mod world_records;

pub use world_records::WRStats;

/// Fetches the leaderboard for most world records held per player.
pub async fn get_wr_top(
	mode: Mode,
	has_teleports: bool,
	courses: Range<u8>,
	client: &crate::Client,
) -> Result<Vec<WRStats>> {
	let url = format!(
		"{}/records/top/world_records{}",
		super::BASE_URL,
		courses
			.map(|course| format!("&stages={course}"))
			.collect::<String>()
			.replacen('&', "?", 1)
	);
	let params = world_records::Params {
		mode_ids: Some(mode as u8),
		has_teleports: Some(has_teleports),
		limit: Some(999),
		..Default::default()
	};
	trace!("> get_wr_top {{ url: {url}, {params:#?} }}");

	let response: Vec<world_records::Response> =
		http::get_with_params(&url, params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|record| record.try_into().ok())
		.collect())
}
