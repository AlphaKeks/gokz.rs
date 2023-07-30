use {
	super::Runtype,
	serde::{de, Deserialize},
};

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
			Deserializable::String(string) => Runtype::try_from(string),
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
