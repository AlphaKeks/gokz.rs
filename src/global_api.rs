#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub enum GOKZMapIdentifier {
	Name(String),
	Id(u16),
}

pub enum GOKZModeIdentifier {
	Name(GOKZModeName),
	Id(u16),
}

#[derive(Clone)]
pub enum GOKZPlayerIdentifier {
	Name(String),
	SteamID(String),
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

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GOKZModeName {
	kz_timer,
	kz_simple,
	kz_vanilla,
}

impl GOKZModeName {
	pub fn as_str(&self) -> &'static str {
		match self {
			GOKZModeName::kz_timer => "kz_timer",
			GOKZModeName::kz_simple => "kz_simple",
			GOKZModeName::kz_vanilla => "kz_vanilla",
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZMode {
	pub id: u16,
	pub name: GOKZModeName,
	pub description: String,
	pub latest_version: u16,
	pub latest_version_description: String,
	pub website: String,
	pub repo: String,
	pub contact_steamid64: String,
	pub supported_tickrates: Option<u16>,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZPlayer {
	pub steamid64: String,
	pub steam_id: String,
	pub is_banned: bool,
	pub total_records: u32,
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZRecord {
	pub id: u32,
	pub steamid64: String,
	pub player_name: String,
	pub steam_id: String,
	pub server_id: u16,
	pub map_id: u16,
	pub stage: u8,
	pub mode: GOKZModeName,
	pub tickrate: u16,
	pub time: f32,
	pub teleports: u32,
	pub created_on: String,
	pub updated_on: String,
	// TODO: this might always be 0, no idea
	pub updated_by: u32,
	pub server_name: Option<String>,
	pub map_name: String,
	pub points: u16,
	pub record_filter_id: u32,
	pub replay_id: u32,
}
