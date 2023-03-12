use {
	crate::{Error, Result},
	regex::Regex,
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// `STEAM_1:1:161178172`
///   - `STEAM_`: prefix (always the same)
///   - `1`: [`AccountUniverse`]
///   - `1`: [`AccountType`]
///   - `161178172`: [account number](Self::account_number)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SteamID(u64);

impl SteamID {
	/// `MAGIC_OFFSET + 1` is the minimum value for any 64-bit [`SteamID`].
	pub const MAGIC_OFFSET: u64 = 76561197960265728u64;

	/// Constructs a new [`SteamID`] from various possible input formats.
	///
	/// # 64-bit format: `76561198282622073`
	///
	/// ```
	/// use gokz_rs::SteamID;
	///
	/// let alphakeks = 76561198282622073_u64;
	/// let zero = 0_u64;
	/// let too_low = 76561197960265728_u64;
	/// let string = String::from("not a number");
	///
	/// assert!(SteamID::new(&alphakeks.to_string()).is_ok());
	/// assert!(SteamID::new(&zero.to_string()).is_err());
	/// assert!(SteamID::new(&too_low.to_string()).is_err());
	/// assert!(SteamID::new(&string).is_err());
	/// ```
	///
	/// # "Default" format: `"STEAM_1:1:161178172"`
	///
	/// ```
	/// use gokz_rs::SteamID;
	///
	/// let alphakeks = String::from("STEAM_1:1:161178172");
	/// let fake1 = String::from("blalbalba");
	/// let fake2 = String::from("STEAM_1:1:161178172blabal");
	/// let fake3 = String::from("blablaSTEAM_1:1:161178172");
	///
	/// assert!(SteamID::new(&alphakeks).is_ok());
	/// assert!(SteamID::new(&fake1).is_err());
	/// assert!(SteamID::new(&fake2).is_err());
	/// assert!(SteamID::new(&fake3).is_err());
	/// ```
	///
	/// # "Commnity ID" format: `"U:1:322356345"` or `"[U:1:322356345]"`
	///
	/// ```
	/// use gokz_rs::SteamID;
	///
	/// let alphakeks1 = String::from("U:1:322356345");
	/// let alphakeks2 = String::from("[U:1:322356345]");
	/// let fake1 = String::from("blalbalba");
	/// let fake2 = String::from("U:1:322356345blabal");
	/// let fake3 = String::from("blablaU:1:322356345");
	/// let fake4 = String::from("[U:1:322356345]blabal");
	/// let fake5 = String::from("blabla[U:1:322356345]");
	///
	/// assert!(SteamID::new(&alphakeks1).is_ok());
	/// assert!(SteamID::new(&alphakeks2).is_ok());
	/// assert!(SteamID::new(&fake1).is_err());
	/// assert!(SteamID::new(&fake2).is_err());
	/// assert!(SteamID::new(&fake3).is_err());
	/// assert!(SteamID::new(&fake4).is_err());
	/// assert!(SteamID::new(&fake5).is_err());
	/// ```
	pub fn new(steam_id: &str) -> Result<Self> {
		if let Ok(numeric_id) = steam_id.parse::<u64>() {
			if numeric_id == 0 {
				return Err(Error::Custom("`0` is not a valid SteamID."));
			}

			if numeric_id <= Self::MAGIC_OFFSET {
				return Err(Error::Custom(
					"SteamID64 must be at least `76561197960265728`.",
				));
			}

			return Ok(Self(numeric_id));
		}

		// Input format is `STEAM_1:1:161178172`.
		if Regex::new(r#"^STEAM_[0-5]:[0-1]:\d+$"#)
			.unwrap()
			.is_match(steam_id)
		{
			let (_, numbers) = steam_id
				.split_once('_')
				.expect("Regex failed.");
			let numbers = numbers.split(':').collect::<Vec<_>>();
			let account_universe: AccountUniverse = numbers
				.first()
				.expect("Regex failed.")
				.parse::<u8>()
				.expect("Regex failed.")
				.try_into()?;
			let account_type: AccountType = numbers
				.get(1)
				.expect("Regex failed.")
				.parse::<u8>()
				.expect("Regex failed.")
				.try_into()?;
			let account_number = numbers
				.get(2)
				.expect("Regex failed.")
				.parse::<u32>()
				.expect("Regex failed.");

			let steam_id64 = (account_universe as u64) << 56
				| 1u64 << 52 | 1u64 << 32
				| (account_number as u64) << 1
				| account_type as u64;

			return Ok(Self(steam_id64));
		}

		// Input format is `"U:1:322356345"` or `"[U:1:322356345]"`.
		if Regex::new(r#"^(?:\[)?[IiUMGAPCgTLca]:1:(\d+)(?:\])?$"#)
			.unwrap()
			.is_match(steam_id)
		{
			let steam_id = steam_id.replace(['[', ']'], "");

			let parts = steam_id.split(':').collect::<Vec<_>>();
			let account_id = parts
				.last()
				.expect("Regex failed.")
				.parse::<u64>()
				.expect("Regex failed.");

			if account_id >= Self::MAGIC_OFFSET {
				return Err(Error::Custom(
					"Account ID must be lower than `76561197960265728`.",
				));
			}

			let steam_id64 = account_id + Self::MAGIC_OFFSET;

			return Ok(Self(steam_id64));
		}

		Err(Error::InvalidSteamID {
			value: steam_id.to_owned(),
		})
	}

	/// Extract the [`AccountUniverse`] out of the [`SteamID`].
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, AccountUniverse, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.account_universe(),
	///         AccountUniverse::Public
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn account_universe(&self) -> AccountUniverse {
		((self.0 >> 56) as u8)
			.try_into()
			.expect("Invalid internal value.")
	}

	/// Extract the [`AccountType`] out of the [`SteamID`].
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, AccountType, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.account_type(),
	///         AccountType::Individual
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn account_type(&self) -> AccountType {
		((self.0 & 1) as u8)
			.try_into()
			.expect("Invalid internal value.")
	}

	/// Extract the "Account Number" out of the [`SteamID`].
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.account_number(),
	///         161178172
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn account_number(&self) -> u32 {
		let offset = (self.0 - Self::MAGIC_OFFSET) as u32;
		let account_type = self.account_type() as u32;
		(offset - account_type) / 2
	}

	/// Extract the inner 64-bit representation out of the [`SteamID`].
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.as_id64(),
	///         76561198282622073_u64
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn as_id64(&self) -> u64 {
		self.0
	}

	/// Format the [`SteamID`] as the "default" representation (e.g. `"STEAM_1:1:161178172"`)
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.as_id32(),
	///         "STEAM_1:1:161178172"
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn as_id32(&self) -> String {
		let account_universe = self.account_universe() as u8;
		let account_type = self.account_type() as u8;
		let account_number = self.account_number();
		format!(
			"STEAM_{}:{}:{}",
			account_universe, account_type, account_number
		)
	}

	/// Format the [`SteamID`] as the "Community" representation (e.g. `"[U:1:322356345]"`)
	///
	/// # Example
	///
	/// ```
	/// use gokz_rs::{SteamID, Result};
	///
	/// fn main() -> Result<()> {
	///     let alphakeks = SteamID::try_from(76561198282622073_u64)?;
	///
	///     assert_eq!(
	///         alphakeks.as_id3(),
	///         "[U:1:322356345]"
	///     );
	///
	///     Ok(())
	/// }
	/// ```
	pub fn as_id3(&self) -> String {
		let offset = self.0 - Self::MAGIC_OFFSET;
		let account_type = self.account_type() as u64;
		let account_id = ((offset - account_type) / 2) + account_type;
		format!("[U:1:{}]", (account_id * 2) - account_type)
	}
}

impl Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.as_id32())
	}
}

