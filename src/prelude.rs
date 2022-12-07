use std::fmt::Display;

/// The different Types for [`Error`].
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ErrorKind {
	GlobalAPI,
	KZGO,
	Parsing,
	Input,
	NoData,
	Other,
}

impl Display for ErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			&ErrorKind::GlobalAPI => "GlobalAPI",
			&ErrorKind::KZGO => "KZ:GO",
			&ErrorKind::Parsing => "Parsing",
			&ErrorKind::Input => "Input",
			&ErrorKind::NoData => "No Data",
			&ErrorKind::Other => "Other",
		};

		return write!(f, "{}", s);
	}
}

/// The default way of representing a failed function call from this crate.
///
/// `kind`: [`ErrorKind`]
/// `origin`: a String representing the absolute path to the function where the Error originated
/// `tldr`: a short description of what the Error is
/// `raw`: a placeholder for an error message emitted by an external function
///
/// # Examples
///
/// ```
/// use gokz_rs::prelude::Mode;
///
/// // results in an [`Error`]
/// let mode = Mode::from_id(69);
///
/// // Err {
/// //     kind: ErrorKind::Input,
/// //     origin: String::from("gokz_rs::prelude::Mode::from_id"),
/// //     tldr: format!("Cannot convert {} to a mode.", input),
/// //     raw: None,
/// // }
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Error {
	pub kind: ErrorKind,
	pub origin: String,
	pub tldr: String,
	pub raw: Option<String>,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "Error {{ kind: {}, tldr: {} }}", self.kind, self.tldr);
	}
}

/// A unique identifier for a [Steam](https://www.steamcommunity.com/) Account.
///
/// Note: [official definition](https://developer.valvesoftware.com/wiki/SteamID).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SteamID(String);

impl<'a> SteamID {
	/// A function to test whether a String qualifies as a [`SteamID`] or not.
	///
	/// # Examples
	/// ```rust
	/// use gokz_rs::prelude::SteamID;
	///
	/// let steam_id = "STEAM_1:1:161178172";
	/// let not_steam_id = "some random text";
	/// let also_not_steam_id = "textSTEAM_1:1:161178172";
	///
	/// assert!(SteamID::test(steam_id));
	/// assert!(!SteamID::test(not_steam_id));
	/// assert!(!SteamID::test(also_not_steam_id));
	/// ```
	pub fn test(input: &'a str) -> bool {
		let regex = regex::Regex::new(r"^STEAM_[0-1]:[0-1]:[0-9]+$");

		if let Ok(regex) = regex {
			if let Some(_) = regex.find(input) {
				return true;
			}
		}

		return false;
	}

	pub fn new(steam_id: &'a str) -> Result<SteamID, Error> {
		if Self::test(steam_id) {
			Ok(SteamID(steam_id.to_owned()))
		} else {
			Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::SteamID::new"),
				tldr: String::from("Invalid SteamID."),
				raw: Some(steam_id.to_owned()),
			})
		}
	}
}

impl Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{}", self.0);
	}
}

impl std::str::FromStr for SteamID {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if Self::test(s) {
			return Ok(SteamID(s.to_owned()));
		} else {
			return Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::SteamID::from_str"),
				tldr: String::from("Invalid SteamID."),
				raw: Some(s.to_owned()),
			});
		}
	}
}

impl TryFrom<String> for SteamID {
	type Error = crate::prelude::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		if Self::test(&value) {
			return Ok(SteamID(value));
		} else {
			return Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::SteamID::from_str"),
				tldr: String::from("Invalid SteamID."),
				raw: Some(value),
			});
		}
	}
}

/// The 3 gamemodes currently available in [GOKZ](https://github.com/KZGlobalTeam/gokz).
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Mode {
	KZTimer,
	SimpleKZ,
	Vanilla,
}

impl Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Mode::KZTimer => "KZTimer",
			Mode::SimpleKZ => "SimpleKZ",
			Mode::Vanilla => "Vanilla",
		};

		return write!(f, "{}", s);
	}
}

impl std::str::FromStr for Mode {
	type Err = crate::prelude::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		return match s.to_lowercase().as_str() {
			"kz_timer" | "kztimer" | "kzt" => Ok(Mode::KZTimer),
			"kz_simple" | "simplekz" | "skz" => Ok(Mode::SimpleKZ),
			"kz_vanilla" | "vanilla" | "vnl" => Ok(Mode::Vanilla),
			input => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::Mode -> FromStr"),
				tldr: format!("{} is not a valid input.", input),
				raw: Some(input.to_owned()),
			}),
		};
	}
}

