use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorKind {
	GlobalAPI,
	KZGO,
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
	Id(u16),
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

	pub fn as_id(&self) -> u8 {
		match self {
			&Mode::KZTimer => 200,
			&Mode::SimpleKZ => 201,
			&Mode::Vanilla => 202,
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

	pub fn as_route(&self) -> String {
		match self {
			&Mode::KZTimer => String::from("modes/name/kz_timer"),
			&Mode::SimpleKZ => String::from("modes/name/kz_simple"),
			&Mode::Vanilla => String::from("modes/name/kz_vanilla"),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rank {
	Legend,
	Master,
	Pro,
	Semipro,
	ExpertPlus,
	Expert,
	ExpertMinus,
	SkilledPlus,
	Skilled,
	SkilledMinus,
	RegularPlus,
	Regular,
	RegularMinus,
	CasualPlus,
	Casual,
	CasualMinus,
	AmateurPlus,
	Amateur,
	AmateurMinus,
	BeginnerPlus,
	Beginner,
	BeginnerMinus,
	New,
}

impl Rank {
	pub fn from_points(points: u32, mode: &Mode) -> Self {
		match mode {
			&Mode::KZTimer => {
				if points > 1_000_000 {
					return Rank::Legend;
				} else if points > 800_000 {
					return Rank::Master;
				} else if points > 600_000 {
					return Rank::Pro;
				} else if points > 400_000 {
					return Rank::Semipro;
				} else if points > 250_000 {
					return Rank::ExpertPlus;
				} else if points > 230_000 {
					return Rank::Expert;
				} else if points > 200_000 {
					return Rank::ExpertMinus;
				} else if points > 150_000 {
					return Rank::SkilledPlus;
				} else if points > 120_000 {
					return Rank::Skilled;
				} else if points > 100_000 {
					return Rank::SkilledMinus;
				} else if points > 80_000 {
					return Rank::RegularPlus;
				} else if points > 70_000 {
					return Rank::Regular;
				} else if points > 60_000 {
					return Rank::RegularMinus;
				} else if points > 40_000 {
					return Rank::CasualPlus;
				} else if points > 30_000 {
					return Rank::Casual;
				} else if points > 20_000 {
					return Rank::CasualMinus;
				} else if points > 10_000 {
					return Rank::AmateurPlus;
				} else if points > 5_000 {
					return Rank::Amateur;
				} else if points > 2_000 {
					return Rank::AmateurMinus;
				} else if points > 1_000 {
					return Rank::BeginnerPlus;
				} else if points > 500 {
					return Rank::Beginner;
				} else if points > 0 {
					return Rank::BeginnerMinus;
				}
			}

			&Mode::SimpleKZ => {
				if points > 800_000 {
					return Rank::Legend;
				} else if points > 500_000 {
					return Rank::Master;
				} else if points > 400_000 {
					return Rank::Pro;
				} else if points > 300_000 {
					return Rank::Semipro;
				} else if points > 250_000 {
					return Rank::ExpertPlus;
				} else if points > 230_000 {
					return Rank::Expert;
				} else if points > 200_000 {
					return Rank::ExpertMinus;
				} else if points > 150_000 {
					return Rank::SkilledPlus;
				}
			}

			&Mode::Vanilla => {
				if points > 600_000 {
					return Rank::Legend;
				} else if points > 400_000 {
					return Rank::Master;
				} else if points > 300_00 {
					return Rank::Pro;
				} else if points > 250_000 {
					return Rank::Semipro;
				} else if points > 200_000 {
					return Rank::ExpertPlus;
				} else if points > 180_000 {
					return Rank::Expert;
				} else if points > 160_000 {
					return Rank::ExpertMinus;
				} else if points > 140_000 {
					return Rank::SkilledPlus;
				}
			}
		}

		if points > 120_000 {
			return Rank::Skilled;
		} else if points > 100_000 {
			return Rank::SkilledMinus;
		} else if points > 80_000 {
			return Rank::RegularPlus;
		} else if points > 70_000 {
			return Rank::Regular;
		} else if points > 60_000 {
			return Rank::RegularMinus;
		} else if points > 40_000 {
			return Rank::CasualPlus;
		} else if points > 30_000 {
			return Rank::Casual;
		} else if points > 20_000 {
			return Rank::CasualMinus;
		} else if points > 10_000 {
			return Rank::AmateurPlus;
		} else if points > 5_000 {
			return Rank::Amateur;
		} else if points > 2_000 {
			return Rank::AmateurMinus;
		} else if points > 1_000 {
			return Rank::BeginnerPlus;
		} else if points > 500 {
			return Rank::Beginner;
		} else if points > 0 {
			return Rank::BeginnerMinus;
		} else {
			return Rank::New;
		}
	}
}
