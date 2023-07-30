use {
	super::MapIdentifier,
	serde::{de, Deserialize, Deserializer},
};

#[derive(Deserialize)]
#[serde(untagged)]
enum Deserializable {
	U16(u16),
	String(String),
}

impl<'de> Deserialize<'de> for MapIdentifier {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		match Deserializable::deserialize(deserializer)? {
			Deserializable::U16(map_id) => MapIdentifier::try_from(map_id),
			Deserializable::String(map_name) => Ok(MapIdentifier::from(map_name)),
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
