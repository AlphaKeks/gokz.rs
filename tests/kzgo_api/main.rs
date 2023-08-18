#![cfg(feature = "kzgo-api")]

use {gokz_rs::http::Client, lazy_regex::Lazy};

static GOKZ_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub mod maps;
pub mod servers;
pub mod completions;
pub mod world_records;
pub mod steam;