impl std::str::FromStr for SteamID {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Self::new(s)
	}
}

impl TryFrom<u64> for SteamID {
	type Error = Error;

	fn try_from(value: u64) -> Result<Self> {
		Self::new(&value.to_string())
	}
}

impl Serialize for SteamID {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for SteamID {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let steam_id = String::deserialize(deserializer)?;
		SteamID::new(&steam_id).map_err(|why| match &why {
			Error::Custom(msg) => serde::de::Error::custom(msg),
			Error::InvalidAccountUniverse { value } => serde::de::Error::invalid_value(
				serde::de::Unexpected::Other(value),
				&why.to_string().as_str(),
			),
			Error::InvalidAccountType { value } => serde::de::Error::invalid_value(
				serde::de::Unexpected::Other(value),
				&why.to_string().as_str(),
			),
			Error::InvalidSteamID { value } => serde::de::Error::custom(value.to_string()),
			_ => unreachable!(),
		})
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

#[cfg(test)]
mod tests {
	use super::*;
	use color_eyre::Result;

	#[test]
	fn new() -> Result<()> {
		let id3_1 = "U:1:322356345";
		let id3_2 = "[U:1:322356345]";
		let id32 = "STEAM_1:1:161178172";
		let id64 = 76561198282622073u64;

		let res_id3_1 = SteamID::new(id3_1)?;
		let res_id3_2 = SteamID::new(id3_2)?;
		let res_id32 = SteamID::new(id32)?;
		let res_id64_1 = SteamID::try_from(id64)?;
		let res_id64_2 = SteamID::new(&id64.to_string())?;

		assert_eq!(res_id3_1, res_id3_2);
		assert_eq!(res_id3_2, res_id32);
		assert_eq!(res_id32, res_id64_1);
		assert_eq!(res_id64_1, res_id64_2);

		assert_eq!(res_id64_2.as_id3(), String::from("[U:1:322356345]"));
		assert_eq!(res_id64_2.as_id32(), String::from("STEAM_1:1:161178172"));
		assert_eq!(res_id64_2.as_id64(), 76561198282622073u64);

		Ok(())
	}
}
