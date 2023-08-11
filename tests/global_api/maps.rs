use {color_eyre::Result, gokz_rs::global_api, std::time::Duration};

#[tokio::test]
async fn get_maps() -> Result<()> {
	let maps = global_api::get_maps(true, &crate::GOKZ_CLIENT).await?;
	assert!(!maps.is_empty());
	std::thread::sleep(Duration::from_millis(500));
	Ok(())
}
