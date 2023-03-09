use {serde::Serialize, std::fmt::Display};

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
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Custom(message) => f.write_str(message),
			Error::InvalidAccountUniverse { value } => f.write_fmt(format_args!(
				"Invalid Account Universe `{value}`. Please use a number from 0-5."
			)),
			Error::InvalidAccountType { value } => f.write_fmt(format_args!(
				"Invalid Account Type `{value}`. Please use a number from 0-10."
			)),
			Error::InvalidSteamID { value } => {
				f.write_fmt(format_args!("Invalid SteamID `{value}`."))
			}
			Error::InvalidMode { value } => f.write_fmt(format_args!("Invalid Mode `{value}`.")),
			Error::InvalidRank { value } => f.write_fmt(format_args!("Invalid Rank `{value}`.")),
			Error::InvalidTier { value } => f.write_fmt(format_args!("Invalid Tier `{value}`.")),
			Error::InvalidUrl { value } => f.write_fmt(format_args!("Invalid URL `{value}`.")),
			#[cfg(feature = "http")]
			Error::Http { status_code } => f.write_fmt(format_args!(
				"Http request failed with code `{status_code}`."
			)),
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
