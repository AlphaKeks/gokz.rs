//! This module contains various functions and submodules covering the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

/// The base URL for all API requests.
pub const API_URL: &str = "https://kztimerglobal.com/api/v2";

/// The URL for the API's SwaggerUI website.
pub const SWAGGER_URL: &str = "https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2";

pub mod health;
pub use health::{healthcheck, Health};

pub mod bans;
pub use bans::get_bans;

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

	pub(super) fn serialize_ban_types<S: Serializer>(
		ban_types: &Option<Vec<super::bans::BanType>>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		use std::fmt::Write;

		let Some(ban_types) = ban_types else {
			return serializer.serialize_none();
		};

		let mut s = String::new();

		for ban_type in ban_types {
			write!(&mut s, "{ban_type:?},").expect("This never fails");
		}

		s.pop();
		s.serialize(serializer)
	}

	pub(super) fn deserialize_ban_types<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Option<Vec<super::bans::BanType>>, D::Error> {
		Ok(Some(
			String::deserialize(deserializer)?
				.split(',')
				.map(|ban_type| match ban_type {
					"bhop_hack" => super::bans::BanType::BhopHack,
					"bhop_macro" => super::bans::BanType::BhopMacro,
					"strafe_hack" => super::bans::BanType::StrafeHack,
					"ban_evasion" => super::bans::BanType::BanEvasion,
					_ => super::bans::BanType::Other,
				})
				.collect(),
		))
	}
}
