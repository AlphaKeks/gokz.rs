use crate::error::{err, Error, Result};

/// NOTE: [Official Documentation](
///   https://developer.valvesoftware.com/wiki/SteamID#Universes_Available_for_Steam_Accounts
/// )
#[allow(missing_docs)] // I honestly have no idea what to put here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize),
	serde(rename_all = "snake_case")
)]
pub enum AccountType {
	Invalid,
	Individual,
	Multiseat,
	GameServer,
	AnonGameServer,
	Pending,
	ContentServer,
	Clan,
	Chat,
	P2PSuperSeeder,
	AnonUser,
}

impl std::fmt::Display for AccountType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// The `Debug` representation is the same as the variant's name, so this is convenient.
		write!(f, "{self:?}")
	}
}

impl std::str::FromStr for AccountType {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"invalid" => AccountType::Invalid,
			"individual" => AccountType::Individual,
			"multiseat" => AccountType::Multiseat,
			"gameserver" => AccountType::GameServer,
			"anongameserver" => AccountType::AnonGameServer,
			"pending" => AccountType::Pending,
			"contentserver" => AccountType::ContentServer,
			"clan" => AccountType::Clan,
			"chat" => AccountType::Chat,
			"p2psuperseeder" => AccountType::P2PSuperSeeder,
			"anonuser" => AccountType::AnonUser,
			input => {
				return Err(err!("`{input}` is not a valid steam account type."));
			}
		})
	}
}

impl TryFrom<char> for AccountType {
	type Error = Error;

	fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
		Ok(match c {
			'I' | 'i' => AccountType::Invalid,
			'U' => AccountType::Individual,
			'M' => AccountType::Multiseat,
			'G' => AccountType::GameServer,
			'A' => AccountType::AnonGameServer,
			'P' => AccountType::Pending,
			'C' => AccountType::ContentServer,
			'g' => AccountType::Clan,
			'T' | 'L' | 'c' => AccountType::Chat,
			'a' => AccountType::AnonUser,
			input => {
				return Err(err!("`{input}` is not a valid steam account type."));
			}
		})
	}
}

impl TryFrom<u8> for AccountType {
	type Error = Error;

	fn try_from(int: u8) -> std::result::Result<Self, Self::Error> {
		Ok(match int {
			0 => Self::Invalid,
			1 => Self::Individual,
			2 => Self::Multiseat,
			3 => Self::GameServer,
			4 => Self::AnonGameServer,
			5 => Self::Pending,
			6 => Self::ContentServer,
			7 => Self::Clan,
			8 => Self::Chat,
			9 => Self::P2PSuperSeeder,
			10 => Self::AnonUser,
			input => {
				return Err(err!("`{input}` is not a valid steam account type."));
			}
		})
	}
}
