use {color_eyre::Result, gokz_rs::global_api};

#[tokio::test]
async fn healthcheck() -> Result<()> {
	global_api::healthcheck(&crate::GOKZ_CLIENT).await?;
	Ok(())
}
