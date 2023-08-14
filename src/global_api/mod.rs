//! This module contains various functions and submodules covering the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

/// The base URL for all API requests.
pub const API_URL: &str = "https://kztimerglobal.com/api/v2";

/// The URL for the API's SwaggerUI website.
pub const SWAGGER_URL: &str = "https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2";

pub mod health;
pub use health::{healthcheck, Health};

pub mod bans;
pub use bans::{get_bans_with, Ban};

pub mod maps;
pub use maps::{get_map, get_maps, get_maps_with, Map};

pub mod servers;
pub use servers::{get_servers_owned_by, get_servers_with, Server};

pub mod players;
pub use players::{get_player, get_players_with, Player};

pub mod filters;
pub use filters::{get_filters_with, RecordFilter};

mod serde {
	use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

	#[cfg(feature = "chrono")]
	pub(super) mod chrono {
		use {
			super::*,
			::chrono::{DateTime, NaiveDateTime, Utc},
		};

		pub fn serialize_date<S: Serializer>(
			date: &DateTime<Utc>,
			serializer: S,
		) -> Result<S::Ok, S::Error> {
			date.format("%Y-%m-%dT%H:%M:%S")
				.to_string()
				.serialize(serializer)
		}

		pub fn deserialize_date<'de, D: Deserializer<'de>>(
			deserializer: D,
		) -> Result<DateTime<Utc>, D::Error> {
			let date = String::deserialize(deserializer)?;
			let ndt = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S")
				.map_err(|err| de::Error::custom(err.to_string()))?;

			Ok(DateTime::<Utc>::from_utc(ndt, Utc))
		}

		pub fn serialize_date_opt<S: Serializer>(
			date: &Option<DateTime<Utc>>,
			serializer: S,
		) -> Result<S::Ok, S::Error> {
			match date {
				Some(date) => date.serialize(serializer),
				None => serializer.serialize_none(),
			}
		}

		pub fn deserialize_date_opt<'de, D: Deserializer<'de>>(
			deserializer: D,
		) -> Result<Option<DateTime<Utc>>, D::Error> {
			let Some(date) = Option::<String>::deserialize(deserializer)? else {
				return Ok(None);
			};

			let ndt = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S")
				.map_err(|err| de::Error::custom(err.to_string()))?;

			Ok(Some(DateTime::<Utc>::from_utc(ndt, Utc)))
		}
	}

	macro_rules! append_pairs {
		($url:expr, $value:expr, $name:expr) => {{
			if let Some(items) = $value {
				let mut query = $url.query_pairs_mut();
				for item in items {
					query.append_pair($name, &item.to_string());
				}
			}
		}};
	}

	pub(super) use append_pairs;
}
