use {
	regex::Regex,
	serde::{Deserialize, Serialize},
};

/// The default Error type which gets returned from any function exposed by this crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
	pub(crate) kind: ErrorKind,
	pub msg: String,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.msg)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum ErrorKind {
	/// `input`: original input which is invalid
	InvalidInput {
		input: String,
	},

	/// `input`: original input which failed to be parsed
	Parsing {
		input: Option<String>,
	},

	/// `status_code`: HTTP Status Code
	/// `raw_message`: the message returned by the GlobalAPI (if there is one)
	GlobalAPI {
		status_code: Option<String>,
		raw_message: Option<String>,
	},

	NoData,
}

/// A unique identifier for a [Steam](https://www.steamcommunity.com/) Account.
///
/// Note: [official definition](https://developer.valvesoftware.com/wiki/SteamID).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
		let regex = Regex::new(r#"^STEAM_[0-1]:[0-1]:[0-9]+$"#).expect("This is a valid regex.");
		regex.is_match(input)
	}

	pub fn new(steam_id: &str) -> Result<Self, Error> {
		let steam_id = steam_id.to_owned();
		if Self::test(&steam_id) {
			Ok(SteamID(steam_id))
		} else {
			Err(Error {
				kind: ErrorKind::InvalidInput { input: steam_id.clone() },
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
			kind: ErrorKind::InvalidInput { input: value.to_string() },
			msg: format!("`{}` is a name, not a SteamID.", value),
		})
	}
}

/* --------------------------------------------------------------------------------------------- */

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
	pub fn api(&self) -> &str {
		match self {
			Mode::KZTimer => "kz_timer",
			Mode::SimpleKZ => "kz_simple",
			Mode::Vanilla => "kz_vanilla",
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
				kind: ErrorKind::InvalidInput { input: input.to_owned() },
				msg: format!("`{}` is not a valid Mode.", input),
			}),
		}
	}
}

impl TryFrom<String> for Mode {
	type Error = Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<u8> for Mode {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			200 => Ok(Self::KZTimer),
			201 => Ok(Self::SimpleKZ),
			202 => Ok(Self::Vanilla),
			_ => Err(Error {
				kind: ErrorKind::InvalidInput { input: value.to_string() },
				msg: format!("`{}` is not a valid Mode ID.", value),
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
			kind: ErrorKind::InvalidInput { input: value.to_string() },
			msg: format!("`{}` is not a SteamID64.", value),
		})
	}
}

