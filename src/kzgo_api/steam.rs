//! `/steam` endpoint

use {
	super::API_URL,
	crate::{http, Result, SteamID},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
	pub name: String,

	#[serde(rename = "avatar")]
	pub avatar_url: String,
	pub country: String,
}

/// `/steam/:steam_id` route
///
/// Fetches information about a player by [`SteamID`].
#[tracing::instrument(level = "TRACE", skip(client))]
pub async fn get_user(
	steam_id: impl Into<SteamID> + std::fmt::Debug,
	client: &crate::http::Client,
) -> Result<User> {
	http::get! {
		url = format!("{API_URL}/steam/{}", steam_id.into().as_id64());
		deserialize = User;
		client = client;
	}
}
