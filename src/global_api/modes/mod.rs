pub mod id;
pub mod name;

use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/modes`
/// - Lets you fetch all modes stored in the GlobalAPI
pub async fn get(client: &crate::Client) -> Result<Vec<APIMode>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/modes?", Params::default(), client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("Vec<Mode>") },
					msg: String::from("No modes found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct APIMode {
	pub id: i32,
	pub name: String,
	pub description: String,
	pub latest_version: i32,
	pub latest_version_description: String,
	pub website: String,
	pub repo: String,
	pub contact_steamid64: String,
	pub supported_tickrates: Option<i8>,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by_id: String,
}

api_response!(APIMode);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
