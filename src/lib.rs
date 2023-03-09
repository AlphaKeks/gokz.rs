//! Rust wrapper for [CS:GO KZ](https://forum.gokz.org/).
#![warn(rust_2018_idioms, missing_docs, missing_debug_implementations)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::correctness, clippy::perf)]

#[cfg(feature = "client")]
pub use reqwest::Client;

#[cfg(feature = "blocking_client")]
pub use reqwest::blocking::Client as BlockingClient;

mod error;
pub use error::{Error, Result};

mod steam_id;
pub use steam_id::{AccountType, AccountUniverse, SteamID};

mod mode;
pub use mode::Mode;

mod player_identifier;
pub use player_identifier::PlayerIdentifier;

mod map_identifier;
pub use map_identifier::MapIdentifier;

mod rank;
pub use rank::Rank;

mod tier;
pub use tier::Tier;
