use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorKind {
	GlobalAPI,
	Parsing,
	InvalidInput,
	Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
	pub kind: ErrorKind,
	pub tldr: &'static str,
	pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamId {
	pub val: &'static str,
}

impl SteamId {
	pub fn test(&self) -> bool {
		let regex = Regex::new(r"STEAM_[0-1]:[0-1]:[0-9]+");

		if let Ok(r) = regex {
			if let Some(_) = r.find(self.val) {
				return true;
			}
		}

		false
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapIdentifier {
	Name(&'static str),
	Id(u32),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mode {
	KZTimer,
	SimpleKZ,
	Vanilla,
}

impl Mode {
	pub fn from_str(input: &'static str) -> Self {
		match input {
			"KZTimer" | "kz_timer" | "KZT" | "kzt" => Mode::KZTimer,
			"SimpleKZ" | "kz_simple" | "SKZ" | "skz" => Mode::SimpleKZ,
			"Vanilla" | "kz_vanilla" | "VNL" | "vnl" => Mode::Vanilla,
			_ => Mode::KZTimer,
		}
	}

	pub fn from_id(input: u8) -> Self {
		match input {
			200 => Mode::KZTimer,
			201 => Mode::SimpleKZ,
			202 => Mode::Vanilla,
			_ => Mode::KZTimer,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "kz_timer",
			&Mode::SimpleKZ => "kz_simple",
			&Mode::Vanilla => "kz_vanilla",
		}
	}

	pub fn fancy(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "KZTimer",
			&Mode::SimpleKZ => "SimpleKZ",
			&Mode::Vanilla => "Vanilla",
		}
	}

	pub fn fancy_short(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "KZT",
			&Mode::SimpleKZ => "SKZ",
			&Mode::Vanilla => "VNL",
		}
	}

	pub fn as_route(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "modes/name/kz_timer",
			&Mode::SimpleKZ => "modes/name/kz_simple",
			&Mode::Vanilla => "modes/name/kz_vanilla",
		}
	}

	pub fn as_id_route(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "modes/id/200",
			&Mode::SimpleKZ => "modes/id/201",
			&Mode::Vanilla => "modes/id/200",
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerIdentifier {
	Name(&'static str),
	SteamId(SteamId),
}
