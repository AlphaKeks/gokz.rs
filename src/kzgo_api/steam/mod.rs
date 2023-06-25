use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct User {
	pub name: String,
	pub avatar_url: String,
	pub country: String,
}
