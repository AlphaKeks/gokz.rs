macro_rules! identifier {
	($type:ident, $id_err:ident) => {
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

		#[allow(missing_docs)]
		#[derive(Debug, Clone, PartialEq, Eq, Hash)]
		#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
		#[cfg_attr(feature = "serde", serde(untagged))]
		pub enum $type {
			ID(u16),
			Name(String),
		}

		#[rustfmt::skip]
		impl $type {
			is!(is_id, ID(_));
			is!(is_name, Name(_));
		}

		try_from!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => $type => |int| {
			let Ok(map_id) = u16::try_from(int) else {
				yeet!($id_err(int))
			};

			Ok($type::ID(map_id))
		});

		try_from!($type => [i8, u8, i16] => |map_identifier| {
			let $type::ID(map_id) = map_identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			map_id.try_into().map_err(|err: std::num::TryFromIntError| Error::Custom(err.to_string()))
		});

		try_from!($type => [u16, i32, u32, i64, u64, i128, u128] => |map_identifier| {
			let $type::ID(map_id) = map_identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			Ok(map_id.into())
		});

		try_from!($type => [isize, usize] => |map_identifier| {
			let $type::ID(map_id) = map_identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			Ok(map_id as _)
		});

		from!([&str, String] => $type => |map_name| {
			$type::Name(map_name.into())
		});

		impl FromStr for $type {
			type Err = std::convert::Infallible;

			fn from_str(input: &str) -> Result<Self, Self::Err> {
				Ok(Self::Name(input.to_owned()))
			}
		}

		#[cfg(feature = "serde")]
		mod serde {
			use {
				super::$type,
				serde::{de, Deserialize, Deserializer},
			};

			#[derive(Deserialize)]
			#[serde(untagged)]
			enum Deserializable {
				U16(u16),
				String(String),
			}

			impl<'de> Deserialize<'de> for $type {
				fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
					match Deserializable::deserialize(deserializer)? {
						Deserializable::U16(map_id) => $type::try_from(map_id),
						Deserializable::String(map_name) => Ok($type::from(map_name)),
					}
					.map_err(|err| de::Error::custom(err.to_string()))
				}
			}
		}
	};
}

pub(crate) use identifier;
