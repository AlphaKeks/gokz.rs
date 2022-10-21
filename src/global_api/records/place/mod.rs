pub fn get_url(record_id: &u32) -> String {
	format!("records/place/{record_id}")
}

#[derive(Debug, serde::Serialize)]
/// All possible parameters for the `records/place/{record_id}` route
pub struct Params;

impl Default for Params {
	fn default() -> Self {
		Params
	}
}

impl super::super::IsParams for Params {}

#[derive(Debug, serde::Deserialize, Clone)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on the `/records/place/{record_id}` route
pub struct Response(pub u32);

impl super::super::IsResponse for Response {}
impl super::super::IsResponse for Vec<Response> {}
