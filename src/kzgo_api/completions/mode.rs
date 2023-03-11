use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Response {
	pub _id: String,
	pub mode: String,
	pub pro: CompletionCount,
	pub tp: CompletionCount,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CompletionCount {
	#[serde(rename(deserialize = "1"))]
	pub one: u16,
	#[serde(rename(deserialize = "2"))]
	pub two: u16,
	#[serde(rename(deserialize = "3"))]
	pub three: u16,
	#[serde(rename(deserialize = "4"))]
	pub four: u16,
	#[serde(rename(deserialize = "5"))]
	pub five: u16,
	#[serde(rename(deserialize = "6"))]
	pub six: u16,
	#[serde(rename(deserialize = "7"))]
	pub seven: u16,
	pub total: u16,
}
