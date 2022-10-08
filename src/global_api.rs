#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub enum GOKZMapIdentifier {
	Name(String),
	Id(u16),
}

#[derive(Clone)]
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
pub enum GOKZRank {
	Legend(String),
	Master(String),
	Pro(String),
	Semipro(String),
	ExpertPlus(String),
	Expert(String),
	ExpertMinus(String),
	SkilledPlus(String),
	Skilled(String),
	SkilledMinus(String),
	RegularPlus(String),
	Regular(String),
	RegularMinus(String),
	CasualPlus(String),
	Casual(String),
	CasualMinus(String),
	AmateurPlus(String),
	Amateur(String),
	AmateurMinus(String),
	BeginnerPlus(String),
	Beginner(String),
	BeginnerMinus(String),
	New(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalAPIStatusPage {
	pub id: String,
	pub name: String,
	pub url: String,
	pub time_zone: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalAPIStatusComponent {
	pub id: String,
	pub name: String,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
	pub position: i32,
	pub description: String,
	pub showcase: bool,
	pub start_date: Option<String>,
	pub group_id: Option<String>,
	pub page_id: String,
	pub group: bool,
	pub only_show_if_degraded: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalAPIStatusStatus {
	pub indicator: String,
	pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalAPIStatus {
	pub page: GlobalAPIStatusPage,
	pub components: Vec<GlobalAPIStatusComponent>,
	pub incidents: Vec<String>,
	pub scheduled_maintenances: Vec<String>,
	pub status: GlobalAPIStatusStatus,
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

	pub fn fancy(&self) -> &'static str {
		match self {
			GOKZModeName::kz_timer => "KZTimer",
			GOKZModeName::kz_simple => "SimpleKZ",
			GOKZModeName::kz_vanilla => "Vanilla",
		}
	}

	pub fn fancy_short(&self) -> &'static str {
		match self {
			GOKZModeName::kz_timer => "KZT",
			GOKZModeName::kz_simple => "SKZ",
			GOKZModeName::kz_vanilla => "VNL",
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
	pub player_name: Option<String>,
	pub steam_id: String,
	pub server_id: u16,
	pub map_id: u16,
	pub stage: u8,
	pub mode: GOKZModeName,
	pub tickrate: u8,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZRecordFilter {
	pub id: u32,
	pub map_id: u16,
	pub stage: u8,
	pub mode_id: u16,
	pub tickrate: u8,
	pub has_teleports: bool,
	pub created_on: String,
	pub updated_on: String,
	pub updated_by_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GOKZPlayerProfile {
	pub name: String,
	pub steam_id: Option<String>,
	pub steam_id64: String,
	pub rank: GOKZRank,
	pub points: (u32, u32),
	pub records: (u32, u32),
	pub completion: [(u32, u32); 8],
	pub completion_percentage: [(f32, f32); 8],
	pub is_banned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KZGOCompletionCount {
	#[serde(rename(deserialize = "1"))]
	pub one: u32,
	#[serde(rename(deserialize = "2"))]
	pub two: u32,
	#[serde(rename(deserialize = "3"))]
	pub three: u32,
	#[serde(rename(deserialize = "4"))]
	pub four: u32,
	#[serde(rename(deserialize = "5"))]
	pub five: u32,
	#[serde(rename(deserialize = "6"))]
	pub six: u32,
	#[serde(rename(deserialize = "7"))]
	pub seven: u32,
	pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KZGOCompletionStats {
	pub _id: String,
	pub mode: String,
	pub pro: KZGOCompletionCount,
	pub tp: KZGOCompletionCount,
}
