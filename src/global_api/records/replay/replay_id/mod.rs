pub fn get_url(replay_id: u32) -> String {
	format!("records/replay/{replay_id}")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `records/replay/{replay_id}` route
pub struct Params;

impl Default for Params {
	fn default() -> Self {
		Params
	}
}

impl super::super::super::IsParams for Params {}

#[derive(Debug, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/records/replay/{replay_id}` route
pub struct Response(pub String);

impl super::super::super::IsResponse for Response {}
impl super::super::super::IsResponse for Vec<Response> {}
