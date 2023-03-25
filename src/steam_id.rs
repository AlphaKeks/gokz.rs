use {
	crate::{Error, Result},
	lazy_static::lazy_static,
	regex::Regex,
	serde::{de, Deserialize, Serialize},
	std::{fmt::Display, str::FromStr},
};

lazy_static! {
	/// Standard format (e.g. `STEAM_1:1:161178172`).
	pub static ref STANDARD_REGEX: Regex = Regex::new(
		r#"^STEAM_[0-5]:[0-1]:\d+$"#
	)
	.unwrap();

	/// Community format (e.g. `"U:1:322356345"` or `"[U:1:322356345]"`).
	pub static ref COMMUNITY_REGEX: Regex = Regex::new(
		r#"^(?:\[)?[IiUMGAPCgTLca]:1:(\d+)(?:\])?$"#
	)
	.unwrap();
}

/// A [SteamID](https://developer.valvesoftware.com/wiki/SteamID) according to Valve's official
/// definition. It is stored as a single [`u64`] and has associated methods and trait
/// implementations to format it in any way you like, as well as custom [`Serialize`] and
/// [`Deserilalize`] implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SteamID(u64);

impl SteamID {
	/// `MAGIC_OFFSET + 1` is the minimum value for any valid `SteamID`.
	pub const MAGIC_OFFSET: u64 = 76561197960265728u64;

