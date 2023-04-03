use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::{fmt::Display, str::FromStr},
};

/// Abstraction layer to accept either a map's name or id as function input in order to stay
/// type-safe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapIdentifier {
	/// `992`
	ID(u16),
	/// `"kz_lionharder"`
	Name(String),
}

impl Display for MapIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MapIdentifier::Name(name) => f.write_str(name),
			MapIdentifier::ID(id) => f.write_str(&id.to_string()),
		}
	}
}

impl From<String> for MapIdentifier {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}

impl FromStr for MapIdentifier {
	type Err = Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		if s.is_empty() {
			return Err(Error::EmptyInput);
		}

		if let Ok(map_id) = s.parse::<u16>() {
			return Ok(map_id.into());
		}

		Ok(s.to_owned().into())
	}
}

impl From<u16> for MapIdentifier {
	fn from(value: u16) -> Self {
		Self::ID(value)
	}
}

impl TryFrom<MapIdentifier> for String {
	type Error = Error;

	fn try_from(value: MapIdentifier) -> Result<Self> {
		if let MapIdentifier::Name(map_name) = value {
			return Ok(map_name);
		}

		Err(Error::Custom("MapIdentifier was not `Name`."))
	}
}

impl TryFrom<MapIdentifier> for u16 {
	type Error = Error;

	fn try_from(value: MapIdentifier) -> Result<Self> {
		if let MapIdentifier::ID(map_id) = value {
			return Ok(map_id);
		}

		Err(Error::Custom("MapIdentifier was not `ID`."))
	}
}

impl Serialize for MapIdentifier {
	/// Serializes based on variant.
	/// (`Name` gets turned into a string, `ID` gets turned into a number)
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			MapIdentifier::Name(map_name) => serializer.serialize_str(map_name),
			MapIdentifier::ID(map_id) => serializer.serialize_u16(*map_id),
		}
	}
}

impl<'de> Deserialize<'de> for MapIdentifier {
	/// Deserializes the input either as [`String`] or [`u16`] and then turns that into [`Self`].
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum StringOrU16 {
			Name(String),
			U16(u16),
		}

		Ok(match StringOrU16::deserialize(deserializer)? {
			StringOrU16::Name(map_name) => map_name.parse::<Self>().map_err(|_| {
				serde::de::Error::invalid_value(
					serde::de::Unexpected::Str("empty string"),
					&"map identifier",
				)
			})?,
			StringOrU16::U16(map_id) => map_id.into(),
		})
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Debug, PartialEq, Serialize, Deserialize)]
	struct Map {
		ident: MapIdentifier,
	}

	#[test]
	fn ser_map_identifier() -> Result<()> {
		let lionharder_id: MapIdentifier = 992.into();
		let lionharder_name: MapIdentifier = String::from("kz_lionharder").into();
		let lionharder1 = Map {
			ident: lionharder_id.clone(),
		};
		let lionharder2 = Map {
			ident: lionharder_name.clone(),
		};

		let serialized_id = serde_json::to_string(&lionharder_id)?;
		let serialized_name = serde_json::to_string(&lionharder_name)?;
		let serialized_map1 = serde_json::to_string(&lionharder1)?;
		let serialized_map2 = serde_json::to_string(&lionharder2)?;

		assert_eq!(serialized_id, "992");
		assert_eq!(serialized_name, "\"kz_lionharder\"");
		assert_eq!(serialized_map1, r#"{"ident":992}"#);
		assert_eq!(serialized_map2, r#"{"ident":"kz_lionharder"}"#);

		Ok(())
	}

	#[test]
	fn deser_map_identifier() -> Result<()> {
		let map_id = "992";
		let map_name = "\"kz_lionharder\"";
		let map1 = r#"{"ident":992}"#;
		let map2 = r#"{"ident":"kz_lionharder"}"#;

		let deserialized_id: MapIdentifier = serde_json::from_str(map_id)?;
		let deserialized_name: MapIdentifier = serde_json::from_str(map_name)?;
		let deserialized_map1: Map = serde_json::from_str(map1)?;
		let deserialized_map2: Map = serde_json::from_str(map2)?;

		assert_eq!(
			MapIdentifier::ID(992),
			serde_json::from_value(serde_json::json!(992))?
		);
		assert_eq!(
			MapIdentifier::ID(992),
			serde_json::from_value(serde_json::json!("992"))?
		);
		assert_eq!(deserialized_id, 992.into());
		assert_eq!(deserialized_name, String::from("kz_lionharder").into());
		assert_eq!(deserialized_map1, Map { ident: 992.into() });
		assert_eq!(
			deserialized_map2,
			Map {
				ident: String::from("kz_lionharder").into()
			}
		);

		Ok(())
	}
}
