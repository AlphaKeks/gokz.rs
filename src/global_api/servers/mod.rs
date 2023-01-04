pub mod id;
pub mod name;

use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/servers`
/// - Lets you fetch information about global servers
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<Server>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/servers?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("Vec<Server>") },
					msg: String::from("No servers found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Server {
	pub id: u32,
	pub port: u32,
	pub ip: String,
	pub name: String,
	pub owner_steamid64: String,
}

api_response!(Server);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub id: Option<u32>,
	pub port: Option<u32>,
	pub ip: Option<String>,
	pub name: Option<String>,
	pub owner_steamid64: Option<u64>,
	pub approval_status: Option<i32>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);
