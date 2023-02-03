//! A Rust wrapper for the [CS:GO KZ GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
//!
//! This crate provides a bunch of utility functions to communicate with KZ's [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2), as well as a bunch of useful Types in the [`prelude`].

pub mod prelude;

mod global_api;
pub use global_api::{
	bans, jumpstats, maps, modes, players, record_filters, records, servers, GlobalAPI,
	GlobalAPIParams, GlobalAPIResponse,
};

pub mod kzgo;

pub mod extra;

pub use reqwest::Client;
