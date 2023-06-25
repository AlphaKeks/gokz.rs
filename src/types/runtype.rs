use crate::error::{err, Error, Result};

/// A KZ record / run can either be done with teleports or without. A run without teleports is
/// generally referred to as a "pro" run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Runtype {
	/// The run was done with teleports.
	TP = 1,

	/// The run was done without teleports.
	Pro = 0,
}

impl Default for Runtype {
	fn default() -> Self { Self::Pro }
}

impl std::fmt::Display for Runtype {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// The `Debug` representation is the same as the variant's name, so this is convenient.
		write!(f, "{self:?}")
	}
}

impl From<bool> for Runtype {
	fn from(b: bool) -> Self {
		match b {
			true => Self::TP,
			false => Self::Pro,
		}
	}
}

impl From<Runtype> for bool {
	fn from(runtype: Runtype) -> Self { runtype == Runtype::TP }
}

impl std::ops::Deref for Runtype {
	type Target = bool;

	fn deref(&self) -> &Self::Target {
		match Self::TP.eq(self) {
			true => &true,
			false => &false,
		}
	}
}

macro_rules! from_int {
	($($int:ty), *) => {
		$(impl From<$int> for $crate::types::Runtype {
			fn from(int: $int) -> $crate::types::Runtype {
				if int == 0 {
					$crate::types::Runtype::Pro
				} else {
					$crate::types::Runtype::TP
				}
			}
		})*
	};
}

from_int!(u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);

impl std::str::FromStr for Runtype {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"tp" => Runtype::TP,
			"pro" => Runtype::Pro,
			input => return Err(err!("`{input}` is not a valid Runtype.")),
		})
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Runtype {
	#[tracing::instrument(level = "DEBUG", skip(serializer), err(Debug))]
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			Runtype::TP => true.serialize(serializer),
			Runtype::Pro => false.serialize(serializer),
		}
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Runtype {
	#[tracing::instrument(level = "DEBUG", skip(deserializer), err(Debug))]
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(if bool::deserialize(deserializer)? { Runtype::TP } else { Runtype::Pro })
	}
}
