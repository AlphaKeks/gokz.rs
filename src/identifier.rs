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
		#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
		pub enum $type {
			ID(u16),
			Name(String),
		}

		#[rustfmt::skip]
		impl $type {
			is!(is_id, ID(_));
			is!(is_name, Name(_));
		}

		impl std::fmt::Display for $type {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
				match self {
					Self::ID(id) => write!(f, "{id}"),
					Self::Name(name) => write!(f, "{name}"),
				}
			}
		}

		try_from!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => $type => |int| {
			let Ok(id) = u16::try_from(int) else {
				yeet!($id_err(int))
			};

			Ok($type::ID(id))
		});

		try_from!($type => [i8, u8, i16] => |identifier| {
			let $type::ID(id) = identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			id.try_into().map_err(|err: std::num::TryFromIntError| Error::Custom(err.to_string()))
		});

		try_from!($type => [u16, i32, u32, i64, u64, i128, u128] => |identifier| {
			let $type::ID(id) = identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			Ok(id.into())
		});

		try_from!($type => [isize, usize] => |identifier| {
			let $type::ID(id) = identifier else {
				yeet!(Custom("`$type` was not an `ID`."));
			};

			Ok(id as _)
		});

		from!([&str, String] => $type => |name| {
			$type::Name(name.into())
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
						Deserializable::U16(id) => $type::try_from(id),
						Deserializable::String(name) => match name.parse::<u16>() {
							Ok(id) => $type::try_from(id),
							Err(_) => Ok($type::from(name)),
						}
					}
					.map_err(|err| de::Error::custom(err.to_string()))
				}
			}
		}
	};
}

pub(crate) use identifier;
