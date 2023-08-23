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
		let request = $crate::http::get!(__process, request $(, $response)?);
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
		// async {
		// 	::serde_json::from_value::<$type>(
		// 		dbg!($request.json::<::serde_json::Value>().await.unwrap())
		// 	)
		// }

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

#[cfg(feature = "global-api")]
macro_rules! append_pairs {
	($url:expr, $value:expr, $name:expr) => {{
		if let Some(items) = $value {
			let mut query = $url.query_pairs_mut();
			for item in items {
				query.append_pair($name, &item.to_string());
			}
		}
	}};
}

#[cfg(feature = "global-api")]
pub(crate) use append_pairs;
