//! Abstraction over a "player". Many APIs accept either a player's [`SteamID`] or name.

use {
	crate::{
		macros::{convert::from, is},
		yeet, SteamID,
	},
	std::{fmt::Display, str::FromStr},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PlayerIdentifier {
	SteamID(SteamID),
	Name(String),
}

#[rustfmt::skip]
impl PlayerIdentifier {
	is!(is_steam_id, SteamID(_));
	is!(is_name, Name(_));
}

impl Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::SteamID(steam_id) => write!(f, "{steam_id}"),
			PlayerIdentifier::Name(name) => write!(f, "{name}"),
		}
	}
}

impl From<SteamID> for PlayerIdentifier {
	fn from(steam_id: SteamID) -> Self {
		PlayerIdentifier::SteamID(steam_id)
	}
}

from!([&str, String] => PlayerIdentifier => |player_name| {
	PlayerIdentifier::Name(player_name.into())
});

impl TryFrom<PlayerIdentifier> for SteamID {
	type Error = crate::Error;

	fn try_from(player_identifier: PlayerIdentifier) -> Result<Self, Self::Error> {
		let PlayerIdentifier::SteamID(steam_id) = player_identifier else {
			yeet!(Custom("PlayerIdentifier was not a `SteamID`."));
		};

		Ok(steam_id)
	}
}

impl TryFrom<PlayerIdentifier> for String {
	type Error = crate::Error;

	fn try_from(player_identifier: PlayerIdentifier) -> Result<Self, Self::Error> {
		let PlayerIdentifier::Name(player_name) = player_identifier else {
			yeet!(Custom("PlayerIdentifier was not a `Name`."));
		};

		Ok(player_name)
	}
}

impl FromStr for PlayerIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Ok(Self::Name(input.to_owned()))
	}
}

#[cfg(feature = "serde")]
mod serde {
	use {
		super::PlayerIdentifier,
		crate::SteamID,
		serde::{Deserialize, Deserializer, Serialize},
	};

	impl Serialize for PlayerIdentifier {
		fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self {
				PlayerIdentifier::SteamID(steam_id) => steam_id.serialize(serializer),
				PlayerIdentifier::Name(player_name) => player_name.serialize(serializer),
			}
		}
	}

	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Deserializable {
		SteamID(SteamID),
		String(String),
	}

	impl<'de> Deserialize<'de> for PlayerIdentifier {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			Ok(match Deserializable::deserialize(deserializer)? {
				Deserializable::SteamID(steam_id) => PlayerIdentifier::SteamID(steam_id),
				Deserializable::String(player_name) => PlayerIdentifier::Name(player_name),
			})
		}
	}
}
