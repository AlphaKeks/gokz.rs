/// Extension trait for any type that is a "KZ Mode".
pub mod mode;
pub use mode::Mode;

/// Extension trait for any type that is a "KZ Map".
pub mod map_identifier;
pub use map_identifier::MapIdentifier;

/// Extension trait for any type that is a "KZ Server".
pub mod server_identifier;
pub use server_identifier::ServerIdentifier;

/// Extension trait for any type that is a "KZ Player".
pub mod player_identifier;
pub use player_identifier::PlayerIdentifier;

/// Extension trait for any type that is a "KZ Record".
pub mod record;
pub use record::Record;
