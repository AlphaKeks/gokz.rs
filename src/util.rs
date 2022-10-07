use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GOKZErrorType {
	GlobalAPI,
	KZGO,
	Parsing,
	Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZError {
	pub r#type: GOKZErrorType,
	pub tldr: String,
	pub raw: Option<String>,
}
