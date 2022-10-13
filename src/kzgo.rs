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
						tldr: String::from("Failed to parse JSON."),
						raw: Some(why.to_string()),
					})
				}
			},
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::KZGO,
					tldr: String::from("KZ:GO API Request failed."),
					raw: Some(why.to_string()),
				})
			}
		}
	}
}

pub mod maps {
	use serde::{Deserialize, Serialize};

	use crate::prelude::*;

	#[allow(non_snake_case)]
	#[derive(Debug, Serialize, Deserialize)]
	pub struct Map {
		pub _id: String,
		pub name: String,
		pub id: u16,
		pub tier: u8,
		pub workshopId: String,
		pub bonuses: u8,
		pub sp: bool,
		pub vp: bool,
		pub mapperNames: Vec<String>,
		pub mapperIds: Vec<String>,
		pub date: String,
	}

	pub async fn get_map(map: &MapIdentifier, client: &reqwest::Client) -> Result<Map, Error> {
		let map = match map {
			MapIdentifier::Name(name) => name,
			MapIdentifier::Id(_) => {
				return Err(Error {
					kind: ErrorKind::InvalidInput,
					tldr: String::from("Please do not use an ID for this function."),
					raw: None,
				})
			}
		};
		match client.get(format!("https://kzgo.eu/api/maps/{}", map)).send().await {
			Ok(data) => match data.json::<Map>().await {
				Ok(json) => return Ok(json),
				Err(why) => {
					return Err(Error {
						kind: ErrorKind::Parsing,
						tldr: String::from("Failed to parse JSON."),
						raw: Some(why.to_string()),
					})
				}
			},
			Err(why) => {
				return Err(Error {
					kind: ErrorKind::KZGO,
					tldr: String::from("KZ:GO API Request failed."),
					raw: Some(why.to_string()),
				})
			}
		}
	}
}
