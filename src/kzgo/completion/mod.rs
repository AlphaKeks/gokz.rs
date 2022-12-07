use crate::prelude::*;

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

/// This function will make an API request to KZ:GO to gather an
/// overview about how many maps are possible in which tier and mode.
pub async fn get_completion_count(
	mode: &Mode,
	client: &reqwest::Client,
) -> Result<Response, Error> {
	use reqwest::StatusCode;

	log::info!("get_completion_count() => Function Input {{ mode: {} }}", mode);

	match client
		.get(format!("https://kzgo.eu/api/completions/{}", mode.as_str()))
		.send()
		.await
	{
		Ok(response) => {
			log::trace!("Successful KZ:GO request");
			log::trace!("Response: {:?}", &response);
			match response.status() {
				StatusCode::OK => match response.json::<Response>().await {
					Ok(parsed_response) => {
						log::info!("Successfully parsed KZ:GO response.");
						log::info!("Parsed Response: {:?}", &parsed_response);
						return Ok(parsed_response);
					},
					Err(why) => {
						log::warn!("Failed parsing KZ:GO response.");
						return Err(Error {
							kind: ErrorKind::Parsing,
							origin: String::from("gokz_rs::kzgo::completion::get_completion_count"),
							tldr: String::from("Failed to parse JSON."),
							raw: Some(why.to_string()),
						});
					},
				},
				code => {
					log::warn!("Got a response from the KZ:GO API, but not an `OK` Code.");
					log::warn!("Code: {}", &code);
					return Err(Error {
						kind: ErrorKind::KZGO,
						origin: String::from("gokz_rs::kzgo::completion::get_completion_count"),
						tldr: String::from("KZ:GO API request failed."),
						raw: Some(code.to_string()),
					});
				},
			}
		},
		Err(why) => {
			log::warn!("Failed KZ:GO request");
			log::warn!("Error: {}", why);
			return Err(Error {
				kind: ErrorKind::KZGO,
				origin: String::from("gokz_rs::kzgo::completion::get_completion_count"),
				tldr: String::from("KZ:GO API request failed."),
				raw: Some(why.to_string()),
			});
		},
	}
}
