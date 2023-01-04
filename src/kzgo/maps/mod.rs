use {super::KZGO, crate::prelude::*};

/// Route: `/maps/{map_name}`
/// - `map_name`: any of [these](https://maps.global-api.com/mapcycles/gokz.txt)
/// - Lets you fetch a map from the KZ:GO API
pub async fn get(map_name: &str, client: &crate::Client) -> Result<Response, Error> {
	let route = format!("/maps/{}", map_name);
	KZGO::get(&route, client).await
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
	pub _id: Option<String>,
	pub name: Option<String>,
	pub id: Option<i16>,
	pub tier: Option<u8>,
	pub workshopId: Option<String>,
	pub bonuses: Option<u8>,
	pub sp: Option<bool>,
	pub vp: Option<bool>,
	pub mapperNames: Option<Vec<String>>,
	pub mapperIds: Option<Vec<String>>,
	pub date: Option<String>,
}
