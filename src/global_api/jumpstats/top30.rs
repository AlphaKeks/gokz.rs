use crate::{
	global_api::{api_params, GlobalAPI, GlobalAPIParams},
	prelude::*,
};

#[allow(dead_code)]
/// Route: `/jumpstats/{jump_type}/top30`
/// - `jump_type`: not documented anywhere.
/// - Note: The last time I tried using this route it didn't work.
pub async fn get(
	params: Params,
	jump_type: u8,
	client: &crate::Client,
) -> Result<Vec<super::Response>, Error> {
	let route = format!("/jumpstats/{jump_type}/top30");
	match GlobalAPI::get::<Vec<_>, _>(&route, params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error {
					kind: ErrorKind::NoData { expected: String::from("top 30 jumpstats") },
					msg: String::from("No jumpstats found."),
				})
			} else {
				Ok(response)
			}
		},
	}
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params;
api_params!(Params);
