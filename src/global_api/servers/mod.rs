use {
	crate::{http, Error, Result, SteamID},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

pub type ServerName = String;
pub type ServerID = u16;

/// Abstraction layer to accept either a server's name or id as function input in order to stay
/// type-safe without unnecessary conversions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServerIdentifier {
	Name(ServerName),
	ID(ServerID),
}

impl Display for ServerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ServerIdentifier::Name(name) => f.write_str(name),
			ServerIdentifier::ID(id) => f.write_str(&id.to_string()),
		}
	}
}

impl From<ServerName> for ServerIdentifier {
	fn from(value: ServerName) -> Self {
		Self::Name(value)
	}
}

impl std::str::FromStr for ServerIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(s.to_owned().into())
	}
}

impl From<ServerID> for ServerIdentifier {
	fn from(value: ServerID) -> Self {
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

impl TryFrom<ServerIdentifier> for ServerID {
	type Error = Error;

	fn try_from(value: ServerIdentifier) -> Result<Self> {
		if let ServerIdentifier::ID(server_id) = value {
			return Ok(server_id);
		}

		Err(Error::Custom("ServerIdentifier was not `ID`."))
	}
}

impl Serialize for ServerIdentifier {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for ServerIdentifier {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(String::deserialize(deserializer)?
			.parse()
			.expect("Infallible"))
	}
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Server {
	pub id: ServerID,
	pub name: String,
	pub owner_steamid: SteamID,
	pub ip: String,
	pub port: u16,
}

/// `/maps` route
pub mod index;
impl TryFrom<index::Response> for Server {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		Ok(Self {
			id: value.id.try_into()?,
			name: value.name,
			owner_steamid: value.owner_steamid64.parse()?,
			ip: value.ip,
			port: value.port.try_into()?,
		})
	}
}

/// Fetches servers with the given `params`.
pub async fn get_servers(params: index::Params, client: &crate::Client) -> Result<Vec<Server>> {
	let response: Vec<index::Response> =
		http::get_with_params(&format!("{}/servers", super::BASE_URL), params, client).await?;

	if response.is_empty() {
		return Err(Error::EmptyResponse);
	}

	Ok(response
		.into_iter()
		.filter_map(|res| res.try_into().ok())
		.collect())
}

/// Fetches a server by its name.
pub async fn get_server_by_name(server_name: &str, client: &crate::Client) -> Result<Server> {
	http::get::<index::Response>(
		&format!("{}/servers/name/{}", super::BASE_URL, server_name),
		client,
	)
	.await?
	.try_into()
}

/// Fetches a server by its ID.
pub async fn get_server_by_id(server_id: ServerID, client: &crate::Client) -> Result<Server> {
	http::get::<index::Response>(
		&format!("{}/servers/id/{}", super::BASE_URL, server_id),
		client,
	)
	.await?
	.try_into()
}
