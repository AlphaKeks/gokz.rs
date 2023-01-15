use {
	log::warn,
	regex::Regex,
	serde::{Deserialize, Serialize},
};

/// The default Error type which gets returned from any fallible function exposed by this crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
	pub kind: ErrorKind,
	pub msg: String,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = match &self.kind {
			ErrorKind::InvalidInput { expected, got } => {
				let msg = format!("Invalid Input. Expected `{}`, but got `{}`", expected, got);
				warn!("{}", &msg);
				msg
			},
			ErrorKind::Parsing { expected, got } => {
				let mut msg = format!("Failed to parse input. Expected `{}`", expected);
				if let Some(got) = got {
					msg += &format!(", but got `{}`.", got);
				} else {
					msg += ".";
				}
				warn!("{}", &msg);
				msg
			},
			ErrorKind::NoData { expected } => {
				let msg = format!("Expected to find `{}`, but found nothing.", expected);
				warn!("{}", &msg);
				msg
			},
			ErrorKind::GlobalAPI { status_code, raw_message } => {
				let msg = if let Some(status_code) = status_code {
					format!("GlobalAPI request failed with Status Code `{}`.", status_code)
				} else {
					String::from("GlobalAPI request failed, but returned no Status Code.")
				};
				warn!("{}", &msg);
				warn!("Raw Message: {:?}", raw_message);
				msg
			},
			ErrorKind::KZGO { status_code, raw_message } => {
				let msg = if let Some(status_code) = status_code {
					format!("KZ:GO API request failed with Status Code `{}`.", status_code)
				} else {
					String::from("KZ:GO API request failed, but returned no Status Code.")
				};
				warn!("{}", &msg);
				warn!("Raw Message: {:?}", raw_message);
				msg
			},
		};

		write!(f, "{}", msg)
	}
}

impl std::error::Error for Error {}

/// The type of [`Error`]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
	InvalidInput {
		/// the expected type
		expected: String,
		/// original input which is invalid
		got: String,
	},

	Parsing {
		/// the expected type
		expected: String,
		/// original input which failed to be parsed
		got: Option<String>,
	},

	NoData {
		/// the expected type
		expected: String,
	},

	GlobalAPI {
		/// HTTP Status Code
		status_code: Option<String>,
		/// the message returned by the GlobalAPI (if there is one)
		raw_message: Option<String>,
	},

	KZGO {
		/// HTTP Status Code
		status_code: Option<String>,
		/// the message returned by the GlobalAPI (if there is one)
		raw_message: Option<String>,
	},
}

/// A unique identifier for a [Steam](https://www.steamcommunity.com/) Account.
///
/// NOTE: [Official Documentation by Valve](https://developer.valvesoftware.com/wiki/SteamID).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SteamID(String);

impl SteamID {
	/// Function to test whether a String qualifies as a [`SteamID`] or not
	///
	/// # Examples
	/// ```
	/// use gokz_rs::prelude::SteamID;
	///
	/// let valid = SteamID::test("STEAM_1:1:161178172");
	/// let invalid1 = SteamID::test("wordSTEAM_1:1:161178172");
	/// let invalid2 = SteamID::test("STEAM_1:1:161178172word");
	/// let invalid3 = SteamID::test("some random text");
	///
	/// assert!(valid);
	/// assert!(!invalid1);
	/// assert!(!invalid2);
	/// assert!(!invalid3);
	/// ```
	pub fn test(input: &str) -> bool {
		Regex::new(r#"^STEAM_[0-5]:[0-1]:[0-9]+$"#)
			.expect("This is a valid regex.")
			.is_match(input)
	}

	pub fn new(steam_id: &str) -> Result<Self, Error> {
		let steam_id = String::from(steam_id);

		if Self::test(&steam_id) {
			Ok(SteamID(steam_id))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("valid SteamID"),
					got: steam_id.clone(),
				},
				msg: format!("`{}` is not a valid SteamID.", steam_id),
			})
		}
	}
}

impl std::fmt::Display for SteamID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl std::str::FromStr for SteamID {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		SteamID::new(s)
	}
}

impl TryFrom<String> for SteamID {
	type Error = Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		SteamID::new(&value)
	}
}

