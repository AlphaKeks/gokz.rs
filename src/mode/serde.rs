use {
	super::Mode,
	serde::{de, Deserialize, Deserializer, Serialize, Serializer},
};

impl Serialize for Mode {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.api().serialize(serializer)
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Deserializable {
	U8(u8),
	String(String),
}

impl<'de> Deserialize<'de> for Mode {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		match Deserializable::deserialize(deserializer)? {
			Deserializable::U8(mode_id) => Mode::try_from(mode_id),
			Deserializable::String(mode_name) => Mode::try_from(mode_name),
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
