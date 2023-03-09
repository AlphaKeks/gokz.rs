use {
	crate::{
		chrono::{parse_date, ser_date},
		http::{get, get_with_params},
		Error, Result, SteamID, Tier,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub filesize: u64,
	pub validated: bool,
	pub difficulty: Tier,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	pub updated_on: NaiveDateTime,
	pub approved_by: SteamID,
	pub workshop_url: String,
	pub download_url: String,
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
			filesize: value.filesize.try_into()?,
			validated: value.validated,
			difficulty: u8::try_from(value.difficulty)?.try_into()?,
			created_on: parse_date!(value.created_on),
			updated_on: parse_date!(value.updated_on),
			approved_by: value.approved_by_steamid64.parse()?,
			workshop_url: value.workshop_url,
			download_url,
		})
	}
}

/// Fetches maps with the given `params`.
pub async fn get_maps(params: index::Params, client: &reqwest::Client) -> Result<Vec<Map>> {
	let response: Vec<index::Response> =
		get_with_params(&format!("{}/maps", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a map by its name.
pub async fn get_map_by_name(map_name: &str, client: &reqwest::Client) -> Result<Map> {
	get::<index::Response>(
		&format!("{}/maps/name/{}", super::BASE_URL, map_name),
		client,
	)
	.await?
	.try_into()
}

/// Fetches a map by its ID.
pub async fn get_map_by_id(map_id: u16, client: &reqwest::Client) -> Result<Map> {
	get::<index::Response>(&format!("{}/maps/id/{}", super::BASE_URL, map_id), client)
		.await?
		.try_into()
}
