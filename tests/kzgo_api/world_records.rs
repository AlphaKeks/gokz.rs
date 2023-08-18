use {
	color_eyre::Result,
	gokz_rs::{kzgo_api, Mode},
};

#[tokio::test]
async fn get_world_records() -> Result<()> {
	kzgo_api::get_world_records(Mode::KZTimer, &crate::GOKZ_CLIENT).await?;
	Ok(())
}
