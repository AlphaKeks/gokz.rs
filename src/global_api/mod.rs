#![allow(dead_code)]
use crate::prelude::*;

pub mod bans;
pub mod maps;

const BASE_URL: &'static str = "https://kztimerglobal.com/api/v2";
trait IsResponse {}
trait IsParams {}

/// The base function that everything else relies on. Every function in this
/// module will at some point call this base function to call the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
///
/// [`api_request`] will try to make an HTTPS request to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and
/// parse the response into a struct.
async fn api_request<'a, T, P>(
	route: &'a str,
	params: P,
	client: &reqwest::Client,
) -> Result<T, Error>
where
	T: serde::de::DeserializeOwned + IsResponse,
	P: serde::Serialize + IsParams,
{
	match client
		.get(BASE_URL.to_owned() + route)
		.query(&params)
		.send()
		.await
	{
		Err(why) => Err(Error {
			kind: ErrorKind::GlobalAPI,
			origin: String::from("gokz_rs::global_api::api_request"),
			tldr: String::from("GlobalAPI request failed."),
			raw: Some(why.to_string()),
		}),
		Ok(response) => match response.json::<T>().await {
			Err(why) => Err(Error {
				kind: ErrorKind::Parsing,
				origin: String::from("gokz_rs::global_api::api_request"),
				tldr: String::from("Failed to parse JSON."),
				raw: Some(why.to_string()),
			}),
			Ok(parsed_response) => Ok(parsed_response),
		},
	}
}

/// This function will request [all of a player's bans](`crate::global_api::bans::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and return them.
/// If there are no bans the function will return an [`Error`].
pub async fn get_bans(
	steam_id: SteamID,
	client: &reqwest::Client,
) -> Result<Vec<bans::Response>, Error> {
	let params = bans::Params {
		steam_id: Some(steam_id.0),
		..Default::default()
	};

	match api_request::<Vec<bans::Response>, bans::Params>("/bans?", params, client).await {
		Err(why) => Err(why),
		Ok(response) => {
			if response.len() < 1 {
				Err(Error {
					kind: ErrorKind::NoData,
					origin: String::from("gokz_rs::global_api::get_ban"),
					tldr: String::from("No bans found."),
					raw: None,
				})
			} else {
				Ok(response)
			}
		}
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_bans_test() {
	let client = reqwest::Client::new();

	let no_bans = SteamID(String::from("STEAM_1:0:165881949"));

	match get_bans(no_bans, &client).await {
		Err(why) => println!("Test successful: {:#?}", why),
		Ok(bans) => panic!("Test failed: {:#?}", bans),
	}

	let bans = SteamID(String::from("STEAM_1:1:161178172"));

	match get_bans(bans, &client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(bans) => println!("Test successful: {:#?}", bans),
	}
}

/// This function will request [all maps](`crate::global_api::bans::Response`) from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) which are marked as `validated` and return them.
/// If there are no maps the function will return an [`Error`]. (very unlikely)
pub async fn get_maps(client: &reqwest::Client) -> Result<Vec<maps::Response>, Error> {
	let params = maps::Params {
		is_validated: Some(true),
		..Default::default()
	};

	match api_request::<Vec<maps::Response>, maps::Params>("/maps?", params, client).await {
		Err(why) => Err(why),
		Ok(maps) => {
			if maps.len() < 1 {
				Err(Error {
					kind: ErrorKind::GlobalAPI,
					origin: String::from("gokz_rs::global_api::get_maps"),
					tldr: String::from("No maps found."),
					raw: None,
				})
			} else {
				Ok(maps)
			}
		}
	}
}

#[cfg(test)]
#[tokio::test]
async fn get_maps_test() {
	let client = reqwest::Client::new();

	match get_maps(&client).await {
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(maps) => println!("Test successful: {} maps", maps.len()),
	}
}
