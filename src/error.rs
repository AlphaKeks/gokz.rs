use thiserror::Error;

/// # Result type for the entire crate.
///
/// This mainly exists for convenience so that you don't have to write the `, gokz_rs::Error` part
/// of [Result](std::result::Result) each time.
pub type Result<T> = std::result::Result<T, Error>;

/// Shortcut to creating a custom error
macro_rules! err {
    ($($args:tt)*) => { $crate::error::Error::Custom(format!($($args)*)) };
}

pub(crate) use err;

/// # Error type for the entire crate.
///
/// This type is returned as the `Err` variant from each fallible function in this crate.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
	/// Any one-off error that does not justify a separate variant.
	#[error("{0}")]
	Custom(String),

	/// An invalid URL was provided for an HTTP request.
	#[cfg(feature = "reqwest")]
	#[error("`{0}` is not a valid URL.")]
	InvalidUrl(String),

	/// An HTTP request with [`reqwest`] failed without a status code.
	#[cfg(feature = "reqwest")]
	#[error("HTTP request failed: {0}")]
	Reqwest(String),

	/// An HTTP request with [`reqwest`] failed with a status code.
	#[cfg(feature = "reqwest")]
	#[error("HTTP request failed with code {code} ({message}).")]
	Http {
		/// The HTTP status code
		code: u16,

		/// The HTTP status code in text form
		message: String,
	},

	/// An HTTP request returned an empty array / string / whatever
	#[cfg(feature = "reqwest")]
	#[error("Got an empty HTTP response.")]
	EmptyResponse,
}

impl From<String> for Error {
	fn from(custom_err: String) -> Self { err!("{custom_err}") }
}

impl From<&str> for Error {
	fn from(custom_err: &str) -> Self { err!("{custom_err}") }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
	#[tracing::instrument(level = "ERROR", fields(code = ?err.status(), error = ?err))]
	fn from(err: reqwest::Error) -> Self {
		err.status()
			.map(|code| Self::Http {
				code: code.as_u16(),
				message: code.to_string(),
			})
			.unwrap_or_else(|| Self::Reqwest(err.to_string()))
	}
}
