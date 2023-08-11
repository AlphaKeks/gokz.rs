use {color_eyre::Result, gokz_rs::global_api, std::time::Duration};

#[tokio::test]
async fn healthcheck() -> Result<()> {
	global_api::healthcheck(&crate::GOKZ_CLIENT).await?;
	std::thread::sleep(Duration::from_millis(500));
	Ok(())
}
