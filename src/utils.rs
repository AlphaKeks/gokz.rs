#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Either<A, B> {
	A(A),
	B(B),
}

#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn serialize_date<S: Serializer>(
	date: &DateTime<Utc>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	date.format("%Y-%m-%dT%H:%M:%S")
		.to_string()
		.serialize(serializer)
}

#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn deserialize_date<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
	let date = String::deserialize(deserializer)?;
	NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S")
		.map(|date| DateTime::<Utc>::from_utc(date, Utc))
		.map_err(|err| de::Error::custom(err.to_string()))
}

#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn serialize_date_opt<S: Serializer>(
	date: &Option<DateTime<Utc>>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	match date {
		None => serializer.serialize_none(),
		Some(date) => serialize_date(date, serializer),
	}
}

#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn deserialize_date_opt<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
	Ok(match <Option<String>>::deserialize(deserializer)? {
		None => None,
		Some(date) => {
			let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S")
				.map(|date| DateTime::<Utc>::from_utc(date, Utc))
				.map_err(|err| de::Error::custom(err.to_string()))?;

			Some(date)
		}
	})
}
