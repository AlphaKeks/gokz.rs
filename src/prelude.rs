/// All possible kinds of errors that this crate can produce.
#[derive(Debug)]
pub enum ErrorKind {
	GlobalAPI,
	KZGO,
	Parsing,
	Input,
	NoData,
	Other,
}

/// The default Error type for this crate. Every fallible function in this
/// crate should return this type of error.
#[derive(Debug)]
pub struct Error {
	pub kind: ErrorKind,
	pub origin: String,
	pub tldr: String,
	pub raw: Option<String>,
}

/// A Steam ID as [defined by Valve](https://developer.valvesoftware.com/wiki/SteamID).
/// It's a unique identifier for a [Steam](https://steamcommunity.com/) account.
///
/// # Examples
/// [My Account](https://steamcommunity.com/profiles/76561198282622073)'s Steam ID is `STEAM_1:1:161178172`
#[derive(Debug, Clone)]
pub struct SteamID(pub String);

impl<'a> SteamID {
	/// A function to test whether a String is a [`SteamID`] or not.
	///
	/// # Examples
	/// ```rust
	/// let steam_id = "STEAM_1:1:161178172";
	/// let not_steam_id = "some random text";
	///
	/// assert_eq!(true, SteamID::test(steam_id));
	/// assert_ne!(true, SteamID::test(not_steam_id));
	/// ```
	pub fn test(input: &'a str) -> bool {
		let regex = regex::Regex::new(r"STEAM_[0-1]:[0-1]:[0-9]+");

		if let Ok(regex) = regex {
			if let Some(_) = regex.find(input) {
				return true;
			}
		}

		return false;
	}
}

impl ToString for SteamID {
	fn to_string(&self) -> String {
		self.0.to_owned()
	}
}

/// All 3 GOKZ modes
#[derive(Debug)]
pub enum Mode {
	KZTimer,
	SimpleKZ,
	Vanilla,
}

impl Mode {
	/// Each of the 3 modes has an associated ID. Because you cannot limit
	/// function parameters to specific values, we need to restrict it as much
	/// as possible and return a Result.
	pub fn from_id(id: u8) -> Result<Self, Error> {
		match id {
			200 => Ok(Mode::KZTimer),
			201 => Ok(Mode::SimpleKZ),
			202 => Ok(Mode::Vanilla),
			_ => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::Mode::from_id"),
				tldr: String::from("Cannot convert an invalid ID to a mode."),
				raw: None,
			}),
		}
	}

	/// Each of the 3 modes has an associated ID. This function will output
	/// the correct ID based on its input.
	pub fn as_id(&self) -> u8 {
		match self {
			&Mode::KZTimer => 200,
			&Mode::SimpleKZ => 201,
			&Mode::Vanilla => 202,
		}
	}

	/// The most commonly used way of displaying a mode in written form.
	pub fn fancy(&self) -> String {
		match self {
			&Mode::KZTimer => String::from("KZTimer"),
			&Mode::SimpleKZ => String::from("SimpleKZ"),
			&Mode::Vanilla => String::from("Vanilla"),
		}
	}

	/// The most commonly used abbreviation for each mode.
	pub fn fancy_short(&self) -> String {
		match self {
			&Mode::KZTimer => String::from("KZT"),
			&Mode::SimpleKZ => String::from("SKZ"),
			&Mode::Vanilla => String::from("VNL"),
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			&Mode::KZTimer => "kz_timer",
			&Mode::SimpleKZ => "kz_simple",
			&Mode::Vanilla => "kz_vanilla",
		}
	}
}

impl std::str::FromStr for Mode {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"kz_timer" | "kztimer" | "kzt" => Ok(Mode::KZTimer),
			"kz_simple" | "simplekz" | "skz" => Ok(Mode::SimpleKZ),
			"kz_vanilla" | "vanilla" | "vnl" => Ok(Mode::Vanilla),
			_ => Err(Error {
				kind: ErrorKind::Input,
				origin: String::from("gokz_rs::prelude::Mode::from_str"),
				tldr: String::from("Invalid Input."),
				raw: None,
			}),
		}
	}
}

impl ToString for Mode {
	fn to_string(&self) -> String {
		match self {
			&Mode::KZTimer => String::from("kz_timer"),
			&Mode::SimpleKZ => String::from("kz_simple"),
			&Mode::Vanilla => String::from("kz_vanilla"),
		}
	}
}

/// All possible ways of representing a KZ map to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)
#[derive(Debug, Clone)]
pub enum MapIdentifier {
	Name(String),
	ID(u16),
}

/// All possible ways of representing a player to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)
#[derive(Debug, Clone)]
pub enum PlayerIdentifier {
	Name(String),
	SteamID(SteamID),
}

/// All Ranks a player can have
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
	/// Each [`Mode`] has different thresholds for ranks. This function will
	/// construct a [`Rank`] based on an amount of points and a specific [`Mode`].
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
				} else {
					return Rank::New;
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
			}

			&Mode::Vanilla => {
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
			}
		}
	}
}

impl ToString for Rank {
	fn to_string(&self) -> String {
		(match self {
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
			Rank::RegularMinus => "Regular+",
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
		})
		.to_owned()
	}
}
