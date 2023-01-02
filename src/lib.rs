//! A Rust wrapper for the [CS:GO KZ GlobalAPI]().
//!
//! This crate provides a bunch of utility functions to communicate with KZ's GlobalAPI.
pub mod prelude;

mod global_api;
pub use global_api::GlobalAPI;

pub use reqwest::Client;
