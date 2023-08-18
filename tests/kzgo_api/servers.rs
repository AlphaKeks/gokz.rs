use {color_eyre::Result, gokz_rs::kzgo_api};

#[tokio::test]
async fn get_servers() -> Result<()> {
	kzgo_api::get_servers(&crate::GOKZ_CLIENT).await?;
	Ok(())
}
