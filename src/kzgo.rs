pub mod completion {
	use serde::{Deserialize, Serialize};

	use crate::prelude::*;

	#[derive(Debug, Serialize, Deserialize)]
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

	#[derive(Debug, Serialize, Deserialize)]
	pub struct CompletionStats {
		pub _id: String,
		pub mode: String,
		pub pro: CompletionCount,
		pub tp: CompletionCount,
	}

	pub async fn get_completion_count(mode: &Mode, client: &reqwest::Client) -> Result<CompletionStats, Error> {
		match client
			.get(format!("https://kzgo.eu/api/completions/{}", mode.as_str()))
			.send()
			.await
		{
			Ok(data) => match data.json::<CompletionStats>().await {
				Ok(json) => return Ok(json),
				Err(why) => {
					return Err(Error {
						kind: ErrorKind::Parsing,
						tldr: "Failed to parse JSON.",
						raw: Some(why.to_string()),
					})
				}
			},
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::KZGO,
					tldr: "KZ:GO API Request failed.",
					raw: Some(why.to_string()),
				})
			}
		}
	}
}
