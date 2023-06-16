use crate::error::{err, Error, Result};

/// The 3 gamemodes that currently exist in GOKZ.
///
/// NOTE: [Official Documentation](https://github.com/KZGlobalTeam/gokz/wiki/Modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
	/// The KZTimer gamemode.
	///
	/// This is the default gamemode on all KZ servers and also the most popular one.
	///
	/// Also known as "KZT".
	KZTimer = 200,

	/// The SimpleKZ gamemode.
	///
	/// A less popular gamemode with more focus on skill rather than RNG.
	///
	/// Also known as "SKZ".
	SimpleKZ = 201,

	/// The Vanilla gamemode.
	///
	/// The "vanilla" CS:GO experience, just like in matchmaking / faceit.
	///
	/// Also known as "VNL".
	Vanilla = 202,
}

impl Mode {
	/// The standard format of a [`Mode`] in the context of most popular APIs.
	#[inline]
	pub fn api(&self) -> String {
		String::from(match self {
			Mode::KZTimer => "kz_timer",
			Mode::SimpleKZ => "kz_simple",
			Mode::Vanilla => "kz_vanilla",
		})
	}

	/// Abbreviation of the given mode's name. This is how players usually refer to the modes.
	#[inline]
	pub fn short(&self) -> String {
		String::from(match self {
			Mode::KZTimer => "KZT",
			Mode::SimpleKZ => "SKZ",
			Mode::Vanilla => "VNL",
		})
	}
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// The `Debug` representation is the same as the variant's name, so this is convenient.
		write!(f, "{self:?}")
	}
}

macro_rules! into_int {
	($($int:ty), *) => {
		$(impl From<$crate::types::Mode> for $int {
			fn from(mode: $crate::types::Mode) -> $int {
				mode as $int
			}
		})*
	};
}

into_int!(u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);

macro_rules! try_from_int {
	($($int:ty), *) => {
		$(impl TryFrom<$int> for $crate::types::Mode {
			type Error = $crate::error::Error;

			fn try_from(
				int: $int,
			) -> ::std::result::Result<$crate::types::Mode, Self::Error> {
				Ok(match int {
					200 => $crate::types::Mode::KZTimer,
					201 => $crate::types::Mode::SimpleKZ,
					202 => $crate::types::Mode::Vanilla,
					int => {
						return Err($crate::error::err!("`{int}` is not a valid mode ID."));
					}
				})
			}
		})*
	};
}

try_from_int!(u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);

