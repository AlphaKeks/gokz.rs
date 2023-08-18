//! This module contains various functions and submodules covering [KZ:GO](https://kzgo.eu/)'s API.

/// The base URL for all API requests.
pub const API_URL: &str = "https://kzgo.eu/api";

pub mod maps;
pub use maps::{get_map, get_maps, Map};

pub mod servers;
pub use servers::{get_servers, Server};

pub mod completions;
pub use completions::{get_completions, CompletionCount, CompletionStats};

pub mod world_records;
pub use world_records::{get_world_records, WorldRecord};

pub mod steam;
pub use steam::{get_user, User};
