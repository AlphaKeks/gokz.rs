use {
	super::{api_params, api_response, GlobalAPI, GlobalAPIParams, GlobalAPIResponse},
	crate::prelude::*,
};

/// Route: `/record_filters`
/// - Lets you fetch record filters for individual courses
pub async fn get(params: Params, client: &crate::Client) -> Result<Vec<RecordFilter>, Error> {
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
pub struct RecordFilter {
	pub id: i32,
	pub map_id: i32,
	pub stage: i32,
	pub mode_id: i32,
	pub tickrate: i32,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_by_id: String,
}

api_response!(RecordFilter);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {
	pub ids: Option<i32>,
	pub map_ids: Option<i32>,
	pub stages: Option<i32>,
	pub mode_ids: Option<i32>,
	pub tickrates: Option<i32>,
	pub has_teleports: Option<bool>,
	pub offset: Option<i32>,
	pub limit: Option<u32>,
}

api_params!(Params);
