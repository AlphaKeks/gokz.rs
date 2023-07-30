//! This module holds an enum for player ranks. Based on points each player will have a rank
//! assigned to them. The threshold for each of these ranks may also depend on the mode.

use crate::{macros::is, Mode};

/// All GOKZ player ranks.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Rank {
	New,
	#[cfg_attr(feature = "serde", serde(rename = "Beginner-"))]
	BeginnerMinus,
	Beginner,
	#[cfg_attr(feature = "serde", serde(rename = "Beginner+"))]
	BeginnerPlus,
	#[cfg_attr(feature = "serde", serde(rename = "Amateur-"))]
	AmateurMinus,
	Amateur,
	#[cfg_attr(feature = "serde", serde(rename = "Amateur+"))]
	AmateurPlus,
	#[cfg_attr(feature = "serde", serde(rename = "Casual-"))]
	CasualMinus,
	Casual,
	#[cfg_attr(feature = "serde", serde(rename = "Casual+"))]
	CasualPlus,
	#[cfg_attr(feature = "serde", serde(rename = "Regular-"))]
	RegularMinus,
	Regular,
	#[cfg_attr(feature = "serde", serde(rename = "Regular+"))]
	RegularPlus,
	#[cfg_attr(feature = "serde", serde(rename = "Skilled-"))]
	SkilledMinus,
	Skilled,
	#[cfg_attr(feature = "serde", serde(rename = "Skilled+"))]
	SkilledPlus,
	#[cfg_attr(feature = "serde", serde(rename = "Expert-"))]
	ExpertMinus,
	Expert,
	#[cfg_attr(feature = "serde", serde(rename = "Expert+"))]
	ExpertPlus,
	Semipro,
	Pro,
	Master,
	Legend,
}

impl Rank {
	/// Constructs a [`Rank`] from the given amount of points and mode.
	pub const fn from_points(points: u32, mode: Mode) -> Self {
		match mode {
			Mode::KZTimer => match points {
				1_000_000.. => Self::Legend,
				800_000.. => Self::Master,
				600_000.. => Self::Pro,
				400_000.. => Self::Semipro,

				250_000.. => Self::ExpertPlus,
				230_000.. => Self::Expert,
				200_000.. => Self::ExpertMinus,

				150_000.. => Self::SkilledPlus,
				120_000.. => Self::Skilled,
				100_000.. => Self::SkilledMinus,

				80_000.. => Self::RegularPlus,
				70_000.. => Self::Regular,
				60_000.. => Self::RegularMinus,

				40_000.. => Self::CasualPlus,
				30_000.. => Self::Casual,
				20_000.. => Self::CasualMinus,

				10_000.. => Self::AmateurPlus,
				5_000.. => Self::Amateur,
				2_000.. => Self::AmateurMinus,

				1_000.. => Self::BeginnerPlus,
				500.. => Self::Beginner,
				1.. => Self::BeginnerMinus,

				0 => Self::New,
			},

			Mode::SimpleKZ => match points {
				800_000.. => Self::Legend,
				500_000.. => Self::Master,
				400_000.. => Self::Pro,
				300_000.. => Self::Semipro,

				250_000.. => Self::ExpertPlus,
				230_000.. => Self::Expert,
				200_000.. => Self::ExpertMinus,

				150_000.. => Self::SkilledPlus,
				120_000.. => Self::Skilled,
				100_000.. => Self::SkilledMinus,

				80_000.. => Self::RegularPlus,
				70_000.. => Self::Regular,
				60_000.. => Self::RegularMinus,

				40_000.. => Self::CasualPlus,
				30_000.. => Self::Casual,
				20_000.. => Self::CasualMinus,

				10_000.. => Self::AmateurPlus,
				5_000.. => Self::Amateur,
				2_000.. => Self::AmateurMinus,

				1_000.. => Self::BeginnerPlus,
				500.. => Self::Beginner,
				1.. => Self::BeginnerMinus,

				0 => Self::New,
			},

			Mode::Vanilla => match points {
				600_000.. => Self::Legend,
				400_000.. => Self::Master,
				300_000.. => Self::Pro,
				250_000.. => Self::Semipro,

				200_000.. => Self::ExpertPlus,
				180_000.. => Self::Expert,
				160_000.. => Self::ExpertMinus,

				140_000.. => Self::SkilledPlus,
				120_000.. => Self::Skilled,
				100_000.. => Self::SkilledMinus,

				80_000.. => Self::RegularPlus,
				70_000.. => Self::Regular,
				60_000.. => Self::RegularMinus,

				40_000.. => Self::CasualPlus,
				30_000.. => Self::Casual,
				20_000.. => Self::CasualMinus,

				10_000.. => Self::AmateurPlus,
				5_000.. => Self::Amateur,
				2_000.. => Self::AmateurMinus,

				1_000.. => Self::BeginnerPlus,
				500.. => Self::Beginner,
				1.. => Self::BeginnerMinus,

				0.. => Self::New,
			},
		}
	}
}

#[rustfmt::skip]
impl Rank {
	is!(is_new, New);
	is!(is_beginner_minus, BeginnerMinus);
	is!(is_beginner, Beginner);
	is!(is_beginner_plus, BeginnerPlus);
	is!(is_amateur_minus, AmateurMinus);
	is!(is_amateur, Amateur);
	is!(is_amateur_plus, AmateurPlus);
	is!(is_casual_minus, CasualMinus);
	is!(is_casual, Casual);
	is!(is_casual_plus, CasualPlus);
	is!(is_regular_minus, RegularMinus);
	is!(is_regular, Regular);
	is!(is_regular_plus, RegularPlus);
	is!(is_skilled_minus, SkilledMinus);
	is!(is_skilled, Skilled);
	is!(is_skilled_plus, SkilledPlus);
	is!(is_expert_minus, ExpertMinus);
	is!(is_expert, Expert);
	is!(is_expert_plus, ExpertPlus);
	is!(is_semipro, Semipro);
	is!(is_pro, Pro);
	is!(is_master, Master);
	is!(is_legend, Legend);
}
