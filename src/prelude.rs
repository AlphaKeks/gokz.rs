#[rustfmt::skip]
#[cfg(feature = "reqwest")]
pub use reqwest::Client;

#[rustfmt::skip]
#[cfg(feature = "global-api")]
pub use crate::types::Rank;

#[rustfmt::skip]
pub use crate::{
	error::{Error, Result},
	types::{steam_id::*, MapIdentifier, Mode, PlayerIdentifier, ServerIdentifier, Tier, Runtype},
};
