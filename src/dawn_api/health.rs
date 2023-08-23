//! GlobalAPI Health Dashboard: <https://health.global-api.com/endpoints/_globalapi>
//!
//! This module gives programmatic access to this endpoint.

use crate::{http, Result};

/// Returns true if DawnAPI is online.
pub async fn healthcheck(client: &http::Client) -> Result<bool> {
	let response = http::get! {
		url = "https://dawn.sh/";
		client = client;
	}?;

	Ok(response == "balls")
}