impl TryFrom<u8> for Mode {
	type Error = crate::prelude::Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			200 => Ok(Mode::KZTimer),
			201 => Ok(Mode::SimpleKZ),
			202 => Ok(Mode::Vanilla),
			input => {
				return Err(Error {
					kind: ErrorKind::Input,
					origin: String::from("gokz_rs::prelude::Mode -> TryFrom<u8>"),
					tldr: format!("Cannot convert {} to a mode.", input),
					raw: Some(value.to_string()),
				})
			},
		}
	}
}

impl Into<u8> for Mode {
	fn into(self) -> u8 {
		return match self {
			Mode::KZTimer => 200,
			Mode::SimpleKZ => 201,
			Mode::Vanilla => 202,
		};
	}
}

impl Mode {
	pub fn as_str<'a>(&'a self) -> &'a str {
		return match self {
			Mode::KZTimer => "kz_timer",
			Mode::SimpleKZ => "kz_simple",
			Mode::Vanilla => "kz_vanilla",
		};
	}

	pub fn to_fancy(&self) -> String {
		return match self {
			Mode::KZTimer => String::from("KZT"),
			Mode::SimpleKZ => String::from("SKZ"),
			Mode::Vanilla => String::from("VNL"),
		};
	}
}

/// A KZ Map can be represented in multiple ways when making requests to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V).
///
/// # Examples
///
/// ```
/// use gokz_rs::prelude::MapIdentifier;
///
/// // both of these represent the same map.
/// let map_name = MapIdentifier::Name(
///     String::from("kz_lionharder")
/// );
/// let map_id = MapIdentifier::ID(992);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MapIdentifier {
	Name(String),
	ID(i16),
}

impl Display for MapIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MapIdentifier::Name(name) => write!(f, "{}", name),
			MapIdentifier::ID(id) => write!(f, "{}", id),
		}
	}
}

impl std::str::FromStr for MapIdentifier {
	type Err = crate::prelude::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		return Ok(MapIdentifier::Name(s.to_owned()));
	}
}

impl From<String> for MapIdentifier {
	fn from(s: String) -> Self {
		return MapIdentifier::Name(s);
	}
}

impl From<i16> for MapIdentifier {
	fn from(id: i16) -> Self {
		return MapIdentifier::ID(id);
	}
}

impl TryInto<i16> for MapIdentifier {
	type Error = crate::prelude::Error;

	fn try_into(self) -> Result<i16, Self::Error> {
		return match self {
			Self::Name(name) => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::MapIdentifier -> TryInto<i16>"),
				tldr: String::from("A name has been provided instead of an ID."),
				raw: Some(name),
			}),
			Self::ID(id) => Ok(id),
		};
	}
}

/// A Player can be represented in multiple ways when making requests to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V).
///
/// # Examples
///
/// ```
/// use gokz_rs::prelude::{PlayerIdentifier, SteamID};
///
/// let player_name = PlayerIdentifier::Name(
///     String::from("AlphaKeks")
/// );
/// let player_steamid = PlayerIdentifier::SteamID(
///     SteamID::new("STEAM_1:1:161178172").unwrap()
/// );
/// let player_steamid64 = PlayerIdentifier::SteamID64(76561198282622073);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerIdentifier {
	Name(String),
	SteamID(SteamID),
	SteamID64(u64),
}

impl Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::Name(name) => write!(f, "{}", name),
			PlayerIdentifier::SteamID(steam_id) => write!(f, "{}", steam_id),
			PlayerIdentifier::SteamID64(steam_id64) => write!(f, "{}", steam_id64),
		}
	}
}

impl From<String> for PlayerIdentifier {
	fn from(name: String) -> Self {
		return PlayerIdentifier::Name(name);
	}
}

impl From<SteamID> for PlayerIdentifier {
	fn from(steam_id: SteamID) -> Self {
		return PlayerIdentifier::SteamID(steam_id);
	}
}

impl From<u64> for PlayerIdentifier {
	fn from(steam_id64: u64) -> Self {
		return PlayerIdentifier::SteamID64(steam_id64);
	}
}

impl TryInto<String> for PlayerIdentifier {
	type Error = crate::prelude::Error;

	fn try_into(self) -> Result<String, Self::Error> {
		return match self {
			Self::Name(name) => Ok(name),
			input => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::PlayerIdentifier -> TryInto<String>"),
				tldr: format!("`{}` cannot be turned into a player name.", input),
				raw: Some(input.to_string()),
			}),
		};
	}
}

impl TryInto<SteamID> for PlayerIdentifier {
	type Error = crate::prelude::Error;

	fn try_into(self) -> Result<SteamID, Self::Error> {
		return match self {
			Self::SteamID(steam_id) => Ok(steam_id),
			input => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::PlayerIdentifier -> TryInto<SteamID>"),
				tldr: format!("`{}` cannot be turned into a SteamID.", input),
				raw: Some(input.to_string()),
			}),
		};
	}
}

