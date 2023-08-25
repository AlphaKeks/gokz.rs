use {
	super::Runtype,
	serde::{de, Deserialize, Serialize},
};

impl Serialize for Runtype {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_bool(bool::from(*self))
	}
}

impl Runtype {
	/// Serializes [`Runtype`] as `"tp"` or `"pro"` instead of as a boolean.
	pub fn serialize_word<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(match self {
			Runtype::Pro => "pro",
			Runtype::TP => "tp",
		})
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Deserializable {
	I8(i8),
	Bool(bool),
	String(String),
}

impl<'de> Deserialize<'de> for Runtype {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		match Deserializable::deserialize(deserializer)? {
			Deserializable::I8(int) => Runtype::try_from(int),
			Deserializable::Bool(bool) => Ok(Runtype::from(bool)),
			Deserializable::String(string) => match string.as_str() {
				"true" => Ok(Runtype::TP),
				"false" => Ok(Runtype::Pro),
				_ => Runtype::try_from(string),
			},
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
