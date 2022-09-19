#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub enum GOKZMapIdentifier {
	Name(String),
	Id(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZMap {
	pub id: u16,
	pub name: String,
	pub filesize: u128,
	pub validated: bool,
	pub difficulty: u8,
	pub created_on: String,
	pub updated_on: String,
	pub approved_by_steamid64: String,
	pub workshop_url: String,
	pub download_url: Option<String>,
}
