use {
	crate::{Error, Mode, Result},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CompletionStats {
	pub mode: Mode,
	/// Index 0 is the total amount of completions. The other indices match their
	/// [Tier](crate::Tier).
	pub tp: [u16; 8],
	/// Index 0 is the total amount of completions. The other indices match their
	/// [Tier](crate::Tier).
	pub pro: [u16; 8],
}

/// `/completions/:mode`
pub mod mode;
impl TryFrom<mode::Response> for CompletionStats {
	type Error = Error;

	fn try_from(value: mode::Response) -> Result<Self> {
		Ok(Self {
			mode: value.mode.parse()?,
			tp: [
				value.tp.total,
				value.tp.one,
				value.tp.two,
				value.tp.three,
				value.tp.four,
				value.tp.five,
				value.tp.six,
				value.tp.seven,
			],
			pro: [
				value.pro.total,
				value.pro.one,
				value.pro.two,
				value.pro.three,
				value.pro.four,
				value.pro.five,
				value.pro.six,
				value.pro.seven,
			],
		})
	}
}
