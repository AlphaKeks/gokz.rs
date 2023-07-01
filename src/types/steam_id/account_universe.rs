use crate::error::{err, Error, Result};

/// NOTE: [Official Documentation](
///   https://developer.valvesoftware.com/wiki/SteamID#Universes_Available_for_Steam_Accounts
/// )
#[allow(missing_docs)] // I honestly have no idea what to put here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize),
	serde(rename_all = "snake_case")
)]
pub enum AccountUniverse {
	Individual,
	Public,
	Beta,
	Internal,
	Dev,
	Rc,
}

impl std::fmt::Display for AccountUniverse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// The `Debug` representation is the same as the variant's name, so this is convenient.
		write!(f, "{self:?}")
	}
}

impl std::str::FromStr for AccountUniverse {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.to_lowercase().as_str() {
			"individual" => AccountUniverse::Individual,
			"public" => AccountUniverse::Public,
			"beta" => AccountUniverse::Beta,
			"internal" => AccountUniverse::Internal,
			"dev" => AccountUniverse::Dev,
			"rc" => AccountUniverse::Rc,
			input => {
				return Err(err!("`{input}` is not a valid steam account universe."));
			}
		})
	}
}
