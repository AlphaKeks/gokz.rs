//! A unique identifier for steam accounts.
//!
//! This implementation is specific to CS:GO and might not work correctly for other games.
//!
//! Official documentation: <https://developer.valvesoftware.com/wiki/SteamID>

use {
	crate::yeet,
	lazy_regex::{regex, Regex},
	std::{fmt::Display, str::FromStr},
};

#[cfg(test)]
mod tests;

#[cfg(feature = "serde")]
mod serde;

#[cfg(all(feature = "serde", test))]
mod serde_tests;

/// Official documentation:
/// <https://developer.valvesoftware.com/wiki/SteamID#Universes_Available_for_Steam_Accounts>
pub type AccountUniverse = u64;

/// Official documentation:
/// <https://developer.valvesoftware.com/wiki/SteamID#Types_of_Steam_Accounts>
pub type AccountType = u64;

/// Official documentation:
/// <https://developer.valvesoftware.com/wiki/SteamID#Steam_ID_as_a_Steam_Community_ID>
pub type AccountNumber = u64;

/// Regular expression that matches a [`SteamID`] of the format `STEAM_1:1:161178172`.
pub static STANDARD_REGEX: &lazy_regex::Lazy<Regex> = regex!(r#"^STEAM_[01]:[01]:\d+$"#);

/// Regular expression that matches a [`SteamID`] of the format `[U:1:322356345]` or `U:1:322356345`.
pub static COMMUNITY_REGEX: &lazy_regex::Lazy<Regex> = regex!(r#"^(\[U:1:\d+\]|U:1:\d+)$"#);

/// A convenient abstraction to work with steam accounts.
///
/// Functions that require unique player identification will take this type as a parameter.
///
/// NOTE: This implementation is specific to CS:GO and might not work correctly for other games.
///
/// See also: [`PlayerIdentifier`](crate::PlayerIdentifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent, no_pg_array))]
#[repr(transparent)]
pub struct SteamID(u64);

impl SteamID {
	/// The maximum value for any given [`SteamID`].
	pub const MAX: u64 = 76561202255233023_u64;
	/// Any valid [`SteamID`] is higher than this number.
	///
	/// This is useful for conversions between different formats and validation when constructing
	/// new [`SteamID`]s.
	pub const OFFSET: u64 = 76561197960265728_u64;

	/// Constructs a new [`SteamID`].
	///
	/// This function tries to exhaustively parse the input in as many ways as possible which is
	/// good for unknown input. If you know the type / kind of input though, look at the other
	/// constructor methods.
	pub fn new(input: impl AsRef<str>) -> crate::Result<SteamID> {
		input.as_ref().parse()
	}

	/// Constructs a [`SteamID`] of the format `STEAM_1:1:161178172`.
	pub fn from_standard(steam_id: impl AsRef<str>) -> crate::Result<SteamID> {
		let steam_id: &str = steam_id.as_ref();

		if !STANDARD_REGEX.is_match(steam_id) {
			yeet!(InvalidSteamID(steam_id));
		}

		assert!(steam_id.is_ascii(), "SteamID is always valid ASCII.");

		let mut numbers = steam_id
			.split_once('_')
			.expect("SteamID has the format of `STEAM_X:Y:Z`.")
			.1
			.split(':');

		let account_universe = {
			numbers.next();
			1 // Always 1 for CS:GO
		};

		let account_type = numbers
			.next()
			.expect("SteamID always has an Account Type as its second segment.")
			.parse::<u64>()
			.expect("Account Type is always an integer.");

		let account_number = numbers
			.next()
			.expect("SteamID always has an Account Number as its third segment.")
			.parse::<u64>()
			.expect("Account Number is always an integer.");

		assert_eq!(numbers.next(), None, "Nothing of the SteamID should be left over.");

		#[rustfmt::skip]
		let steam_id64 = account_universe << 56
			| 1 << 52
			| 1 << 32
			| account_number << 1
			| account_type;

		assert!(steam_id64 > Self::OFFSET, "SteamID should be larger than `SteamID::OFFSET`.");
		assert!(steam_id64 < Self::MAX, "SteamID should be smaller than `SteamID::OFFSET`.");

		Ok(SteamID(steam_id64))
	}

	/// Constructs a [`SteamID`] of the format `322356345`.
	///
	/// Also accepts `[U:1:322356345]` or `U:1:322356345`.
	pub fn from_community(steam_id: impl AsRef<str>) -> crate::Result<SteamID> {
		let steam_id: &str = steam_id.as_ref();

		if !COMMUNITY_REGEX.is_match(steam_id) {
			yeet!(InvalidSteamID(steam_id));
		}

		assert!(steam_id.is_ascii(), "SteamID is always valid ASCII.");

		let (_, mut id) = steam_id
			.rsplit_once(':')
			.expect("Community SteamID should have a `:`.");

		if id.ends_with(']') {
			id = &id[..(id.len() - 1)];
		}

		let steam_id32 = id
			.parse::<u32>()
			.expect("At this point only an ID should be left.");

		SteamID::try_from(steam_id32)
	}

	/// Constructs a new [`SteamID`] from the given integer without doing any checks.
	///
	/// # Safety
	///
	/// This function will not directly cause undefined behavior, but it might lead to panics later
	/// on. Only use it if you are 100% sure your input is valid. If it is not, it might break
	/// several invariants later down the road.
	#[inline(always)]
	pub unsafe fn new_unchecked(steam_id: u64) -> SteamID {
		SteamID(steam_id)
	}

	/// Extracts the [Account Universe](https://developer.valvesoftware.com/wiki/SteamID#Universes_Available_for_Steam_Accounts).
	#[inline(always)]
	pub const fn account_universe(&self) -> AccountUniverse {
		self.0 >> 56
	}

	/// Extracts the [Account Type](https://developer.valvesoftware.com/wiki/SteamID#Types_of_Steam_Accounts).
	#[inline(always)]
	pub const fn account_type(&self) -> AccountType {
		self.0 & 1
	}

	/// Extracts the [Account Number](https://developer.valvesoftware.com/wiki/SteamID#As_Represented_Textually).
	/// The account number is the last segment of the standard format.
	///
	/// Example: `STEAM_1:1:161178172` -> `161178172`
	#[inline]
	pub const fn account_number(&self) -> AccountNumber {
		let offset = self.0 - Self::OFFSET;
		let account_type = self.account_type();

		(offset - account_type) / 2
	}

	/// Extracts the inner 64-bit integer.
	#[inline(always)]
	pub const fn as_id64(&self) -> u64 {
		self.0
	}

	/// Integer representation of the [community format](https://developer.valvesoftware.com/wiki/SteamID#Steam_ID_as_a_Steam_Community_ID).
	/// The account number is the last segment of the standard format.
	///
	/// Example: `[U:1:322356345]` -> `322356345`
	#[inline]
	pub const fn community_id(&self) -> u32 {
		let account_number = self.account_number();
		let account_type = self.account_type();
		let community_id = ((account_number + account_type) * 2) - account_type;

		community_id as u32
	}
}

impl Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "STEAM_1:{}:{}", self.account_type(), self.account_number())
	}
}