impl TryFrom<PlayerIdentifier> for SteamID {
	type Error = Error;

	fn try_from(value: PlayerIdentifier) -> Result<Self, Self::Error> {
		if let PlayerIdentifier::SteamID(steam_id) = value {
			return Ok(steam_id);
		}

		Err(Error {
			kind: ErrorKind::InvalidInput {
				expected: String::from("PlayerIdentifier::SteamID"),
				got: value.to_string(),
			},
			msg: format!("`{}` is a name, not a SteamID.", value),
		})
	}
}

impl From<u64> for SteamID {
	/// NOTE: [Documentation](https://developer.valvesoftware.com/wiki/SteamID#As_Represented_in_Computer_Programs)
	fn from(value: u64) -> Self {
		// universe of the account
		let x = value >> 56;

		// ID number part
		let y = value & 1;

		// account number
		let z = (value >> 1) & 0b0111_1111_1111_1111_1111_1111_1111_1111;

		let steam_id = format!("STEAM_{}:{}:{}", x, y, z);

		Self(steam_id)
	}
}

/// The 3 gamemodes currently available in [GOKZ](https://github.com/KZGlobalTeam/gokz)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
	KZTimer = 200,
	SimpleKZ = 201,
	Vanilla = 202,
}

impl Mode {
	/// Format a given [`Mode`] so it can be sent to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)
	/// - KZTimer => `"kz_timer"`
	/// - SimpleKZ => `"kz_simple"`
	/// - Vanilla => `"kz_vanilla"`
	pub fn api(&self) -> String {
		match self {
			Mode::KZTimer => String::from("kz_timer"),
			Mode::SimpleKZ => String::from("kz_simple"),
			Mode::Vanilla => String::from("kz_vanilla"),
		}
	}

	/// Turn a [`Mode`] into an abbreviated String
	/// - KZTimer => `"KZT"`
	/// - SimpleKZ => `"SKZ"`
	/// - Vanilla => `"VNL"`
	pub fn short(&self) -> String {
		match self {
			Mode::KZTimer => String::from("KZT"),
			Mode::SimpleKZ => String::from("SKZ"),
			Mode::Vanilla => String::from("VNL"),
		}
	}
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Mode::KZTimer => "KZTimer",
			Mode::SimpleKZ => "SimpleKZ",
			Mode::Vanilla => "Vanilla",
		};
		write!(f, "{}", s)
	}
}

impl std::str::FromStr for Mode {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"kztimer" | "kz_timer" | "kzt" => Ok(Self::KZTimer),
			"simplekz" | "simple_kz" | "kz_simple" | "skz" => Ok(Self::SimpleKZ),
			"vanilla" | "vanillakz" | "vanilla_kz" | "kz_vanilla" | "vnl" => Ok(Self::Vanilla),
			input => Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("Mode"),
					got: input.to_owned(),
				},
				msg: format!("`{}` is not a valid Mode.", input),
			}),
		}
	}
}

impl TryFrom<u8> for Mode {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			200 => Ok(Self::KZTimer),
			201 => Ok(Self::SimpleKZ),
			202 => Ok(Self::Vanilla),
			id => Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("valid ModeID (200/201/202)"),
					got: id.to_string(),
				},
				msg: format!("`{}` is not a valid Mode ID.", id),
			}),
		}
	}
}

impl From<Mode> for u8 {
	fn from(val: Mode) -> Self {
		match val {
			Mode::KZTimer => 200,
			Mode::SimpleKZ => 201,
			Mode::Vanilla => 202,
		}
	}
}

/// A Map can be represented in multiple ways when making requests to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V).
/// - ID => `992`
/// - Name => `"kz_lionharder"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapIdentifier {
	ID(i32),
	Name(String),
}

impl std::fmt::Display for MapIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MapIdentifier::ID(map_id) => write!(f, "{}", map_id),
			MapIdentifier::Name(map_name) => write!(f, "{}", map_name),
		}
	}
}

/// A Player can be represented in multiple ways when making requests to the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V).
/// - Name => `"AlphaKeks"`
/// - SteamID => `SteamID("STEAM_1:1:161178172")`
/// - SteamID64 => `76561198282622073`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerIdentifier {
	Name(String),
	SteamID(SteamID),
	SteamID64(u64),
}

