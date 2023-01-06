// ignore   - %s/\/\/ \#\[/\#\[
// unignore - %s/\#\[ign/\/\/ \#\[ign

use gokz_rs::prelude::*;
use gokz_rs::KZGO;
use log::info;

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_map() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let response = KZGO::get_map("kz_lionharder", &client).await?;
	info!("{:#?}", response);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_completion() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let response = KZGO::get_completion_count(Mode::SimpleKZ, &client).await?;
	info!("{:#?}", response);

	Ok(())
}
