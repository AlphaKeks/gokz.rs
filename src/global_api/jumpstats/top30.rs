use crate::{
	global_api::{GlobalAPI, GlobalAPIParams},
	prelude::*,
};

#[allow(dead_code)]
/// Route: `/jumpstats/{jump_type}/top30`
/// - `jump_type`: not documented anywhere.
/// - Note: The last time I tried using this route it didn't work.
pub(super) async fn get(
	params: Params,
	jump_type: u8,
	client: &crate::Client,
) -> Result<Vec<super::Response>, Error> {
	let route = format!("/jumpstats/{jump_type}/top30");
	match GlobalAPI::get_raw::<Vec<super::Response>, Params>(&route, params, client).await {
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

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Params {}
impl GlobalAPIParams for Params {}
