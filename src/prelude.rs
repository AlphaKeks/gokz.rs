#[rustfmt::skip]
#[cfg(feature = "reqwest")]
pub use reqwest::Client;

#[rustfmt::skip]
#[cfg(feature = "global-api")]
pub use crate::types::Rank;

#[rustfmt::skip]
pub use crate::{
	error::{Error, Result},
	traits::{
		MapIdentifier as _, Mode as _, PlayerIdentifier as _, Record as _, ServerIdentifier as _,
	},
	types::{steam_id::*, MapIdentifier, Mode, PlayerIdentifier, Runtype, ServerIdentifier, Tier},
};
