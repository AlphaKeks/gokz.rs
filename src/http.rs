use {
	crate::error::{Error, Result},
	reqwest::Url,
	serde::{de::DeserializeOwned, Serialize},
};

#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn get_json<T, P>(url: &str, params: &P, client: &crate::Client) -> Result<T>
where
	T: DeserializeOwned + std::fmt::Debug,
	P: Serialize + std::fmt::Debug,
{
	Ok(get(url, params, client).await?.json().await?)
}

#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn get_text<P>(url: &str, params: &P, client: &crate::Client) -> Result<String>
where
	P: Serialize + std::fmt::Debug,
{
	Ok(get(url, params, client).await?.text().await?)
}

#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn get<P>(url: &str, params: &P, client: &crate::Client) -> Result<reqwest::Response>
where
	P: Serialize + std::fmt::Debug,
{
	let Ok(url) = Url::parse(url) else {
        return Err(Error::InvalidUrl(url.to_owned()));
    };

	let res = client.get(url).query(params).send().await?.error_for_status()?;

	Ok(res)
}
