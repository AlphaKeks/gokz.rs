use crate::{global_api::GlobalAPI, prelude::*};

#[allow(dead_code)]
/// Route: `/jumpstats/{jump_type}/top`
/// - `jump_type`: not documented anywhere.
/// - Lets you fetch the top "global" jumpstats from legacy KZTimer servers
pub(super) async fn get(
	params: super::Params,
	jump_type: u8,
	client: &crate::Client,
) -> Result<Vec<super::Response>, Error> {
	let route = format!("/jumpstats/{jump_type}/top");
	match GlobalAPI::get_raw::<Vec<super::Response>, super::Params>(&route, params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.is_empty() {
				Err(Error { kind: ErrorKind::NoData, msg: String::from("No jumpstats found.") })
			} else {
				Ok(response)
			}
		},
	}
}
