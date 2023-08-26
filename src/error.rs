//! The error type for this crate.
//!
//! Any fallible function in this crate should return [`Result`] with an error type of
//! [`enum@Error`].

use {std::result::Result as StdResult, thiserror::Error};

/// Any fallible function in this crate will return this type.
pub type Result<T> = StdResult<T, Error>;

/// The crate-wide error type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
	/// Some error that is very specific and only appears once.
	#[error("{0}")]
	Custom(String),

	/// Some input failed to parse into a [`SteamID`](crate::SteamID).
	#[error("`{0}` is not a valid SteamID.")]
	InvalidSteamID(String),

	/// Some input failed to parse into a [`Mode`](crate::Mode).
	#[error("`{0}` is not a valid Mode.")]
	InvalidMode(String),

	/// A given amount of teleports was negative.
	#[error("There cannot be a negative amount of teleports.")]
	InvalidTeleportAmount,

	/// Some input failed to parse into a [`Runtype`](crate::Runtype).
	#[error("`{0}` is not a valid Runtype.")]
	InvalidRuntype(String),

	/// Some input failed to parse into a [`Tier`](crate::Tier).
	#[error("`{0}` is not a valid Tier.")]
	InvalidTier(String),

	/// Some input failed to parse into a [`MapIdentifier`](crate::MapIdentifier).
	#[error("`{0}` is out of range for a valid MapID.")]
	InvalidMapID(String),

	/// Some input failed to parse into a [`ServerIdentifier`](crate::ServerIdentifier).
	#[error("`{0}` is out of range for a valid ServerID.")]
	InvalidServerID(String),

	/// An HTTP Request failed.
	#[cfg(feature = "reqwest")]
	#[error("HTTP Request failed{}: {message}", code.map(|code| format!(" with code {}", code.as_u16())).unwrap_or_default())]
	Http {
		/// The HTTP status code returned by the failed request.
		#[serde(
			serialize_with = "crate::http::serde::serialize_status_code",
			deserialize_with = "crate::http::serde::deserialize_status_code"
		)]
		code: Option<crate::http::StatusCode>,

		/// The error message for the failed request.
		message: String,
	},

	/// An HTTP Response failed to deserialize.
	#[cfg(feature = "reqwest")]
	#[error("Failed to deserialize response: {0}")]
	DeserializeResponse(String),

	/// An HTTP Response was empty.
	#[cfg(feature = "reqwest")]
	#[error("Empty API response.")]
	EmptyResponse,
}

/// Early return with the given [`enum@Error`] variant.
#[macro_export]
macro_rules! yeet {
	($err:ident) => {{
		return Err($crate::Error::$err);
	}};

	($err:ident($into:expr)) => {{
		return Err($crate::Error::$err($into.to_string()));
	}};
}
