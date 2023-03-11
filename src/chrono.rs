// I want this module to always be accessible within the crate, but because of feature flags the
// compiler keeps complaining
#![allow(unused)]

use {
	chrono::NaiveDateTime,
	serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// Parses a date from the GlobalAPI
macro_rules! parse_date {
	($date:expr) => {{
		use {chrono::NaiveDateTime, $crate::Error};

		NaiveDateTime::parse_from_str(&$date, "%Y-%m-%dT%H:%M:%S")
			.map_err(|_| Error::InvalidDate { value: $date })?
	}};
}

pub(crate) use parse_date;

pub(crate) fn ser_date<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	date.to_string().serialize(serializer)
}

pub(crate) fn ser_opt_date<S>(
	date: &Option<NaiveDateTime>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	if let Some(date) = date {
		date.to_string().serialize(serializer)
	} else {
		serializer.serialize_none()
	}
}

pub fn deser_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
	D: Deserializer<'de>,
{
	let date = String::deserialize(deserializer)?;
	NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S").map_err(|_| {
		serde::de::Error::invalid_value(
			serde::de::Unexpected::Other(&date),
			&"Date with format `%Y-%m-%dT%H:%M:%S`",
		)
	})
}
