pub fn get_url(mode: &crate::prelude::Mode) -> String {
	format!("modes/id/{}", mode.as_id())
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `/modes/id` route
pub struct Params;

impl Default for Params {
	fn default() -> Self {
		Params
	}
}

impl super::super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/modes/id` route
pub struct Response {
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

impl super::super::IsResponse for Response {}
impl super::super::IsResponse for Vec<Response> {}
