use {
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// For some reason [`reqwest::StatusCode`] does not implement [`serde::Serialize`] so I guess I
/// have to make my own wrapper for it...
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(pub(crate) reqwest::StatusCode);

impl Display for StatusCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.0.as_str())
	}
}

impl Serialize for StatusCode {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(self.0.as_str())
	}
}

impl<'de> Deserialize<'de> for StatusCode {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self(
			reqwest::StatusCode::from_u16(u16::deserialize(deserializer)?).map_err(|_| {
				serde::de::Error::invalid_value(
					serde::de::Unexpected::Other("invalid HTTP status code"),
					&"valid HTTP status code",
				)
			})?,
		))
	}
}
