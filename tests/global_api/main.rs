#![cfg(feature = "global-api")]

use {gokz_rs::http::Client, lazy_regex::Lazy};

static GOKZ_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub mod health;
pub mod bans;
pub mod maps;
pub mod players;
pub mod record_filters;
pub mod servers;
pub mod records;

#[ctor::ctor]
fn setup() {
	color_eyre::install().expect("Failed to setup color-eyre");
}
