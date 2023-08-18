#![allow(missing_docs)]

#[cfg(feature = "chrono")]
pub mod chrono {
	use {
		::chrono::{DateTime, NaiveDateTime, Utc},
		serde::{de, Deserialize, Deserializer, Serialize, Serializer},
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

#[cfg(feature = "kzgo-api")]
pub mod kzgo {
	use {
		crate::SteamID,
		serde::{de, Deserialize, Deserializer},
	};

	pub fn deserialize_mapper_names<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Vec<String>, D::Error> {
		Ok(Vec::<Option<String>>::deserialize(deserializer)?
			.into_iter()
			.flatten()
			.filter(|s| !s.is_empty())
			.collect())
	}

	pub fn deserialize_mapper_ids<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Vec<SteamID>, D::Error> {
		Ok(Vec::<Option<String>>::deserialize(deserializer)?
			.into_iter()
			.flatten()
			.flat_map(|s| s.parse())
			.collect())
	}

	pub fn deserialize_workshop_id<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Option<u32>, D::Error> {
		let Some(id) = Option::<String>::deserialize(deserializer)? else {
			return Ok(None);
		};

		let id = id
			.parse::<u32>()
			.map_err(|_| de::Error::invalid_type(de::Unexpected::Str("string"), &"u32"))?;

		Ok(Some(id))
	}
}