/// Every player who has joined a [GOKZ](https://github.com/KZGlobalTeam/gokz) server with version 3.0.0 or higher will get a [`Rank`]
/// assigned to them. Which [`Rank`] they will have is based on the player's total points in a
/// given [`Mode`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
		let is_kzt = mode == &Mode::KZTimer;
		let is_skz = mode == &Mode::SimpleKZ;
		let is_vnl = mode == &Mode::Vanilla;

		match points {
			0 => Self::New,
			1..=499 => Self::BeginnerMinus,
			500..=999 => Self::Beginner,
			1_000..=1_999 => Self::BeginnerPlus,
			2_000..=4_999 => Self::AmateurMinus,
			5_000..=9_999 => Self::Amateur,
			10_000..=19_999 => Self::AmateurPlus,
			20_000..=29_999 => Self::CasualMinus,
			30_000..=39_999 => Self::Casual,
			40_000..=59_999 => Self::CasualPlus,
			60_000..=69_999 => Self::RegularMinus,
			70_000..=79_999 => Self::Regular,
			80_000..=99_999 => Self::RegularPlus,
			100_000..=119_999 => Self::SkilledMinus,
			120_000..=139_999 if is_vnl => Self::Skilled,
			120_000..=149_999 if is_vnl => Self::Skilled,
			140_000..=159_999 if is_vnl => Self::SkilledPlus,
			150_000..=199_999 if is_vnl => Self::SkilledPlus,
			160_000..=179_999 if is_vnl => Self::ExpertMinus,
			200_000..=229_999 if is_vnl => Self::ExpertMinus,
			180_000..=199_999 if is_vnl => Self::Expert,
			230_000..=249_999 if is_vnl => Self::Expert,
			200_000..=249_999 if is_vnl => Self::ExpertPlus,
			250_000..=299_999 if is_skz => Self::ExpertPlus,
			250_000..=399_999 if is_kzt => Self::ExpertPlus,
			250_000..=299_999 if is_vnl => Self::Semipro,
			300_000..=399_999 if is_skz => Self::Semipro,
			400_000..=599_999 if is_kzt => Self::Semipro,
			300_000..=399_999 if is_vnl => Self::Pro,
			400_000..=499_999 if is_skz => Self::Pro,
			600_000..=799_999 if is_kzt => Self::Pro,
			400_000..=599_999 if is_vnl => Self::Master,
			500_000..=799_999 if is_skz => Self::Master,
			800_000..=999_999 if is_kzt => Self::Master,
			(600_000..) if is_vnl => Self::Legend,
			(800_000..) if is_skz => Self::Legend,
			(1_000_000..) if is_kzt => Self::Legend,
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
				kind: ErrorKind::InvalidInput { input: input.to_owned() },
				msg: format!("`{}` is not a valid Rank.", input),
			}),
		}
	}
}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn steam_id_constructor() {
		let valid = SteamID::new("STEAM_1:1:161178172");
		let invalid1 = SteamID::new("wordSTEAM_1:1:161178172");
		let invalid2 = SteamID::new("STEAM_1:1:161178172word");
		let invalid3 = SteamID::new("some random text");

		assert!(dbg!(valid).is_ok());
		assert!(dbg!(invalid1).is_err());
		assert!(dbg!(invalid2).is_err());
		assert!(dbg!(invalid3).is_err());
	}

	#[test]
	fn steam_id_from_playeridentifier() {
		let name = PlayerIdentifier::Name(String::from("AlphaKeks"));
		let steam_id = PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172").unwrap());
		let steam_id64 = PlayerIdentifier::SteamID64(76561198282622073);

		assert!(SteamID::try_from(name).is_err());
		assert!(SteamID::try_from(steam_id).is_ok());
		assert!(SteamID::try_from(steam_id64).is_err());
	}

	#[test]
	fn mode_api() {
		let kzt = Mode::KZTimer;
		let skz = Mode::SimpleKZ;
		let vnl = Mode::Vanilla;

		assert_eq!("kz_timer", kzt.api());
		assert_eq!("kz_simple", skz.api());
		assert_eq!("kz_vanilla", vnl.api());
	}

	#[test]
	fn mode_short() {
		let kzt = Mode::KZTimer;
		let skz = Mode::SimpleKZ;
		let vnl = Mode::Vanilla;

		assert_eq!("KZT", &kzt.short());
		assert_eq!("SKZ", &skz.short());
		assert_eq!("VNL", &vnl.short());
	}

	#[test]
	fn mode_display() {
		let kzt = Mode::KZTimer;
		let skz = Mode::SimpleKZ;
		let vnl = Mode::Vanilla;

		assert_eq!("KZTimer", &kzt.to_string());
		assert_eq!("SimpleKZ", &skz.to_string());
		assert_eq!("Vanilla", &vnl.to_string());
	}

	#[test]
	fn mode_from_str() {
		assert!("kztimer".parse::<Mode>().is_ok());
		assert!("kz_timer".parse::<Mode>().is_ok());
		assert!("kzt".parse::<Mode>().is_ok());
		assert!("simplekz".parse::<Mode>().is_ok());
		assert!("simple_kz".parse::<Mode>().is_ok());
		assert!("kz_simple".parse::<Mode>().is_ok());
		assert!("skz".parse::<Mode>().is_ok());
		assert!("vanilla".parse::<Mode>().is_ok());
		assert!("vanillakz".parse::<Mode>().is_ok());
		assert!("vanilla_kz".parse::<Mode>().is_ok());
		assert!("vnl".parse::<Mode>().is_ok());
	}

	#[test]
	fn mode_try_from_u8() {
		for i in 0..=u8::MAX {
			match i {
				200 => assert!(Mode::try_from(200).is_ok()),
				201 => assert!(Mode::try_from(201).is_ok()),
				202 => assert!(Mode::try_from(202).is_ok()),
				n => assert!(Mode::try_from(n).is_err()),
			}
		}
	}

	#[test]
	fn mode_into_u8() {
		assert_eq!(200, Mode::KZTimer as u8);
		assert_eq!(201, Mode::SimpleKZ as u8);
		assert_eq!(202, Mode::Vanilla as u8);

		let kzt: u8 = Mode::KZTimer.into();
		assert_eq!(200, kzt);
		let skz: u8 = Mode::SimpleKZ.into();
		assert_eq!(201, skz);
		let vnl: u8 = Mode::Vanilla.into();
		assert_eq!(202, vnl);
	}
}
