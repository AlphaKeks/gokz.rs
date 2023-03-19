use {
	crate::{Error, Result, SteamID},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// Abstraction layer to accept either a players's name or SteamID as function input in order to
/// stay type-safe.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(untagged)]
pub enum PlayerIdentifier {
	SteamID(SteamID),
	Name(String),
}

impl Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::SteamID(steam_id) => f.write_str(&steam_id.to_string()),
			PlayerIdentifier::Name(name) => f.write_str(name),
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

impl TryFrom<PlayerIdentifier> for SteamID {
	type Error = Error;

	fn try_from(value: PlayerIdentifier) -> Result<Self> {
		if let PlayerIdentifier::SteamID(steam_id) = value {
			return Ok(steam_id);
		}

		Err(Error::Custom("PlayerIdentifier was not `SteamID`."))
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

impl Serialize for PlayerIdentifier {
	/// Based on the variant of [`Self`] this will either use [`SteamID`]'s implementation of
	/// [`Serialize`] or serialize as [`String`].
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			PlayerIdentifier::Name(player_name) => player_name.serialize(serializer),
			PlayerIdentifier::SteamID(steam_id) => steam_id.serialize(serializer),
		}
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Debug, PartialEq, Serialize, Deserialize)]
	struct Player {
		ident: PlayerIdentifier,
	}

	#[test]
	fn ser_player_identifier() -> Result<()> {
		let alphakeks_steam_id: PlayerIdentifier = SteamID::try_from(76561198282622073u64)?.into();
		let alphakeks_name: PlayerIdentifier = String::from("AlphaKeks").into();
		let alphakeks1 = Player {
			ident: alphakeks_steam_id.clone(),
		};
		let alphakeks2 = Player {
			ident: alphakeks_name.clone(),
		};

		let serialized_steam_id = serde_json::to_string(&alphakeks_steam_id)?;
		let serialized_name = serde_json::to_string(&alphakeks_name)?;
		let serialized_player1 = serde_json::to_string(&alphakeks1)?;
		let serialized_player2 = serde_json::to_string(&alphakeks2)?;

		assert_eq!(serialized_steam_id, "\"STEAM_1:1:161178172\"");
		assert_eq!(serialized_name, "\"AlphaKeks\"");
		assert_eq!(serialized_player1, r#"{"ident":"STEAM_1:1:161178172"}"#);
		assert_eq!(serialized_player2, r#"{"ident":"AlphaKeks"}"#);

		Ok(())
	}

	#[test]
	fn deser_player_identifier() -> Result<()> {
		let steam_id = "\"U:1:322356345\"";
		let name = "\"AlphaKeks\"";
		let player1 = r#"{"ident":"STEAM_1:1:161178172"}"#;
		let player2 = r#"{"ident":"AlphaKeks"}"#;

		let deserialized_steam_id: PlayerIdentifier = serde_json::from_str(steam_id)?;
		let deserialized_name: PlayerIdentifier = serde_json::from_str(name)?;
		let deserialized_player1: Player = serde_json::from_str(player1)?;
		let deserialized_player2: Player = serde_json::from_str(player2)?;

		let steam_id = SteamID::new("STEAM_1:1:161178172")?;

		assert_eq!(deserialized_steam_id, steam_id.into());
		assert_eq!(deserialized_name, String::from("AlphaKeks").into());
		assert_eq!(
			deserialized_player1,
			Player {
				ident: steam_id.into()
			}
		);
		assert_eq!(
			deserialized_player2,
			Player {
				ident: String::from("AlphaKeks").into()
			}
		);

		Ok(())
	}
}
