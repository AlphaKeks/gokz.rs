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

pub mod prelude;

pub(crate) mod macros;

#[cfg(test)]
mod test_setup;
