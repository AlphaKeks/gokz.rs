/// The base URL for all API requests.
pub const BASE_URL: &str = "https://kzgo.eu/api";

/// The `/completions` route
pub mod completions;
pub use completions::{CompletionCount, Completions};

/// The `/maps` route
pub mod maps;
pub use maps::Map;

/// The `/steam` route
pub mod steam;
pub use steam::User;

/// The `/servers` route
pub mod servers;
pub use servers::Server;

/// The `/wrs` routes
pub mod world_records;
pub use world_records::WorldRecord;
