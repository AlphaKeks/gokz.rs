use {
	crate::{
		chrono::{parse_date, ser_date},
		http::get_with_params,
		Error, Mode, Result, SteamID,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct RecordFilter {
	pub id: u32,
	pub map_id: u16,
	pub stage: u8,
	pub mode: Mode,
	pub tickrate: u8,
	pub has_teleports: bool,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
	pub updated_by: SteamID,
}

/// `/record_filters` route
pub mod index;
impl TryFrom<index::Response> for RecordFilter {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			map_id: value.id.try_into()?,
			stage: value.id.try_into()?,
			mode: u8::try_from(value.mode_id)?.try_into()?,
			tickrate: value.tickrate.try_into()?,
			has_teleports: value.has_teleports,
			created_on: parse_date!(value.created_on),
			updated_by: value.updated_by_id.parse()?,
		})
	}
}

/// Fetches filters with the given `params`.
pub async fn get_filters(
	params: index::Params,
	client: &reqwest::Client,
) -> Result<Vec<RecordFilter>> {
	let response: Vec<index::Response> = get_with_params(
		&format!("{}/record_filters", super::BASE_URL),
		params,
		client,
	)
	.await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}
