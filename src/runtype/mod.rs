//! This module contains an enum for TP / PRO.

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

/// The two runtypes.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Runtype {
	/// The run was done without teleports.
	#[default]
	Pro = 0,

	/// The run was done with teleports.
	TP = 1,
}

impl Display for Runtype {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

#[rustfmt::skip]
impl Runtype {
	is!(is_pro, Pro);
	is!(is_tp, TP);
}

impl From<bool> for Runtype {
	fn from(value: bool) -> Self {
		match value {
			true => Runtype::TP,
			false => Runtype::Pro,
		}
	}
}

impl From<Runtype> for bool {
	fn from(value: Runtype) -> Self {
		value.is_tp()
	}
}

from!([u8, u16, u32, u64, u128, usize] => Runtype => |int| {
	match int > 0 {
		true => Runtype::TP,
		false => Runtype::Pro,
	}
});

try_from!([i8, i16, i32, i64, i128, isize] => Runtype => |int| {
	use std::cmp::Ordering::*;
	match int.cmp(&0) {
		Less => yeet!(InvalidTeleportAmount),
		Equal => Ok(Runtype::Pro),
		Greater => Ok(Runtype::TP),
	}
});

impl TryFrom<&str> for Runtype {
	type Error = crate::Error;

	fn try_from(input: &str) -> crate::Result<Self> {
		FromStr::from_str(input)
	}
}

impl TryFrom<String> for Runtype {
	type Error = crate::Error;

	fn try_from(input: String) -> crate::Result<Self> {
		Self::try_from(input.as_str())
	}
}

impl FromStr for Runtype {
	type Err = crate::Error;

	fn from_str(input: &str) -> crate::Result<Self> {
		Ok(match input {
			"PRO" | "PRo" | "PrO" | "pRO" | "Pro" | "pRo" | "prO" | "pro" => Self::Pro,
			"TP" | "Tp" | "tP" | "tp" => Self::TP,
			_ => yeet!(InvalidRuntype(input)),
		})
	}
}
