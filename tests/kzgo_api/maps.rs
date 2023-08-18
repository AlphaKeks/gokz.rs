use {color_eyre::Result, gokz_rs::kzgo_api, serde_json::json};

#[tokio::test]
async fn get_map() -> Result<()> {
	let lionharder = kzgo_api::get_map("kz_lionharder", &crate::GOKZ_CLIENT).await?;

	let expected = json!({
		"id": 992,
		"name": "kz_lionharder",
		"tier": 7,
		"workshopId": "2420807980",
		"bonuses": 2,
		"sp": true,
		"vp": false,
		"mapperNames": [
			"iBUYFL0WER Birgit"
		],
		"mapperIds": [
			"76561198078014747"
		],
		"date": "2021-06-05T15:52:16"
	});

	let expected = serde_json::from_value::<kzgo_api::Map>(expected)?;

	assert_eq!(lionharder, expected);

	Ok(())
}

#[tokio::test]
async fn get_maps() -> Result<()> {
	let maps = kzgo_api::get_maps(&crate::GOKZ_CLIENT).await?;

	assert!(!maps.is_empty());

	Ok(())
}
