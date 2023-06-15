use crate::error::{err, Error, Result};

/// The 7 levels of difficulty a global KZ map can have.
#[allow(missing_docs)] // These should be self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize),
	serde(rename_all = "snake_case")
)]
pub enum Tier {
	VeryEasy = 1,
	Easy = 2,
	Medium = 3,
	Hard = 4,
	VeryHard = 5,
	Extreme = 6,
	Death = 7,
}

impl std::fmt::Display for Tier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tier::VeryEasy => f.write_str("Very Easy"),
			Tier::VeryHard => f.write_str("Very Hard"),
			Tier::Easy | Tier::Medium | Tier::Hard | Tier::Extreme | Tier::Death => {
				write!(f, "{self:?}")
			}
		}
	}
}

macro_rules! into_int {
	($($int:ty), *) => {
		$(impl From<$crate::types::Tier> for $int {
			fn from(mode: $crate::types::Tier) -> $int {
				mode as $int
			}
		})*
	};
}

into_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! try_from_int {
	($($int:ty), *) => {
		$(impl TryFrom<$int> for $crate::types::Tier {
			type Error = $crate::error::Error;

			fn try_from(
				int: $int,
			) -> ::std::result::Result<$crate::types::Tier, Self::Error> {
				Ok(match int {
					1 => $crate::types::Tier::VeryEasy,
					2 => $crate::types::Tier::Easy,
					3 => $crate::types::Tier::Medium,
					4 => $crate::types::Tier::Hard,
					5 => $crate::types::Tier::VeryHard,
					6 => $crate::types::Tier::Extreme,
					7 => $crate::types::Tier::Death,
					int => {
						return Err($crate::error::err!("`{int}` is not a valid tier."));
					}
				})
			}
		})*
	};
}

try_from_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl std::str::FromStr for Tier {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"very easy" | "very_easy" => Self::VeryEasy,
			"easy" => Self::Easy,
			"medium" => Self::Medium,
			"hard" => Self::Hard,
			"very hard" | "very_hard" => Self::VeryHard,
			"extreme" => Self::Extreme,
			"death" => Self::Death,
			input => return Err(err!("`{input}` is not a valid tier.")),
		})
	}
}
