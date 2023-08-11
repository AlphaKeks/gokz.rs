use {
	color_eyre::Result,
	gokz_rs::{global_api, SteamID},
	std::time::Duration,
};

#[tokio::test]
async fn get_bans() -> Result<()> {
	let alphakeks = unsafe { SteamID::new_unchecked(76561198282622073) };
	let params = global_api::bans::Params { steam_id: Some(alphakeks), ..Default::default() };

	let bans = global_api::get_bans_with(&params, &crate::GOKZ_CLIENT).await?;
	assert_eq!(bans.len(), 1);
	std::thread::sleep(Duration::from_millis(500));
	Ok(())
}
