//! Module containing HTTP functionality to interact with various APIs.

pub mod serde;
pub use reqwest::{self, Client, StatusCode};

macro_rules! get {
	(
		url = $url:expr;
		$( params = $params:expr; )?
		$( deserialize = $response:ty; )?
		client = $client:expr;
	) => {{
		let params = $crate::http::get!(__params $(, $params)?);
		let request = $crate::http::get!(__req, $url, params, $client);
		let request = $crate::http::get!(__process, request, $($response)?);
		$crate::http::get!(__finish, request)
	}};

	(__params, $params:expr) => {
		$params
	};

	(__params) => {
		&::serde_json::json!({})
	};

	(__req, $url:expr, $params:expr, $client:expr) => {
		$client.get($url)
			.query($params)
			.send()
			.await
			.map_err(|err| {
				let code = err.status();
				let message = err.to_string();
				$crate::Error::Http { code, message }
			})?
	};

	(__process, $request:expr, $type:ty) => {
		$request.json::<$type>()
	};

	(__process, $request:expr) => {
		$request.text()
	};

	(__finish, $response:expr) => {
		$response.await
			.map_err(|err| {
				$crate::Error::DeserializeResponse(err.to_string())
			})
	};
}

pub(crate) use get;
