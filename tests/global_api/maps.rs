use {color_eyre::Result, gokz_rs::global_api};

#[tokio::test]
async fn get_maps() -> Result<()> {
	let maps = global_api::get_maps(true, &crate::GOKZ_CLIENT).await?;
	assert!(!maps.is_empty());
	Ok(())
}
