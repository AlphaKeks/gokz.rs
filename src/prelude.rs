use {
	log::warn,
	regex::Regex,
	serde::{Deserialize, Serialize},
};

/// The default Error type which gets returned from any function exposed by this crate.
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
		};

		write!(f, "{}", msg)
	}
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
	/// `expected`: the expected type
	/// `got`: original input which is invalid
	InvalidInput { expected: String, got: String },

	/// `expected`: the expected type
	/// `got`: original input which failed to be parsed
	Parsing { expected: String, got: Option<String> },

	/// `expected`: the expected type
	NoData { expected: String },

	/// `status_code`: HTTP Status Code
	/// `raw_message`: the message returned by the GlobalAPI (if there is one)
	GlobalAPI { status_code: Option<String>, raw_message: Option<String> },
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
				kind: ErrorKind::InvalidInput {
					expected: String::from("Mode"),
					got: input.to_owned(),
				},
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
/// - Name => `"kz_lionharder"`
/// - ID => `992`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapIdentifier {
	ID(u32),
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

// impl std::str::FromStr for MapIdentifier {
// 	type Err = Error;
//
// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		match s.parse::<u16>() {
// 			Ok(map_id) => {
// 				// use mapcycle
// 				let total_maps = todo!();
// 				(200..=total_maps).contains(map_id)
// 			}
// 			_ => if let Ok(map) = global_api::is_global(s.to_owned()) {
// 				Self::Name(map.name)
// 			}
// 		}
//
// 		Err(Error {
// 			kind: ErrorKind::InvalidInput {
// 				input: s.to_owned(),
// 			},
// 			msg: format!("`{}` is not a global map.", s)
// 		})
// 	}
// }

// impl TryFrom<u16> for MapIdentifier {
// 	type Error = Error;
//
// 	fn try_from(value: u16) -> Result<Self, Self::Error> {
// 		&(value.to_string()).parse()
// 	}
// }

// impl TryFrom<MapIdentifier> for u16 {
// 	type Error = Error;
//
// 	fn try_from(value: MapIdentifier) -> Result<Self, Self::Error> {
// 		if let MapIdentifier::ID(map_id) = value {
// 			return Ok(map_id);
// 		}
//
// 		Err(Error {
// 			kind: ErrorKind::InvalidInput { input: value.to_string() },
// 			msg: format!("`{}` is not a MapID.", value),
// 		})
// 	}
// }

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
				kind: ErrorKind::InvalidInput {
					expected: String::from("Rank"),
					got: input.to_owned(),
				},
				msg: format!("`{}` is not a valid Rank.", input),
			}),
		}
	}
}

/* ---------------------------------------------------------------------------------------------- */

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
	fn steam_id_from_u64() {
		let alphakeks_64 = 76561198282622073;
		let alphakeks_32 = "STEAM_1:1:161178172";
		assert_eq!(alphakeks_32, SteamID::from(alphakeks_64).to_string());

		println!("{}", SteamID::from(76561198231238712));

		let blacky_64 = 76561198091592005;
		let blacky_32 = "STEAM_1:1:65663138";
		assert_eq!(blacky_32, SteamID::from(blacky_64).to_string());

		let charlie_64 = 76561198054062420;
		let charlie_32 = "STEAM_1:0:46898346";
		assert_eq!(charlie_32, SteamID::from(charlie_64).to_string());

		let idot_64 = 76561198955057247;
		let idot_32 = "STEAM_1:1:497395759";
		assert_eq!(idot_32, SteamID::from(idot_64).to_string());

		let ibra_64 = 76561198264939817;
		let ibra_32 = "STEAM_1:1:152337044";
		assert_eq!(ibra_32, SteamID::from(ibra_64).to_string());

		let szwagi_64 = 76561198857828380;
		let szwagi_32 = "STEAM_1:0:448781326";
		assert_eq!(szwagi_32, SteamID::from(szwagi_64).to_string());

		let gosh_64 = 76561198292029626;
		let gosh_32 = "STEAM_1:0:165881949";
		assert_eq!(gosh_32, SteamID::from(gosh_64).to_string());
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
