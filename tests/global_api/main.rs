use {gokz_rs::http::Client, lazy_regex::Lazy};

static GOKZ_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub mod health;

#[ctor::ctor]
fn setup() {
	color_eyre::install().expect("Failed to setup color-eyre");
}
