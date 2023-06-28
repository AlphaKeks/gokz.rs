use crate::error::{err, Error, Result};

/// Usually a server in KZ has a name and an ID. Functions might be fine with either, so this enum
/// abstracts this union away.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServerIdentifier {
	/// A server id, e.g. `999`
	Id(u16),

	/// A server name, e.g. `"Hikari KZ"`
	Name(String),
}

impl ServerIdentifier {
	/// Provides a link to the server's associated
	/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) route.
	#[cfg(feature = "global-api")]
	pub fn global_api(&self) -> String {
		use crate::global_api::BASE_URL;
		match self {
			ServerIdentifier::Id(server_id) => format!("{BASE_URL}/servers/{server_id}"),
			ServerIdentifier::Name(server_name) => {
				format!("{BASE_URL}/servers/name/{server_name}")
			}
		}
	}

	/// Provides a link to the server's associated [SchnoseAPI](https://schnose.xyz/) route.
	#[cfg(feature = "schnose-api")]
	pub fn schnose_api(&self) -> String {
		use crate::schnose_api::BASE_URL;
		match self {
			ServerIdentifier::Id(server_id) => format!("{BASE_URL}/servers/{server_id}"),
			ServerIdentifier::Name(server_name) => format!("{BASE_URL}/servers/{server_name}"),
		}
	}
}

impl std::fmt::Display for ServerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ServerIdentifier::Id(server_id) => write!(f, "{server_id}"),
			ServerIdentifier::Name(server_name) => write!(f, "{server_name}"),
		}
	}
}

impl From<String> for ServerIdentifier {
	fn from(server_name: String) -> Self { Self::Name(server_name) }
}

impl From<&str> for ServerIdentifier {
	fn from(server_name: &str) -> Self { Self::Name(server_name.to_owned()) }
}

macro_rules! try_into_int {
	($($int:ty), *) => {
		$(impl TryFrom<$crate::types::ServerIdentifier> for $int {
			type Error = $crate::error::Error;

			fn try_from(
				server_identifier: $crate::types::ServerIdentifier,
			) -> ::std::result::Result<$int, Self::Error> {
				Ok(match server_identifier {
					$crate::types::ServerIdentifier::Id(server_id) => server_id.try_into().map_err(|err| {
						$crate::error::err!(
							"ServerId `{server_id}` could not be converted into a valid {}. ({err:?})",
							stringify!($int),
						)
					})?,
					$crate::types::ServerIdentifier::Name(server_name) => {
						return Err($crate::error::err!(
							"ServerIdentifier (`{server_name}`) was not an Id."
						));
					}
				})
			}
		})*
	};
}

try_into_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! from_int {
	($($int:ty), *) => {
		$(impl From<$int> for $crate::types::ServerIdentifier {
			fn from(int: $int) -> $crate::types::ServerIdentifier {
				$crate::types::ServerIdentifier::Id(int.into())
			}
		})*
	};
}

from_int!(u8, u16);

macro_rules! try_from_int {
	($($int:ty), *) => {
		$(impl TryFrom<$int> for $crate::types::ServerIdentifier {
			type Error = $crate::error::Error;

			fn try_from(
				int: $int,
			) -> ::std::result::Result<$crate::types::ServerIdentifier, Self::Error> {
				Ok($crate::types::ServerIdentifier::Id(int.try_into().map_err(|err| {
					$crate::error::err!(
						"`{int}` could not be converted into a valid id. ({err:?})",
					)
				})?))
			}
		})*
	};
}

try_from_int!(u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl std::str::FromStr for ServerIdentifier {
	type Err = Error;

	fn from_str(input: &str) -> Result<Self> {
		if input.is_empty() {
			return Err(err!("An empty string is not a valid ServerIdentifier."));
		}

		macro_rules! try_parse_id {
			($s:expr, u8, u16) => {
				if let Ok(server_id) = $s.parse::<u8>() {
					return Ok(server_id.into());
				} else if let Ok(server_id) = $s.parse::<u8>() {
					return Ok(server_id.into());
				}
			};

			($s:expr, $($t:ty), *) => {
				$({
					if let Ok(server_id) = $s.parse::<$t>() {
						return server_id.try_into();
					}
				})*
			};
		}

		try_parse_id!(input, u8, u16);
		try_parse_id!(input, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

		Ok(Self::Name(input.to_owned()))
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for ServerIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			ServerIdentifier::Id(server_id) => serializer.serialize_u16(*server_id),
			ServerIdentifier::Name(server_name) => serializer.serialize_str(server_name.as_str()),
		}
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ServerIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use crate::utils::Either;

		Ok(match Either::<String, u16>::deserialize(deserializer)? {
			Either::A(server_name) => server_name.into(),
			Either::B(server_id) => server_id.into(),
		})
	}
}
