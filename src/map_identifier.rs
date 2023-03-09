use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

pub type MapName = String;
pub type MapID = u16;

/// Abstraction layer to accept either a map's name or id as function input in order to stay
/// type-safe without unnecessary conversions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapIdentifier {
	/// `"kz_lionharder"`
	Name(MapName),
	/// `992`
	ID(MapID),
}

impl Display for MapIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MapIdentifier::Name(name) => f.write_str(name),
			MapIdentifier::ID(id) => f.write_str(&id.to_string()),
		}
	}
}

impl From<MapName> for MapIdentifier {
	fn from(value: MapName) -> Self {
		Self::Name(value)
	}
}

impl std::str::FromStr for MapIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(s.to_owned().into())
	}
}

impl From<MapID> for MapIdentifier {
	fn from(value: MapID) -> Self {
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

impl TryFrom<MapIdentifier> for MapID {
	type Error = Error;

	fn try_from(value: MapIdentifier) -> Result<Self> {
		if let MapIdentifier::ID(map_id) = value {
			return Ok(map_id);
		}

		Err(Error::Custom("MapIdentifier was not `ID`."))
	}
}

impl Serialize for MapIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for MapIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(String::deserialize(deserializer)?
			.parse()
			.expect("Infallible"))
	}
}