impl std::fmt::Display for PlayerIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PlayerIdentifier::Name(name) => write!(f, "{}", name),
			PlayerIdentifier::SteamID(steam_id) => write!(f, "{}", steam_id),
			PlayerIdentifier::SteamID64(steam_id64) => write!(f, "{}", steam_id64),
		}
	}
}

impl std::str::FromStr for PlayerIdentifier {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(steam_id) = SteamID::new(s) {
			return Ok(Self::SteamID(steam_id));
		}

		Ok(Self::Name(s.to_owned()))
	}
}

impl From<String> for PlayerIdentifier {
	fn from(input: String) -> Self {
		input.parse().expect("Infallible Error")
	}
}

impl From<SteamID> for PlayerIdentifier {
	fn from(steam_id: SteamID) -> Self {
		Self::SteamID(steam_id)
	}
}

impl From<u64> for PlayerIdentifier {
	fn from(steam_id64: u64) -> Self {
		Self::SteamID64(steam_id64)
	}
}

impl TryFrom<PlayerIdentifier> for u64 {
	type Error = Error;

	fn try_from(value: PlayerIdentifier) -> Result<Self, Self::Error> {
		if let PlayerIdentifier::SteamID64(steam_id64) = value {
			return Ok(steam_id64);
		}

		Err(Error {
			kind: ErrorKind::InvalidInput {
				expected: String::from("SteamID64"),
				got: value.to_string(),
			},
			msg: format!("`{}` is not a SteamID64.", value),
		})
	}
}

/// Every player who has joined a [GOKZ](https://github.com/KZGlobalTeam/gokz) server with version 3.0.0 or higher will get a [`Rank`]
/// assigned to them. Which [`Rank`] they will have is based on the player's total points in a
/// given [`Mode`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
	pub fn from_points(points: u32, mode: Mode) -> Self {
		use Rank::*;

		let is_kzt = mode == Mode::KZTimer;
		let is_skz = mode == Mode::SimpleKZ;
		let is_vnl = mode == Mode::Vanilla;

		// This was harder than I want to admit.
		match points {
			0 => New,
			1..=499 => BeginnerMinus,
			500..=999 => Beginner,
			1_000..=1_999 => BeginnerPlus,
			2_000..=4_999 => AmateurMinus,
			5_000..=9_999 => Amateur,
			10_000..=19_999 => AmateurPlus,
			20_000..=29_999 => CasualMinus,
			30_000..=39_999 => Casual,
			40_000..=59_999 => CasualPlus,
			60_000..=69_999 => RegularMinus,
			70_000..=79_999 => Regular,
			80_000..=99_999 => RegularPlus,
			100_000..=119_999 => SkilledMinus,
			120_000..=139_999 if is_vnl => Skilled,
			120_000..=149_999 if (is_skz || is_kzt) => Skilled,
			140_000..=159_999 if is_vnl => SkilledPlus,
			150_000..=199_999 if (is_skz || is_kzt) => SkilledPlus,
			160_000..=179_999 if is_vnl => ExpertMinus,
			200_000..=229_999 if (is_skz || is_kzt) => ExpertMinus,
			180_000..=199_999 if is_vnl => Expert,
			230_000..=249_999 if (is_skz || is_kzt) => Expert,
			200_000..=249_999 if is_vnl => ExpertPlus,
			250_000..=299_999 if is_skz => ExpertPlus,
			250_000..=399_999 if is_kzt => ExpertPlus,
			250_000..=299_999 if is_vnl => Semipro,
			300_000..=399_999 if is_skz => Semipro,
			400_000..=599_999 if is_kzt => Semipro,
			300_000..=399_999 if is_vnl => Pro,
			400_000..=499_999 if is_skz => Pro,
			600_000..=799_999 if is_kzt => Pro,
			400_000..=599_999 if is_vnl => Master,
			500_000..=799_999 if is_skz => Master,
			800_000..=999_999 if is_kzt => Master,
			(600_000..) if is_vnl => Legend,
			(800_000..) if is_skz => Legend,
			(1_000_000..) if is_kzt => Legend,
			_ => unreachable!(),
		}
	}
}

