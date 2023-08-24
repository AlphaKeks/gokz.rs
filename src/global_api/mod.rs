//! This module contains various functions and submodules covering the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

/// The base URL for all API requests.
pub const API_URL: &str = "https://kztimerglobal.com/api/v2";

/// The URL for the API's SwaggerUI website.
pub const SWAGGER_URL: &str = "https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2";

pub mod health;
pub use health::{healthcheck, Health};

pub mod bans;
pub use bans::{get_bans_with, Ban};

pub mod maps;
pub use maps::{get_map, get_maps, get_maps_with, Map};

pub mod servers;
pub use servers::{get_server, get_servers_owned_by, get_servers_with, Server};

pub mod players;
pub use players::{get_player, get_players_with, Player};

pub mod filters;
pub use filters::{get_filters_with, RecordFilter};

pub mod records;
pub use records::{
	get_maptop, get_pb, get_place, get_record, get_records_with, get_wr,
	world_records::{self, get_wr_leaderboard},
	Record,
};
