use {
	crate::error::{err, Error, Result},
	lazy_static::lazy_static,
	regex::Regex,
};

mod account_universe;
pub use account_universe::AccountUniverse;

mod account_type;
pub use account_type::AccountType;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "serde"))]
mod serde_tests;

#[rustfmt::skip]
lazy_static! {
	/// The "standard" representation of a [`SteamID`].
	///
	/// Example: `STEAM_1:1:161178172`
	pub static ref STANDARD_REGEX: Regex = Regex::new(
		// TODO: figure out how many digits are realistically allowed.
		r#"^STEAM_[0-5]:[01]:\d+$"#
	).unwrap();

	/// The "community" representation of a [`SteamID`].
	///
	/// Example: `U:1:322356345` or `[U:1:322356345]`
	pub static ref COMMUNITY_REGEX: Regex = Regex::new(
		// TODO: figure out how many digits are realistically allowed.
		r#"^([IiUMGAPCgTLca]:1:\d+|\[[IiUMGAPCgTLca]:1:\d+\])$"#
	).unwrap();
}

/// A unique identifier for a [Steam](https://developer.valvesoftware.com/wiki/SteamID) user.
///
/// It is composed of 3 parts:
/// - an "account universe"
/// - an "account type"
/// - an "account number"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SteamID(u64);

impl SteamID {
	/// Any valid [`SteamID`] is at least `MAGIC_OFFSET` + 1.
	pub const MAGIC_OFFSET: u64 = 76561197960265728_u64;

	/// Extracts the inner 64-bit id.
	pub const fn as_id64(&self) -> u64 {
		self.0
	}

	/// Extracts the inner [`AccountUniverse`].
	pub const fn account_universe(&self) -> AccountUniverse {
		match (self.0 >> 56) as u8 {
			0 => AccountUniverse::Individual,
			1 => AccountUniverse::Public,
			2 => AccountUniverse::Beta,
			3 => AccountUniverse::Internal,
			4 => AccountUniverse::Dev,
			5 => AccountUniverse::Rc,
			_ => {
				panic!("Internal SteamID invalid. Detected incorrect account universe.");
			}
		}
	}

	/// Extracts the inner [`AccountType`].
	pub const fn account_type(&self) -> AccountType {
		match self.0 & 1 {
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
			_ => {
				panic!("Internal SteamID invalid. Detected incorrect account type.");
			}
		}
	}

	/// Extracts the inner account number.
	pub const fn account_number(&self) -> u32 {
		let offset = (self.0 - Self::MAGIC_OFFSET) as u32;
		let account_type = self.account_type() as u32;
		(offset - account_type) / 2
	}

	/// Extracts the last segment of the "community" representation.
	///
	/// Example: `U:1:322356345` -> `322356345`
	pub const fn community_id(&self) -> u32 {
		let account_number = self.account_number();
		let account_type = self.account_type() as u32;
		((account_number + account_type) * 2) - account_type
	}

	/// Creates a new [`SteamID`]
	pub fn new<S>(steam_id: S) -> Result<Self>
	where
		S: TryInto<Self, Error = Error>,
	{
		steam_id.try_into()
	}
}

impl std::fmt::Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"STEAM_{}:{}:{}",
			self.account_universe() as u8,
			self.account_type() as u8,
			self.account_number()
		)
	}
}

impl TryFrom<u32> for SteamID {
	type Error = Error;

	fn try_from(id: u32) -> Result<Self> {
		let id = id as u64;
		let limit = u64::MAX - Self::MAGIC_OFFSET;
		if id > limit {
			return Err(err!("32-bit SteamID can't be larger than `{limit}`."));
		}

		Ok(Self(id + Self::MAGIC_OFFSET))
	}
}

impl TryFrom<u64> for SteamID {
	type Error = Error;

	fn try_from(id: u64) -> Result<Self> {
		if id <= Self::MAGIC_OFFSET {
			return Err(err!("64-bit SteamID must be at least `{}`.", Self::MAGIC_OFFSET + 1));
		}

		Ok(Self(id))
	}
}

impl std::str::FromStr for SteamID {
	type Err = Error;

	fn from_str(steam_id: &str) -> Result<Self> {
		if let Ok(Ok(steam_id)) = steam_id
			.parse::<u32>()
			.map(Self::try_from)
		{
			return Ok(steam_id);
		}

		if let Ok(Ok(steam_id)) = steam_id
			.parse::<u64>()
			.map(Self::try_from)
		{
			return Ok(steam_id);
		}

		if STANDARD_REGEX.is_match(steam_id) {
			let mut numbers = steam_id
				.split_once('_')
				.expect("The regex should prevent this from failing")
				.1
				.split(':');

			let account_universe = {
				_ = numbers.next();
				AccountUniverse::Public
			};

			let account_type: AccountType = numbers
				.next()
				.expect("The regex should prevent this from failing")
				.parse::<u8>()
				.expect("The regex should prevent this from failing")
				.try_into()?;

			let account_number = numbers
				.next()
				.expect("The regex should prevent this from failing")
				.parse::<u32>()
				.expect("The regex should prevent this from failing");

			let steam_id64 = (account_universe as u64) << 56
				| 1u64 << 52 | 1u64 << 32
				| (account_number as u64) << 1
				| account_type as u64;

			return Ok(Self(steam_id64));
		}

		if COMMUNITY_REGEX.is_match(steam_id) {
			let steam_id = steam_id.replace(['[', ']'], "");
			let parts = steam_id.split(':');
			let id32 = parts
				.last()
				.expect("The regex should prevent this from failing")
				.parse::<u32>()
				.expect("The regex should prevent this from failing");

			return Self::try_from(id32);
		}

		Err(err!("`{steam_id}` is not a valid SteamID."))
	}
}

impl TryFrom<&str> for SteamID {
	type Error = Error;

	fn try_from(steam_id: &str) -> Result<Self> {
		steam_id.parse()
	}
}

impl TryFrom<String> for SteamID {
	type Error = Error;

	fn try_from(steam_id: String) -> Result<Self> {
		steam_id.parse()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for SteamID {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.to_string().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SteamID {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de;

		match crate::utils::Either::<String, u64>::deserialize(deserializer)? {
			crate::utils::Either::A(steam_id) => steam_id
				.parse()
				.map_err(de::Error::custom),

			crate::utils::Either::B(steam_id) => match u32::try_from(steam_id) {
				Ok(steam_id) => SteamID::try_from(steam_id).map_err(de::Error::custom),
				Err(_) => SteamID::try_from(steam_id).map_err(de::Error::custom),
			},
		}
	}
}
