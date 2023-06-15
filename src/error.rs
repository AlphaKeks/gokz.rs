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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
	/// Any one-off error that does not justify a separate variant.
	#[error("{0}")]
	Custom(String),
}

impl From<String> for Error {
	fn from(custom_err: String) -> Self {
		err!("{custom_err}")
	}
}

impl From<&str> for Error {
	fn from(custom_err: &str) -> Self {
		err!("{custom_err}")
	}
}
