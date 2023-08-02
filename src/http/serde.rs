//! Utility functions for interop with [`serde`].

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Serializes a [`reqwest::StatusCode`] using [`serde`].
pub fn serialize_status_code<S: Serializer>(
	status_code: &Option<reqwest::StatusCode>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	status_code
		.map(|code| code.as_u16())
		.serialize(serializer)
}

/// Deserializes a [`reqwest::StatusCode`] using [`serde`].
pub fn deserialize_status_code<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> Result<Option<reqwest::StatusCode>, D::Error> {
	Option::<u16>::deserialize(deserializer)?
		.map(|code| {
			reqwest::StatusCode::from_u16(code).map_err(|err| de::Error::custom(err.to_string()))
		})
		.transpose()
}
