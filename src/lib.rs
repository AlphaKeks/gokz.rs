#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::correctness, clippy::perf)]
#![warn(rust_2018_idioms, missing_debug_implementations)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]

pub mod error;
pub use error::{Error, Result};

pub mod steam_id;
pub use steam_id::SteamID;

pub mod mode;
pub use mode::Mode;

pub mod rank;
pub use rank::Rank;

pub mod runtype;
pub use runtype::Runtype;

pub mod tier;
pub use tier::Tier;

mod identifier;
pub(crate) use identifier::identifier;

pub mod map_identifier;
pub use map_identifier::MapIdentifier;

pub mod server_identifier;
pub use server_identifier::ServerIdentifier;

pub mod prelude;

pub(crate) mod macros;

#[cfg(test)]
mod test_setup;
