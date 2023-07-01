#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms, missing_debug_implementations)]
#![deny(missing_docs)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::correctness, clippy::perf)]

#[cfg(feature = "serde")]
pub(crate) mod utils;

#[cfg(feature = "reqwest")]
pub(crate) mod http;

/// Some basics you will probably need when working with this crate.
pub mod prelude;

/// Anything related to errors in this crate.
pub mod error;

/// All the custom KZ-related types exposed by this crate.
pub mod types;

/// Various extension traits for types in the [`types`] module.
pub mod traits;

/// Functions and types related to the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).
#[cfg(feature = "global-api")]
pub mod global_api;

/// Functions and types related to the [KZ:GO API](https://kzgo.eu/).
#[cfg(feature = "kzgo-api")]
pub mod kzgo_api;

/// Functions and types related to the [SchnoseAPI](https://schnose.xyz/api).
#[cfg(feature = "schnose-api")]
pub mod schnose_api;

#[cfg(feature = "reqwest")]
pub use reqwest::{self, Client};
