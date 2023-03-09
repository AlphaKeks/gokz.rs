use {serde::Serialize, std::fmt::Display};

/// Crate-level `Result` type for convenience.
pub type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[non_exhaustive]
pub enum Error {
	/// Any error that only occurs once and therefore does not deserve its own variant.
	Custom(&'static str),

	/// Failed to parse an [Account Universe](crate::AccountUniverse).
	InvalidAccountUniverse { value: String },

	/// Failed to parse an [Account Type](crate::AccountType).
	InvalidAccountType { value: String },

	/// Failed to parse a [SteamID](crate::SteamID).
	InvalidSteamID { value: String },

	/// Failed to parse a [Mode](crate::Mode).
	InvalidMode { value: String },

	/// Failed to parse a [Rank](crate::Rank).
	InvalidRank { value: String },
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
		}
	}
}

impl std::error::Error for Error {}