	/// Extract the [`AccountUniverse`] out of [`Self`].
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{AccountUniverse, SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.account_universe(),
	///         AccountUniverse::Public,
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn account_universe(&self) -> AccountUniverse {
		let account_universe = self.0 >> 56;
		let value = account_universe as u8;
		match value {
			0 => AccountUniverse::Individual,
			1 => AccountUniverse::Public,
			2 => AccountUniverse::Beta,
			3 => AccountUniverse::Internal,
			4 => AccountUniverse::Dev,
			5 => AccountUniverse::RC,
			_ => panic!("Failed to extract AccountUniverse. Invalid internal SteamID."),
		}
	}

	/// Extract the [`AccountType`] out of [`Self`].
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{AccountType, SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.account_type(),
	///         AccountType::Individual
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn account_type(&self) -> AccountType {
		let account_type = self.0 & 1;
		let value = account_type as u8;
		match value {
			0 => AccountType::Invalid,
			1 => AccountType::Individual,
			2 => AccountType::Multiseat,
			3 => AccountType::GameServer,
			4 => AccountType::AnonGameServer,
			5 => AccountType::Pending,
			6 => AccountType::ContentServer,
			7 => AccountType::Clan,
			8 => AccountType::Chat,
			9 => AccountType::P2PSuperSeeder,
			10 => AccountType::AnonUser,
			_ => panic!("Failed to extract AccountType. Invalid internal SteamID."),
		}
	}

	/// Extract the "account number" out of [`Self`]. If we take `"STEAM_1:1:161178172"` as an
	/// example, then `161178172` is the account number.
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.account_number(),
	///         161178172
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn account_number(&self) -> u32 {
		let offset = (self.0 - Self::MAGIC_OFFSET) as u32;
		let account_type = self.account_type() as u32;
		(offset - account_type) / 2
	}

	/// Turns [`Self`] into its 32-bit representation. If we take `"STEAM_1:1:161178172"` as an
	/// example, then `322356345` is the 32-bit representation.
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.as_id32(),
	///         322356345
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn as_id32(&self) -> u32 {
		let account_number = self.account_number();
		let account_type = self.account_type() as u32;
		((account_number + account_type) * 2) - account_type
	}

	/// Extracts the inner 64-bit value.
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.as_id64(),
	///         76561198282622073u64
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn as_id64(&self) -> u64 {
		self.0
	}

	/// Formats [`Self`] as a Community ID. If we take `"STEAM_1:1:161178172"` as an example,
	/// then `"[U:1:322356345]"` is the Community ID equivalent.
	///
	/// # Examples
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	///
	///     assert_eq!(
	///         alphakeks.as_community_id(),
	///         "[U:1:322356345]"
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn as_community_id(&self) -> String {
		let letter: char = self
			.account_type()
			.try_into()
			.unwrap_or('I');

		let id32 = self.as_id32();

		format!("[{letter}:1:{id32}]")
	}

	/// Takes in a 64-bit `SteamID` and turns it into [`Self`].
	/// NOTE: This is only exposed for the case that you need this specific parsing implementation.
	/// Use [`Self::new`] for normal purposes instead.
	pub const fn from_id64(steam_id64: u64) -> Result<Self> {
		// TODO: Come up with a better check for this.
		if steam_id64 <= Self::MAGIC_OFFSET {
			Err(Error::Custom(
				"64-bit SteamID must be higher than `76561197960265728`.",
			))
		} else {
			Ok(Self(steam_id64))
		}
	}

	/// Takes a 32-bit `SteamID` and turns it into [`Self`]. This function assumes that the input is
	/// correct.
	/// NOTE: This is only exposed for the case that you need this specific parsing implementation.
	pub const fn from_id32(steam_id32: u32) -> Self {
		Self(steam_id32 as u64 + Self::MAGIC_OFFSET)
	}

	/// Parses a standard `SteamID` (e.g. `"STEAM_1:1:161178172"`) into [`Self`].
	/// NOTE: This is only exposed for the case that you need this specific parsing implementation.
	/// Use [`Self::new`] for normal purposes instead.
	pub fn from_standard(steam_id: impl AsRef<str>) -> Result<Self> {
		let (_, numbers) = steam_id
			.as_ref()
			.split_once('_')
			.ok_or(Error::Custom("Failed to split SteamID on `_`."))?;

		let mut numbers = numbers.split(':');

		let account_universe: AccountUniverse = numbers
			.next()
			.ok_or(Error::Custom("Failed to get AccountUniverse from SteamID."))?
			.parse::<u8>()
			.map_err(|_| Error::Custom("Failed to parse AccountUniverse from SteamID."))?
			.try_into()?;

		let account_type: AccountType = numbers
			.next()
			.ok_or(Error::Custom("Failed to get AccountType from SteamID."))?
			.parse::<u8>()
			.map_err(|_| Error::Custom("Failed to parse AccountType from SteamID."))?
			.try_into()?;

		let account_number = numbers
			.next()
			.ok_or(Error::Custom("Failed to get account number from SteamID."))?
			.parse::<u32>()
			.map_err(|_| Error::Custom("Failed to parse account number from SteamID."))?;

		let steam_id64 = (account_universe as u64) << 56
			| 1u64 << 52 | 1u64 << 32
			| (account_number as u64) << 1
			| account_type as u64;

		Ok(Self(steam_id64))
	}

	/// Parses a Community `SteamID` (e.g. `"U:1:322356345"` or `"[U:1:322356345]"`) into
	/// [`Self`].
	/// NOTE: This is only exposed for the case that you need this specific parsing implementation.
	/// Use [`Self::new`] for normal purposes instead.
	pub fn from_community(steam_id: impl AsRef<str>) -> Result<Self> {
		let steam_id = steam_id
			.as_ref()
			.replace(['[', ']'], "");

		let parts = steam_id.split(':');

		let id32 = parts
			.last()
			.ok_or(Error::Custom(
				"Failed to extract 32-bit ID from Community SteamID.",
			))?
			.parse::<u64>()
			.map_err(|_| Error::Custom("Failed to parse 32-bit ID from Community SteamID."))?;

		// TODO: Come up with a better check for this.
		if id32 >= Self::MAGIC_OFFSET {
			return Err(Error::Custom(
				"32-bit ID must be less than `76561197960265728`.",
			));
		}

		Ok(Self::from_id32(id32 as u32))
	}

	/// Constructs a new [`Self`] from arbitrary input.
	///
	/// # Examples
	///
	/// ```
	/// use gokz_rs::{AccountType, AccountUniverse, SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let representations = [
	///         "76561198282622073",   // 64-bit
	///         "STEAM_1:1:161178172", // Standard
	///         "U:1:322356345",       // Community without brackets
	///         "[U:1:322356345]",     // Community with brackets
	///     ];
	///
	///     for repr in representations {
	///         let alphakeks = SteamID::new(repr)?;
	///         let account_universe = alphakeks.account_universe();
	///         let account_type = alphakeks.account_type();
	///         let account_number = alphakeks.account_number();
	///         let id32 = alphakeks.as_id32();
	///         let id64 = alphakeks.as_id64();
	///         let community_id = alphakeks.as_community_id();
	///         let display = alphakeks.to_string();
	///
	///         assert_eq!(account_universe, AccountUniverse::Public);
	///         assert_eq!(account_type, AccountType::Individual);
	///         assert_eq!(account_number, 161178172);
	///         assert_eq!(id32, 322356345);
	///         assert_eq!(id64, 76561198282622073);
	///         assert_eq!(community_id, "[U:1:322356345]");
	///         assert_eq!(display, "STEAM_1:1:161178172");
	///     }
	///
	///     Ok(())
	/// }
	/// ```
	pub fn new(input: impl AsRef<str>) -> Result<Self> {
		let input = input.as_ref();

		if let Ok(steam_id64) = input.parse::<u64>() {
			Self::from_id64(steam_id64)
		} else if STANDARD_REGEX.is_match(input) {
			Self::from_standard(input)
		} else if COMMUNITY_REGEX.is_match(input) {
			Self::from_community(input)
		} else {
			Err(Error::InvalidSteamID {
				value: String::from(input),
			})
		}
	}
}

impl Display for SteamID {
	/// Formats a [`SteamID`] in the standard format (e.g. `"STEAM_1:1:161178172"`).
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"STEAM_{}:{}:{}",
			self.account_universe() as u8,
			self.account_type() as u8,
			self.account_number()
		))
	}
}

