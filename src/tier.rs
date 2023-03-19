use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// The current 7 difficulty categories that all maps fall into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
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
			Tier::VeryEasy => "Very Easy",
			Tier::Easy => "Easy",
			Tier::Medium => "Medium",
			Tier::Hard => "Hard",
			Tier::VeryHard => "Very Hard",
			Tier::Extreme => "Extreme",
			Tier::Death => "Death",
		})
	}
}

impl std::str::FromStr for Tier {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().as_str() {
			"very easy" | "very_easy" => Ok(Self::VeryEasy),
			"easy" => Ok(Self::Easy),
			"medium" => Ok(Self::Medium),
			"hard" => Ok(Self::Hard),
			"very hard" | "very_hard" => Ok(Self::VeryHard),
			"extreme" => Ok(Self::Extreme),
			"death" => Ok(Self::Death),
			input => Err(Error::InvalidTier {
				value: String::from(input),
			}),
		}
	}
}

impl TryFrom<u8> for Tier {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			1 => Ok(Self::VeryEasy),
			2 => Ok(Self::Easy),
			3 => Ok(Self::Medium),
			4 => Ok(Self::Hard),
			5 => Ok(Self::VeryHard),
			6 => Ok(Self::Extreme),
			7 => Ok(Self::Death),
			input => Err(Error::InvalidTier {
				value: input.to_string(),
			}),
		}
	}
}

impl From<Tier> for u8 {
	fn from(val: Tier) -> Self {
		match val {
			Tier::VeryEasy => 1,
			Tier::Easy => 2,
			Tier::Medium => 3,
			Tier::Hard => 4,
			Tier::VeryHard => 5,
			Tier::Extreme => 6,
			Tier::Death => 7,
		}
	}
}

impl Serialize for Tier {
	/// Serializes [`Self`] as [`u8`].
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u8(*self as u8)
	}
}

impl<'de> Deserialize<'de> for Tier {
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
			StringOrU8::Name(tier_name) => tier_name.parse(),
			StringOrU8::U8(tier_number) => tier_number.try_into(),
		}
		.map_err(|why| match &why {
			Error::InvalidTier { value } => serde::de::Error::invalid_value(
				serde::de::Unexpected::Other(value),
				&why.to_string().as_str(),
			),
			other => unreachable!("Encountered `{other}` while deserializing into `Tier`."),
		})
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Debug, PartialEq, Serialize, Deserialize)]
	struct T {
		mode: Tier,
	}

	#[test]
	fn ser_tier() -> Result<()> {
		let easy = Tier::Easy;
		let extreme = Tier::Extreme;
		let death = Tier::Death;

		let serialized_easy = serde_json::to_string(&easy)?;
		let serialized_extreme = serde_json::to_string(&extreme)?;
		let serialized_death = serde_json::to_string(&death)?;

		assert_eq!(serialized_easy, "2");
		assert_eq!(serialized_extreme, "6");
		assert_eq!(serialized_death, "7");

		Ok(())
	}

	#[test]
	fn deser_tier() -> Result<()> {
		let deserialized_death_number: Tier = serde_json::from_str("7")?;
		let deserialized_death_name: Tier = serde_json::from_str("\"Death\"")?;

		assert_eq!(deserialized_death_number, Tier::Death);
		assert_eq!(deserialized_death_name, Tier::Death);

		Ok(())
	}
}
