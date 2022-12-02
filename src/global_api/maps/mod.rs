/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("maps?")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for this route
pub struct MapParams {
	pub id: Option<i16>,
	pub name: Option<String>,
	pub larger_than_filesize: Option<u32>,
	pub smaller_than_filesize: Option<u32>,
	pub is_validated: Option<bool>,
	pub difficulty: Option<u8>,
	pub created_since: Option<String>,
	pub updated_since: Option<String>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

impl Default for MapParams {
	fn default() -> Self {
		MapParams {
			id: None,
			name: None,
			larger_than_filesize: None,
			smaller_than_filesize: None,
			is_validated: None,
			difficulty: None,
			created_since: None,
			updated_since: None,
			offset: None,
			limit: Some(9999),
		}
	}
}

impl super::IsParams for MapParams {}

#[derive(Debug, serde::Deserialize, Clone)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct KZMap {
	pub id: i16,
	pub name: String,
	pub filesize: u64,
	pub validated: bool,
	pub difficulty: u8,
	pub created_on: String,
	pub updated_on: String,
	pub approved_by_steamid64: String,
	pub workshop_url: String,
	pub download_url: Option<String>,
}

impl super::IsResponse for KZMap {}
impl super::IsResponse for Vec<KZMap> {}
