use {
	color_eyre::{eyre::Context, Result},
	gokz_rs::{global_api, global_api::Player, SteamID},
};

#[tokio::test]
async fn get_players() -> Result<()> {
	let steam_id = SteamID::try_from(76561198282622073_u64)?;
	let player = global_api::get_player(steam_id, &crate::GOKZ_CLIENT).await?;

	assert_eq!(player.name, "AlphaKeks");
	assert_eq!(player.steam_id.to_string(), "STEAM_1:1:161178172");
	assert!(!player.is_banned);

	Ok(())
}

#[tokio::test]
#[ignore = "requires `STEAM_WEB_API_KEY` environment variable"]
async fn get_player_avatar() -> Result<()> {
	let player = Player {
		name: String::from("AlphaKeks"),
		steam_id: SteamID::try_from(76561198282622073_u64)?,
		is_banned: false,
	};

	let api_key = std::env::var("STEAM_WEB_API_KEY")
		.context("This test requires the `STEAM_WEB_API_KEY` environment variable to be set")?;

	player
		.avatar_url(&api_key, &crate::GOKZ_CLIENT)
		.await?;

	Ok(())
}
