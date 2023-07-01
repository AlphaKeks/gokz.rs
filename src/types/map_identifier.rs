use crate::error::{err, Error};

/// Usually a map in KZ has a name and an ID. Functions might be fine with either, so this enum
/// abstracts this union away.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapIdentifier {
	/// A map id, e.g. `992`
	Id(u16),

	/// A map name, e.g. `"kz_lionharder"`
	Name(String),
}

impl crate::traits::MapIdentifier for MapIdentifier {
	fn image_url(&self) -> Option<String> {
		if let MapIdentifier::Name(map_name) = self {
			return Some(format!(
				"https://raw.githubusercontent.com/KZGlobalTeam/map-images/master/images/{map_name}.jpg"
			));
		}

		None
	}

	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String {
		use crate::global_api::BASE_URL;
		match self {
			MapIdentifier::Id(map_id) => format!("{BASE_URL}/maps/{map_id}"),
			MapIdentifier::Name(map_name) => {
				format!("{BASE_URL}/maps/name/{map_name}")
			}
		}
	}

	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> Option<String> {
		use crate::kzgo_api::BASE_URL;

		if let MapIdentifier::Name(map_name) = self {
			return Some(format!("{BASE_URL}/maps/{map_name}"));
		}

		None
	}

	#[cfg(feature = "kzgo-api")]
	fn kzgo_api(&self) -> Option<String> {
		use crate::kzgo_api::BASE_URL;

		if let MapIdentifier::Name(map_name) = self {
			return Some(format!("{BASE_URL}/maps/{map_name}"));
		}

		None
	}

	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String {
		use crate::schnose_api::BASE_URL;
		match self {
			MapIdentifier::Id(map_id) => format!("{BASE_URL}/maps/{map_id}"),
			MapIdentifier::Name(map_name) => format!("{BASE_URL}/maps/{map_name}"),
		}
	}
}

impl std::fmt::Display for MapIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MapIdentifier::Id(map_id) => write!(f, "{map_id}"),
			MapIdentifier::Name(map_name) => write!(f, "{map_name}"),
		}
	}
}

impl From<String> for MapIdentifier {
	fn from(map_identifier: String) -> Self {
		map_identifier.parse::<u16>().map(Self::Id).unwrap_or(Self::Name(map_identifier))
	}
}

impl From<&str> for MapIdentifier {
	fn from(map_identifier: &str) -> Self {
		map_identifier.parse::<u16>().map(Self::Id).unwrap_or(Self::Name(map_identifier.to_owned()))
	}
}

macro_rules! try_into_int {
	($($int:ty), *) => {
		$(impl TryFrom<$crate::types::MapIdentifier> for $int {
			type Error = $crate::error::Error;

			fn try_from(
				map_identifier: $crate::types::MapIdentifier,
			) -> ::std::result::Result<$int, Self::Error> {
				Ok(match map_identifier {
					$crate::types::MapIdentifier::Id(map_id) => map_id.try_into().map_err(|err| {
						$crate::error::err!(
							"MapId `{map_id}` could not be converted into a valid {}. ({err:?})",
							stringify!($int),
						)
					})?,
					$crate::types::MapIdentifier::Name(map_name) => {
						return Err($crate::error::err!(
							"MapIdentifier (`{map_name}`) was not an Id."
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
		$(impl From<$int> for $crate::types::MapIdentifier {
			fn from(int: $int) -> $crate::types::MapIdentifier {
				$crate::types::MapIdentifier::Id(int.into())
			}
		})*
	};
}

from_int!(u8, u16);

macro_rules! try_from_int {
	($($int:ty), *) => {
		$(impl TryFrom<$int> for $crate::types::MapIdentifier {
			type Error = $crate::error::Error;

			fn try_from(
				int: $int,
			) -> ::std::result::Result<$crate::types::MapIdentifier, Self::Error> {
				Ok($crate::types::MapIdentifier::Id(int.try_into().map_err(|err| {
					$crate::error::err!(
						"`{int}` could not be converted into a valid id. ({err:?})",
					)
				})?))
			}
		})*
	};
}

try_from_int!(u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl std::str::FromStr for MapIdentifier {
	type Err = Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		if s.is_empty() {
			return Err(err!("An empty string is not a valid MapIdentifier."));
		}

		macro_rules! try_parse_id {
			($s:expr, u8, u16) => {
				if let Ok(map_id) = $s.parse::<u8>() {
					return Ok(map_id.into());
				} else if let Ok(map_id) = $s.parse::<u16>() {
					return Ok(map_id.into());
				}
			};

			($s:expr, $($t:ty), *) => {
				$({
					if let Ok(map_id) = $s.parse::<$t>() {
						return map_id.try_into();
					}
				})*
			};
		}

		try_parse_id!(s, u8, u16);
		try_parse_id!(s, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

		Ok(Self::Name(s.to_owned()))
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for MapIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			MapIdentifier::Id(map_id) => serializer.serialize_u16(*map_id),
			MapIdentifier::Name(map_name) => serializer.serialize_str(map_name.as_str()),
		}
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MapIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use crate::utils::Either;

		Ok(match Either::<String, u16>::deserialize(deserializer)? {
			Either::A(map_name) => map_name.into(),
			Either::B(map_id) => map_id.into(),
		})
	}
}
