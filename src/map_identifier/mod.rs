//! This module contains a utility type which is used a lot in the optional modules of this crate.
//! It's also really useful when working with serialized data.

use {
	crate::{
		macros::{
			convert::{from, try_from},
			is,
		},
		yeet, Error,
	},
	std::str::FromStr,
};

#[cfg(feature = "serde")]
mod serde;

/// A utility type for when you don't care how a map is represented.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MapIdentifier {
	ID(u16),
	Name(String),
}

#[rustfmt::skip]
impl MapIdentifier {
	is!(is_id, ID(_));
	is!(is_name, Name(_));
}

try_from!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => MapIdentifier => |int| {
	let Ok(map_id) = u16::try_from(int) else {
		yeet!(InvalidMapID(int))
	};

	Ok(MapIdentifier::ID(map_id))
});

try_from!(MapIdentifier => [i8, u8, i16] => |map_identifier| {
	let MapIdentifier::ID(map_id) = map_identifier else {
		yeet!(Custom("`MapIdentifier` was not an `ID`."));
	};

	map_id.try_into().map_err(|err: std::num::TryFromIntError| Error::Custom(err.to_string()))
});

try_from!(MapIdentifier => [u16, i32, u32, i64, u64, i128, u128] => |map_identifier| {
	let MapIdentifier::ID(map_id) = map_identifier else {
		yeet!(Custom("`MapIdentifier` was not an `ID`."));
	};

	Ok(map_id.into())
});

try_from!(MapIdentifier => [isize, usize] => |map_identifier| {
	let MapIdentifier::ID(map_id) = map_identifier else {
		yeet!(Custom("`MapIdentifier` was not an `ID`."));
	};

	Ok(map_id as _)
});

from!([&str, String] => MapIdentifier => |map_name| {
	MapIdentifier::Name(map_name.into())
});

impl FromStr for MapIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Ok(Self::Name(input.to_owned()))
	}
}
