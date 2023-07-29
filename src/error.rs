//! The error type for this crate.
//!
//! Any fallible function in this crate should return [`Result`] with an error type of [`Error`].

use {std::result::Result as StdResult, thiserror::Error};

/// Any fallible function in this crate will return this type.
pub type Result<T> = StdResult<T, Error>;

/// The crate-wide error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
	/// Some input failed to parse into a [`SteamID`].
	#[error("The provided SteamID is invalid.")]
	InvalidSteamID,
}

/// Early return with the given [`Error`] variant.
#[macro_export]
macro_rules! yeet {
	($err:ident) => {{
		return Err($crate::Error::$err);
	}};
}
