use {super::KZGO, crate::prelude::*};

/// Route: `/completions/{mode_name}`
/// - `mode_name`: obtainable via [this method](crate::prelude::Mode::api)
/// - Lets you fetch the amount of completable maps for a given mode
pub async fn get(mode: Mode, client: &crate::Client) -> Result<Response, Error> {
	let route = format!("/completions/{}", mode.api());
	KZGO::get(&route, client).await
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CompletionCount {
	#[serde(rename(deserialize = "1"))]
	pub one: u32,
	#[serde(rename(deserialize = "2"))]
	pub two: u32,
	#[serde(rename(deserialize = "3"))]
	pub three: u32,
	#[serde(rename(deserialize = "4"))]
	pub four: u32,
	#[serde(rename(deserialize = "5"))]
	pub five: u32,
	#[serde(rename(deserialize = "6"))]
	pub six: u32,
	#[serde(rename(deserialize = "7"))]
	pub seven: u32,
	pub total: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
	pub _id: String,
	pub mode: String,
	pub pro: CompletionCount,
	pub tp: CompletionCount,
}
