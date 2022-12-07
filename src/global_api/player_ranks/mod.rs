/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("player_ranks?")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// All possible parameters for this route
pub struct PlayerRankParams {
	pub points_greater_than: Option<u32>,
	pub average_greater_than: Option<u32>,
	pub rating_greater_than: Option<u32>,
	pub finishes_greater_than: Option<u32>,
	pub steamid64s: Option<u64>,
	pub record_filter_ids: Option<u32>,
	pub map_ids: Option<u16>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
	#[serde(rename = "camelCase")]
	pub map_tag: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for PlayerRankParams {
	fn default() -> Self {
		PlayerRankParams {
			points_greater_than: None,
			average_greater_than: None,
			rating_greater_than: None,
			finishes_greater_than: None,
			steamid64s: None,
			record_filter_ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: Some(128),
			has_teleports: None,
			map_tag: None,
			offset: None,
			limit: Some(1),
		}
	}
}

impl super::IsParams for PlayerRankParams {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct PlayerRankResponse {
	pub points: u32,
	pub average: u32,
	pub rating: u32,
	pub finishes: u32,
	pub steamid64: String,
	pub steamid: String,
	pub player_name: String,
}

impl super::IsResponse for PlayerRankResponse {}
impl super::IsResponse for Vec<PlayerRankResponse> {}
