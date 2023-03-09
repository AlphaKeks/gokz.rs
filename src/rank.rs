use {
	crate::{Error, Mode, Result},
	serde::{Deserialize, Serialize},
	std::fmt::Display,
};

/// Every player who has joined a [GOKZ](https://github.com/KZGlobalTeam/gokz) server with version 3.0.0 or higher will get a [`Rank`]
/// assigned to them. Which [`Rank`] they will have is based on the player's total points in a
/// given [`Mode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(missing_docs)]
#[non_exhaustive]
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
	/// Constructs a [`Rank`] from a specified amount of points and a mode. Ranks have different
	/// tresholds depending on mode.
	pub fn from_points(points: u32, mode: Mode) -> Self {
		use Rank::*;

		let is_kzt = mode == Mode::KZTimer;
		let is_skz = mode == Mode::SimpleKZ;
		let is_vnl = mode == Mode::Vanilla;

		// I just hope this is correct
		match points {
			(1_000_000..) if is_kzt => Legend,
			(800_000..) if is_skz => Legend,
			(600_000..) if is_vnl => Legend,
			(800_000..) if is_kzt => Master,
			(500_000..) if is_skz => Master,
			(400_000..) if is_vnl => Master,
			(600_000..) if is_kzt => Pro,
			(400_000..) if is_skz => Pro,
			(300_000..) if is_vnl => Pro,
			(400_000..) if is_kzt => Semipro,
			(300_000..) if is_skz => Semipro,
			(250_000..) if is_vnl => Semipro,
			(250_000..) if is_kzt || is_skz => ExpertPlus,
			(200_000..) if is_vnl => ExpertPlus,
			(230_000..) if is_kzt || is_skz => Expert,
			(180_000..) if is_vnl => Expert,
			(200_000..) if is_kzt || is_skz => ExpertMinus,
			(160_000..) if is_vnl => ExpertMinus,
			(150_000..) if is_kzt || is_skz => SkilledPlus,
			(140_000..) if is_vnl => SkilledPlus,
			120_000.. => Skilled,
			100_000.. => SkilledMinus,
			80_000.. => RegularPlus,
			70_000.. => Regular,
			60_000.. => RegularMinus,
			40_000.. => CasualPlus,
			30_000.. => Casual,
			20_000.. => CasualMinus,
			10_000.. => AmateurPlus,
			5_000.. => AmateurPlus,
			1_000.. => AmateurMinus,
			500.. => Beginner,
			1.. => BeginnerMinus,
			_ => New,
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
		write!(f, "{}", s)
	}
}

impl std::str::FromStr for Rank {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
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
			input => Err(Error::InvalidRank {
				value: String::from(input),
			}),
		}
	}
}
