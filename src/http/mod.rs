mod status_code;
pub use status_code::StatusCode;

use {
	crate::{Error, Result},
	log::{debug, error, trace},
	reqwest::Url,
	serde::{de::DeserializeOwned, Serialize},
	std::fmt::Debug,
};

#[derive(Debug, Clone, Copy, Serialize)]
struct EmptyParams;

async fn _get<Params>(
	url: &str,
	params: Params,
	client: &reqwest::Client,
) -> Result<reqwest::Response>
where
	Params: Debug + Serialize,
{
	trace!("Making an HTTP request to `{url}`.");
	debug!("Params: {params:#?}");

	let Ok(url) = Url::parse(url) else {
		error!("Failed to parse `{url}` as URL.");
		return Err(Error::InvalidUrl { value: String::from(url) });
	};
	debug!("Successfully parsed URL `{url}`");

	Ok(client
		.get(url)
		.query(&params)
		.send()
		.await?
		.error_for_status()?)
}

/// Makes an HTTP GET request without any params and parses the response as Json.
pub async fn get<Response>(url: &str, client: &reqwest::Client) -> Result<Response>
where
	Response: Debug + DeserializeOwned,
{
	Ok(_get(url, EmptyParams, client)
		.await?
		.json::<Response>()
		.await?)
}

/// Makes an HTTP GET request without any params and parses the response as Text.
pub async fn get_text(url: &str, client: &reqwest::Client) -> Result<String> {
	Ok(_get(url, EmptyParams, client)
		.await?
		.text()
		.await?)
}

/// Makes an HTTP GET request params and parses the response as Json.
pub async fn get_with_params<Params, Response>(
	url: &str,
	params: Params,
	client: &reqwest::Client,
) -> Result<Response>
where
	Params: Debug + Serialize,
	Response: Debug + DeserializeOwned,
{
	Ok(_get(url, params, client)
		.await?
		.json::<Response>()
		.await?)
}

/// Makes an HTTP GET request params and parses the response as Text.
pub async fn get_text_with_params<Params>(
	url: &str,
	params: Params,
	client: &reqwest::Client,
) -> Result<String>
where
	Params: Debug + Serialize,
{
	Ok(_get(url, params, client)
		.await?
		.text()
		.await?)
}
