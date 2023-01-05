use {
	crate::prelude::*,
	log::{debug, info, trace, warn},
};

pub mod maps;
use maps::Response as MapResponse;

pub mod completions;
use completions::Response as CompletionResponse;

/// Methods for [KZ:GO](https://kzgo.eu/)'s API
pub struct KZGO;
impl KZGO {
	pub const BASE_URL: &str = "https://kzgo.eu/api";

	pub async fn get<Response>(route: &str, client: &crate::Client) -> Result<Response, Error>
	where
		Response: std::fmt::Debug + serde::de::DeserializeOwned,
	{
		info!("[KZGO::get] starting...");
		debug!("[KZGO::get] `route`: {:?}", route);

		// construct full URL
		// e.g. `https://kzgo.eu/api/maps/kz_lionharder`
		let full_route = format!("{}{}", Self::BASE_URL, route);
		let url = match reqwest::Url::parse(&full_route) {
			Err(why) => {
				warn!("[KZGO::get] Failed to parse URL: {:?}", why);
				return Err(Error {
					kind: ErrorKind::Parsing {
						expected: String::from("valid URL"),
						got: Some(full_route),
					},
					msg: String::from("Failed to parse URL."),
				});
			},
			Ok(url) => {
				debug!("[KZGO::get] Successfully constructed URL `{}`.", &url);
				url
			},
		};

		// make a GET request to the KZ:GO API
		let response = match client.get(url).send().await {
			Err(why) => {
				warn!("[KZGO::get] HTTPS Request failed.");
				if let Some(code) = why.status() {
					warn!("[KZGO::get] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::KZGO {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("KZ:GO API request failed with code `{}`.", code),
					});
				}

				warn!("[KZGO::get] Request failed with no status code.");
				return Err(Error {
					kind: ErrorKind::KZGO { status_code: None, raw_message: Some(why.to_string()) },
					msg: String::from(
						"KZ:GO API request failed, but no status code has been returned.",
					),
				});
			},
			Ok(response) => match response.error_for_status() {
				Err(why) => {
					let Some(code) = why.status() else {
						warn!("[KZGO::get] Request failed with no status code.");
						return Err(Error {
							kind: ErrorKind::KZGO { status_code: None, raw_message: Some(why.to_string()) },
						msg: String::from("KZ:GO API request failed, but no status code has been returned.")
						});
					};

					warn!("[KZGO::get] Request failed with status code `{}`.", &code);
					return Err(Error {
						kind: ErrorKind::KZGO {
							status_code: Some(code.to_string()),
							raw_message: Some(why.to_string()),
						},
						msg: format!("KZ:GO API request failed with code `{}`.", code),
					});
				},
				Ok(response) => {
					trace!(
						"[KZGO::get] API responded successfully with code `{}`.",
						response.status()
					);
					response
				},
			},
		};

		// parse the response into the desired `Response` format
		let parsed_response = match response.json::<Response>().await {
			Err(why) => {
				warn!("[KZGO::get] Failed to parse response.");
				warn!("[KZGO::get] {:?}", why);

				return Err(Error {
					kind: ErrorKind::Parsing { expected: String::from("JSON"), got: None },
					msg: String::from("Failed to parse KZ:GO API response"),
				});
			},
			Ok(parsed_response) => {
				trace!("[KZGO::get] Successfully parsed response.");
				parsed_response
			},
		};

		info!("[KZGO::get] completed successfully.");
		debug!("[KZGO::get] Response: {:?}", &parsed_response);

		// return the `Response`
		Ok(parsed_response)
	}

	/// Route: `/maps/{map_name}`
	/// - `map_name`: any of [these](https://maps.global-api.com/mapcycles/gokz.txt)
	/// - Lets you fetch a map from the KZ:GO API
	pub async fn get_map(map_name: &str, client: &crate::Client) -> Result<MapResponse, Error> {
		maps::get(map_name, client).await
	}

	/// Route: `/completions/{mode_name}`
	/// - `mode_name`: obtainable via [this method](crate::prelude::Mode::api)
	/// - Lets you fetch the amount of completable maps for a given mode
	pub async fn get_completion_count(
		mode: Mode,
		client: &crate::Client,
	) -> Result<CompletionResponse, Error> {
		completions::get(mode, client).await
	}
}
