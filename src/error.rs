use {serde::Serialize, std::fmt::Display, std::num::TryFromIntError};

/// Crate-level `Result` type for convenience.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Error {
	/// Any error that only occurs once and therefore does not deserve its own variant.
	Custom(&'static str),

	InvalidAccountUniverse {
		value: String,
	},

	InvalidAccountType {
		value: String,
	},

	InvalidSteamID {
		value: String,
	},

	InvalidMode {
		value: String,
	},

	InvalidRank {
		value: String,
	},

	InvalidTier {
		value: String,
	},

	InvalidUrl {
		value: String,
	},

	#[cfg(feature = "http")]
	Http {
		status_code: crate::http::StatusCode,
	},

	InvalidDate {
		value: String,
	},

	EmptyResponse,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Custom(message) => f.write_str(message),
			Self::InvalidAccountUniverse { value } => f.write_fmt(format_args!(
				"Invalid Account Universe `{value}`. Please use a number from 0-5."
			)),
			Self::InvalidAccountType { value } => f.write_fmt(format_args!(
				"Invalid Account Type `{value}`. Please use a number from 0-10."
			)),
			Self::InvalidSteamID { value } => {
				f.write_fmt(format_args!("Invalid SteamID `{value}`."))
			}
			Self::InvalidMode { value } => f.write_fmt(format_args!("Invalid Mode `{value}`.")),
			Self::InvalidRank { value } => f.write_fmt(format_args!("Invalid Rank `{value}`.")),
			Self::InvalidTier { value } => f.write_fmt(format_args!("Invalid Tier `{value}`.")),
			Self::InvalidUrl { value } => f.write_fmt(format_args!("Invalid URL `{value}`.")),
			#[cfg(feature = "http")]
			Self::Http { status_code } => f.write_fmt(format_args!(
				"Http request failed with code `{status_code}`."
			)),
			Self::InvalidDate { value } => f.write_fmt(format_args!("Invalid Date `{value}`.")),
			Self::EmptyResponse => f.write_str("Got an empty API response."),
		}
	}
}

impl std::error::Error for Error {}

#[cfg(feature = "http")]
impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		let status_code = value
			.status()
			.unwrap_or(reqwest::StatusCode::IM_A_TEAPOT);
		Self::Http {
			status_code: crate::http::StatusCode(status_code),
		}
	}
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::Custom("Failed to cast integer.")
	}
}
