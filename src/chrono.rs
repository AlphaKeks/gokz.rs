// I want this module to always be accessible within the crate, but because of
// feature flags the compiler keeps complaining
#![allow(unused)]

use {
	chrono::NaiveDateTime,
	serde::{Serialize, Serializer},
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
