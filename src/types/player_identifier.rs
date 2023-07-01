use crate::{
	error::{err, Error, Result},
	types::SteamID,
};

/// A lot of functions might take a "player" as an argument and be fine with either a name or a
/// [`SteamID`].
#[allow(missing_docs)] // These should be self-explanatory
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerIdentifier {
	SteamID(SteamID),
	Name(String),
}

impl crate::traits::PlayerIdentifier for PlayerIdentifier {
	#[inline]
	fn steam_profile(&self) -> String {
		match self {
			PlayerIdentifier::SteamID(steam_id) => {
				let steam_id = steam_id.as_id64();
				format!("https://steamcommunity.com/profiles/{steam_id}")
			}
			PlayerIdentifier::Name(name) => format!("https://steamcommunity.com/id/{name}"),
		}
	}

	#[inline]
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String {
		use crate::global_api::BASE_URL;
		match self {
			PlayerIdentifier::SteamID(steam_id) => {
				format!("{BASE_URL}/players?steam_id={steam_id}")
			}
			PlayerIdentifier::Name(name) => format!("{BASE_URL}/players?name={name}"),
		}
	}

	#[inline]
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> String {
		use crate::kzgo_api::BASE_URL;
		match self {
			PlayerIdentifier::SteamID(steam_id) => format!("{BASE_URL}/players/{steam_id}"),
			PlayerIdentifier::Name(name) => format!("{BASE_URL}/players/{name}"),
		}
	}

	#[inline]
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String {
		use crate::schnose_api::BASE_URL;
		match self {
			PlayerIdentifier::SteamID(steam_id) => format!("{BASE_URL}/players/{steam_id}"),
			PlayerIdentifier::Name(name) => format!("{BASE_URL}/players/{name}"),
		}
	}
}

impl std::fmt::Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::SteamID(steam_id) => write!(f, "{steam_id}"),
			PlayerIdentifier::Name(name) => write!(f, "{name}"),
		}
	}
}

impl From<String> for PlayerIdentifier {
	fn from(player_identifier: String) -> Self {
		player_identifier
			.parse::<SteamID>()
			.map(Self::SteamID)
			.unwrap_or(Self::Name(player_identifier))
	}
}

impl From<&str> for PlayerIdentifier {
	fn from(player_identifier: &str) -> Self {
		player_identifier
			.parse::<SteamID>()
			.map(Self::SteamID)
			.unwrap_or(Self::Name(player_identifier.to_owned()))
	}
}

impl From<SteamID> for PlayerIdentifier {
	fn from(steam_id: SteamID) -> Self { Self::SteamID(steam_id) }
}

impl TryFrom<PlayerIdentifier> for SteamID {
	type Error = Error;

	fn try_from(player_identifier: PlayerIdentifier) -> Result<Self> {
		match player_identifier {
			PlayerIdentifier::SteamID(steam_id) => Ok(steam_id),
			PlayerIdentifier::Name(_) => Err(err!("PlayerIdentifier was not a SteamID.")),
		}
	}
}

impl std::str::FromStr for PlayerIdentifier {
	type Err = Error;

	fn from_str(input: &str) -> Result<Self> {
		if input.is_empty() {
			return Err(err!("An empty string is not a valid PlayerIdentifier."));
		}

		Ok(input.parse::<SteamID>().map(Self::SteamID).unwrap_or(Self::Name(input.to_owned())))
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for PlayerIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			PlayerIdentifier::SteamID(steam_id) => steam_id.serialize(serializer),
			PlayerIdentifier::Name(name) => name.serialize(serializer),
		}
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PlayerIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use crate::utils::Either;

		Ok(match Either::<u64, String>::deserialize(deserializer)? {
			Either::A(steam_id) => SteamID::try_from(steam_id)
				.map_err(|err| serde::de::Error::custom(err.to_string()))?
				.into(),
			Either::B(name) => name.into(),
		})
	}
}
