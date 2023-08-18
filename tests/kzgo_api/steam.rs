use {
	color_eyre::Result,
	gokz_rs::{kzgo_api, SteamID},
};

#[tokio::test]
async fn get_user() -> Result<()> {
	let steam_id = SteamID::try_from(76561198282622073_u64)?;
	let user = kzgo_api::get_user(steam_id, &crate::GOKZ_CLIENT).await?;

	assert_eq!(user.name, "AlphaKeks");

	Ok(())
}
