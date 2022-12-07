pub mod id;
pub mod name;

/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url() -> String {
	String::from("modes?")
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct APIMode {
	pub id: u8,
	pub name: String,
	pub description: String,
	pub latest_version: u8,
	pub latest_version_description: String,
	pub website: String,
	pub repo: String,
	pub contact_steamid64: String,
	pub supported_tickrates: Option<u8>,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by_id: String,
}

impl super::IsResponse for APIMode {}
impl super::IsResponse for Vec<APIMode> {}
