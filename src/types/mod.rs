/// GOKZ currently features 3 gamemodes you can play in. This module aims to model those modes with
/// convenient associated methods and trait implementations so you can work with them in any
/// context.
pub mod mode;
pub use mode::Mode;

/// A lot of functions in this crate take a "map" as input, but don't particularly care whether
/// it's a name or an id. This type makes interacting with a "map" much easier if it's not
/// important whether you have a name or an id.
pub mod map_identifier;
pub use map_identifier::MapIdentifier;

/// A lot of functions in this crate take a "server" as input, but don't particularly care whether
/// it's a name or an id. This type makes interacting with a "server" much easier if it's not
/// important whether you have a name or an id.
pub mod server_identifier;
pub use server_identifier::ServerIdentifier;

/// Every player who joined a GOKZ server since 3.0.0 has a Rank assigned to them. This module
/// holds a [`Rank`] type which represents that ingame rank.
#[cfg(feature = "global-api")]
pub mod rank;
#[cfg(feature = "global-api")]
pub use rank::Rank;

/// Every global map has a difficulty associated with it, which is called a "tier".
pub mod tier;
pub use tier::Tier;

/// Every record in KZ is done either with or without teleports. The [`Runtype`] struct
/// encapsulates these two types of records.
pub mod runtype;
pub use runtype::Runtype;

/// Every steam user has a unique identifier called a "steam id". This wrapper type provides a lot
/// of useful methods and validation when working with those ids.
pub mod steam_id;
pub use steam_id::{AccountType, AccountUniverse, SteamID};

/// A lot of functions in this crate take a "player" as input, but don't particularly care whether
/// it's a name or an id. This type makes interacting with a "player" much easier if it's not
/// important whether you have a name or a SteamID.
pub mod player_identifier;
pub use player_identifier::PlayerIdentifier;
