//! This module contains an enum for the map tiers.

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

/// The 7 current map tiers in CS:GO KZ.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "Tier", rename_all = "snake_case"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(rename_all = "snake_case"))]
pub enum Tier {
	VeryEasy = 1,
	Easy = 2,
	Medium = 3,
	Hard = 4,
	VeryHard = 5,
	Extreme = 6,
	Death = 7,
}

impl Display for Tier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Tier::VeryEasy => "very_easy",
			Tier::Easy => "easy",
			Tier::Medium => "medium",
			Tier::Hard => "hard",
			Tier::VeryHard => "very_hard",
			Tier::Extreme => "extreme",
			Tier::Death => "death",
		})
	}
}

#[rustfmt::skip]
impl Tier {
	is!(is_very_easy, VeryEasy);
	is!(is_easy, Easy);
	is!(is_medium, Medium);
	is!(is_hard, Hard);
	is!(is_very_hard, VeryHard);
	is!(is_extreme, Extreme);
	is!(is_death, Death);
}

try_from!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => Tier => |int| {
	Ok(match int {
		1 => Tier::VeryEasy,
		2 => Tier::Easy,
		3 => Tier::Medium,
		4 => Tier::Hard,
		5 => Tier::VeryHard,
		6 => Tier::Extreme,
		7 => Tier::Death,
		int => yeet!(InvalidTier(int))
	})
});

from!(Tier => [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => |tier| {
	tier as _
});

impl TryFrom<&str> for Tier {
	type Error = crate::Error;

	fn try_from(input: &str) -> crate::Result<Self> {
		FromStr::from_str(input)
	}
}

impl TryFrom<String> for Tier {
	type Error = crate::Error;

	fn try_from(input: String) -> crate::Result<Self> {
		Self::try_from(input.as_str())
	}
}

impl FromStr for Tier {
	type Err = crate::Error;

	fn from_str(input: &str) -> crate::Result<Self> {
		Ok(match input.to_lowercase().as_str() {
			"1" | "veryeasy" | "very_easy" | "very easy" => Self::VeryEasy,
			"2" | "easy" => Self::Easy,
			"3" | "medium" => Self::Medium,
			"4" | "hard" => Self::Hard,
			"5" | "veryhard" | "very_hard" | "very hard" => Self::VeryHard,
			"6" | "extreme" => Self::Extreme,
			"7" | "death" => Self::Death,
			_ => yeet!(InvalidTier(input)),
		})
	}
}
