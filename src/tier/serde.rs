use {
	super::Tier,
	serde::{de, Deserialize, Deserializer},
};

#[derive(Deserialize)]
#[serde(untagged)]
enum Deserializable {
	U8(u8),
	String(String),
}

impl<'de> Deserialize<'de> for Tier {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		match Deserializable::deserialize(deserializer)? {
			Deserializable::U8(mode_id) => Tier::try_from(mode_id),
			Deserializable::String(mode_name) => Tier::try_from(mode_name),
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
