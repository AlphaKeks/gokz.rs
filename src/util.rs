use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GOKZErrorType {
	GlobalAPI,
	KZGO,
	Conversion,
	Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZError {
	pub r#type: GOKZErrorType,
	pub tldr: String,
	pub raw: String,
}
