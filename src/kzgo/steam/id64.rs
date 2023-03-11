use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub country: String,
	pub avatar: String,
	pub name: String,
}
