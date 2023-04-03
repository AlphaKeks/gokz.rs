use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct User {
	pub name: String,
	pub avatar_url: String,
	pub country: String,
}

/// `/steam/:steam_id64`
pub mod id64;
impl From<id64::Response> for User {
	fn from(value: id64::Response) -> Self {
		Self {
			name: value.name,
			avatar_url: value.avatar,
			country: value.country,
		}
	}
}
