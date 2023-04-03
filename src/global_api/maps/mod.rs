use {
	crate::{
		chrono::{deser_date, parse_date, ser_date},
		http, Error, Result, SteamID, Tier,
	},
	chrono::NaiveDateTime,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub difficulty: Tier,
	pub validated: bool,
	pub filesize: u64,
	pub approved_by: SteamID,
	pub workshop_url: String,
	pub download_url: String,
	#[serde(serialize_with = "ser_date")]
	#[serde(deserialize_with = "deser_date")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	#[serde(deserialize_with = "deser_date")]
	pub updated_on: NaiveDateTime,
}

/// `/maps` route
pub mod index;
impl TryFrom<index::Response> for Map {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		let download_url = format!("https://maps.global-api.com/bsps/{}.bsp", &value.name);

		Ok(Self {
			id: value.id.try_into()?,
			name: value.name,
			difficulty: u8::try_from(value.difficulty)?.try_into()?,
			validated: value.validated,
			filesize: value.filesize.try_into()?,
			approved_by: value.approved_by_steamid64.parse()?,
			workshop_url: value.workshop_url,
			download_url,
			created_on: parse_date!(value.created_on),
			updated_on: parse_date!(value.updated_on),
		})
	}
}

/// Fetches maps with the given `params`.
pub async fn get_maps(params: index::Params, client: &crate::Client) -> Result<Vec<Map>> {
	let response: Vec<index::Response> =
		http::get_with_params(&format!("{}/maps", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a map by its name.
pub async fn get_map_by_name(map_name: &str, client: &crate::Client) -> Result<Map> {
	http::get::<index::Response>(
		&format!("{}/maps/name/{}", super::BASE_URL, map_name),
		client,
	)
	.await?
	.try_into()
}

/// Fetches a map by its ID.
pub async fn get_map_by_id(map_id: u16, client: &crate::Client) -> Result<Map> {
	http::get::<index::Response>(&format!("{}/maps/id/{}", super::BASE_URL, map_id), client)
		.await?
		.try_into()
}
