use {
	regex::Regex,
	serde::{Deserialize, Serialize},
};

/* --------------------------------------------------------------------------------------------- */

///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
	/// `msg` => [{}:{}] <function> failed => `{}`
	/// `tldr` => Message to be sent to the user
	/// `raw` => original user input
	InvalidInput { msg: String, tldr: String, raw: Option<String> },
}

/* --------------------------------------------------------------------------------------------- */

/// A unique identifier for a [Steam](https://www.steamcommunity.com/) Account.
///
/// Note: [official definition](https://developer.valvesoftware.com/wiki/SteamID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamID(String);

impl SteamID {
	/// Function to test whether a String qualifies as a [`SteamID`] or not
	///
	/// # Example
	/// ```
	/// use gokz_rs::prelude::SteamID;
	/// let valid = SteamID::test("STEAM_1:1:161178172");
	/// let invalid1 = SteamID::test("wordSTEAM_1:1:161178172");
	/// let invalid2 = SteamID::test("STEAM_1:1:161178172word");
	/// let invalid3 = SteamID::test("some random text");
	///
	/// assert!(valid);
	/// assert!(!invalid1);
	/// assert!(!invalid2);
	/// assert!(!invalid3);
	/// ```
	pub fn test(input: &str) -> bool {
		let regex = Regex::new(r#"^STEAM_[0-1]:[0-1]:[0-9]+$"#).expect("This is a valid regex.");
		regex.is_match(input)
	}

	pub fn new(steam_id: &str) -> Result<Self, Error> {
		let steam_id = steam_id.to_owned();
		if Self::test(&steam_id) {
			Ok(SteamID(steam_id))
		} else {
			Err(Error::InvalidInput {
				msg: format!("[{}:{}] SteamID::new() failed => `{}`", file!(), line!(), steam_id),
				tldr: format!("`{}` is not a valid SteamID.", steam_id),
				raw: Some(steam_id),
			})
		}
	}
}

impl std::fmt::Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl std::str::FromStr for SteamID {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		SteamID::new(s)
	}
}

impl TryFrom<String> for SteamID {
	type Error = Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		SteamID::new(&value)
	}
}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn steam_id_constructor() {
		let valid = SteamID::new("STEAM_1:1:161178172");
		let invalid1 = SteamID::new("wordSTEAM_1:1:161178172");
		let invalid2 = SteamID::new("STEAM_1:1:161178172word");
		let invalid3 = SteamID::new("some random text");

		assert!(dbg!(valid).is_ok());
		assert!(dbg!(invalid1).is_err());
		assert!(dbg!(invalid2).is_err());
		assert!(dbg!(invalid3).is_err());
	}
}
