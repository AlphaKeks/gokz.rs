use crate::{
	error::{err, Error, Result},
	types::Mode,
};

/// All ranks that currently exist in GOKZ.
#[allow(missing_docs)] // These should be self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize),
	serde(rename_all = "snake_case")
)]
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

impl std::fmt::Display for Rank {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if matches!(
			self,
			Rank::Legend
				| Rank::Master | Rank::Pro
				| Rank::Semipro | Rank::Expert
				| Rank::Skilled | Rank::Regular
				| Rank::Casual | Rank::Amateur
				| Rank::Beginner | Rank::New
		) {
			return write!(f, "{self:?}");
		}

		f.write_str(match self {
			Rank::ExpertPlus => "Expert+",
			Rank::ExpertMinus => "Expert-",
			Rank::SkilledPlus => "Skilled+",
			Rank::SkilledMinus => "Skilled-",
			Rank::RegularPlus => "Regular+",
			Rank::RegularMinus => "Regular-",
			Rank::CasualPlus => "Casual+",
			Rank::CasualMinus => "Casual-",
			Rank::AmateurPlus => "Amateur+",
			Rank::AmateurMinus => "Amateur-",
			Rank::BeginnerPlus => "Beginner+",
			Rank::BeginnerMinus => "Beginner-",
			_ => unreachable!(),
		})
	}
}

impl std::str::FromStr for Rank {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"legend" => Rank::Legend,
			"master" => Rank::Master,
			"pro" => Rank::Pro,
			"semipro" => Rank::Semipro,
			"expert+" | "expertplus" => Rank::ExpertPlus,
			"expert" => Rank::Expert,
			"expert-" | "expertminus" => Rank::ExpertMinus,
			"skilled+" | "skilledplus" => Rank::SkilledPlus,
			"skilled" => Rank::Skilled,
			"skilled-" | "skilledminus" => Rank::SkilledMinus,
			"regular+" | "regularplus" => Rank::RegularPlus,
			"regular" => Rank::Regular,
			"regular-" | "regularminus" => Rank::RegularMinus,
			"casual+" | "casualplus" => Rank::CasualPlus,
			"casual" => Rank::Casual,
			"casual-" | "casualminus" => Rank::CasualMinus,
			"amateur+" | "amateurplus" => Rank::AmateurPlus,
			"amateur" => Rank::Amateur,
			"amateur-" | "amateurminus" => Rank::AmateurMinus,
			"beginner+" | "beginnerplus" => Rank::BeginnerPlus,
			"beginner" => Rank::Beginner,
			"beginner-" | "beginnerminus" => Rank::BeginnerMinus,
			"new" => Rank::New,
			input => return Err(err!("`{input}` is not a valid rank.")),
		})
	}
}
