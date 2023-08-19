use {
	super::Tier,
	serde::{de, Deserialize, Deserializer, Serialize},
};

impl Serialize for Tier {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		(*self as u8).serialize(serializer)
	}
}

impl Tier {
	/// [`serde`] function to serialize a [`Tier`] with its name rather than as a number.
	pub fn serialize_name<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_string().serialize(serializer)
	}
}

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