impl FromStr for SteamID {
	type Err = Error;

	/// Uses [`Self::new`] to parse an input string.
	fn from_str(s: &str) -> Result<Self> {
		Self::new(s)
	}
}

impl TryFrom<u64> for SteamID {
	type Error = Error;

	fn try_from(value: u64) -> Result<Self> {
		Self::from_id64(value)
	}
}

impl Serialize for SteamID {
	/// Serializes [`Self`] using the [`Display`] implementation.
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for SteamID {
	/// Deserializes the input as either a [`u64`] or a [`String`] and passes it to
	/// [`Self::new`].
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum StringOrU64 {
			String(String),
			U64(u64),
		}

		match StringOrU64::deserialize(deserializer)? {
			StringOrU64::String(steam_id) => SteamID::new(steam_id),
			StringOrU64::U64(steam_id64) => SteamID::from_id64(steam_id64),
		}
		.map_err(|why| match &why {
			Error::Custom(msg) => de::Error::custom(msg),
			Error::InvalidAccountUniverse { value } => de::Error::invalid_value(
				de::Unexpected::Other(value.as_str()),
				&"valid account universe",
			),
			Error::InvalidAccountType { value } => de::Error::invalid_value(
				de::Unexpected::Other(value.as_str()),
				&"valid account type",
			),
			Error::InvalidSteamID { value } => {
				de::Error::invalid_value(de::Unexpected::Other(value.as_str()), &"valid steam id")
			}
			why => unreachable!(
				"Encountered unexpected error while deserializing into `SteamID`.\n{why}"
			),
		})
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Serialize, Deserialize)]
	struct Player {
		steam_id: SteamID,
	}

	#[test]
	fn ser_steam_id() -> Result<()> {
		let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
		let p = Player {
			steam_id: alphakeks,
		};

		let serialized = serde_json::to_string(&p.steam_id)?;
		let serialized_player = serde_json::to_string(&p)?;

		assert_eq!(serialized, "\"STEAM_1:1:161178172\"");
		assert_eq!(serialized_player, r#"{"steam_id":"STEAM_1:1:161178172"}"#);

		Ok(())
	}

	#[test]
	fn deser_steam_id() -> Result<()> {
		let alphakeks = "\"STEAM_1:1:161178172\"";
		let p = r#"{"steam_id":"STEAM_1:1:161178172"}"#;

		let deserialized: SteamID = serde_json::from_str(alphakeks)?;
		let deserialized_player: Player = serde_json::from_str(p)?;

		assert_eq!(deserialized, SteamID(76561198282622073u64));
		assert_eq!(deserialized_player.steam_id, SteamID(76561198282622073u64));

		Ok(())
	}
}

