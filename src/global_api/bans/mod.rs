use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/bans`
/// - Lets you fetch ban entries of players
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<Ban>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/bans?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("ban data") },
					msg: String::from("No bans found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub ban_types: Option<String>,
	pub ban_types_list: Option<String>,
	pub is_expired: Option<bool>,
	pub ip: Option<String>,
	pub steamid64: Option<String>,
	pub steam_id: Option<String>,
	pub notes_contains: Option<String>,
	pub stats_contains: Option<String>,
	pub server_id: Option<u32>,
	pub created_since: Option<String>,
	pub updated_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);

impl Default for Params {
	fn default() -> Self {
		Self {
			ban_types: None,
			ban_types_list: None,
			is_expired: None,
			ip: None,
			steamid64: None,
			steam_id: None,
			notes_contains: None,
			stats_contains: None,
			server_id: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(1),
		}
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ban {
	pub id: u32,
	pub ban_type: String,
	pub expires_on: String,
	pub steamid64: String,
	pub player_name: String,
	pub steam_id: String,
	pub notes: String,
	pub stats: String,
	pub server_id: u32,
	pub updated_by_id: String,
	pub created_on: String,
	pub updated_on: String,
}

api_response!(Ban);
