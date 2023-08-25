//! This module contains an enum for TP / PRO.

use {
	crate::{
		macros::{
			convert::{from, try_from},
			is,
		},
		yeet,
	},
	std::{fmt::Display, str::FromStr},
};

#[cfg(feature = "serde")]
mod serde;

/// The two runtypes.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(rename_all = "lowercase"))]
pub enum Runtype {
	/// The run was done without teleports.
	#[default]
	Pro = 0,

	/// The run was done with teleports.
	TP = 1,
}

#[rustfmt::skip]
impl Runtype {
	is!(is_pro, Pro);
	is!(is_tp, TP);
}

impl Display for Runtype {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl From<bool> for Runtype {
	fn from(value: bool) -> Self {
		match value {
			true => Runtype::TP,
			false => Runtype::Pro,
		}
	}
}

impl From<Runtype> for bool {
	fn from(value: Runtype) -> Self {
		value.is_tp()
	}
}

from!([u8, u16, u32, u64, u128, usize] => Runtype => |int| {
	match int > 0 {
		true => Runtype::TP,
		false => Runtype::Pro,
	}
});

try_from!([i8, i16, i32, i64, i128, isize] => Runtype => |int| {
	use std::cmp::Ordering::*;
	match int.cmp(&0) {
		Less => yeet!(InvalidTeleportAmount),
		Equal => Ok(Runtype::Pro),
		Greater => Ok(Runtype::TP),
	}
});

impl TryFrom<&str> for Runtype {
	type Error = crate::Error;

	fn try_from(input: &str) -> crate::Result<Self> {
		FromStr::from_str(input)
	}
}

impl TryFrom<String> for Runtype {
	type Error = crate::Error;

	fn try_from(input: String) -> crate::Result<Self> {
		Self::try_from(input.as_str())
	}
}

impl FromStr for Runtype {
	type Err = crate::Error;

	fn from_str(input: &str) -> crate::Result<Self> {
		Ok(match input {
			"PRO" | "PRo" | "PrO" | "pRO" | "Pro" | "pRo" | "prO" | "pro" => Self::Pro,
			"TP" | "Tp" | "tP" | "tp" => Self::TP,
			_ => yeet!(InvalidRuntype(input)),
		})
	}
}

#[cfg(feature = "poise")]
#[poise::async_trait]
impl poise::SlashArgument for Runtype {
	async fn extract(
		_: &poise::serenity_prelude::Context,
		_: poise::ApplicationCommandOrAutocompleteInteraction<'_>,
		value: &poise::serenity_prelude::json::Value,
	) -> Result<Self, poise::SlashArgError> {
		let choice_key = value
			.as_u64()
			.ok_or(poise::SlashArgError::CommandStructureMismatch("expected u64"))?;

		Ok(match choice_key {
			0 => Runtype::Pro,
			1 => Runtype::TP,
			_ => {
				return Err(poise::SlashArgError::CommandStructureMismatch(
					"out of bounds choice key",
				));
			}
		})
	}

	fn create(builder: &mut poise::serenity_prelude::CreateApplicationCommandOption) {
		builder.kind(poise::serenity_prelude::CommandOptionType::Integer);
	}

	fn choices() -> Vec<poise::CommandParameterChoice> {
		vec![
			poise::CommandParameterChoice {
				name: String::from("Pro"),
				localizations: Default::default(),
			},
			poise::CommandParameterChoice {
				name: String::from("TP"),
				localizations: Default::default(),
			},
		]
	}
}
