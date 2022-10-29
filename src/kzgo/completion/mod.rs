use crate::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response {
	pub _id: String,
	pub mode: String,
	pub pro: CompletionCount,
	pub tp: CompletionCount,
}

/// This function will make an API request to KZ:GO to gather an
/// overview about how many maps are possible in which tier and mode.
pub async fn get_completion_count(
	mode: &Mode,
	client: &reqwest::Client,
) -> Result<Response, Error> {
	match client
		.get(format!("https://kzgo.eu/api/completions/{}", mode.as_str()))
		.send()
		.await
	{
		Err(why) => {
			return Err(Error {
				kind: ErrorKind::KZGO,
				origin: String::from("gokz_rs::kzgo::completion::get_completion_count"),
				tldr: String::from("KZ:GO API request failed."),
				raw: Some(why.to_string()),
			})
		},
		Ok(data) => match data.json::<Response>().await {
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::Parsing,
					origin: String::from("gokz_rs::kzgo::completion::get_completion_count"),
					tldr: String::from("Failed to parse JSON."),
					raw: Some(why.to_string()),
				})
			},
			Ok(json) => Ok(json),
		},
	}
}