impl TryFrom<u64> for SteamID {
	type Error = crate::Error;

	/// Constructs a [`SteamID`] in the format `76561198282622073`. This value must be higher than
	/// [`SteamID::OFFSET`].
	fn try_from(steam_id64: u64) -> crate::Result<Self> {
		if steam_id64 <= Self::OFFSET || steam_id64 > Self::MAX {
			yeet!(InvalidSteamID(steam_id64));
		}

		Ok(Self(steam_id64))
	}
}

impl TryFrom<i64> for SteamID {
	type Error = crate::Error;

	fn try_from(value: i64) -> Result<Self, Self::Error> {
		let id64: u64 = value
			.try_into()
			.map_err(|_| crate::Error::InvalidSteamID(value.to_string()))?;

		if let Ok(id32) = u32::try_from(id64) {
			return Self::try_from(id32);
		}

		Self::try_from(id64)
	}
}

impl TryFrom<u32> for SteamID {
	type Error = crate::Error;

	/// Constructs a [`SteamID`] in the format `322356345`.
	fn try_from(steam_id32: u32) -> crate::Result<Self> {
		let steam_id64 = steam_id32 as u64 + Self::OFFSET;

		if steam_id64 > Self::MAX {
			yeet!(InvalidSteamID(steam_id32));
		}

		Ok(Self(steam_id64))
	}
}

impl TryFrom<&str> for SteamID {
	type Error = crate::Error;

	fn try_from(input: &str) -> crate::Result<Self> {
		FromStr::from_str(input)
	}
}

impl TryFrom<String> for SteamID {
	type Error = crate::Error;

	fn try_from(input: String) -> crate::Result<Self> {
		Self::try_from(input.as_str())
	}
}

impl FromStr for SteamID {
	type Err = crate::Error;

	fn from_str(input: &str) -> crate::Result<Self> {
		if let Ok(steam_id) = Self::from_standard(input) {
			return Ok(steam_id);
		}

		if let Ok(steam_id) = Self::from_community(input) {
			return Ok(steam_id);
		}

		if let Ok(steam_id32) = input.parse::<u32>() {
			return Self::try_from(steam_id32);
		}

		if let Ok(steam_id64) = input.parse::<u64>() {
			return Self::try_from(steam_id64);
		}

		yeet!(InvalidSteamID(input));
	}
}
