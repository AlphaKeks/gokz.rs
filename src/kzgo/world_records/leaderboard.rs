use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct Response {
	pub playerName: String,
	pub _id: String,
	pub count: u16,
}
