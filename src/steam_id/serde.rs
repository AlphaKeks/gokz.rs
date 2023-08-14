use {
	super::SteamID,
	serde::{de, Deserialize, Deserializer, Serialize, Serializer},
};

impl Serialize for SteamID {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_string().serialize(serializer)
	}
}

impl SteamID {
	/// Method that matches [`serde`]'s `serialize` signature.
	pub fn serialize_as_u64<S: Serializer>(
		steam_id: &Self,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		steam_id.as_id64().serialize(serializer)
	}

	/// Method that matches [`serde`]'s `serialize` signature.
	pub fn serialize_opt_as_u64<S: Serializer>(
		steam_id: &Option<Self>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		match steam_id {
			None => serializer.serialize_none(),
			Some(steam_id) => serializer.serialize_u64(steam_id.as_id64()),
		}
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Deserializable {
	U32(u32),
	U64(u64),
	String(String),
}

impl<'de> Deserialize<'de> for SteamID {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		match Deserializable::deserialize(deserializer)? {
			Deserializable::U32(steam_id32) => SteamID::try_from(steam_id32),
			Deserializable::U64(steam_id64) => SteamID::try_from(steam_id64),
			Deserializable::String(steam_id) => SteamID::new(steam_id),
		}
		.map_err(|err| de::Error::custom(err.to_string()))
	}
}
