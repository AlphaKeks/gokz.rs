use gokz_rs::extra;
use gokz_rs::prelude::*;
use log::info;

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_uncompleted() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let alphakeks = PlayerIdentifier::from(76561198282622073);

	let uncompleted =
		extra::get_unfinished(&alphakeks, Mode::SimpleKZ, false, Some(Tier::Death), &client)
			.await?;

	info!("{:#?}", uncompleted);
	info!("{} maps", uncompleted.len());

	Ok(())
}
