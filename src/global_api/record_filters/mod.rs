use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/record_filters`
/// - Lets you fetch record filters for individual courses
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<Response>, Error> {
	match GlobalAPI::get::<Vec<_>, _>("/record_filters?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("Vec<RecordFilter>") },
					msg: String::from("No filters found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
	pub id: u32,
	pub map_id: u32,
	pub stage: u8,
	pub mode_id: u8,
	pub tickrate: u8,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_by_id: String,
}

api_response!(Response);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub ids: Option<u32>,
	pub map_ids: Option<u32>,
	pub stages: Option<u8>,
	pub mode_ids: Option<u8>,
	pub tickrates: Option<u8>,
	pub has_teleports: Option<bool>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);

impl Default for Params {
	fn default() -> Self {
		Self {
			ids: None,
			map_ids: None,
			stages: None,
			mode_ids: None,
			tickrates: None,
			has_teleports: None,
			offset: None,
			limit: Some(1),
		}
	}
}
