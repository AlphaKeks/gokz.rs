#![allow(unused_imports, unused_variables, dead_code)]

mod global_api;
mod kzgo;
mod util;

use crate::global_api::*;
use crate::util::*;

async fn api_request<T>(path: String, params: Vec<(&str, String)>) -> Result<T, GOKZError>
where
	T: serde::de::DeserializeOwned,
{
	let url = format!("https://kztimerglobal.com/api/v2/{path}");
	let url = match reqwest::Url::parse_with_params(&url, params) {
		Ok(url) => url,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Conversion,
				tldr: String::from("Invalid params."),
				raw: why.to_string(),
			})
		}
	};

	let client = reqwest::Client::new();
	let request = match client.get(url).send().await {
		Ok(data) => data,
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::GlobalAPI,
				tldr: String::from("GlobalAPI request failed."),
				raw: why.to_string(),
			})
		}
	};

	match request.json::<T>().await {
		Ok(json) => Ok(json),
		Err(why) => {
			return Err(GOKZError {
				r#type: GOKZErrorType::Conversion,
				tldr: String::from("Failed to parse to JSON."),
				raw: why.to_string(),
			})
		}
	}
}

pub async fn get_map(identifier: GOKZMapIdentifier) -> Result<GOKZMap, GOKZError> {
	let mut params = vec![("is_validated", true.to_string()), ("limit", 1.to_string())];

	let map = match identifier {
		GOKZMapIdentifier::Name(name) => ("name", name),
		GOKZMapIdentifier::Id(id) => ("id", id.to_string()),
	};

	params.push((map.0, map.1));

	match api_request::<Vec<GOKZMap>>(String::from("maps?"), params).await {
		Ok(mut maps) => Ok(maps.remove(0)),
		Err(why) => Err(why),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	pub async fn test_get_map() {
		let lionharder = GOKZMap {
			id: 992,
			name: String::from("kz_lionharder"),
			filesize: 100007576,
			validated: true,
			difficulty: 7,
			created_on: String::from("2021-06-05T15:52:16"),
			updated_on: String::from("2021-06-05T15:52:16"),
			approved_by_steamid64: String::from("76561198143205331"),
			workshop_url: String::from(
				"https://steamcommunity.com/sharedfiles/filedetails/?id=2420807980",
			),
			download_url: None,
		};

		let result_name = get_map(GOKZMapIdentifier::Name(String::from("kz_lionharder"))).await;
		let result_id = get_map(GOKZMapIdentifier::Id(992)).await;

		assert_eq!(Ok(lionharder.clone()), result_name);
		assert_eq!(Ok(lionharder), result_id);
	}
}
