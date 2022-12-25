pub mod id;
pub mod name;

use {super::GlobalAPI, crate::prelude::*};

/// Route: `/maps`
/// - Lets you fetch all maps stored in the GlobalAPI
pub(super) async fn get(params: Params, client: &crate::Client) -> Result<Vec<Response>, Error> {
	match GlobalAPI::get_raw::<Vec<Response>, Params>("/maps?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error { kind: ErrorKind::NoData, msg: String::from("No maps found.") })
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
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
impl super::GlobalAPIResponse for Response {}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub id: Option<u16>,
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
impl super::GlobalAPIParams for Params {}
