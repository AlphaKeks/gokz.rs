use {
	crate::{Error, Result},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// Abstraction layer to accept either a server's name or id as function input in order to stay
/// type-safe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(untagged)]
pub enum ServerIdentifier {
	/// `"Hikari KZ"`
	Name(String),
	/// `999`
	ID(u16),
}

impl Display for ServerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ServerIdentifier::Name(name) => f.write_str(name),
			ServerIdentifier::ID(id) => f.write_str(&id.to_string()),
		}
	}
}

impl From<String> for ServerIdentifier {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}

impl std::str::FromStr for ServerIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(s.to_owned().into())
	}
}

impl From<u16> for ServerIdentifier {
	fn from(value: u16) -> Self {
		Self::ID(value)
	}
}

impl TryFrom<ServerIdentifier> for String {
	type Error = Error;

	fn try_from(value: ServerIdentifier) -> Result<Self> {
		if let ServerIdentifier::Name(server_name) = value {
			return Ok(server_name);
		}

		Err(Error::Custom("ServerIdentifier was not `Name`."))
	}
}

impl TryFrom<ServerIdentifier> for u16 {
	type Error = Error;

	fn try_from(value: ServerIdentifier) -> Result<Self> {
		if let ServerIdentifier::ID(server_id) = value {
			return Ok(server_id);
		}

		Err(Error::Custom("ServerIdentifier was not `ID`."))
	}
}

impl Serialize for ServerIdentifier {
	/// Serializes based on variant.
	/// (`Name` gets turned into a string, `ID` gets turned into a number)
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			ServerIdentifier::Name(server_name) => serializer.serialize_str(server_name),
			ServerIdentifier::ID(server_id) => serializer.serialize_u16(*server_id),
		}
	}
}

#[cfg(test)]
mod serde_tests {
	use super::*;
	use color_eyre::Result;

	#[derive(Debug, PartialEq, Serialize, Deserialize)]
	struct Server {
		ident: ServerIdentifier,
	}

	#[test]
	fn ser_server_identifier() -> Result<()> {
		let hikari_id: ServerIdentifier = 999.into();
		let hikari_name: ServerIdentifier = String::from("Hikari KZ").into();
		let hikari1 = Server {
			ident: hikari_id.clone(),
		};
		let hikari2 = Server {
			ident: hikari_name.clone(),
		};

		let serialized_id = serde_json::to_string(&hikari_id)?;
		let serialized_name = serde_json::to_string(&hikari_name)?;
		let serialized_map1 = serde_json::to_string(&hikari1)?;
		let serialized_map2 = serde_json::to_string(&hikari2)?;

		assert_eq!(serialized_id, "999");
		assert_eq!(serialized_name, "\"Hikari KZ\"");
		assert_eq!(serialized_map1, r#"{"ident":999}"#);
		assert_eq!(serialized_map2, r#"{"ident":"Hikari KZ"}"#);

		Ok(())
	}

	#[test]
	fn deser_server_identifier() -> Result<()> {
		let server_id = "999";
		let server_name = "\"Hikari KZ\"";
		let server1 = r#"{"ident":999}"#;
		let server2 = r#"{"ident":"Hikari KZ"}"#;

		let deserialized_id: ServerIdentifier = serde_json::from_str(server_id)?;
		let deserialized_name: ServerIdentifier = serde_json::from_str(server_name)?;
		let deserialized_map1: Server = serde_json::from_str(server1)?;
		let deserialized_map2: Server = serde_json::from_str(server2)?;

		assert_eq!(deserialized_id, 999.into());
		assert_eq!(deserialized_name, String::from("Hikari KZ").into());
		assert_eq!(deserialized_map1, Server { ident: 999.into() });
		assert_eq!(
			deserialized_map2,
			Server {
				ident: String::from("Hikari KZ").into()
			}
		);

		Ok(())
	}
}
