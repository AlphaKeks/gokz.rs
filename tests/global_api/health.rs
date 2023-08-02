use {color_eyre::Result, gokz_rs::global_api};

#[tokio::test]
async fn healthcheck() -> Result<()> {
	let health = global_api::healthcheck(&crate::GOKZ_CLIENT).await?;
	eprintln!("{health:#?}");
	Ok(())
}
