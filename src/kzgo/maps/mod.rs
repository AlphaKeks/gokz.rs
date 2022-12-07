use crate::prelude::*;

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// This function will make an API request to KZ:GO to gather data about a specified map
pub async fn get_map(
	map_identifier: &MapIdentifier,
	client: &reqwest::Client,
) -> Result<Response, Error> {
	use reqwest::StatusCode;

	log::info!("get_map() => Function Input {{ map_identifier: {} }}", map_identifier);

	match client
		.get(format!(
			"https://kzgo.eu/api/maps/{}",
			match map_identifier {
				MapIdentifier::Name(map_name) => map_name,
				MapIdentifier::ID(_map_id) =>
					return Err(Error {
						kind: ErrorKind::Input,
						origin: String::from("gokz_rs::kzgo::maps::get_map"),
						tldr: String::from("You can only use map names for this function."),
						raw: None
					}),
			}
		))
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
							origin: String::from("gokz_rs::kzgo::maps::get_map"),
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
				origin: String::from("gokz_rs::kzgo::maps::get_map"),
				tldr: String::from("KZ:GO API request failed."),
				raw: Some(why.to_string()),
			});
		},
	}
}

#[cfg(test)]
#[tokio::test]
#[ignore = "expensive"]
async fn kzgo_get_map_test() {
	let client = reqwest::Client::new();

	match get_map(&MapIdentifier::ID(992), &client).await {
		Err(why) => println!("Test failed successfully: {:#?}", why),
		Ok(how) => panic!("the fuck {:#?}", how),
	}

	match get_map(&MapIdentifier::Name(String::from("kz_lionharder")), &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(map) => println!("Test succuessful: {:#?}", map),
	}
}
