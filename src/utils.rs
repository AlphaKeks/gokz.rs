#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(all(feature = "chrono", feature = "serde"))]
use serde::{de, Deserialize, Deserializer};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
#[cfg(feature = "serde")]
use serde_json::json;

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
	date.format("%Y-%m-%dT%H:%M:%S").to_string().serialize(serializer)
}

#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn deserialize_date<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
	let date = String::deserialize(deserializer)?;
	let naive_date_time = 'scope: {
		if let Ok(ndt) = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S") {
			break 'scope ndt;
		}

		if let Ok(ndt) = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%SZ") {
			break 'scope ndt;
		}

		return Err(de::Error::custom("Failed to parse date"));
	};

	Ok(DateTime::<Utc>::from_utc(naive_date_time, Utc))
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
			let naive_date_time = 'scope: {
				if let Ok(ndt) = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S") {
					break 'scope ndt;
				}

				if let Ok(ndt) = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%SZ") {
					break 'scope ndt;
				}

				return Err(de::Error::custom("Failed to parse date"));
			};

			Some(DateTime::<Utc>::from_utc(naive_date_time, Utc))
		}
	})
}

#[cfg(feature = "serde")]
#[derive(Debug, Clone, Copy)]
pub struct EmptyParams;

#[cfg(feature = "serde")]
impl Serialize for EmptyParams {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		json!({}).serialize(serializer)
	}
}

#[cfg(all(feature = "reqwest", feature = "serde"))]
pub fn serialize_status_code<S: Serializer>(
	status_code: &reqwest::StatusCode,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	status_code.as_u16().serialize(serializer)
}