impl std::str::FromStr for Mode {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"200" | "kztimer" | "kz_timer" | "kzt" => Self::KZTimer,
			"201" | "simplekz" | "simple_kz" | "kz_simple" | "skz" => Self::SimpleKZ,
			"202" | "vanilla" | "vanillakz" | "vanilla_kz" | "kz_vanilla" | "vnl" => Self::Vanilla,
			input => {
				return Err(err!("`{input}` is not a valid Mode."));
			}
		})
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Mode {
	#[tracing::instrument(level = "debug", skip(serializer), err(Debug))]
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.api().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Mode {
	#[tracing::instrument(level = "debug", skip(deserializer))]
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use {crate::utils::Either, serde::de};

		match Either::<String, u8>::deserialize(deserializer)? {
			Either::A(mode_name) => mode_name.parse(),
			Either::B(mode_id) => mode_id.try_into(),
		}
		.map_err(|err| match err {
			Error::Custom(err) => {
				de::Error::invalid_value(de::Unexpected::Other(&err), &err.to_string().as_str())
			}
			_ => unreachable!(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod formatting {
		use {super::*, pretty_assertions::assert_eq};

		#[test]
		fn display() {
			let mode = Mode::KZTimer;
			let formatted = mode.to_string();
			assert_eq!(formatted, "KZTimer");

			let mode = Mode::SimpleKZ;
			let formatted = mode.to_string();
			assert_eq!(formatted, "SimpleKZ");

			let mode = Mode::Vanilla;
			let formatted = mode.to_string();
			assert_eq!(formatted, "Vanilla");
		}

		#[test]
		fn debug() {
			let mode = Mode::KZTimer;
			let formatted = format!("{mode:?}");
			assert_eq!(formatted, "KZTimer");

			let mode = Mode::SimpleKZ;
			let formatted = format!("{mode:?}");
			assert_eq!(formatted, "SimpleKZ");

			let mode = Mode::Vanilla;
			let formatted = format!("{mode:?}");
			assert_eq!(formatted, "Vanilla");
		}

		#[test]
		fn api() {
			let mode = Mode::KZTimer;
			let formatted = mode.api();
			assert_eq!(formatted, "kz_timer");

			let mode = Mode::SimpleKZ;
			let formatted = mode.api();
			assert_eq!(formatted, "kz_simple");

			let mode = Mode::Vanilla;
			let formatted = mode.api();
			assert_eq!(formatted, "kz_vanilla");
		}

		#[test]
		fn short() {
			let mode = Mode::KZTimer;
			let formatted = mode.short();
			assert_eq!(formatted, "KZT");

			let mode = Mode::SimpleKZ;
			let formatted = mode.short();
			assert_eq!(formatted, "SKZ");

			let mode = Mode::Vanilla;
			let formatted = mode.short();
			assert_eq!(formatted, "VNL");
		}

		#[test]
		#[allow(clippy::cognitive_complexity)]
		fn ints() {
			assert_eq!(200_u8, u8::from(Mode::KZTimer));
			assert_eq!(200_u16, u16::from(Mode::KZTimer));
			assert_eq!(200_u32, u32::from(Mode::KZTimer));
			assert_eq!(200_u64, u64::from(Mode::KZTimer));
			assert_eq!(200_u128, u128::from(Mode::KZTimer));
			assert_eq!(200_usize, usize::from(Mode::KZTimer));
			assert_eq!(200_i16, i16::from(Mode::KZTimer));
			assert_eq!(200_i32, i32::from(Mode::KZTimer));
			assert_eq!(200_i64, i64::from(Mode::KZTimer));
			assert_eq!(200_i128, i128::from(Mode::KZTimer));
			assert_eq!(200_isize, isize::from(Mode::KZTimer));

			assert_eq!(201_u8, u8::from(Mode::SimpleKZ));
			assert_eq!(201_u16, u16::from(Mode::SimpleKZ));
			assert_eq!(201_u32, u32::from(Mode::SimpleKZ));
			assert_eq!(201_u64, u64::from(Mode::SimpleKZ));
			assert_eq!(201_u128, u128::from(Mode::SimpleKZ));
			assert_eq!(201_usize, usize::from(Mode::SimpleKZ));
			assert_eq!(201_i16, i16::from(Mode::SimpleKZ));
			assert_eq!(201_i32, i32::from(Mode::SimpleKZ));
			assert_eq!(201_i64, i64::from(Mode::SimpleKZ));
			assert_eq!(201_i128, i128::from(Mode::SimpleKZ));
			assert_eq!(201_isize, isize::from(Mode::SimpleKZ));

			assert_eq!(202_u8, u8::from(Mode::Vanilla));
			assert_eq!(202_u16, u16::from(Mode::Vanilla));
			assert_eq!(202_u32, u32::from(Mode::Vanilla));
			assert_eq!(202_u64, u64::from(Mode::Vanilla));
			assert_eq!(202_u128, u128::from(Mode::Vanilla));
			assert_eq!(202_usize, usize::from(Mode::Vanilla));
			assert_eq!(202_i16, i16::from(Mode::Vanilla));
			assert_eq!(202_i32, i32::from(Mode::Vanilla));
			assert_eq!(202_i64, i64::from(Mode::Vanilla));
			assert_eq!(202_i128, i128::from(Mode::Vanilla));
			assert_eq!(202_isize, isize::from(Mode::Vanilla));
		}
	}

	mod parsing {
		use super::*;

		#[test]
		fn from_str() {
			for variant in ["200", "kztimer", "kz_timer", "kzt"] {
				let result = variant.parse::<Mode>().unwrap();
				assert_eq!(result, Mode::KZTimer);
			}

			for variant in [
				"201", "simplekz", "simple_kz", "kz_simple", "skz",
			] {
				let result = variant.parse::<Mode>().unwrap();
				assert_eq!(result, Mode::SimpleKZ);
			}

			for variant in [
				"202", "vanilla", "vanillakz", "vanilla_kz", "kz_vanilla", "vnl",
			] {
				let result = variant.parse::<Mode>().unwrap();
				assert_eq!(result, Mode::Vanilla);
			}
		}
	}

	#[cfg(feature = "serde")]
	mod serde {
		use {super::*, pretty_assertions::assert_eq};

		#[test]
		fn serialize() {
			let serialized = serde_json::to_string(&Mode::KZTimer).unwrap();
			assert_eq!(serialized, "\"kz_timer\"");
			let serialized = serde_json::to_string(&Mode::SimpleKZ).unwrap();
			assert_eq!(serialized, "\"kz_simple\"");
			let serialized = serde_json::to_string(&Mode::Vanilla).unwrap();
			assert_eq!(serialized, "\"kz_vanilla\"");
		}
	}
}