/// NOTE: [Valve Documentation](https://developer.valvesoftware.com/wiki/SteamID#Universes_Available_for_Steam_Accounts)
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountUniverse {
	Individual = 0,
	Public = 1,
	Beta = 2,
	Internal = 3,
	Dev = 4,
	RC = 5,
}

impl TryFrom<u8> for AccountUniverse {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			0 => Ok(Self::Individual),
			1 => Ok(Self::Public),
			2 => Ok(Self::Beta),
			3 => Ok(Self::Internal),
			4 => Ok(Self::Dev),
			5 => Ok(Self::RC),
			value => Err(Error::InvalidAccountUniverse {
				value: value.to_string(),
			}),
		}
	}
}

/// NOTE: [Valve Documentation](https://developer.valvesoftware.com/wiki/SteamID#Types_of_Steam_Accounts)
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountType {
	Invalid = 0,
	Individual = 1,
	Multiseat = 2,
	GameServer = 3,
	AnonGameServer = 4,
	Pending = 5,
	ContentServer = 6,
	Clan = 7,
	Chat = 8,
	P2PSuperSeeder = 9,
	AnonUser = 10,
}

impl TryFrom<u8> for AccountType {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			0 => Ok(Self::Invalid),
			1 => Ok(Self::Individual),
			2 => Ok(Self::Multiseat),
			3 => Ok(Self::GameServer),
			4 => Ok(Self::AnonGameServer),
			5 => Ok(Self::Pending),
			6 => Ok(Self::ContentServer),
			7 => Ok(Self::Clan),
			8 => Ok(Self::Chat),
			9 => Ok(Self::P2PSuperSeeder),
			10 => Ok(Self::AnonUser),
			value => Err(Error::InvalidAccountType {
				value: value.to_string(),
			}),
		}
	}
}

impl TryFrom<char> for AccountType {
	type Error = Error;

	fn try_from(value: char) -> Result<Self> {
		match value {
			'I' | 'i' => Ok(Self::Invalid),
			'U' => Ok(Self::Individual),
			'M' => Ok(Self::Multiseat),
			'G' => Ok(Self::GameServer),
			'A' => Ok(Self::AnonGameServer),
			'P' => Ok(Self::Pending),
			'C' => Ok(Self::ContentServer),
			'g' => Ok(Self::Clan),
			'T' | 'L' | 'c' => Ok(Self::Chat),
			'a' => Ok(Self::AnonUser),
			value => Err(Error::InvalidAccountType {
				value: value.to_string(),
			}),
		}
	}
}

impl TryFrom<AccountType> for char {
	type Error = Error;

	fn try_from(value: AccountType) -> Result<Self> {
		match value {
			AccountType::Invalid => Ok('I'),
			AccountType::Individual => Ok('U'),
			AccountType::Multiseat => Ok('M'),
			AccountType::GameServer => Ok('G'),
			AccountType::AnonGameServer => Ok('A'),
			AccountType::Pending => Ok('P'),
			AccountType::ContentServer => Ok('C'),
			AccountType::Clan => Ok('g'),
			AccountType::Chat => Ok('T'),
			AccountType::P2PSuperSeeder => Err(Error::Custom(
				"`P2PSuperSeeder` does not have an associated character.",
			)),
			AccountType::AnonUser => Ok('a'),
		}
	}
}
