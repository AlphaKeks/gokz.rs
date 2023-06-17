use {
	crate::types::Mode,
	serde::{Deserialize, Serialize},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Completions {
	#[serde(rename = "_id")]
	pub id: String,
	pub mode: Mode,
	pub tp: CompletionCount,
	pub pro: CompletionCount,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
