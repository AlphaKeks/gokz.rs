use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::{fmt::Display, str::FromStr},
};

/// The 3 gamemodes that (currently) exist in GOKZ.
///
/// NOTE: [Official Documentation](https://github.com/KZGlobalTeam/gokz/wiki/Modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
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

impl FromStr for Mode {
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
	/// Serializes [`Self`] as [`String`] using [`Self::api`].
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.api())
	}
}

impl<'de> Deserialize<'de> for Mode {
	/// Deserializes the input either as [`String`] or [`u8`] and then turns that into [`Self`].
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum StringOrU8 {
			Name(String),
			U8(u8),
		}

		match StringOrU8::deserialize(deserializer)? {
			StringOrU8::Name(mode_name) => mode_name.parse(),
			StringOrU8::U8(mode_id) => mode_id.try_into(),
		}
		.map_err(|why| match &why {
			Error::InvalidMode { value } => serde::de::Error::invalid_value(
				serde::de::Unexpected::Other(value),
				&why.to_string().as_str(),
			),
			other => unreachable!("Encountered `{other}` while deserializing into `Mode`."),
		})
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Debug, PartialEq, Serialize, Deserialize)]
	struct M {
		mode: Mode,
	}

	#[test]
	fn ser_mode() -> Result<()> {
		let kzt = Mode::KZTimer;
		let skz = Mode::SimpleKZ;
		let vnl = Mode::Vanilla;

		let serialized_kzt = serde_json::to_string(&kzt)?;
		let serialized_skz = serde_json::to_string(&skz)?;
		let serialized_vnl = serde_json::to_string(&vnl)?;

		assert_eq!(serialized_kzt, "\"kz_timer\"");
		assert_eq!(serialized_skz, "\"kz_simple\"");
		assert_eq!(serialized_vnl, "\"kz_vanilla\"");

		Ok(())
	}

	#[test]
	fn deser_mode() -> Result<()> {
		let deserialized_kzt_id: Mode = serde_json::from_str("200")?;
		let deserialized_kzt_name: Mode = serde_json::from_str("\"kz_timer\"")?;

		assert_eq!(deserialized_kzt_id, Mode::KZTimer);
		assert_eq!(deserialized_kzt_name, Mode::KZTimer);

		let deserialized_skz_id: Mode = serde_json::from_str("201")?;
		let deserialized_skz_name: Mode = serde_json::from_str("\"kz_simple\"")?;

		assert_eq!(deserialized_skz_id, Mode::SimpleKZ);
		assert_eq!(deserialized_skz_name, Mode::SimpleKZ);

		let deserialized_vnl_id: Mode = serde_json::from_str("202")?;
		let deserialized_vnl_name: Mode = serde_json::from_str("\"kz_vanilla\"")?;

		assert_eq!(deserialized_vnl_id, Mode::Vanilla);
		assert_eq!(deserialized_vnl_name, Mode::Vanilla);

		Ok(())
	}
}
