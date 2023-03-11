use {
	crate::{Error, Result, SteamID},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// Abstraction layer to accept either a players's name or SteamID as function input in order to stay
/// type-safe without unnecessary conversions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum PlayerIdentifier {
	Name(String),
	SteamID(SteamID),
}

impl Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::Name(name) => f.write_str(name),
			PlayerIdentifier::SteamID(steam_id) => f.write_str(&steam_id.to_string()),
		}
	}
}

impl From<String> for PlayerIdentifier {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}

impl std::str::FromStr for PlayerIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(s.to_owned().into())
	}
}

impl From<SteamID> for PlayerIdentifier {
	fn from(value: SteamID) -> Self {
		Self::SteamID(value)
	}
}

impl TryFrom<PlayerIdentifier> for String {
	type Error = Error;

	/// `TryFrom` because it only converts successfully if the [`PlayerIdentifier`] was `Name`.
	fn try_from(value: PlayerIdentifier) -> Result<Self> {
		if let PlayerIdentifier::Name(name) = value {
			return Ok(name);
		}

		Err(Error::Custom("PlayerIdentifier was not `Name`."))
	}
}

impl TryFrom<PlayerIdentifier> for SteamID {
	type Error = Error;

	fn try_from(value: PlayerIdentifier) -> Result<Self> {
		if let PlayerIdentifier::SteamID(steam_id) = value {
			return Ok(steam_id);
		}

		Err(Error::Custom("PlayerIdentifier was not `SteamID`."))
	}
}

impl Serialize for PlayerIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for PlayerIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(String::deserialize(deserializer)?
			.parse()
			.expect("Infallible"))
	}
}
