use {
	crate::{
		error::{Error, Result},
		http::get_json,
		prelude,
		schnose_api::BASE_URL,
		utils::EmptyParams,
	},
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mode {
	pub id: u8,
	pub name: String,
}

impl crate::traits::Mode for Mode {
	#[inline]
	fn api(&self) -> String {
		prelude::Mode::try_from(self.id)
			.expect("Mode returned by the GlobalAPI should be a valid `Mode` type.")
			.api()
	}

	#[inline]
	fn short(&self) -> String {
		prelude::Mode::try_from(self.id)
			.expect("Mode returned by the GlobalAPI should be a valid `Mode` type.")
			.short()
	}
}

/// # /modes
///
/// Fetches all modes
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn root(client: &crate::Client) -> Result<Vec<Mode>> {
	let response: Vec<_> = get_json(&format!("{BASE_URL}/modes"), &EmptyParams, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response)
}

/// # /modes/:ident
///
/// Fetches a single mode
#[tracing::instrument(level = "TRACE", skip(client), err(Debug))]
pub async fn ident(mode: prelude::Mode, client: &crate::Client) -> Result<Mode> {
	get_json(&format!("{BASE_URL}/modes/{}", mode as u8), &EmptyParams, client).await
}
