use {
	super::{maps::Course, players::Player},
	crate::{
		chrono::{parse_date, ser_date},
		http, Error, MapIdentifier, Mode, PlayerIdentifier, Result,
	},
	chrono::NaiveDateTime,
	log::trace,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Record {
	pub id: u32,
	pub map_name: String,
	pub course: Course,
	pub mode: Mode,
	pub player: Player,
	pub server_name: String,
	pub time: f64,
	pub teleports: u32,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
}

impl Record {
	/// Returns a link to download a global replay by its ID.
	pub fn replay_download_link(&self) -> String {
		format!("{}/records/{}/replay", super::BASE_URL, self.id)
	}
}

/// The `/records` route.
pub mod index;

/// The `/records/:id` route.
pub mod id;
impl TryFrom<id::Response> for Record {
	type Error = Error;

	fn try_from(value: id::Response) -> Result<Self> {
		Ok(Self {
			id: value.id,
			map_name: value.map_name,
			course: value.course,
			mode: value.mode.parse()?,
			player: value.player.try_into()?,
			server_name: value.server_name,
			time: value.time,
			teleports: value.teleports,
			created_on: parse_date!(value.created_on),
		})
	}
}

/// The `/records/top` route.
pub mod top;

/// Fetches a single record by its ID.
pub async fn get_record(record_id: u32, client: &crate::Client) -> Result<Record> {
	trace!("> get_record {{ record_id: {record_id} }}");
	http::get::<super::Response<id::Response>>(
		&format!("{}/records/{}", super::BASE_URL, record_id),
		client,
	)
	.await?
	.result
	.try_into()
}

/// Fetches records with the given params.
pub async fn get_records(params: index::Params, client: &crate::Client) -> Result<Vec<Record>> {
	Ok(
		http::get_with_params::<_, super::Response<Vec<id::Response>>>(
			&format!("{}/records", super::BASE_URL),
			params,
			client,
		)
		.await
		.map(|response| response.result)?
		.into_iter()
		.filter_map(|record| record.try_into().ok())
		.collect(),
	)
}

/// The `/records/top/player/:ident` route.
pub async fn get_top_player(
	player: PlayerIdentifier,
	params: top::PlayerParams,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let response: Vec<id::Response> = http::get_with_params(
		&format!("{}/top/player/{}", super::BASE_URL, player),
		params,
		client,
	)
	.await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|record| record.try_into().ok())
		.collect())
}

/// The `/records/top/map/:ident` route.
pub async fn get_top_map(
	map: MapIdentifier,
	params: top::MapParams,
	client: &crate::Client,
) -> Result<Vec<Record>> {
	let response: Vec<id::Response> = http::get_with_params(
		&format!("{}/top/map/{}", super::BASE_URL, map),
		params,
		client,
	)
	.await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|record| record.try_into().ok())
		.collect())
}
