//! This module contains various functions and submodules covering
//! [DawnAPI](https://dawn.sh/api/docs/swagger-ui).

/// The base URL for all API requests.
pub const API_URL: &str = "https://dawn.sh/api/kz";

/// The URL for the API's SwaggerUI website.
pub const SWAGGER_URL: &str = "https://dawn.sh/api/docs/swagger-ui";

pub mod health;
pub use health::healthcheck;

pub mod maps;
pub use maps::{get_map, get_maps, get_maps_by, get_maps_with, Map};

pub mod servers;
pub use servers::{get_server, get_servers_owned_by, get_servers_with, Server};

pub mod players;
pub use players::{
	get_player, get_players_with, Completion, CompletionCount, Player, PlayerWithCompletion,
};

pub mod records;
pub use records::{get_maptop, get_pb, get_record, get_records_with, get_wr, Record};

mod serde {
	pub mod chrono {
		use {
			::chrono::{DateTime, NaiveDateTime, Utc},
			serde::{de, Deserialize, Deserializer, Serialize, Serializer},
		};

		pub fn serialize_date<S: Serializer>(
			date: &DateTime<Utc>,
			serializer: S,
		) -> Result<S::Ok, S::Error> {
			date.format("%Y-%m-%dT%H:%M:%SZ")
				.to_string()
				.serialize(serializer)
		}

		pub fn deserialize_date<'de, D: Deserializer<'de>>(
			deserializer: D,
		) -> Result<DateTime<Utc>, D::Error> {
			let date = String::deserialize(deserializer)?;
			let ndt = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%SZ")
				.map_err(|err| de::Error::custom(err.to_string()))?;

			Ok(DateTime::<Utc>::from_utc(ndt, Utc))
		}
	}
}