impl std::fmt::Display for Rank {
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
		write!(f, "{}", s)
	}
}

impl std::str::FromStr for Rank {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"legend" => Ok(Rank::Legend),
			"master" => Ok(Rank::Master),
			"pro" => Ok(Rank::Pro),
			"semipro" => Ok(Rank::Semipro),
			"expert+" | "expertplus" => Ok(Rank::ExpertPlus),
			"expert" => Ok(Rank::Expert),
			"expert-" | "expertminus" => Ok(Rank::ExpertMinus),
			"skilled+" | "skilledplus" => Ok(Rank::SkilledPlus),
			"skilled" => Ok(Rank::Skilled),
			"skilled-" | "skilledminus" => Ok(Rank::SkilledMinus),
			"regular+" | "regularplus" => Ok(Rank::RegularPlus),
			"regular" => Ok(Rank::Regular),
			"regular-" | "regularminus" => Ok(Rank::RegularMinus),
			"casual+" | "casualplus" => Ok(Rank::CasualPlus),
			"casual" => Ok(Rank::Casual),
			"casual-" | "casualminus" => Ok(Rank::CasualMinus),
			"amateur+" | "amateurplus" => Ok(Rank::AmateurPlus),
			"amateur" => Ok(Rank::Amateur),
			"amateur-" | "amateurminus" => Ok(Rank::AmateurMinus),
			"beginner+" | "beginnerplus" => Ok(Rank::BeginnerPlus),
			"beginner" => Ok(Rank::Beginner),
			"beginner-" | "beginnerminus" => Ok(Rank::BeginnerMinus),
			"new" => Ok(Rank::New),
			input => Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("Rank"),
					got: input.to_owned(),
				},
				msg: format!("`{}` is not a valid Rank.", input),
			}),
		}
	}
}

/// Every global map in [GOKZ](https://github.com/KZGlobalTeam/gokz) has a difficulty level which
/// is referred to as its "tier". Currently there are the 7 following tiers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Tier {
	VeryEasy = 1,
	Easy = 2,
	Medium = 3,
	Hard = 4,
	VeryHard = 5,
	Extreme = 6,
	Death = 7,
}

impl std::fmt::Display for Tier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Tier::VeryEasy => "Very Easy",
			Tier::Easy => "Easy",
			Tier::Medium => "Medium",
			Tier::Hard => "Hard",
			Tier::VeryHard => "Very Hard",
			Tier::Extreme => "Extreme",
			Tier::Death => "Death",
		};
		write!(f, "{}", s)
	}
}

impl std::str::FromStr for Tier {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"very easy" | "very_easy" => Ok(Self::VeryEasy),
			"easy" => Ok(Self::Easy),
			"medium" => Ok(Self::Medium),
			"hard" => Ok(Self::Hard),
			"very hard" | "very_hard" => Ok(Self::VeryHard),
			"extreme" => Ok(Self::Extreme),
			"death" => Ok(Self::Death),
			input => Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("Tier"),
					got: input.to_owned(),
				},
				msg: format!("`{}` is not a valid Tier.", input),
			}),
		}
	}
}

impl TryFrom<u8> for Tier {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::VeryEasy),
			2 => Ok(Self::Easy),
			3 => Ok(Self::Medium),
			4 => Ok(Self::Hard),
			5 => Ok(Self::VeryHard),
			6 => Ok(Self::Extreme),
			7 => Ok(Self::Death),
			input => Err(Error {
				kind: ErrorKind::InvalidInput {
					expected: String::from("valid Tier (1-7)"),
					got: input.to_string(),
				},
				msg: format!("`{}` is not a valid Tier.", input),
			}),
		}
	}
}

impl From<Tier> for u8 {
	fn from(val: Tier) -> Self {
		match val {
			Tier::VeryEasy => 1,
			Tier::Easy => 2,
			Tier::Medium => 3,
			Tier::Hard => 4,
			Tier::VeryHard => 5,
			Tier::Extreme => 6,
			Tier::Death => 7,
		}
	}
}
