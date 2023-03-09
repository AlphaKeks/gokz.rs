use {
	crate::{
		chrono::{parse_date, ser_date},
		global_api::ServerID,
		http, Error, Result, SteamID,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum BanType {
	Other,
	BhopHack,
	BhopMacro,
	StrafeHack,
}

impl Serialize for BanType {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(match self {
			BanType::Other => "other",
			BanType::BhopHack => "bhop_hack",
			BanType::BhopMacro => "bhop_macro",
			BanType::StrafeHack => "strafe_hack",
		})
	}
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Ban {
	pub id: u32,
	pub ban_type: BanType,
	pub player_name: String,
	pub steam_id: SteamID,
	pub server_id: ServerID,
	pub stats: String,
	pub notes: String,
	pub updated_by_id: SteamID,
	#[serde(serialize_with = "ser_date")]
	pub expires_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_date")]
	pub updated_on: NaiveDateTime,
}

/// `/bans` route
pub mod index;
impl TryFrom<index::Response> for Ban {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			ban_type: match value.ban_type.as_str() {
				"other" => BanType::Other,
				"bhop_hack" => BanType::BhopHack,
				"bhop_macro" => BanType::BhopMacro,
				"strafe_hack" => BanType::StrafeHack,
				_ => BanType::Other,
			},
			player_name: value.player_name,
			steam_id: value.steam_id.parse()?,
			server_id: value.server_id.try_into()?,
			stats: value.stats,
			notes: value.notes,
			updated_by_id: value.updated_by_id.parse()?,
			expires_on: parse_date!(value.expires_on),
			created_on: parse_date!(value.created_on),
			updated_on: parse_date!(value.updated_on),
		})
	}
}

/// Fetches bans with the given `params`.
pub async fn get_bans(params: index::Params, client: &crate::Client) -> Result<Vec<Ban>> {
	let response: Vec<index::Response> =
		http::get_with_params(&format!("{}/bans", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}
