//! This module contains the 3 KZ modes.
//!
//! Official documentation: <https://github.com/KZGlobalTeam/gokz/wiki/Modes>

use {
	crate::{
		macros::{
			convert::{from, try_from},
			is,
		},
		yeet,
	},
	std::{fmt::Display, str::FromStr},
};

#[cfg(feature = "serde")]
mod serde;

#[cfg(all(feature = "serde", test))]
mod serde_tests;

/// The 3 game modes in CS:GO KZ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
	/// The default mode. Most people play this.
	KZTimer = 200,

	/// The mode for based individuals.
	SimpleKZ = 201,

	/// In case you really hate yourself.
	Vanilla = 202,
}

impl Mode {
	/// Format the given [`Mode`] to be used in query parameters for various APIs.
	pub const fn api(&self) -> &'static str {
		match self {
			Mode::KZTimer => "kz_timer",
			Mode::SimpleKZ => "kz_simple",
			Mode::Vanilla => "kz_vanilla",
		}
	}

	/// Shortened name for the given [`Mode`].
	pub const fn short(&self) -> &'static str {
		match self {
			Mode::KZTimer => "KZT",
			Mode::SimpleKZ => "SKZ",
			Mode::Vanilla => "VNL",
		}
	}
}

#[rustfmt::skip]
impl Mode {
	is!(is_kzt, KZTimer);
	is!(is_skz, SimpleKZ);
	is!(is_vnl, Vanilla);
}

impl Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl Default for Mode {
	#[rustfmt::skip]
	fn default() -> Self { Self::KZTimer }
}

from!(Mode => [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => |mode| {
	mode as _
});

try_from!([u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => Mode => |int| {
	Ok(match int {
		200 => Mode::KZTimer,
		201 => Mode::SimpleKZ,
		202 => Mode::Vanilla,
		int => yeet!(InvalidMode(int)),
	})
});

impl TryFrom<&str> for Mode {
	type Error = crate::Error;

	fn try_from(input: &str) -> crate::Result<Self> {
		FromStr::from_str(input)
	}
}

impl TryFrom<String> for Mode {
	type Error = crate::Error;

	fn try_from(input: String) -> crate::Result<Self> {
		Self::try_from(input.as_str())
	}
}

impl FromStr for Mode {
	type Err = crate::Error;

	fn from_str(input: &str) -> crate::Result<Self> {
		Ok(match input.to_lowercase().as_str() {
			"200" | "kztimer" | "kz_timer" | "kzt" => Self::KZTimer,
			"201" | "simplekz" | "kz_simple" | "skz" | "simple_kz" => Self::SimpleKZ,
			"202" | "vanilla" | "kz_vanilla" | "vnl" | "vanilla_kz" | "vanillakz" => Self::Vanilla,
			_ => yeet!(InvalidMode(input)),
		})
	}
}
