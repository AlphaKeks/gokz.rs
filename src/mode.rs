use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// The 3 gamemodes that (currently) exist in GOKZ.
///
/// NOTE: [Official Documentation](https://github.com/KZGlobalTeam/gokz/wiki/Modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Mode {
	KZTimer = 200,
	SimpleKZ = 201,
	Vanilla = 202,
}

impl Mode {
	/// Formats a given [`Mode`] so that the GlobalAPI and KZ:GO can understand it.
	pub fn api(&self) -> String {
		String::from(match self {
			Self::KZTimer => "kz_timer",
			Self::SimpleKZ => "kz_simple",
			Self::Vanilla => "kz_vanilla",
		})
	}

	/// Abbreviates a given [`Mode`]; this is how players will usually call them.
	pub fn short(&self) -> String {
		String::from(match self {
			Self::KZTimer => "KZT",
			Self::SimpleKZ => "SKZ",
			Self::Vanilla => "VNL",
		})
	}
}

impl Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Mode::KZTimer => "KZTimer",
			Mode::SimpleKZ => "SimpleKZ",
			Mode::Vanilla => "Vanilla",
		})
	}
}

impl TryFrom<u8> for Mode {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			200 => Ok(Self::KZTimer),
			201 => Ok(Self::SimpleKZ),
			202 => Ok(Self::Vanilla),
			value => Err(Error::InvalidMode {
				value: value.to_string(),
			}),
		}
	}
}

impl From<Mode> for u8 {
	fn from(value: Mode) -> Self {
		match value {
			Mode::KZTimer => 200,
			Mode::SimpleKZ => 201,
			Mode::Vanilla => 202,
		}
	}
}

impl std::str::FromStr for Mode {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		if let Ok(mode_id) = s.parse::<u8>() {
			return mode_id.try_into();
		}

		match s.to_lowercase().as_str() {
			"kztimer" | "kz_timer" | "kzt" => Ok(Self::KZTimer),
			"simplekz" | "simple_kz" | "kz_simple" | "skz" => Ok(Self::SimpleKZ),
			"vanilla" | "vanillakz" | "vanilla_kz" | "kz_vanilla" | "vnl" => Ok(Self::Vanilla),
			input => Err(Error::InvalidMode {
				value: String::from(input),
			}),
		}
	}
}

impl Serialize for Mode {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.api())
	}
}

impl<'de> Deserialize<'de> for Mode {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.parse()
			.map_err(|why| match &why {
				Error::InvalidMode { value } => serde::de::Error::invalid_value(
					serde::de::Unexpected::Other(value),
					&why.to_string().as_str(),
				),
				_ => unreachable!(),
			})
	}
}