impl TryInto<u64> for PlayerIdentifier {
	type Error = crate::prelude::Error;

	fn try_into(self) -> Result<u64, Self::Error> {
		return match self {
			Self::SteamID64(steam_id64) => Ok(steam_id64),
			input => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::PlayerIdentifier -> TryInto<u64>"),
				tldr: format!("`{}` cannot be turned into a SteamID64.", input),
				raw: Some(input.to_string()),
			}),
		};
	}
}

/// Every player who has joined a [GOKZ](https://github.com/KZGlobalTeam/gokz) server with version 3.0.0 or higher will get a [`Rank`]
/// assigned to them. Which [`Rank`] they will have is based on the player's total points in a
/// given [`Mode`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
	/// Since each [`Mode`] has different thresholds for ranks. This function will construct a
	/// [`Rank`] based on an amount of points and a specific [`Mode`].
	pub fn from_points(points: u32, mode: &Mode) -> Self {
		match mode {
			Mode::KZTimer => {
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
				} else {
					return Rank::New;
				}
			},
			Mode::SimpleKZ => {
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
				} else {
					return Rank::New;
				}
			},
			Mode::Vanilla => {
				if points > 600_000 {
					return Rank::Legend;
				} else if points > 400_000 {
					return Rank::Master;
				} else if points > 300_000 {
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
				} else {
					return Rank::New;
				}
			},
		}
	}
}

impl Display for Rank {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Rank::Legend => "Legend",
			Rank::Master => "Master",
			Rank::Pro => "Pro",
			Rank::Semipro => "Semipro",
			Rank::ExpertPlus => "Expert+",
			Rank::Expert => "Expert",
			Rank::ExpertMinus => "Expert-",
			Rank::SkilledPlus => "Skilled+",
			Rank::Skilled => "Skilled",
			Rank::SkilledMinus => "Skilled-",
			Rank::RegularPlus => "Regular+",
			Rank::Regular => "Regular",
			Rank::RegularMinus => "Regular-",
			Rank::CasualPlus => "Casual+",
			Rank::Casual => "Casual",
			Rank::CasualMinus => "Casual-",
			Rank::AmateurPlus => "Amateur+",
			Rank::Amateur => "Amateur",
			Rank::AmateurMinus => "Amateur-",
			Rank::BeginnerPlus => "Beginner+",
			Rank::Beginner => "Beginner",
			Rank::BeginnerMinus => "Beginner-",
			Rank::New => "New",
		};

		return write!(f, "{}", s);
	}
}

impl std::str::FromStr for Rank {
	type Err = crate::prelude::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		return match s {
			"Legend" => Ok(Rank::Legend),
			"Master" => Ok(Rank::Master),
			"Pro" => Ok(Rank::Pro),
			"Semipro" => Ok(Rank::Semipro),
			"Expert+" => Ok(Rank::ExpertPlus),
			"Expert" => Ok(Rank::Expert),
			"Expert-" => Ok(Rank::ExpertMinus),
			"Skilled+" => Ok(Rank::SkilledPlus),
			"Skilled" => Ok(Rank::Skilled),
			"Skilled-" => Ok(Rank::SkilledMinus),
			"Regular+" => Ok(Rank::RegularPlus),
			"Regular" => Ok(Rank::Regular),
			"Regular-" => Ok(Rank::RegularMinus),
			"Casual+" => Ok(Rank::CasualPlus),
			"Casual" => Ok(Rank::Casual),
			"Casual-" => Ok(Rank::CasualMinus),
			"Amateur+" => Ok(Rank::AmateurPlus),
			"Amateur" => Ok(Rank::Amateur),
			"Amateur-" => Ok(Rank::AmateurMinus),
			"Beginner+" => Ok(Rank::BeginnerPlus),
			"Beginner" => Ok(Rank::Beginner),
			"Beginner-" => Ok(Rank::BeginnerMinus),
			"New" => Ok(Rank::New),
			input => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::Rank -> FromStr"),
				tldr: format!("`{}` cannot be turned into a Rank.", input),
				raw: Some(input.to_owned()),
			}),
		};
	}
}

impl TryFrom<String> for Rank {
	type Error = crate::prelude::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		return match value.as_str().parse::<Rank>() {
			Ok(output) => Ok(output),
			Err(err) => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::Rank -> TryFrom<String>"),
				tldr: format!(
					"`{}` cannot be turned into a Rank.",
					err.raw.expect("This should have a value.")
				),
				raw: Some(value),
			}),
		};
	}
}
