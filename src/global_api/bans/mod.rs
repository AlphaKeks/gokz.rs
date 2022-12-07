/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("bans?")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// All possible parameters for this route
pub struct BanParams {
	pub ban_types: Option<String>,
	pub ban_types_list: Option<Vec<String>>,
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

impl Default for BanParams {
	fn default() -> Self {
		BanParams {
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
			limit: Some(99),
		}
	}
}

impl super::IsParams for BanParams {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct Ban {
	pub id: u32,
	pub ban_type: String,
	pub expires_on: String,
	pub steamid64: String,
	pub player_name: String,
	pub steam_id: String,
	pub notes: String,
	pub stats: String,
	pub server_id: u16,
	pub updated_by_id: String,
	pub created_on: String,
	pub updated_on: String,
}

impl super::IsResponse for Ban {}
impl super::IsResponse for Vec<Ban> {}
