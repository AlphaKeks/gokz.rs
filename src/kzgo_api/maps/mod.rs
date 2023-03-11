use {
	crate::{
		chrono::{parse_date, ser_date},
		Error, Result, SteamID, Tier,
	},
	chrono::NaiveDateTime,
	serde::Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct Map {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub bonuses: u8,
	pub mappers: Vec<(String, SteamID)>,
	pub skz: bool,
	pub vnl: bool,
	pub workshop_id: u32,
	#[serde(serialize_with = "ser_date")]
	pub date: NaiveDateTime,
}

/// `/maps`
pub mod index;
impl TryFrom<index::Response> for Map {
	type Error = Error;

	fn try_from(value: index::Response) -> Result<Self> {
		let mappers = std::iter::zip(value.mapperNames, value.mapperIds)
			.filter_map(|(mapper_name, mapper_id)| {
				Some((mapper_name, SteamID::new(&mapper_id).ok()?))
			})
			.collect();

		Ok(Self {
			id: value.id,
			name: value.name,
			tier: value.tier.try_into()?,
			bonuses: value.bonuses,
			mappers,
			skz: value.sp,
			vnl: value.vp,
			workshop_id: value.workshopId.parse()?,
			date: parse_date!(value.date),
		})
	}
}
