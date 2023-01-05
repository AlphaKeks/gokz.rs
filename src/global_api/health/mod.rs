use {
	crate::prelude::*,
	log::{debug, info, trace, warn},
};

/// GlobalAPI Health Report
///
/// NOTE: [source](https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1)
pub async fn get(client: &crate::Client) -> Result<HealthReport, Error> {
	let response = match client
		.get("https://health.global-api.com/api/v1/endpoints/_globalapi/statuses?page=1")
		.send()
		.await
	{
		Err(why) => {
			warn!("[GlobalAPI::checkhealth] HTTPS Request failed.");
			if let Some(code) = why.status() {
				warn!("[GlobalAPI::checkhealth] Request failed with status code `{}`.", &code);
				return Err(Error {
					kind: ErrorKind::GlobalAPI {
						status_code: Some(code.to_string()),
						raw_message: Some(why.to_string()),
					},
					msg: format!("GlobalAPI request failed with code `{}`.", code),
				});
			}

			warn!("[GlobalAPI::checkhealth] Request failed with no status code.");
			return Err(Error {
				kind: ErrorKind::GlobalAPI {
					status_code: None,
					raw_message: Some(why.to_string()),
				},
				msg: String::from(
					"GlobalAPI request failed, but no status code has been returned.",
				),
			});
		},
		Ok(response) => match response.error_for_status() {
			Err(why) => {
				let Some(code) = why.status() else {
					warn!("[GlobalAPI::checkhealth] Request failed with no status code.");
					return Err(Error {
						kind: ErrorKind::GlobalAPI {
							status_code: None,
							raw_message: Some(why.to_string()),
						},
						msg: String::from(
							"GlobalAPI request failed, but no status code has been returned.",
						),
					});
				};

				warn!("[GlobalAPI::checkhealth] Request failed with status code `{}`.", &code);
				return Err(Error {
					kind: ErrorKind::GlobalAPI {
						status_code: Some(code.to_string()),
						raw_message: Some(why.to_string()),
					},
					msg: format!("GlobalAPI request failed with code `{}`.", code),
				});
			},
			Ok(response) => {
				trace!(
					"[GlobalAPI::checkhealth] GlobalAPI responded successfully with code `{}`.",
					response.status()
				);
				response
			},
		},
	};

	// parse the response into the desired `Response` format
	let parsed_response = match response.json::<HealthReport>().await {
		Err(why) => {
			warn!("[GlobalAPI::checkhealth] Failed to parse response.");
			warn!("[GlobalAPI::checkhealth] {:?}", why);

			return Err(Error {
				kind: ErrorKind::Parsing { expected: String::from("JSON"), got: None },
				msg: String::from("Failed to parse GlobalAPI response."),
			});
		},
		Ok(parsed_response) => {
			trace!("[GlobalAPI::checkhealth] Successfully parsed response.");
			parsed_response
		},
	};

	info!("[GlobalAPI::checkhealth] completed successfully.");
	debug!("[GlobalAPI::checkhealth] Response: {:?}", &parsed_response);

	// return the `Response`
	Ok(parsed_response)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConditionResult {
	pub condition: String,
	pub success: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResult {
	pub status: u64,
	pub hostname: String,
	pub duration: u64,
	pub condition_results: [ConditionResult; 2],
	pub success: bool,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusEvent {
	pub r#type: String,
	pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthReport {
	pub name: String,
	pub key: String,
	pub results: Vec<StatusResult>,
	pub events: Vec<StatusEvent>,
}
