//! Most of what you probably need from this crate.
//!
//! This module re-exports a bunch of types you probably want to use anyway and is meant for glob
//! imports.
//!
//! ```
//! use gokz_rs::prelude::*;
//! ```

pub use crate::{
	error::{Error, Result},
	map_identifier::MapIdentifier,
	mode::Mode,
	player_identifier::PlayerIdentifier,
	rank::Rank,
	runtype::Runtype,
	server_identifier::ServerIdentifier,
	steam_id::SteamID,
	tier::Tier,
};
