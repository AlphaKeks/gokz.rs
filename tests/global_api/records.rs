use {
	color_eyre::Result,
	gokz_rs::{
		global_api::{self, Record},
		Mode,
	},
	serde_json::json,
};

#[tokio::test]
async fn get_record() -> Result<()> {
	let record_id = 14202658;
	let record = global_api::get_record(record_id, &crate::GOKZ_CLIENT).await?;
	let expected = json!({
		"id": 14202658,
		"steamid64": "76561198282622073",
		"player_name": "AlphaKeks",
		"steam_id": "STEAM_1:1:161178172",
		"server_id": 664,
		"map_id": 992,
		"stage": 0,
		"mode": "kz_simple",
		"tickrate": 128,
		"time": 1424.766,
		"teleports": 0,
		"created_on": "2021-11-03T15:09:11",
		"updated_on": "2021-11-03T15:09:11",
		"updated_by": 0,
		"record_filter_id": 0,
		"server_name": "Loaf of Bread #6 VIP/30%",
		"map_name": "kz_lionharder",
		"points": 0,
		"replay_id": 0
	});

	let expected = serde_json::from_value::<Record>(expected)?;

	assert_eq!(record, expected);
	Ok(())
}

#[tokio::test]
async fn get_place() -> Result<()> {
	let record_id = 14202658;
	let place = global_api::get_place(record_id, &crate::GOKZ_CLIENT).await?;

	assert!(place > 0);

	Ok(())
}

#[tokio::test]
async fn get_wr() -> Result<()> {
	global_api::get_wr("kz_lionharder", 0, Mode::SimpleKZ, true, &crate::GOKZ_CLIENT).await?;

	Ok(())
}

#[tokio::test]
async fn get_pb() -> Result<()> {
	let record = global_api::get_pb(
		"AlphaKeks",
		"kz_lionharder",
		0,
		Mode::SimpleKZ,
		true,
		&crate::GOKZ_CLIENT,
	)
	.await?;

	let expected = json!({
		"id": 13469730,
		"steamid64": "76561198282622073",
		"player_name": "AlphaKeks",
		"steam_id": "STEAM_1:1:161178172",
		"server_id": 538,
		"map_id": 992,
		"stage": 0,
		"mode": "kz_simple",
		"tickrate": 128,
		"time": 598.898,
		"teleports": 153,
		"created_on": "2021-08-25T23:39:29",
		"updated_on": "2021-08-25T23:39:29",
		"updated_by": 0,
		"record_filter_id": 0,
		"server_name": "Loaf of Bread #7 VIP/50%",
		"map_name": "kz_lionharder",
		"points": 935,
		"replay_id": 0
	});

	let expected = serde_json::from_value::<Record>(expected)?;

	assert_eq!(record, expected);
	Ok(())
}

#[tokio::test]
async fn get_maptop() -> Result<()> {
	let maptop = global_api::get_maptop(
		"kz_beginnerblock_go",
		0,
		Mode::KZTimer,
		true,
		100,
		&crate::GOKZ_CLIENT,
	)
	.await?;

	assert_eq!(maptop.len(), 100);
	Ok(())
}

#[tokio::test]
async fn get_wr_leaderboard() -> Result<()> {
	global_api::get_wr_leaderboard(0..=0, Mode::KZTimer, false, 1, &crate::GOKZ_CLIENT).await?;

	Ok(())
}
